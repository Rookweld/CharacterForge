//! Closed, data-only portable project packages and recoverable local restore.

use forge_storage::{
    CURRENT_SCHEMA_VERSION, ProjectSession, ProjectStore, StoreError, StoredEntity, StoredProject,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    error::Error,
    fmt, fs,
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
use zip::{CompressionMethod, ZipArchive, ZipWriter, write::SimpleFileOptions};

pub const FORMAT_VERSION: u32 = 1;
pub const MANIFEST_MEMBER: &str = "manifest.json";
pub const DATABASE_MEMBER: &str = "project.sqlite3";
pub const MAX_ARCHIVE_BYTES: u64 = 2 * 1024 * 1024 * 1024;
pub const MAX_DATABASE_BYTES: u64 = 1024 * 1024 * 1024;
const MAX_MANIFEST_BYTES: u64 = 64 * 1024;
const MAX_COMPRESSION_RATIO: u64 = 200;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemberIntegrity {
    pub name: String,
    pub size: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub format: String,
    pub format_version: u32,
    pub application: String,
    pub application_version: String,
    pub project_id: String,
    pub project_name: String,
    pub database_schema_version: u16,
    pub generator_version: u16,
    pub content_version: u16,
    pub exported_at_unix_ms: u64,
    pub members: Vec<MemberIntegrity>,
}

#[derive(Debug)]
pub enum ExportError {
    Io(std::io::Error),
    Archive(String),
    Invalid(String),
    UnsupportedFormat(u32),
    UnsupportedDatabaseSchema(u32),
    Integrity,
    DestinationExists,
    Storage(String),
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "file operation failed: {error}"),
            Self::Archive(error) => write!(f, "invalid Forge project archive: {error}"),
            Self::Invalid(error) => write!(f, "invalid Forge project: {error}"),
            Self::UnsupportedFormat(version) => {
                write!(f, "unsupported portable format version {version}")
            }
            Self::UnsupportedDatabaseSchema(version) => {
                write!(f, "unsupported SQLite schema version {version}")
            }
            Self::Integrity => write!(f, "portable project integrity check failed"),
            Self::DestinationExists => write!(f, "a file already exists at the chosen destination"),
            Self::Storage(error) => write!(f, "project database validation failed: {error}"),
        }
    }
}
impl Error for ExportError {}
impl From<std::io::Error> for ExportError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

/// Creates a consistent SQLite snapshot and atomically finalizes the portable archive.
pub fn export_project(
    store: &ProjectStore,
    destination: impl AsRef<Path>,
) -> Result<(), ExportError> {
    let destination = destination.as_ref();
    if destination.exists() {
        return Err(ExportError::DestinationExists);
    }
    let project = store
        .load_project()
        .map_err(storage)?
        .ok_or_else(|| ExportError::Invalid("the database contains no project".into()))?;
    let staging = Staging::new("forge-export")?;
    let snapshot = staging.path.join(DATABASE_MEMBER);
    store.snapshot_into(&snapshot).map_err(storage)?;
    validate_database(&snapshot, Some(&project))?;
    let db_size = fs::metadata(&snapshot)?.len();
    if db_size > MAX_DATABASE_BYTES {
        return Err(ExportError::Invalid(
            "project database exceeds the portable size limit".into(),
        ));
    }
    let digest = hash_file(&snapshot)?;
    let manifest = Manifest {
        format: "forgeproject".into(),
        format_version: FORMAT_VERSION,
        application: "Character Forge / Quest Forge".into(),
        application_version: env!("CARGO_PKG_VERSION").into(),
        project_id: project.envelope.id.uuid().to_string(),
        project_name: project.name.clone(),
        database_schema_version: project.envelope.schema_version.0,
        generator_version: project.envelope.generator_version.0,
        content_version: project.envelope.content_version.0,
        exported_at_unix_ms: now_ms()?,
        members: vec![MemberIntegrity {
            name: DATABASE_MEMBER.into(),
            size: db_size,
            blake3: digest,
        }],
    };
    let manifest_bytes =
        serde_json::to_vec(&manifest).map_err(|e| ExportError::Invalid(e.to_string()))?;
    let temp = unique_sibling(destination, "partial")?;
    let result = (|| {
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        let mut zip = ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
        zip.start_file(MANIFEST_MEMBER, options).map_err(archive)?;
        zip.write_all(&manifest_bytes)?;
        zip.start_file(DATABASE_MEMBER, options).map_err(archive)?;
        let mut input = fs::File::open(&snapshot)?;
        std::io::copy(&mut input, &mut zip)?;
        let file = zip.finish().map_err(archive)?;
        file.sync_all()?;
        if fs::metadata(&temp)?.len() > MAX_ARCHIVE_BYTES {
            return Err(ExportError::Invalid(
                "portable archive exceeds the size limit".into(),
            ));
        }
        finalize_new_file(&temp, destination)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

/// Validates and imports to a new database path. Existing destinations are never overwritten.
pub fn import_project(
    archive_path: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> Result<StoredProject, ExportError> {
    let destination = destination.as_ref();
    if destination.exists() {
        return Err(ExportError::DestinationExists);
    }
    let staging = Staging::new("forge-import")?;
    let database = staging.path.join(DATABASE_MEMBER);
    let project = validate_archive_to_database(archive_path.as_ref(), &database)?;
    let parent = destination.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    copy_finalized_new_file(&database, destination)?;
    Ok(project)
}

/// Imports a portable project into a folder using a safe filename derived from its display name.
/// The project display name in the database is retained independently of that filename.
pub fn import_project_to_directory(
    archive_path: impl AsRef<Path>,
    directory: impl AsRef<Path>,
) -> Result<(StoredProject, PathBuf), ExportError> {
    let directory = directory.as_ref();
    fs::create_dir_all(directory)?;
    if !directory.is_dir() {
        return Err(ExportError::Invalid(
            "the selected project location is not a folder".into(),
        ));
    }
    let directory = fs::canonicalize(directory)?;
    let staging = Staging::new("forge-import")?;
    let database = staging.path.join(DATABASE_MEMBER);
    let project = validate_archive_to_database(archive_path.as_ref(), &database)?;
    let destination = project_database_path(&directory, &project.name)?;
    if destination.exists() {
        return Err(ExportError::DestinationExists);
    }
    copy_finalized_new_file(&database, &destination)?;
    Ok((project, destination))
}

/// Builds a Windows-safe `.sqlite3` filename without changing the visible project name.
pub fn project_database_path(
    directory: impl AsRef<Path>,
    name: &str,
) -> Result<PathBuf, ExportError> {
    if name.trim().is_empty() {
        return Err(ExportError::Invalid("project name is empty".into()));
    }
    let mut stem = String::new();
    let mut previous_was_separator = false;
    for character in name.trim().chars() {
        let invalid = character.is_control() || "<>:\"/\\|?*".contains(character);
        if invalid {
            if !previous_was_separator {
                stem.push('_');
            }
            previous_was_separator = true;
        } else {
            stem.push(character);
            previous_was_separator = false;
        }
    }
    let stem = stem.trim().trim_end_matches(['.', ' ']).trim();
    if stem.is_empty() {
        return Err(ExportError::Invalid(
            "project name cannot form a usable filename".into(),
        ));
    }
    let base = stem.split('.').next().unwrap_or(stem).to_ascii_uppercase();
    let numbered_device = ["COM", "LPT"].iter().any(|prefix| {
        base.strip_prefix(prefix)
            .is_some_and(|number| number.len() == 1 && matches!(number.as_bytes()[0], b'1'..=b'9'))
    });
    let stem = if matches!(base.as_str(), "CON" | "PRN" | "AUX" | "NUL") || numbered_device {
        format!("_{stem}")
    } else {
        stem.to_owned()
    };
    Ok(directory.as_ref().join(format!("{stem}.sqlite3")))
}

pub fn create_backup(
    store: &ProjectStore,
    destination: impl AsRef<Path>,
) -> Result<(), ExportError> {
    export_project(store, destination)
}

#[derive(Serialize)]
struct EntityExportDocument<'a> {
    format: &'static str,
    format_version: u32,
    entity: &'a StoredEntity,
}

/// Returns a versioned structured export retaining the complete canonical entity payload.
pub fn entity_json(entity: &StoredEntity) -> Result<Vec<u8>, ExportError> {
    serde_json::to_vec_pretty(&EntityExportDocument {
        format: "forge-entity",
        format_version: 1,
        entity,
    })
    .map_err(|error| ExportError::Invalid(error.to_string()))
}

/// Returns a readable Markdown presentation followed by the complete canonical JSON payload.
pub fn entity_markdown(entity: &StoredEntity) -> Result<String, ExportError> {
    let encoded = serde_json::to_string_pretty(entity)
        .map_err(|error| ExportError::Invalid(error.to_string()))?;
    let ticks = "`".repeat(encoded.matches('`').count().max(3) + 1);
    let (kind, envelope, dna) = match entity {
        StoredEntity::CharacterCandidate(value) => {
            ("Character", &value.character.envelope, Some(&value.dna))
        }
        StoredEntity::QuestCandidate(value) => ("Quest", &value.quest.envelope, Some(&value.dna)),
        StoredEntity::Character(value) => (
            "Character",
            &value.envelope,
            value
                .envelope
                .generation_provenance
                .as_ref()
                .map(|p| &p.dna),
        ),
        StoredEntity::Quest(value) => (
            "Quest",
            &value.envelope,
            value
                .envelope
                .generation_provenance
                .as_ref()
                .map(|p| &p.dna),
        ),
    };
    let preset = dna
        .map(|value| value.recipe.universe_preset.as_str())
        .unwrap_or("not recorded");
    let seed = dna
        .map(|value| value.recipe.root_seed.to_string())
        .unwrap_or_else(|| "not recorded".into());
    Ok(format!(
        "# {kind}: {}\n\n- Project ID: `{}`\n- Entity ID: `{}`\n- State: {:?}\n- Preset: `{}`\n- Seed: `{}`\n\n## Complete canonical data\n\n{ticks}json\n{encoded}\n{ticks}\n",
        escape_markdown(&envelope.name),
        envelope.project_id.uuid(),
        envelope.id.uuid(),
        envelope.canon_state,
        preset,
        seed
    ))
}

fn escape_markdown(text: &str) -> String {
    text.chars()
        .map(|ch| match ch {
            '\r' | '\n' => " ".into(),
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '.'
            | '!' | '|' => format!("\\{ch}"),
            '<' => "&lt;".into(),
            '>' => "&gt;".into(),
            _ => ch.to_string(),
        })
        .collect()
}

pub fn write_entity_json(
    entity: &StoredEntity,
    destination: impl AsRef<Path>,
) -> Result<(), ExportError> {
    atomic_write(&entity_json(entity)?, destination.as_ref())
}
pub fn write_entity_markdown(
    entity: &StoredEntity,
    destination: impl AsRef<Path>,
) -> Result<(), ExportError> {
    atomic_write(entity_markdown(entity)?.as_bytes(), destination.as_ref())
}
fn atomic_write(bytes: &[u8], destination: &Path) -> Result<(), ExportError> {
    if destination.exists() {
        return Err(ExportError::DestinationExists);
    }
    let temp = unique_sibling(destination, "entity-export")?;
    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        finalize_new_file(&temp, destination)
    })();
    if result.is_err() {
        let _ = fs::remove_file(temp);
    }
    result
}

/// Restores a validated package to a closed destination, retaining a recoverable pre-restore copy.
pub fn restore_backup(
    archive_path: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> Result<StoredProject, ExportError> {
    let destination = destination.as_ref();
    if !destination.exists() {
        return import_project(archive_path, destination);
    }
    restore_existing_with(archive_path.as_ref(), destination, validate_database)
}

fn restore_existing_with(
    archive_path: &Path,
    destination: &Path,
    validate: impl FnOnce(&Path, Option<&StoredProject>) -> Result<(), ExportError>,
) -> Result<StoredProject, ExportError> {
    let staging = Staging::new("forge-restore")?;
    let restored = staging.path.join("restored.sqlite3");
    let project = validate_archive_to_database(archive_path, &restored)?;
    restore_staged(&restored, destination, &project, validate)?;
    Ok(project)
}

fn restore_staged(
    restored: &Path,
    destination: &Path,
    expected: &StoredProject,
    validate: impl FnOnce(&Path, Option<&StoredProject>) -> Result<(), ExportError>,
) -> Result<(), ExportError> {
    let before = ProjectStore::open(destination).map_err(storage)?;
    before.integrity_check().map_err(storage)?;
    let backup = unique_sibling(destination, "pre-restore")?;
    before.snapshot_into(&backup).map_err(storage)?;
    drop(before);
    let replacement = unique_sibling(destination, "replacement")?;
    fs::copy(restored, &replacement)?;
    let displaced = unique_sibling(destination, "displaced")?;
    if let Err(error) = fs::rename(destination, &displaced) {
        let _ = fs::remove_file(&replacement);
        let _ = fs::remove_file(&backup);
        return Err(error.into());
    }
    for suffix in ["-wal", "-shm"] {
        let sidecar = PathBuf::from(format!("{}{}", destination.to_string_lossy(), suffix));
        if sidecar.exists()
            && let Err(error) = fs::remove_file(sidecar)
        {
            if rollback_file(&backup, &displaced, destination).is_ok() {
                let _ = fs::remove_file(&displaced);
            }
            let _ = fs::remove_file(&replacement);
            let _ = fs::remove_file(&backup);
            return Err(error.into());
        }
    }
    if let Err(error) = fs::rename(&replacement, destination) {
        if rollback_file(&backup, &displaced, destination).is_ok() {
            let _ = fs::remove_file(&displaced);
        }
        let _ = fs::remove_file(&replacement);
        let _ = fs::remove_file(&backup);
        return Err(error.into());
    }
    match validate(destination, Some(expected)) {
        Ok(()) => {
            let _ = fs::remove_file(&displaced);
            let _ = fs::remove_file(&backup);
            Ok(())
        }
        Err(error) => {
            if let Err(rollback) = rollback_file(&backup, &displaced, destination) {
                return Err(ExportError::Storage(format!(
                    "restore failed ({error}); original recovery file remains at {} after rollback error: {rollback}",
                    displaced.display()
                )));
            }
            let _ = fs::remove_file(&displaced);
            let _ = fs::remove_file(&backup);
            Err(ExportError::Storage(format!(
                "restore failed; previous project was recovered: {error}"
            )))
        }
    }
}

fn rollback_file(snapshot: &Path, displaced: &Path, destination: &Path) -> std::io::Result<()> {
    let recovery = unique_sibling(destination, "rollback")
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    let restore = (|| {
        if destination.exists() {
            fs::remove_file(destination)?;
        }
        fs::copy(snapshot, &recovery)?;
        fs::rename(&recovery, destination)
    })();
    if restore.is_err() {
        let _ = fs::remove_file(&recovery);
        if !destination.exists() {
            let _ = fs::rename(displaced, destination);
        }
    }
    restore
}

/// Closes the active SQLite handle before restoring, then reopens the project after success.
pub fn restore_active(
    session: &mut ProjectSession,
    archive: impl AsRef<Path>,
) -> Result<StoredProject, ExportError> {
    let path = session
        .active()
        .ok_or_else(|| ExportError::Invalid("no active project".into()))?
        .path()
        .to_path_buf();
    session.close();
    match restore_existing_with(archive.as_ref(), &path, |database, expected| {
        validate_database(database, expected)?;
        session.open(database).map_err(storage)?;
        Ok(())
    }) {
        Ok(project) => Ok(project),
        Err(error) => {
            let _ = session.open(path);
            Err(error)
        }
    }
}

fn validate_archive_to_database(
    path: &Path,
    database: &Path,
) -> Result<StoredProject, ExportError> {
    let size = fs::metadata(path)?.len();
    if size > MAX_ARCHIVE_BYTES {
        return Err(ExportError::Invalid(
            "archive exceeds the input size limit".into(),
        ));
    }
    let file = fs::File::open(path)?;
    let mut archive = ZipArchive::new(file).map_err(archive_error)?;
    if archive.len() > 2 {
        return Err(ExportError::Invalid(
            "archive must contain exactly two members".into(),
        ));
    }
    let mut names = BTreeSet::new();
    let mut manifest_data = None;
    let mut database_index = None;
    for index in 0..archive.len() {
        let entry = archive.by_index(index).map_err(archive_error)?;
        let raw_name = std::str::from_utf8(entry.name_raw())
            .map_err(|_| ExportError::Invalid("archive contains a non-UTF8 path".into()))?;
        validate_member_path(raw_name)?;
        if !names.insert(raw_name.to_ascii_lowercase()) {
            return Err(ExportError::Invalid(
                "archive contains duplicate or ambiguous member names".into(),
            ));
        }
        if entry.is_dir()
            || entry
                .unix_mode()
                .is_some_and(|mode| mode & 0o170000 != 0 && mode & 0o170000 != 0o100000)
        {
            return Err(ExportError::Invalid(
                "archive contains an unsupported special file".into(),
            ));
        }
        match raw_name {
            MANIFEST_MEMBER if entry.size() <= MAX_MANIFEST_BYTES => {
                manifest_data = Some((index, entry.size()))
            }
            DATABASE_MEMBER if entry.size() <= MAX_DATABASE_BYTES => {
                database_index = Some((index, entry.size(), entry.compressed_size()))
            }
            MANIFEST_MEMBER | DATABASE_MEMBER => {
                return Err(ExportError::Invalid(
                    "archive member exceeds its size limit".into(),
                ));
            }
            _ => {
                return Err(ExportError::Invalid(
                    "archive contains an unexpected member".into(),
                ));
            }
        }
    }
    let (manifest_index, manifest_size) =
        manifest_data.ok_or_else(|| ExportError::Invalid("manifest is missing".into()))?;
    let (database_index, database_size, compressed_size) =
        database_index.ok_or_else(|| ExportError::Invalid("project database is missing".into()))?;
    if archive.len() != 2 {
        return Err(ExportError::Invalid(
            "archive must contain exactly two members".into(),
        ));
    }
    if database_size
        > compressed_size
            .saturating_mul(MAX_COMPRESSION_RATIO)
            .max(1024 * 1024)
    {
        return Err(ExportError::Invalid(
            "archive compression ratio exceeds the limit".into(),
        ));
    }
    let mut bytes = Vec::with_capacity(manifest_size as usize);
    archive
        .by_index(manifest_index)
        .map_err(archive_error)?
        .read_to_end(&mut bytes)?;
    let manifest: Manifest = serde_json::from_slice(&bytes)
        .map_err(|e| ExportError::Invalid(format!("manifest is invalid: {e}")))?;
    if manifest.format != "forgeproject" {
        return Err(ExportError::Invalid(
            "manifest format identifier is not forgeproject".into(),
        ));
    }
    if manifest.format_version != FORMAT_VERSION {
        return Err(ExportError::UnsupportedFormat(manifest.format_version));
    }
    if manifest.members.len() != 1
        || manifest.members[0].name != DATABASE_MEMBER
        || manifest.members[0].size != database_size
    {
        return Err(ExportError::Invalid(
            "manifest member list does not match archive contents".into(),
        ));
    }
    {
        let mut input = archive.by_index(database_index).map_err(archive_error)?;
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(database)?;
        let mut hasher = blake3::Hasher::new();
        let mut copied = 0_u64;
        let mut buffer = [0_u8; 64 * 1024];
        loop {
            let count = input.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            copied = copied
                .checked_add(count as u64)
                .ok_or_else(|| ExportError::Invalid("member size overflow".into()))?;
            if copied > MAX_DATABASE_BYTES {
                return Err(ExportError::Invalid(
                    "project database exceeds the size limit".into(),
                ));
            }
            hasher.update(&buffer[..count]);
            output.write_all(&buffer[..count])?;
        }
        output.sync_all()?;
        if copied != database_size
            || hasher.finalize().to_hex().as_str() != manifest.members[0].blake3
        {
            return Err(ExportError::Integrity);
        }
    }
    let store = ProjectStore::open(database).map_err(storage)?;
    store.integrity_check().map_err(storage)?;
    let project = store
        .load_project()
        .map_err(storage)?
        .ok_or_else(|| ExportError::Invalid("database contains no project".into()))?;
    validate_manifest_project(&manifest, &project)?;
    Ok(project)
}

fn validate_manifest_project(
    manifest: &Manifest,
    project: &StoredProject,
) -> Result<(), ExportError> {
    let schema_compatible = manifest.database_schema_version == project.envelope.schema_version.0
        || (manifest.database_schema_version == 1
            && project.envelope.schema_version.0 == CURRENT_SCHEMA_VERSION as u16);
    if manifest.project_id != project.envelope.id.uuid().to_string()
        || manifest.project_name != project.name
        || !schema_compatible
        || manifest.database_schema_version > CURRENT_SCHEMA_VERSION as u16
        || manifest.generator_version != project.envelope.generator_version.0
        || manifest.content_version != project.envelope.content_version.0
    {
        return Err(ExportError::Invalid(
            "manifest metadata does not match project database".into(),
        ));
    }
    Ok(())
}

fn validate_database(path: &Path, expected: Option<&StoredProject>) -> Result<(), ExportError> {
    let store = ProjectStore::open(path).map_err(storage)?;
    store.integrity_check().map_err(storage)?;
    let project = store
        .load_project()
        .map_err(storage)?
        .ok_or_else(|| ExportError::Invalid("database contains no project".into()))?;
    if let Some(expected) = expected
        && project != *expected
    {
        return Err(ExportError::Invalid(
            "snapshot metadata changed during export".into(),
        ));
    }
    Ok(())
}

fn validate_member_path(name: &str) -> Result<(), ExportError> {
    if name.is_empty()
        || name.starts_with('/')
        || name.starts_with('\\')
        || name.contains('\\')
        || name.contains(':')
        || name
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
        || name.trim_end_matches([' ', '.']) != name
    {
        return Err(ExportError::Invalid(
            "archive contains an unsafe path".into(),
        ));
    }
    for component in name.split('/') {
        let stem = component
            .split('.')
            .next()
            .unwrap_or(component)
            .to_ascii_uppercase();
        if matches!(
            stem.as_str(),
            "CON"
                | "PRN"
                | "AUX"
                | "NUL"
                | "COM1"
                | "COM2"
                | "COM3"
                | "COM4"
                | "COM5"
                | "COM6"
                | "COM7"
                | "COM8"
                | "COM9"
                | "LPT1"
                | "LPT2"
                | "LPT3"
                | "LPT4"
                | "LPT5"
                | "LPT6"
                | "LPT7"
                | "LPT8"
                | "LPT9"
        ) {
            return Err(ExportError::Invalid(
                "archive contains a reserved Windows path".into(),
            ));
        }
    }
    Ok(())
}

struct Staging {
    path: PathBuf,
}
impl Staging {
    fn new(label: &str) -> Result<Self, ExportError> {
        for _ in 0..100 {
            let path = std::env::temp_dir().join(unique_name(label));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e.into()),
            }
        }
        Err(ExportError::Invalid(
            "could not create isolated staging directory".into(),
        ))
    }
}
impl Drop for Staging {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
static COUNTER: AtomicU64 = AtomicU64::new(0);
fn unique_name(label: &str) -> String {
    format!(
        "character-forge-{label}-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    )
}
fn unique_sibling(path: &Path, suffix: &str) -> Result<PathBuf, ExportError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    for _ in 0..100 {
        let candidate = parent.join(format!(
            ".{}.{}",
            unique_name(suffix),
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err(ExportError::Invalid(
        "could not allocate temporary file name".into(),
    ))
}
fn finalize_new_file(temp: &Path, destination: &Path) -> Result<(), ExportError> {
    match fs::hard_link(temp, destination) {
        Ok(()) => {
            fs::remove_file(temp)?;
            Ok(())
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            Err(ExportError::DestinationExists)
        }
        Err(error) => Err(error.into()),
    }
}
fn copy_finalized_new_file(source: &Path, destination: &Path) -> Result<(), ExportError> {
    if destination.exists() {
        return Err(ExportError::DestinationExists);
    }
    let temp = unique_sibling(destination, "import")?;
    let result = (|| {
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        std::io::copy(&mut fs::File::open(source)?, &mut output)?;
        output.sync_all()?;
        drop(output);
        finalize_new_file(&temp, destination)
    })();
    if result.is_err() {
        let _ = fs::remove_file(temp);
    }
    result
}
fn hash_file(path: &Path) -> Result<String, ExportError> {
    let mut file = fs::File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}
fn now_ms() -> Result<u64, ExportError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis().try_into().unwrap_or(u64::MAX))
        .map_err(|e| ExportError::Invalid(e.to_string()))
}
fn archive(error: impl fmt::Display) -> ExportError {
    ExportError::Archive(error.to_string())
}
fn archive_error(error: impl fmt::Display) -> ExportError {
    ExportError::Archive(error.to_string())
}
fn storage(error: StoreError) -> ExportError {
    match error {
        StoreError::UnsupportedSchema(version) => ExportError::UnsupportedDatabaseSchema(version),
        other => ExportError::Storage(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_app_core::{
        CharacterField, CharacterFieldValue, CharacterForge, QuestField, QuestFieldValue,
        QuestForge,
    };
    use forge_contracts::{
        ContentVersion, CreativeAxes, FieldKey, INITIAL_CONTENT_VERSION, INITIAL_GENERATOR_VERSION,
        ProjectEnvelope, ProjectId, RootSeed, SchemaVersion,
    };
    use std::collections::BTreeMap;
    use zip::write::SimpleFileOptions;

    struct Dir(PathBuf);
    impl Dir {
        fn new() -> Self {
            let path = std::env::temp_dir().join(unique_name("test"));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
        fn path(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }
    }
    impl Drop for Dir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn project() -> StoredProject {
        StoredProject {
            envelope: ProjectEnvelope {
                id: ProjectId::from_bytes([1; 16]),
                schema_version: SchemaVersion(CURRENT_SCHEMA_VERSION as u16),
                generator_version: INITIAL_GENERATOR_VERSION,
                content_version: INITIAL_CONTENT_VERSION,
            },
            name: "Vault Test World".into(),
            created_at_unix_ms: 1,
            updated_at_unix_ms: 1,
        }
    }
    fn populated(path: &Path) -> (ProjectId, StoredEntity, StoredEntity, StoredEntity) {
        let p = project();
        let id = p.envelope.id;
        let mut store = ProjectStore::open(path).unwrap();
        store.create_project(&p).unwrap();
        let dna = CharacterForge::new_dna(
            RootSeed::new(42),
            "dark-fantasy",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        let mut character = CharacterForge::generate_in_project(&dna, id).unwrap();
        character
            .manual_edit(
                CharacterField::Name,
                CharacterFieldValue::Text("A Manual Name".into()),
            )
            .unwrap();
        CharacterForge::lock(&mut character, CharacterField::Appearance).unwrap();
        let character = StoredEntity::CharacterCandidate(character);
        let cid = character.envelope().id;
        store.persist_candidate(character.clone()).unwrap();
        store.promote_candidate(cid, 1).unwrap();
        let character = store.get_entity(cid).unwrap().unwrap().entity;
        let qdna = QuestForge::new_dna(
            RootSeed::new(73),
            "pirate",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        let quest =
            StoredEntity::QuestCandidate(QuestForge::generate_in_project(&qdna, id).unwrap());
        store.persist_candidate(quest.clone()).unwrap();
        let mut legacy_dna = QuestForge::new_dna(
            RootSeed::new(74),
            "pirate",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        let clue = FieldKey::new("16-clue").unwrap();
        let briefing = FieldKey::new("17-briefing").unwrap();
        legacy_dna.recipe.field_controls.remove(&clue);
        legacy_dna.recipe.field_controls.remove(&briefing);
        legacy_dna.recipe.dependencies.remove(&clue);
        legacy_dna.recipe.dependencies.remove(&briefing);
        for dependencies in legacy_dna.recipe.dependencies.values_mut() {
            dependencies.remove(&clue);
            dependencies.remove(&briefing);
        }
        legacy_dna.content_version = ContentVersion(2);
        legacy_dna.normalize().unwrap();
        let legacy_quest =
            StoredEntity::QuestCandidate(QuestForge::generate_in_project(&legacy_dna, id).unwrap());
        store.persist_candidate(legacy_quest.clone()).unwrap();
        store
            .set_setting("vault.test", "\"committed immediately before export\"")
            .unwrap();
        (id, character, quest, legacy_quest)
    }

    #[test]
    fn real_project_roundtrip_preserves_identity_entities_history_locks_manual_edits_and_canon() {
        let dir = Dir::new();
        let source = dir.path("source.sqlite3");
        let package = dir.path("world.forgeproject");
        let second_package = dir.path("second.forgeproject");
        let destination = dir.path("imported.sqlite3");
        let (project_id, character, quest, legacy_quest) = populated(&source);
        let mut store = ProjectStore::open(&source).unwrap();
        let initial_entities = store.list_entities().unwrap();
        let initial_history = store.list_history(None).unwrap();
        store
            .set_setting("vault.wal", "\"saved while the project remained open\"")
            .unwrap();
        assert!(source.with_extension("sqlite3-wal").exists());
        export_project(&store, &package).unwrap();
        export_project(&store, &second_package).unwrap();
        assert_eq!(store.list_entities().unwrap(), initial_entities);
        assert_eq!(store.list_history(None).unwrap(), initial_history);
        let mut zip = ZipArchive::new(fs::File::open(&package).unwrap()).unwrap();
        assert_eq!(zip.len(), 2);
        let mut manifest_bytes = Vec::new();
        zip.by_name(MANIFEST_MEMBER)
            .unwrap()
            .read_to_end(&mut manifest_bytes)
            .unwrap();
        let manifest: Manifest = serde_json::from_slice(&manifest_bytes).unwrap();
        assert_eq!(manifest.format_version, FORMAT_VERSION);
        assert_eq!(manifest.project_id, project_id.uuid().to_string());
        assert_eq!(manifest.project_name, "Vault Test World");
        assert_eq!(manifest.database_schema_version, 2);
        assert_eq!(manifest.application, "Character Forge / Quest Forge");
        assert_eq!(manifest.members[0].name, DATABASE_MEMBER);
        assert!(source.with_extension("sqlite3-wal").exists() || source.exists());
        drop(store);
        let imported = import_project(&package, &destination).unwrap();
        assert_eq!(imported.envelope.id, project_id);
        let restored = ProjectStore::open(&destination).unwrap();
        assert_eq!(
            restored
                .get_entity(character.envelope().id)
                .unwrap()
                .unwrap()
                .entity,
            character
        );
        assert_eq!(
            restored
                .get_entity(quest.envelope().id)
                .unwrap()
                .unwrap()
                .entity,
            quest
        );
        assert_eq!(
            restored
                .get_entity(legacy_quest.envelope().id)
                .unwrap()
                .unwrap()
                .entity,
            legacy_quest
        );
        match &legacy_quest {
            StoredEntity::QuestCandidate(value) => {
                assert_eq!(value.dna.content_version.0, 2);
                assert!(value.clue.is_none() && value.briefing.is_none());
            }
            _ => unreachable!(),
        }
        match &quest {
            StoredEntity::QuestCandidate(value) => {
                assert_eq!(value.dna.content_version.0, 3);
                assert!(value.clue.is_some() && value.briefing.is_some());
            }
            _ => unreachable!(),
        }
        assert_eq!(
            restored.get_setting("vault.test").unwrap().as_deref(),
            Some("\"committed immediately before export\"")
        );
        assert_eq!(
            restored.get_setting("vault.wal").unwrap().as_deref(),
            Some("\"saved while the project remained open\"")
        );
        assert_eq!(restored.list_history(None).unwrap().len(), 3);
        assert_eq!(
            restored
                .list_revisions(character.envelope().id)
                .unwrap()
                .len(),
            1
        );
        restored.integrity_check().unwrap();
    }

    #[test]
    fn gauntlet_cross_feature_roundtrip_preserves_revision_history_unicode_and_restore() {
        let dir = Dir::new();
        let source = dir.path("source.sqlite3");
        let package = dir.path("complete.forgeproject");
        let destination = dir.path("imported.sqlite3");
        let backup = dir.path("pre-mutation.forgeproject");
        let (project_id, character, quest, _) = populated(&source);
        let character_id = character.envelope().id;
        let quest_id = quest.envelope().id;

        let mut store = ProjectStore::open(&source).unwrap();
        for revision in 1..=3 {
            let record = store.get_entity(character_id).unwrap().unwrap();
            let mut changed = record.entity;
            let StoredEntity::CharacterCandidate(candidate) = &mut changed else {
                panic!("Character candidate wrapper was lost");
            };
            candidate
                .manual_edit(
                    CharacterField::Fear,
                    CharacterFieldValue::Text(format!("Canon revision {revision} — Räven 🜁")),
                )
                .unwrap();
            store
                .revise_canon(
                    changed,
                    record.write_version,
                    &format!("Gauntlet Character canon {revision}"),
                )
                .unwrap();
        }

        let character_dna = CharacterForge::new_dna(
            RootSeed::new(73_001),
            "nordic-myth",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        let mut working_character =
            CharacterForge::generate_in_project(&character_dna, project_id).unwrap();
        let working_character_id = working_character.character.envelope.id;
        let mut session = forge_storage::SessionHistory::new(StoredEntity::CharacterCandidate(
            working_character.clone(),
        ));
        working_character
            .manual_edit(
                CharacterField::Name,
                CharacterFieldValue::Text(
                    "Eydis — Seattle, \"North\" \\ route\nsecond line".into(),
                ),
            )
            .unwrap();
        session.push(StoredEntity::CharacterCandidate(working_character.clone()));
        CharacterForge::lock(&mut working_character, CharacterField::Appearance).unwrap();
        let locked_snapshot = StoredEntity::CharacterCandidate(working_character.clone());
        session.push(locked_snapshot.clone());
        CharacterForge::reroll_field(&mut working_character, CharacterField::Fear).unwrap();
        session.push(StoredEntity::CharacterCandidate(working_character));
        let forward = session.current().clone();
        assert_eq!(session.undo(), Some(&locked_snapshot));
        assert_eq!(session.redo(), Some(&forward));
        let mut saved_character = match forward {
            StoredEntity::CharacterCandidate(candidate) => candidate,
            _ => unreachable!(),
        };
        let mut working_character_version = store
            .persist_candidate(StoredEntity::CharacterCandidate(saved_character.clone()))
            .unwrap();
        saved_character
            .manual_edit(
                CharacterField::Title,
                CharacterFieldValue::Text("Second saved Character revision".into()),
            )
            .unwrap();
        working_character_version = store
            .save_candidate(
                StoredEntity::CharacterCandidate(saved_character.clone()),
                working_character_version,
            )
            .unwrap();
        working_character_version = store
            .promote_candidate(working_character_id, working_character_version)
            .unwrap();
        saved_character.character.envelope.canon_state = forge_contracts::CanonState::Canon;
        saved_character
            .manual_edit(
                CharacterField::Fear,
                CharacterFieldValue::Text("Explicit canon revision after promotion".into()),
            )
            .unwrap();
        store
            .revise_canon(
                StoredEntity::CharacterCandidate(saved_character),
                working_character_version,
                "Gauntlet explicit Character canon revision",
            )
            .unwrap();

        let mut quest_candidate = match store.get_entity(quest_id).unwrap().unwrap().entity {
            StoredEntity::QuestCandidate(candidate) => candidate,
            _ => panic!("Quest candidate wrapper was lost"),
        };
        let mut quest_version = store.get_entity(quest_id).unwrap().unwrap().write_version;
        for revision in 1..=4 {
            quest_candidate
                .manual_edit(
                    QuestField::Title,
                    QuestFieldValue::Text(format!("Working history {revision} — Öresund 🧭")),
                )
                .unwrap();
            QuestForge::lock(&mut quest_candidate, QuestField::Title).unwrap();
            quest_version = store
                .save_candidate(
                    StoredEntity::QuestCandidate(quest_candidate.clone()),
                    quest_version,
                )
                .unwrap();
            quest_candidate = match store.get_entity(quest_id).unwrap().unwrap().entity {
                StoredEntity::QuestCandidate(candidate) => candidate,
                _ => panic!("Quest candidate wrapper was lost after save"),
            };
        }
        store
            .set_setting("vault.live-wal", "\"latest WAL-backed commit\"")
            .unwrap();

        let expected_entities = store.list_entities().unwrap();
        let expected_history = store.list_history(None).unwrap();
        let expected_revisions = store.list_revisions(character_id).unwrap();
        let expected_working_character_revisions =
            store.list_revisions(working_character_id).unwrap();
        assert_eq!(expected_revisions.len(), 4);
        assert_eq!(expected_working_character_revisions.len(), 2);
        assert!(source.with_extension("sqlite3-wal").exists() || source.exists());
        let expected_snapshots = expected_history
            .iter()
            .map(|event| {
                (
                    event.event_id,
                    store
                        .load_history_event_snapshot(event.event_id)
                        .unwrap()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let quest_history = expected_history
            .iter()
            .filter(|event| event.entity_id == Some(quest_id))
            .collect::<Vec<_>>();
        assert!(quest_history.len() >= 5);
        assert!(
            quest_history
                .iter()
                .map(|event| event.run_id)
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                < quest_history.len(),
            "regression case requires repeated generator run ids"
        );

        let older_snapshot = store
            .load_history_event_snapshot(quest_history[1].event_id)
            .unwrap()
            .unwrap();
        let older_json: serde_json::Value =
            serde_json::from_slice(&entity_json(&older_snapshot).unwrap()).unwrap();
        let older_markdown = entity_markdown(&older_snapshot).unwrap();
        assert!(
            serde_json::to_string(&older_json)
                .unwrap()
                .contains("Öresund")
        );
        assert!(older_markdown.contains("Öresund"));
        export_project(&store, &package).unwrap();
        drop(store);

        assert_eq!(
            import_project(&package, &destination).unwrap().envelope.id,
            project_id
        );
        let mut imported = ProjectStore::open(&destination).unwrap();
        assert_eq!(imported.list_entities().unwrap(), expected_entities);
        assert_eq!(imported.list_history(None).unwrap(), expected_history);
        assert_eq!(
            imported.list_revisions(character_id).unwrap(),
            expected_revisions
        );
        assert_eq!(
            imported.list_revisions(working_character_id).unwrap(),
            expected_working_character_revisions
        );
        for (event_id, snapshot) in &expected_snapshots {
            assert_eq!(
                imported
                    .load_history_event_snapshot(*event_id)
                    .unwrap()
                    .as_ref(),
                Some(snapshot)
            );
        }
        assert_eq!(
            imported.get_setting("vault.live-wal").unwrap().as_deref(),
            Some("\"latest WAL-backed commit\"")
        );
        let imported_old = imported
            .load_history_event_snapshot(quest_history[1].event_id)
            .unwrap()
            .unwrap();
        assert_eq!(imported_old, older_snapshot);
        let mut old_copy = match imported_old {
            StoredEntity::QuestCandidate(candidate) => candidate,
            _ => panic!("imported historical Quest is not editable candidate data"),
        };
        old_copy.quest.envelope.canon_state = forge_contracts::CanonState::Candidate;
        old_copy
            .manual_edit(
                QuestField::Objective,
                QuestFieldValue::Text(
                    "Restored old branch — Göteborg, \"quotation\" \\ path\nsecond line".into(),
                ),
            )
            .unwrap();
        let current_version = imported
            .get_entity(quest_id)
            .unwrap()
            .unwrap()
            .write_version;
        imported
            .save_candidate(StoredEntity::QuestCandidate(old_copy), current_version)
            .unwrap();
        let unicode_branch = imported.get_entity(quest_id).unwrap().unwrap().entity;
        let branch_document: serde_json::Value =
            serde_json::from_slice(&entity_json(&unicode_branch).unwrap()).unwrap();
        let expected_unicode = "Restored old branch — Göteborg, \"quotation\" \\ path\nsecond line";
        assert_eq!(
            branch_document["entity"]["value"]["quest"]["objective"],
            expected_unicode
        );
        assert!(
            entity_markdown(&unicode_branch)
                .unwrap()
                .contains("Restored old branch")
        );
        assert_eq!(
            imported.list_revisions(character_id).unwrap(),
            expected_revisions
        );
        assert_eq!(
            imported.list_history(Some(quest_id)).unwrap().len(),
            quest_history.len() + 1
        );

        let saved_entities = imported.list_entities().unwrap();
        let saved_history = imported.list_history(None).unwrap();
        let saved_revisions = imported.list_revisions(character_id).unwrap();
        let saved_project = imported.load_project().unwrap().unwrap();
        create_backup(&imported, &backup).unwrap();
        let mut changed = match imported.get_entity(quest_id).unwrap().unwrap().entity {
            StoredEntity::QuestCandidate(candidate) => candidate,
            _ => panic!("Quest candidate wrapper was lost before mutation"),
        };
        changed
            .manual_edit(
                QuestField::Stakes,
                QuestFieldValue::Text("Post-backup mutation must disappear".into()),
            )
            .unwrap();
        let write_version = imported
            .get_entity(quest_id)
            .unwrap()
            .unwrap()
            .write_version;
        imported
            .save_candidate(StoredEntity::QuestCandidate(changed), write_version)
            .unwrap();
        drop(imported);

        restore_backup(&backup, &destination).unwrap();
        let reopened = ProjectStore::open(&destination).unwrap();
        reopened.integrity_check().unwrap();
        assert_eq!(reopened.load_project().unwrap().unwrap(), saved_project);
        assert_eq!(reopened.list_entities().unwrap(), saved_entities);
        assert_eq!(reopened.list_history(None).unwrap(), saved_history);
        assert_eq!(
            reopened.list_revisions(character_id).unwrap(),
            saved_revisions
        );
        assert!(reopened.get_setting("ui.reducedMotion").unwrap().is_none());
        assert!(
            reopened
                .get_entity(quest_id)
                .unwrap()
                .unwrap()
                .entity
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn identically_named_projects_remain_isolated_during_export_import_and_restore() {
        let dir = Dir::new();
        let project_a = dir.path("project-a.sqlite3");
        let project_b = dir.path("project-b.sqlite3");
        let imported_a = dir.path("project-a-copy.sqlite3");
        let restore_target_a = dir.path("project-a-restore.sqlite3");
        let archive_a = dir.path("project-a.forgeproject");
        let dna = CharacterForge::new_dna(
            RootSeed::new(91_001),
            "dark-fantasy",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();

        let metadata_a = project();
        let mut candidate_a =
            CharacterForge::generate_in_project(&dna, metadata_a.envelope.id).unwrap();
        let id_a = candidate_a.character.envelope.id;
        let mut store_a = ProjectStore::open(&project_a).unwrap();
        store_a.create_project(&metadata_a).unwrap();
        store_a
            .persist_candidate(StoredEntity::CharacterCandidate(candidate_a.clone()))
            .unwrap();

        let mut metadata_b = project();
        metadata_b.envelope.id = ProjectId::from_bytes([9; 16]);
        let candidate_b =
            CharacterForge::generate_in_project(&dna, metadata_b.envelope.id).unwrap();
        let id_b = candidate_b.character.envelope.id;
        let mut store_b = ProjectStore::open(&project_b).unwrap();
        store_b.create_project(&metadata_b).unwrap();
        store_b
            .persist_candidate(StoredEntity::CharacterCandidate(candidate_b.clone()))
            .unwrap();
        assert_eq!(
            candidate_a.character.envelope.name,
            candidate_b.character.envelope.name
        );
        assert_eq!(
            store_a.load_project().unwrap().unwrap().name,
            store_b.load_project().unwrap().unwrap().name
        );

        candidate_a
            .manual_edit(
                CharacterField::Title,
                CharacterFieldValue::Text("Only Project A changes — å".into()),
            )
            .unwrap();
        store_a
            .save_candidate(StoredEntity::CharacterCandidate(candidate_a.clone()), 1)
            .unwrap();
        candidate_a = match store_a.get_entity(id_a).unwrap().unwrap().entity {
            StoredEntity::CharacterCandidate(value) => value,
            _ => panic!("Project A entity type changed after save"),
        };
        assert_eq!(
            store_b.get_entity(id_b).unwrap().unwrap().entity,
            StoredEntity::CharacterCandidate(candidate_b.clone())
        );
        export_project(&store_a, &archive_a).unwrap();
        drop(store_a);
        assert_eq!(
            import_project(&archive_a, &imported_a).unwrap().envelope.id,
            metadata_a.envelope.id
        );
        let copy = ProjectStore::open(&imported_a).unwrap();
        assert_eq!(
            copy.get_entity(id_a).unwrap().unwrap().entity,
            StoredEntity::CharacterCandidate(candidate_a)
        );

        let mut restore_target = ProjectStore::open(&restore_target_a).unwrap();
        let mut target_metadata = project();
        target_metadata.name = "Temporary target".into();
        restore_target.create_project(&target_metadata).unwrap();
        drop(restore_target);
        restore_backup(&archive_a, &restore_target_a).unwrap();
        assert_eq!(
            ProjectStore::open(&restore_target_a)
                .unwrap()
                .get_entity(id_a)
                .unwrap()
                .unwrap()
                .entity,
            copy.get_entity(id_a).unwrap().unwrap().entity
        );
        assert_eq!(
            store_b.get_entity(id_b).unwrap().unwrap().entity,
            StoredEntity::CharacterCandidate(candidate_b)
        );
        store_b.integrity_check().unwrap();
    }

    #[test]
    fn project_filenames_are_safe_unicode_preserving_and_never_overwrite() {
        let dir = Dir::new();
        assert_eq!(
            project_database_path(&dir.0, "  Café: North?  ").unwrap(),
            dir.path("Café_ North_.sqlite3")
        );
        assert_eq!(
            project_database_path(&dir.0, "CON").unwrap(),
            dir.path("_CON.sqlite3")
        );
        assert_eq!(
            project_database_path(&dir.0, "Story. ").unwrap(),
            dir.path("Story.sqlite3")
        );
        assert!(project_database_path(&dir.0, "... ").is_err());
        assert!(project_database_path(&dir.0, "  ").is_err());

        let existing = dir.path("Café_ North_.sqlite3");
        fs::write(&existing, b"preserve this file").unwrap();
        let before = fs::read(&existing).unwrap();
        assert!(import_project_to_directory(dir.path("missing.forgeproject"), &dir.0).is_err());
        assert_eq!(fs::read(&existing).unwrap(), before);
    }

    #[test]
    fn folder_import_uses_portable_display_name_for_safe_database_filename() {
        let dir = Dir::new();
        let source = dir.path("source.sqlite3");
        let package = dir.path("world.forgeproject");
        let target_directory = dir.path("moved-projects");
        let mut stored = project();
        stored.name = "Café: North?".into();
        let mut store = ProjectStore::open(&source).unwrap();
        store.create_project(&stored).unwrap();
        export_project(&store, &package).unwrap();

        let (imported, path) = import_project_to_directory(&package, &target_directory).unwrap();
        assert_eq!(imported.name, "Café: North?");
        assert_eq!(
            path,
            fs::canonicalize(&target_directory)
                .unwrap()
                .join("Café_ North_.sqlite3")
        );
        assert_eq!(
            ProjectStore::open(path)
                .unwrap()
                .load_project()
                .unwrap()
                .unwrap()
                .name,
            "Café: North?"
        );
        assert!(matches!(
            import_project_to_directory(&package, &target_directory),
            Err(ExportError::DestinationExists)
        ));
    }

    #[test]
    fn export_rejects_existing_destination_and_keeps_live_project_untouched() {
        let dir = Dir::new();
        let source = dir.path("source.sqlite3");
        let output = dir.path("existing.forgeproject");
        populated(&source);
        fs::write(&output, b"keep me").unwrap();
        let store = ProjectStore::open(&source).unwrap();
        assert!(matches!(
            export_project(&store, &output),
            Err(ExportError::DestinationExists)
        ));
        assert_eq!(fs::read(output).unwrap(), b"keep me");
        assert_eq!(
            store.get_setting("vault.test").unwrap().as_deref(),
            Some("\"committed immediately before export\"")
        );
        assert!(
            fs::read_dir(&dir.0).unwrap().flatten().all(|entry| !entry
                .file_name()
                .to_string_lossy()
                .contains("character-forge-partial")),
            "failed export left a partial sibling file"
        );
        let later = dir.path("later.forgeproject");
        export_project(&store, &later).unwrap();
        assert_eq!(
            import_project(&later, dir.path("later.sqlite3"))
                .unwrap()
                .envelope
                .id,
            project().envelope.id
        );
    }

    #[test]
    fn export_and_import_failures_leave_sources_healthy_and_allow_retry() {
        let dir = Dir::new();
        let source = dir.path("source.sqlite3");
        let (project_id, _, _, _) = populated(&source);
        let store = ProjectStore::open(&source).unwrap();
        let impossible = dir.path("missing-parent/out.forgeproject");
        assert!(export_project(&store, &impossible).is_err());
        assert!(!impossible.exists());
        store.integrity_check().unwrap();
        let healthy_archive = dir.path("healthy.forgeproject");
        export_project(&store, &healthy_archive).unwrap();

        let bad_archive = dir.path("bad.forgeproject");
        fs::write(&bad_archive, b"not a zip archive").unwrap();
        let original_bad_archive = fs::read(&bad_archive).unwrap();
        let failed_destination = dir.path("failed-import.sqlite3");
        assert!(import_project(&bad_archive, &failed_destination).is_err());
        assert!(!failed_destination.exists());
        assert_eq!(fs::read(&bad_archive).unwrap(), original_bad_archive);
        store.integrity_check().unwrap();

        let successful_destination = dir.path("retry-import.sqlite3");
        assert_eq!(
            import_project(&healthy_archive, &successful_destination)
                .unwrap()
                .envelope
                .id,
            project_id
        );
        ProjectStore::open(&successful_destination)
            .unwrap()
            .integrity_check()
            .unwrap();
    }

    #[test]
    fn import_collision_does_not_overwrite_existing_destination() {
        let dir = Dir::new();
        let source = dir.path("source.sqlite3");
        let _ = populated(&source);
        let package = dir.path("source.forgeproject");
        export_project(&ProjectStore::open(&source).unwrap(), &package).unwrap();
        let destination = dir.path("existing.sqlite3");
        fs::write(&destination, b"preserve").unwrap();
        assert!(matches!(
            import_project(&package, &destination),
            Err(ExportError::DestinationExists)
        ));
        assert_eq!(fs::read(destination).unwrap(), b"preserve");
    }

    #[test]
    fn isolated_staging_directory_is_removed_on_drop() {
        let staging = Staging::new("cleanup-test").unwrap();
        let path = staging.path.clone();
        assert!(path.is_dir());
        drop(staging);
        assert!(!path.exists());
    }

    #[test]
    fn json_and_markdown_entity_exports_retain_structured_values() {
        let dir = Dir::new();
        let source = dir.path("source.sqlite3");
        let (_, character, _, _) = populated(&source);
        let json = entity_json(&character).unwrap();
        let document: serde_json::Value = serde_json::from_slice(&json).unwrap();
        assert_eq!(document["format"], "forge-entity");
        assert_eq!(document["format_version"], 1);
        assert_eq!(
            document["entity"]["value"]["character"]["envelope"]["name"],
            "A Manual Name"
        );
        let markdown = entity_markdown(&character).unwrap();
        assert!(markdown.contains("# Character: A Manual Name"));
        assert!(markdown.contains("Complete canonical data"));
        assert!(markdown.contains("user_modified"));
    }

    #[test]
    fn malformed_and_unsafe_archives_fail_before_destination_creation() {
        let dir = Dir::new();
        let archive_path = dir.path("unsafe.forgeproject");
        let destination = dir.path("must-not-exist.sqlite3");
        let file = fs::File::create(&archive_path).unwrap();
        let mut zip = ZipWriter::new(file);
        zip.start_file("../project.sqlite3", SimpleFileOptions::default())
            .unwrap();
        zip.write_all(b"no extraction").unwrap();
        zip.finish().unwrap();
        let error = import_project(&archive_path, &destination).unwrap_err();
        assert!(error.to_string().contains("unsafe path"), "{error}");
        assert!(!destination.exists());
    }

    #[test]
    fn windows_and_absolute_archive_paths_are_rejected() {
        for (index, name) in [
            "..\\project.sqlite3",
            "C:/project.sqlite3",
            "//server/share/project.sqlite3",
            "/tmp/project.sqlite3",
        ]
        .into_iter()
        .enumerate()
        {
            let dir = Dir::new();
            let archive_path = dir.path("unsafe.forgeproject");
            let file = fs::File::create(&archive_path).unwrap();
            let mut zip = ZipWriter::new(file);
            zip.start_file(name, SimpleFileOptions::default()).unwrap();
            zip.write_all(b"unused").unwrap();
            zip.finish().unwrap();
            let error = import_project(&archive_path, dir.path(&format!("out-{index}.sqlite3")))
                .unwrap_err();
            assert!(error.to_string().contains("unsafe path"), "{name}: {error}");
        }
    }

    #[test]
    fn duplicate_unknown_and_oversized_members_are_rejected() {
        let dir = Dir::new();
        let destination = dir.path("out.sqlite3");
        let duplicate = dir.path("duplicate.forgeproject");
        let file = fs::File::create(&duplicate).unwrap();
        let mut zip = ZipWriter::new(file);
        zip.start_file(MANIFEST_MEMBER, SimpleFileOptions::default())
            .unwrap();
        zip.write_all(b"{}").unwrap();
        zip.start_file("MANIFEST.JSON", SimpleFileOptions::default())
            .unwrap();
        zip.write_all(b"{}").unwrap();
        zip.finish().unwrap();
        assert!(
            import_project(&duplicate, &destination)
                .unwrap_err()
                .to_string()
                .contains("duplicate")
        );

        let unknown = dir.path("unknown.forgeproject");
        let file = fs::File::create(&unknown).unwrap();
        let mut zip = ZipWriter::new(file);
        zip.start_file("notes.txt", SimpleFileOptions::default())
            .unwrap();
        zip.write_all(b"not allowed").unwrap();
        zip.finish().unwrap();
        assert!(
            import_project(&unknown, &destination)
                .unwrap_err()
                .to_string()
                .contains("unexpected member")
        );

        let oversized = dir.path("oversized.forgeproject");
        let file = fs::File::create(&oversized).unwrap();
        let mut zip = ZipWriter::new(file);
        zip.start_file(MANIFEST_MEMBER, SimpleFileOptions::default())
            .unwrap();
        zip.write_all(&vec![b' '; MAX_MANIFEST_BYTES as usize + 1])
            .unwrap();
        zip.finish().unwrap();
        assert!(
            import_project(&oversized, &destination)
                .unwrap_err()
                .to_string()
                .contains("size limit")
        );
        assert!(!destination.exists());
    }

    #[test]
    fn checksum_mismatch_and_future_format_are_structured_rejections() {
        let dir = Dir::new();
        let source = dir.path("source.sqlite3");
        let _ = populated(&source);
        let package = dir.path("source.forgeproject");
        export_project(&ProjectStore::open(&source).unwrap(), &package).unwrap();
        let mut input = ZipArchive::new(fs::File::open(&package).unwrap()).unwrap();
        let mut manifest = Vec::new();
        input
            .by_name(MANIFEST_MEMBER)
            .unwrap()
            .read_to_end(&mut manifest)
            .unwrap();
        let mut database = Vec::new();
        input
            .by_name(DATABASE_MEMBER)
            .unwrap()
            .read_to_end(&mut database)
            .unwrap();
        let bad_hash = dir.path("bad-hash.forgeproject");
        let mut changed_database = database.clone();
        let last = changed_database.len() - 1;
        changed_database[last] ^= 0x5a;
        write_package(&bad_hash, &manifest, &changed_database);
        assert!(matches!(
            import_project(&bad_hash, dir.path("bad.sqlite3")),
            Err(ExportError::Integrity)
        ));
        let mut future: serde_json::Value = serde_json::from_slice(&manifest).unwrap();
        future["format_version"] = serde_json::json!(FORMAT_VERSION + 1);
        let future_path = dir.path("future.forgeproject");
        write_package(
            &future_path,
            &serde_json::to_vec(&future).unwrap(),
            &database,
        );
        assert!(
            matches!(import_project(&future_path, dir.path("future.sqlite3")), Err(ExportError::UnsupportedFormat(v)) if v == FORMAT_VERSION + 1)
        );
    }

    #[test]
    fn restore_success_replaces_state_and_invalid_restore_preserves_current_database() {
        let dir = Dir::new();
        let source = dir.path("source.sqlite3");
        populated(&source);
        let package = dir.path("backup.forgeproject");
        export_project(&ProjectStore::open(&source).unwrap(), &package).unwrap();
        let destination = dir.path("target.sqlite3");
        let mut target = ProjectStore::open(&destination).unwrap();
        let mut different = project();
        different.name = "Current State".into();
        target.create_project(&different).unwrap();
        drop(target);
        restore_backup(&package, &destination).unwrap();
        assert_eq!(
            ProjectStore::open(&destination)
                .unwrap()
                .load_project()
                .unwrap()
                .unwrap()
                .name,
            "Vault Test World"
        );
        let before = fs::read(&destination).unwrap();
        let malformed = dir.path("bad.forgeproject");
        fs::write(&malformed, b"not zip").unwrap();
        assert!(restore_backup(&malformed, &destination).is_err());
        assert_eq!(fs::read(&destination).unwrap(), before);
    }

    #[test]
    fn injected_post_replace_restore_failure_recovers_the_previous_project() {
        let dir = Dir::new();
        let source = dir.path("source.sqlite3");
        populated(&source);
        let destination = dir.path("target.sqlite3");
        populated(&destination);
        let previous = ProjectStore::open(&destination).unwrap();
        let previous_entities = previous.list_entities().unwrap();
        let previous_history = previous.list_history(None).unwrap();
        let previous_character_id = previous_entities[0].entity.envelope().id;
        let previous_revisions = previous.list_revisions(previous_character_id).unwrap();
        drop(previous);
        let error = restore_staged(&source, &destination, &project(), |_, _| {
            Err(ExportError::Storage("injected validation failure".into()))
        })
        .unwrap_err();
        assert!(error.to_string().contains("previous project was recovered"));
        let recovered = ProjectStore::open(&destination).unwrap();
        recovered.integrity_check().unwrap();
        assert_eq!(recovered.list_entities().unwrap(), previous_entities);
        assert_eq!(recovered.list_history(None).unwrap(), previous_history);
        assert_eq!(
            recovered.list_revisions(previous_character_id).unwrap(),
            previous_revisions
        );
        assert!(
            !fs::read_dir(&dir.0)
                .unwrap()
                .flatten()
                .any(|entry| entry.file_name().to_string_lossy().contains("displaced"))
        );
    }

    #[test]
    fn active_restore_closes_and_reopens_on_success_and_failure() {
        let dir = Dir::new();
        let source = dir.path("source.sqlite3");
        populated(&source);
        let package = dir.path("backup.forgeproject");
        export_project(&ProjectStore::open(&source).unwrap(), &package).unwrap();
        let active_path = dir.path("active.sqlite3");
        let mut current = ProjectStore::open(&active_path).unwrap();
        let mut prior = project();
        prior.name = "Active Prior".into();
        current.create_project(&prior).unwrap();
        drop(current);
        let mut session = ProjectSession::default();
        session.open(&active_path).unwrap();
        restore_active(&mut session, &package).unwrap();
        assert_eq!(
            session
                .active()
                .unwrap()
                .load_project()
                .unwrap()
                .unwrap()
                .name,
            "Vault Test World"
        );
        let malformed = dir.path("bad.forgeproject");
        fs::write(&malformed, b"bad archive").unwrap();
        assert!(restore_active(&mut session, &malformed).is_err());
        assert_eq!(
            session
                .active()
                .unwrap()
                .load_project()
                .unwrap()
                .unwrap()
                .name,
            "Vault Test World"
        );
    }

    fn write_package(path: &Path, manifest: &[u8], database: &[u8]) {
        let file = fs::File::create(path).unwrap();
        let mut zip = ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        zip.start_file(MANIFEST_MEMBER, options).unwrap();
        zip.write_all(manifest).unwrap();
        zip.start_file(DATABASE_MEMBER, options).unwrap();
        zip.write_all(database).unwrap();
        zip.finish().unwrap();
    }

    fn manifest_for_database(bytes: &[u8]) -> Vec<u8> {
        serde_json::to_vec(&Manifest {
            format: "forgeproject".into(),
            format_version: FORMAT_VERSION,
            application: "Character Forge / Quest Forge".into(),
            application_version: "0.1.0".into(),
            project_id: project().envelope.id.uuid().to_string(),
            project_name: "Vault Test World".into(),
            database_schema_version: 1,
            generator_version: 1,
            content_version: 1,
            exported_at_unix_ms: 1,
            members: vec![MemberIntegrity {
                name: DATABASE_MEMBER.into(),
                size: bytes.len() as u64,
                blake3: blake3::hash(bytes).to_hex().to_string(),
            }],
        })
        .unwrap()
    }

    #[test]
    fn corrupt_database_and_future_database_schema_are_rejected() {
        let dir = Dir::new();
        let corrupt_bytes = b"this is not SQLite";
        let corrupt = dir.path("corrupt.forgeproject");
        write_package(
            &corrupt,
            &manifest_for_database(corrupt_bytes),
            corrupt_bytes,
        );
        assert!(matches!(
            import_project(&corrupt, dir.path("corrupt.sqlite3")),
            Err(ExportError::Storage(_))
        ));

        let future_db = dir.path("future.sqlite3");
        let connection = rusqlite::Connection::open(&future_db).unwrap();
        connection.pragma_update(None, "user_version", 999).unwrap();
        drop(connection);
        let bytes = fs::read(&future_db).unwrap();
        let future = dir.path("future-db.forgeproject");
        write_package(&future, &manifest_for_database(&bytes), &bytes);
        assert!(matches!(
            import_project(&future, dir.path("future-db.sqlite3")),
            Err(ExportError::UnsupportedDatabaseSchema(999))
        ));
    }

    #[test]
    fn compressed_size_and_special_file_limits_are_enforced() {
        let dir = Dir::new();
        let ratio = dir.path("ratio.forgeproject");
        let file = fs::File::create(&ratio).unwrap();
        let mut zip = ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
        zip.start_file(MANIFEST_MEMBER, options).unwrap();
        zip.write_all(b"{}").unwrap();
        zip.start_file(DATABASE_MEMBER, options).unwrap();
        zip.write_all(&vec![0u8; 2 * 1024 * 1024]).unwrap();
        zip.finish().unwrap();
        assert!(
            import_project(&ratio, dir.path("ratio.sqlite3"))
                .unwrap_err()
                .to_string()
                .contains("compression ratio")
        );

        let special = dir.path("special.forgeproject");
        let file = fs::File::create(&special).unwrap();
        let mut zip = ZipWriter::new(file);
        zip.add_symlink(MANIFEST_MEMBER, "target", SimpleFileOptions::default())
            .unwrap();
        zip.finish().unwrap();
        assert!(
            import_project(&special, dir.path("special.sqlite3"))
                .unwrap_err()
                .to_string()
                .contains("special file")
        );
    }
}
