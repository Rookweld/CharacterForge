use forge_app_core::{
    CharacterCandidate, CharacterField, CharacterFieldValue, CharacterForge, LocationField,
    LocationFieldValue, LocationForge, QuestField, QuestFieldValue, QuestForge,
};
use forge_contracts::{
    CanonState, CreativeAxes, CreativeAxis, EntityId, INITIAL_CONTENT_VERSION,
    INITIAL_GENERATOR_VERSION, LocationProfile, ProjectEnvelope, ProjectId, RootSeed,
};
use forge_export::{
    ExportError, create_backup, export_project, import_project_to_directory, project_database_path,
    restore_active, write_entity_json, write_entity_markdown,
};
use forge_storage::{
    CURRENT_SCHEMA_VERSION, EntityRecord, EntityRevision, ForgeRunRecord, ProjectSession,
    ProjectStore, SessionHistory, StoreError, StoredEntity, StoredProject,
};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager, State};

#[derive(Default)]
struct AppSession {
    project: ProjectSession,
    working: Option<StoredEntity>,
    write_version: Option<i64>,
    history: Option<SessionHistory<StoredEntity>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct UserSettings {
    version: u8,
    reduced_motion: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            version: 1,
            reduced_motion: false,
        }
    }
}

const USER_SETTINGS_FILE: &str = "user-settings.json";

fn user_settings_path(app: &AppHandle) -> Result<PathBuf, UiError> {
    app.path()
        .app_config_dir()
        .map(|directory| directory.join(USER_SETTINGS_FILE))
        .map_err(|_| ui_storage_error("Could not locate local application settings."))
}

fn read_user_settings(path: &Path) -> UserSettings {
    if path
        .metadata()
        .is_ok_and(|metadata| metadata.len() > 16 * 1024)
    {
        return UserSettings::default();
    }
    let Ok(contents) = std::fs::read_to_string(path) else {
        return UserSettings::default();
    };
    serde_json::from_str::<UserSettings>(&contents)
        .ok()
        .filter(|settings| settings.version == 1)
        .unwrap_or_default()
}

fn write_user_settings(path: &Path, settings: &UserSettings) -> Result<(), UiError> {
    if settings.version != 1 {
        return Err(UiError {
            kind: "validation",
            message: "These application settings use an unsupported version.".into(),
            expected: Some(1),
            actual: Some(i64::from(settings.version)),
        });
    }
    let parent = path
        .parent()
        .ok_or_else(|| ui_storage_error("Could not locate local application settings."))?;
    std::fs::create_dir_all(parent)
        .map_err(|_| ui_storage_error("Could not save application settings."))?;
    let temporary = path.with_extension(format!("json.{}.tmp", std::process::id()));
    let bytes = serde_json::to_vec(settings)
        .map_err(|_| ui_storage_error("Could not save application settings."))?;
    std::fs::write(&temporary, bytes)
        .map_err(|_| ui_storage_error("Could not save application settings."))?;
    if path.exists() {
        std::fs::remove_file(path)
            .map_err(|_| ui_storage_error("Could not update application settings."))?;
    }
    std::fs::rename(&temporary, path)
        .map_err(|_| ui_storage_error("Could not update application settings."))
}

#[derive(Default)]
struct AppState(Mutex<AppSession>);

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectInfo {
    name: String,
    path: String,
    id: ProjectId,
    created_at_unix_ms: i64,
    updated_at_unix_ms: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RecentProject {
    name: String,
    path: String,
    last_opened_at_unix_ms: i64,
}

#[derive(Default, Deserialize, Serialize)]
struct RecentProjectsFile {
    version: u8,
    projects: Vec<RecentProject>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RecentProjectView {
    name: String,
    path: String,
    last_opened_at_unix_ms: i64,
    exists: bool,
}

const MAX_RECENT_PROJECTS: usize = 8;
const RECENT_PROJECTS_FILE: &str = "recent-projects.json";

fn ui_storage_error(message: impl Into<String>) -> UiError {
    UiError {
        kind: "storage",
        message: message.into(),
        expected: None,
        actual: None,
    }
}

fn reserve_project_database(path: &Path) -> Result<std::fs::File, UiError> {
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                UiError {
                    kind: "project_exists",
                    message: "A project with that name already exists in this folder. Choose another name or location.".into(),
                    expected: None,
                    actual: None,
                }
            } else {
                ui_storage_error("Character Forge couldn’t create the project in that folder. Choose a folder you can write to and try again.")
            }
        })
}

fn recent_projects_path(app: &AppHandle) -> Result<PathBuf, UiError> {
    app.path()
        .app_config_dir()
        .map(|directory| directory.join(RECENT_PROJECTS_FILE))
        .map_err(|error| ui_storage_error(format!("Could not locate local app settings: {error}")))
}

fn read_recent_projects(path: &Path) -> Result<RecentProjectsFile, UiError> {
    if path
        .metadata()
        .is_ok_and(|metadata| metadata.len() > 64 * 1024)
    {
        return Err(ui_storage_error(
            "Recent project information is larger than expected and could not be read safely.",
        ));
    }
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(RecentProjectsFile {
                version: 1,
                projects: Vec::new(),
            });
        }
        Err(error) => {
            return Err(ui_storage_error(format!(
                "Could not read recent projects: {error}"
            )));
        }
    };
    let mut recent: RecentProjectsFile = serde_json::from_str(&contents)
        .map_err(|_| ui_storage_error("Recent project information could not be read safely."))?;
    if recent.version != 1 {
        return Err(ui_storage_error(
            "Recent project information uses an unsupported version.",
        ));
    }
    recent.projects.truncate(MAX_RECENT_PROJECTS);
    Ok(recent)
}

fn write_recent_projects(path: &Path, recent: &RecentProjectsFile) -> Result<(), UiError> {
    let parent = path
        .parent()
        .ok_or_else(|| ui_storage_error("Could not locate local app settings."))?;
    std::fs::create_dir_all(parent)
        .map_err(|_| ui_storage_error("Could not save the Recent Projects list."))?;
    let temp = path.with_extension(format!("json.{}.tmp", std::process::id()));
    let bytes = serde_json::to_vec(recent)
        .map_err(|_| ui_storage_error("Could not save the Recent Projects list."))?;
    std::fs::write(&temp, bytes)
        .map_err(|_| ui_storage_error("Could not save the Recent Projects list."))?;
    if path.exists() {
        std::fs::remove_file(path)
            .map_err(|_| ui_storage_error("Could not update the Recent Projects list."))?;
    }
    std::fs::rename(&temp, path)
        .map_err(|_| ui_storage_error("Could not update the Recent Projects list."))?;
    Ok(())
}

fn remember_recent_project(app: &AppHandle, project: &ProjectInfo) {
    let Ok(path) = recent_projects_path(app) else {
        return;
    };
    let Ok(mut recent) = read_recent_projects(&path) else {
        return;
    };
    recent
        .projects
        .retain(|entry| !entry.path.eq_ignore_ascii_case(&project.path));
    recent.projects.insert(
        0,
        RecentProject {
            name: project.name.clone(),
            path: project.path.clone(),
            last_opened_at_unix_ms: now_ms().unwrap_or_default(),
        },
    );
    recent.projects.truncate(MAX_RECENT_PROJECTS);
    let _ = write_recent_projects(&path, &recent);
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EntityView {
    entity: StoredEntity,
    write_version: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HistoryView {
    run_id: String,
    entity_id: Option<EntityId>,
    action: String,
    created_at_unix_ms: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RevisionView {
    revision_number: i64,
    reason: String,
    entity: StoredEntity,
    created_at_unix_ms: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UiError {
    kind: &'static str,
    message: String,
    expected: Option<i64>,
    actual: Option<i64>,
}

impl From<StoreError> for UiError {
    fn from(error: StoreError) -> Self {
        let kind = match &error {
            StoreError::Conflict { .. } => "stale_write",
            StoreError::UnsupportedSchema(_) => "unsupported_schema",
            StoreError::Corrupt(_) | StoreError::Serialization(_) => "corrupt_project",
            StoreError::Validation(_) => "validation",
            StoreError::NotFound(_) => "not_found",
            StoreError::Io(_) | StoreError::Sqlite(_) => "storage",
            StoreError::InvalidOperation(_) => "invalid_operation",
        };
        let (expected, actual) = match &error {
            StoreError::Conflict { expected, actual } => (Some(*expected), *actual),
            _ => (None, None),
        };
        Self {
            kind,
            message: error.to_string(),
            expected,
            actual,
        }
    }
}

impl From<ExportError> for UiError {
    fn from(error: ExportError) -> Self {
        let kind = match &error {
            ExportError::DestinationExists => "project_exists",
            ExportError::UnsupportedFormat(_) => "unsupported_format",
            ExportError::UnsupportedDatabaseSchema(_) => "unsupported_schema",
            ExportError::Integrity => "integrity",
            ExportError::Invalid(message) if message.contains("unsafe path") => "unsafe_archive",
            ExportError::Invalid(_) | ExportError::Archive(_) => "invalid_archive",
            ExportError::Storage(_) => "corrupt_project",
            ExportError::Io(_) => "storage",
        };
        Self {
            kind,
            message: error.to_string(),
            expected: None,
            actual: None,
        }
    }
}

impl From<forge_contracts::ValidationErrors> for UiError {
    fn from(error: forge_contracts::ValidationErrors) -> Self {
        Self {
            kind: "validation",
            message: error.to_string(),
            expected: None,
            actual: None,
        }
    }
}

fn lock_state<'a>(
    state: &'a State<'_, AppState>,
) -> Result<std::sync::MutexGuard<'a, AppSession>, UiError> {
    state.0.lock().map_err(|_| UiError {
        kind: "internal",
        message: "The application session is unavailable. Restart the app and reopen the project."
            .into(),
        expected: None,
        actual: None,
    })
}

fn active_store(session: &mut AppSession) -> Result<&mut ProjectStore, UiError> {
    session.project.active_mut().ok_or_else(|| UiError {
        kind: "no_project",
        message: "Open or create a project first.".into(),
        expected: None,
        actual: None,
    })
}

fn project_info(session: &AppSession) -> Result<ProjectInfo, UiError> {
    let store = session.project.active().ok_or_else(|| UiError {
        kind: "no_project",
        message: "No project is open.".into(),
        expected: None,
        actual: None,
    })?;
    let project = store.load_project()?.ok_or_else(|| UiError {
        kind: "invalid_project",
        message: "This database does not contain a Character Forge project.".into(),
        expected: None,
        actual: None,
    })?;
    Ok(ProjectInfo {
        name: project.name,
        path: store.path().to_string_lossy().into_owned(),
        id: project.envelope.id,
        created_at_unix_ms: project.created_at_unix_ms,
        updated_at_unix_ms: project.updated_at_unix_ms,
    })
}

fn now_ms() -> Result<i64, UiError> {
    let value = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| UiError {
            kind: "internal",
            message: error.to_string(),
            expected: None,
            actual: None,
        })?
        .as_millis();
    i64::try_from(value).map_err(|error| UiError {
        kind: "internal",
        message: error.to_string(),
        expected: None,
        actual: None,
    })
}

static PROJECT_COUNTER: AtomicU64 = AtomicU64::new(0);
fn new_project_id() -> ProjectId {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut bytes = [0_u8; 16];
    bytes[..8].copy_from_slice(&(nanos as u64).to_le_bytes());
    let salt = (std::process::id() as u64) ^ PROJECT_COUNTER.fetch_add(1, Ordering::Relaxed);
    bytes[8..].copy_from_slice(&salt.to_le_bytes());
    ProjectId::from_bytes(bytes)
}

#[tauri::command]
fn create_project(
    app: AppHandle,
    state: State<'_, AppState>,
    directory: String,
    name: String,
) -> Result<ProjectInfo, UiError> {
    if name.trim().is_empty() {
        return Err(UiError {
            kind: "validation",
            message: "Enter a project name to continue.".into(),
            expected: None,
            actual: None,
        });
    }
    if directory.trim().is_empty() {
        return Err(UiError {
            kind: "validation",
            message: "Choose a folder for this project.".into(),
            expected: None,
            actual: None,
        });
    }
    let directory = PathBuf::from(directory.trim());
    std::fs::create_dir_all(&directory).map_err(|_| {
        ui_storage_error("Character Forge couldn’t create that folder. Choose a folder you can write to and try again.")
    })?;
    if !directory.is_dir() {
        return Err(ui_storage_error(
            "That location isn’t a folder. Choose a folder for this project and try again.",
        ));
    }
    let directory = std::fs::canonicalize(&directory).map_err(|_| {
        ui_storage_error(
            "Character Forge couldn’t use that folder. Choose another location and try again.",
        )
    })?;
    let path = project_database_path(&directory, &name).map_err(|error| UiError {
        kind: "validation",
        message: error.to_string(),
        expected: None,
        actual: None,
    })?;
    let mut session = lock_state(&state)?;
    let reservation = reserve_project_database(&path)?;
    drop(reservation);

    let result = (|| {
        session.project.open(&path)?;
        let timestamp = now_ms()?;
        active_store(&mut session)?.create_project(&StoredProject {
            envelope: ProjectEnvelope {
                id: new_project_id(),
                schema_version: forge_contracts::SchemaVersion(CURRENT_SCHEMA_VERSION as u16),
                generator_version: INITIAL_GENERATOR_VERSION,
                content_version: INITIAL_CONTENT_VERSION,
            },
            name: name.trim().to_owned(),
            created_at_unix_ms: timestamp,
            updated_at_unix_ms: timestamp,
        })?;
        project_info(&session)
    })();
    let project = match result {
        Ok(project) => project,
        Err(error) => {
            session.project.close();
            let _ = std::fs::remove_file(&path);
            let _ = std::fs::remove_file(path.with_extension("sqlite3-wal"));
            let _ = std::fs::remove_file(path.with_extension("sqlite3-shm"));
            return Err(error);
        }
    };
    session.working = None;
    session.write_version = None;
    session.history = None;
    remember_recent_project(&app, &project);
    Ok(project)
}

#[tauri::command]
fn open_project(
    app: AppHandle,
    state: State<'_, AppState>,
    path: String,
) -> Result<ProjectInfo, UiError> {
    let path = std::fs::canonicalize(Path::new(path.trim())).map_err(|_| UiError {
        kind: "not_found",
        message: "Character Forge couldn’t find that project file. Choose an existing project file and try again.".into(),
        expected: None,
        actual: None,
    })?;
    if path
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("forgeproject"))
    {
        return Err(UiError {
            kind: "validation",
            message: "Portable .forgeproject files must be brought in with Import Project.".into(),
            expected: None,
            actual: None,
        });
    }
    let mut session = lock_state(&state)?;
    let store = session.project.open(&path)?;
    if store.load_project()?.is_none() {
        session.project.close();
        return Err(UiError { kind: "invalid_project", message: "This database does not contain a project. Create a new project or choose another file.".into(), expected: None, actual: None });
    }
    session.working = None;
    session.write_version = None;
    session.history = None;
    let project = project_info(&session)?;
    remember_recent_project(&app, &project);
    Ok(project)
}

#[tauri::command]
fn default_project_directory(app: AppHandle) -> Result<String, UiError> {
    let documents = app.path().document_dir().map_err(|error| {
        ui_storage_error(format!("Could not locate your Documents folder: {error}"))
    })?;
    Ok(documents
        .join("Character Forge")
        .to_string_lossy()
        .into_owned())
}

#[tauri::command]
fn get_user_settings(app: AppHandle) -> Result<UserSettings, UiError> {
    Ok(read_user_settings(&user_settings_path(&app)?))
}

#[tauri::command]
fn update_user_settings(app: AppHandle, settings: UserSettings) -> Result<UserSettings, UiError> {
    write_user_settings(&user_settings_path(&app)?, &settings)?;
    Ok(settings)
}

#[tauri::command]
fn recent_projects(app: AppHandle) -> Result<Vec<RecentProjectView>, UiError> {
    let path = recent_projects_path(&app)?;
    let recent = read_recent_projects(&path)?;
    Ok(recent
        .projects
        .into_iter()
        .map(|project| RecentProjectView {
            exists: Path::new(&project.path).is_file(),
            name: project.name,
            path: project.path,
            last_opened_at_unix_ms: project.last_opened_at_unix_ms,
        })
        .collect())
}

#[tauri::command]
fn forget_recent_project(app: AppHandle, path: String) -> Result<(), UiError> {
    let file_path = recent_projects_path(&app)?;
    let mut recent = read_recent_projects(&file_path)?;
    recent
        .projects
        .retain(|project| !project.path.eq_ignore_ascii_case(path.trim()));
    write_recent_projects(&file_path, &recent)
}

#[tauri::command]
fn close_project(state: State<'_, AppState>) -> Result<(), UiError> {
    let mut session = lock_state(&state)?;
    session.project.close();
    session.working = None;
    session.write_version = None;
    session.history = None;
    Ok(())
}

#[tauri::command]
fn export_project_command(state: State<'_, AppState>, path: String) -> Result<(), UiError> {
    let mut session = lock_state(&state)?;
    let store = active_store(&mut session)?;
    export_project(store, Path::new(path.trim()))?;
    Ok(())
}

#[tauri::command]
fn create_backup_command(state: State<'_, AppState>, path: String) -> Result<(), UiError> {
    let mut session = lock_state(&state)?;
    let store = active_store(&mut session)?;
    create_backup(store, Path::new(path.trim()))?;
    Ok(())
}

#[tauri::command]
fn import_project_command(
    app: AppHandle,
    state: State<'_, AppState>,
    archive_path: String,
    destination_directory: String,
) -> Result<ProjectInfo, UiError> {
    if destination_directory.trim().is_empty() {
        return Err(UiError {
            kind: "validation",
            message: "Choose a folder for the imported project.".into(),
            expected: None,
            actual: None,
        });
    }
    let mut session = lock_state(&state)?;
    let (imported, destination) = import_project_to_directory(
        Path::new(archive_path.trim()),
        Path::new(destination_directory.trim()),
    )?;
    let store = session.project.open(&destination)?;
    if store.load_project()?.is_none() {
        session.project.close();
        return Err(UiError {
            kind: "invalid_archive",
            message: "The imported file does not contain a project.".into(),
            expected: None,
            actual: None,
        });
    }
    session.working = None;
    session.write_version = None;
    session.history = None;
    let project = project_info(&session)?;
    debug_assert_eq!(project.name, imported.name);
    remember_recent_project(&app, &project);
    Ok(project)
}

#[tauri::command]
fn restore_backup_command(state: State<'_, AppState>, archive_path: String) -> Result<(), UiError> {
    let mut session = lock_state(&state)?;
    restore_active(&mut session.project, Path::new(archive_path.trim()))?;
    session.working = None;
    session.write_version = None;
    session.history = None;
    Ok(())
}

#[tauri::command]
fn export_entity_command(
    state: State<'_, AppState>,
    id: String,
    path: String,
    format: String,
) -> Result<(), UiError> {
    let mut session = lock_state(&state)?;
    let store = active_store(&mut session)?;
    let id = parse_entity_id(&id)?;
    let entity = store
        .get_entity(id)?
        .ok_or_else(|| UiError {
            kind: "not_found",
            message: "The saved item could not be found.".into(),
            expected: None,
            actual: None,
        })?
        .entity;
    match format.as_str() {
        "json" => write_entity_json(&entity, Path::new(path.trim()))?,
        "markdown" => write_entity_markdown(&entity, Path::new(path.trim()))?,
        _ => {
            return Err(UiError {
                kind: "validation",
                message: "Choose JSON or Markdown export.".into(),
                expected: None,
                actual: None,
            });
        }
    }
    Ok(())
}

#[tauri::command]
fn get_project(state: State<'_, AppState>) -> Result<Option<ProjectInfo>, UiError> {
    let session = lock_state(&state)?;
    if session.project.active().is_none() {
        Ok(None)
    } else {
        project_info(&session).map(Some)
    }
}

fn set_entity_field_lock(
    entity: &mut StoredEntity,
    field: &str,
    locked: bool,
) -> Result<(), UiError> {
    match entity {
        StoredEntity::CharacterCandidate(candidate) => {
            let target = character_field(field)?;
            if locked {
                CharacterForge::lock(candidate, target)?
            } else {
                CharacterForge::unlock(candidate, target)?
            }
        }
        StoredEntity::QuestCandidate(candidate) => {
            let target = quest_field(field)?;
            if locked {
                QuestForge::lock(candidate, target)?
            } else {
                QuestForge::unlock(candidate, target)?
            }
        }
        StoredEntity::LocationCandidate(candidate) => {
            let target = location_field(field)?;
            if locked {
                LocationForge::lock(candidate, target)?
            } else {
                LocationForge::unlock(candidate, target)?
            }
        }
        _ => {
            return Err(invalid_candidate_operation(
                "Canon records cannot be edited as candidates.",
            ));
        }
    }
    Ok(())
}

fn reroll_entity_field(entity: &mut StoredEntity, field: &str) -> Result<(), UiError> {
    match entity {
        StoredEntity::CharacterCandidate(candidate) => {
            CharacterForge::reroll_field(candidate, character_field(field)?)?;
        }
        StoredEntity::QuestCandidate(candidate) => {
            QuestForge::reroll_field(candidate, quest_field(field)?)?;
        }
        StoredEntity::LocationCandidate(candidate) => {
            LocationForge::reroll_field(candidate, location_field(field)?)?;
        }
        _ => {
            return Err(invalid_candidate_operation(
                "Canon records cannot be rerolled.",
            ));
        }
    }
    Ok(())
}

fn reroll_entity_unlocked(entity: &mut StoredEntity) -> Result<(), UiError> {
    match entity {
        StoredEntity::CharacterCandidate(candidate) => {
            CharacterForge::reroll_all_unlocked(candidate)?;
        }
        StoredEntity::QuestCandidate(candidate) => {
            QuestForge::reroll_all(candidate)?;
        }
        StoredEntity::LocationCandidate(candidate) => {
            LocationForge::reroll_all_unlocked(candidate)?;
        }
        _ => {
            return Err(invalid_candidate_operation(
                "Canon records cannot be rerolled.",
            ));
        }
    }
    Ok(())
}

fn edit_entity_field(entity: &mut StoredEntity, field: &str, value: Value) -> Result<(), UiError> {
    match entity {
        StoredEntity::CharacterCandidate(candidate) => {
            let target = character_field(field)?;
            candidate.manual_edit(target, character_value(target, value)?)?;
        }
        StoredEntity::QuestCandidate(candidate) => {
            let target = quest_field(field)?;
            candidate.manual_edit(target, quest_value(target, value)?)?;
        }
        StoredEntity::LocationCandidate(candidate) => {
            let target = location_field(field)?;
            candidate.manual_edit(target, location_value(target, value)?)?;
        }
        _ => {
            return Err(invalid_candidate_operation(
                "Canon records must be revised explicitly.",
            ));
        }
    }
    Ok(())
}

fn invalid_candidate_operation(message: &str) -> UiError {
    UiError {
        kind: "invalid_operation",
        message: message.into(),
        expected: None,
        actual: None,
    }
}

fn generate_character_candidate(
    project_id: ProjectId,
    preset: &str,
    seed: u64,
    axes: BTreeMap<CreativeAxis, u8>,
    constraints: BTreeMap<forge_contracts::ConstraintKey, String>,
) -> Result<CharacterCandidate, UiError> {
    let mut creative_axes = CreativeAxes { levels: axes };
    creative_axes.normalize()?;
    let dna = CharacterForge::new_dna(RootSeed::new(seed), preset, creative_axes, constraints)?;
    Ok(CharacterForge::generate_in_project(&dna, project_id)?)
}

#[tauri::command]
fn forge_character(
    state: State<'_, AppState>,
    preset: String,
    seed: u64,
    axes: BTreeMap<CreativeAxis, u8>,
    constraints: BTreeMap<forge_contracts::ConstraintKey, String>,
) -> Result<StoredEntity, UiError> {
    let mut session = lock_state(&state)?;
    let project = active_store(&mut session)?
        .load_project()?
        .ok_or_else(|| UiError {
            kind: "no_project",
            message: "Open or create a project first.".into(),
            expected: None,
            actual: None,
        })?;
    let candidate =
        generate_character_candidate(project.envelope.id, &preset, seed, axes, constraints)?;
    let entity = StoredEntity::CharacterCandidate(candidate);
    session.working = Some(entity.clone());
    session.write_version = None;
    session.history = Some(SessionHistory::new(entity.clone()));
    Ok(entity)
}

#[tauri::command]
fn forge_quest(
    state: State<'_, AppState>,
    preset: String,
    seed: u64,
    axes: BTreeMap<CreativeAxis, u8>,
    constraints: BTreeMap<forge_contracts::ConstraintKey, String>,
) -> Result<StoredEntity, UiError> {
    let mut session = lock_state(&state)?;
    let project = active_store(&mut session)?
        .load_project()?
        .ok_or_else(|| UiError {
            kind: "no_project",
            message: "Open or create a project first.".into(),
            expected: None,
            actual: None,
        })?;
    let mut creative_axes = CreativeAxes { levels: axes };
    creative_axes.normalize()?;
    let dna = QuestForge::new_dna(RootSeed::new(seed), &preset, creative_axes, constraints)?;
    let candidate = QuestForge::generate_in_project(&dna, project.envelope.id)?;
    let entity = StoredEntity::QuestCandidate(candidate);
    session.working = Some(entity.clone());
    session.write_version = None;
    session.history = Some(SessionHistory::new(entity.clone()));
    Ok(entity)
}

#[tauri::command]
fn forge_location(
    state: State<'_, AppState>,
    preset: String,
    seed: u64,
    axes: BTreeMap<CreativeAxis, u8>,
) -> Result<StoredEntity, UiError> {
    let mut session = lock_state(&state)?;
    let project = active_store(&mut session)?
        .load_project()?
        .ok_or_else(|| UiError {
            kind: "no_project",
            message: "Open or create a project first.".into(),
            expected: None,
            actual: None,
        })?;
    let mut creative_axes = CreativeAxes { levels: axes };
    creative_axes.normalize()?;
    let dna = LocationForge::new_dna(RootSeed::new(seed), &preset, creative_axes)?;
    let candidate = LocationForge::generate_in_project(&dna, project.envelope.id)?;
    let entity = StoredEntity::LocationCandidate(candidate);
    session.working = Some(entity.clone());
    session.write_version = None;
    session.history = Some(SessionHistory::new(entity.clone()));
    Ok(entity)
}

#[tauri::command]
fn set_field_lock(
    state: State<'_, AppState>,
    field: String,
    locked: bool,
) -> Result<StoredEntity, UiError> {
    let mut session = lock_state(&state)?;
    let mut entity = session.working.clone().ok_or_else(|| UiError {
        kind: "no_candidate",
        message: "Forge or open a candidate first.".into(),
        expected: None,
        actual: None,
    })?;
    let previous = entity.clone();
    set_entity_field_lock(&mut entity, &field, locked)?;
    push_session_state(&mut session, previous, entity.clone());
    Ok(entity)
}

#[tauri::command]
fn reroll_field(state: State<'_, AppState>, field: String) -> Result<StoredEntity, UiError> {
    let mut session = lock_state(&state)?;
    let mut entity = session.working.clone().ok_or_else(|| UiError {
        kind: "no_candidate",
        message: "Forge or open a candidate first.".into(),
        expected: None,
        actual: None,
    })?;
    let previous = entity.clone();
    reroll_entity_field(&mut entity, &field)?;
    push_session_state(&mut session, previous, entity.clone());
    Ok(entity)
}

#[tauri::command]
fn reroll_unlocked(state: State<'_, AppState>) -> Result<StoredEntity, UiError> {
    let mut session = lock_state(&state)?;
    let mut entity = session.working.clone().ok_or_else(|| UiError {
        kind: "no_candidate",
        message: "Forge or open a candidate first.".into(),
        expected: None,
        actual: None,
    })?;
    let previous = entity.clone();
    reroll_entity_unlocked(&mut entity)?;
    push_session_state(&mut session, previous, entity.clone());
    Ok(entity)
}

#[tauri::command]
fn edit_field(
    state: State<'_, AppState>,
    field: String,
    value: Value,
) -> Result<StoredEntity, UiError> {
    let mut session = lock_state(&state)?;
    let mut entity = session.working.clone().ok_or_else(|| UiError {
        kind: "no_candidate",
        message: "Forge or open a candidate first.".into(),
        expected: None,
        actual: None,
    })?;
    let previous = entity.clone();
    edit_entity_field(&mut entity, &field, value)?;
    push_session_state(&mut session, previous, entity.clone());
    Ok(entity)
}

fn push_session_state(session: &mut AppSession, previous: StoredEntity, next: StoredEntity) {
    let history = session
        .history
        .get_or_insert_with(|| SessionHistory::new(previous));
    history.push(next.clone());
    session.working = Some(next);
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkingState {
    entity: Option<StoredEntity>,
    can_undo: bool,
    can_redo: bool,
}

fn working_state(session: &AppSession) -> WorkingState {
    WorkingState {
        entity: session.working.clone(),
        can_undo: session
            .history
            .as_ref()
            .is_some_and(|history| history.can_undo()),
        can_redo: session
            .history
            .as_ref()
            .is_some_and(|history| history.can_redo()),
    }
}

#[tauri::command]
fn undo_session(state: State<'_, AppState>) -> Result<WorkingState, UiError> {
    let mut session = lock_state(&state)?;
    let history = session.history.as_mut().ok_or_else(|| UiError {
        kind: "no_undo",
        message: "There is no earlier change in this editing session.".into(),
        expected: None,
        actual: None,
    })?;
    let previous = history.undo().cloned().ok_or_else(|| UiError {
        kind: "no_undo",
        message: "There is no earlier change in this editing session.".into(),
        expected: None,
        actual: None,
    })?;
    session.working = Some(previous);
    Ok(working_state(&session))
}

#[tauri::command]
fn redo_session(state: State<'_, AppState>) -> Result<WorkingState, UiError> {
    let mut session = lock_state(&state)?;
    let history = session.history.as_mut().ok_or_else(|| UiError {
        kind: "no_redo",
        message: "There is no change to redo in this editing session.".into(),
        expected: None,
        actual: None,
    })?;
    let next = history.redo().cloned().ok_or_else(|| UiError {
        kind: "no_redo",
        message: "There is no change to redo in this editing session.".into(),
        expected: None,
        actual: None,
    })?;
    session.working = Some(next);
    Ok(working_state(&session))
}

#[tauri::command]
fn save_candidate(state: State<'_, AppState>) -> Result<EntityView, UiError> {
    let mut session = lock_state(&state)?;
    let entity = session.working.clone().ok_or_else(|| UiError {
        kind: "no_candidate",
        message: "Forge or open a candidate first.".into(),
        expected: None,
        actual: None,
    })?;
    let version = match session.write_version {
        Some(expected) => active_store(&mut session)?.save_candidate(entity.clone(), expected)?,
        None => active_store(&mut session)?.persist_candidate(entity.clone())?,
    };
    session.write_version = Some(version);
    Ok(EntityView {
        entity,
        write_version: version,
    })
}

#[tauri::command]
fn revise_canon(state: State<'_, AppState>, reason: String) -> Result<EntityView, UiError> {
    if reason.trim().is_empty() {
        return Err(UiError {
            kind: "validation",
            message: "Add a short reason for this Canon revision.".into(),
            expected: None,
            actual: None,
        });
    }
    let mut session = lock_state(&state)?;
    let mut entity = session.working.clone().ok_or_else(|| UiError {
        kind: "no_candidate",
        message: "Open a Canon item as a working copy before revising it.".into(),
        expected: None,
        actual: None,
    })?;
    let expected = session.write_version.ok_or_else(|| UiError {
        kind: "invalid_operation",
        message: "This working copy is not linked to a saved Canon item.".into(),
        expected: None,
        actual: None,
    })?;
    match &mut entity {
        StoredEntity::CharacterCandidate(candidate) => {
            candidate.character.envelope.canon_state = CanonState::Canon;
        }
        StoredEntity::QuestCandidate(candidate) => {
            candidate.quest.envelope.canon_state = CanonState::Canon;
        }
        StoredEntity::LocationCandidate(candidate) => {
            candidate.location.envelope.canon_state = CanonState::Canon;
        }
        _ => {
            return Err(invalid_candidate_operation(
                "Canon revisions require the structured working candidate.",
            ));
        }
    }
    let version =
        active_store(&mut session)?.revise_canon(entity.clone(), expected, reason.trim())?;
    session.working = Some(entity.clone());
    session.write_version = Some(version);
    Ok(EntityView {
        entity,
        write_version: version,
    })
}

#[tauri::command]
fn list_entities(state: State<'_, AppState>) -> Result<Vec<EntityView>, UiError> {
    let mut session = lock_state(&state)?;
    Ok(active_store(&mut session)?
        .list_entities()?
        .into_iter()
        .map(entity_view)
        .collect())
}

fn entity_view(record: EntityRecord) -> EntityView {
    EntityView {
        entity: record.entity,
        write_version: record.write_version,
    }
}

#[tauri::command]
fn get_entity(state: State<'_, AppState>, id: String) -> Result<Option<EntityView>, UiError> {
    let mut session = lock_state(&state)?;
    let id = parse_entity_id(&id)?;
    let record = active_store(&mut session)?.get_entity(id)?;
    if let Some(record) = &record {
        session.working = Some(record.entity.clone());
        session.write_version = Some(record.write_version);
        session.history = Some(SessionHistory::new(record.entity.clone()));
    }
    Ok(record.map(entity_view))
}

#[tauri::command]
fn toggle_favorite(
    state: State<'_, AppState>,
    id: String,
    favorite: bool,
    expected_version: i64,
) -> Result<EntityView, UiError> {
    let mut session = lock_state(&state)?;
    let id = parse_entity_id(&id)?;
    active_store(&mut session)?.toggle_favorite(id, favorite, expected_version)?;
    let record = active_store(&mut session)?
        .get_entity(id)?
        .ok_or_else(|| UiError {
            kind: "not_found",
            message: "That saved item no longer exists.".into(),
            expected: None,
            actual: None,
        })?;
    Ok(entity_view(record))
}

#[tauri::command]
fn promote_to_canon(
    state: State<'_, AppState>,
    id: String,
    expected_version: i64,
) -> Result<EntityView, UiError> {
    let mut session = lock_state(&state)?;
    let id = parse_entity_id(&id)?;
    active_store(&mut session)?.promote_candidate(id, expected_version)?;
    let record = active_store(&mut session)?
        .get_entity(id)?
        .ok_or_else(|| UiError {
            kind: "not_found",
            message: "That saved item no longer exists.".into(),
            expected: None,
            actual: None,
        })?;
    Ok(entity_view(record))
}

#[tauri::command]
fn list_history(
    state: State<'_, AppState>,
    id: Option<String>,
) -> Result<Vec<HistoryView>, UiError> {
    let mut session = lock_state(&state)?;
    let records = active_store(&mut session)?
        .list_history(id.as_deref().map(parse_entity_id).transpose()?)?;
    Ok(records.into_iter().map(history_view).collect())
}

fn history_view(record: ForgeRunRecord) -> HistoryView {
    HistoryView {
        run_id: record.event_id.to_string(),
        entity_id: record.entity_id,
        action: record.action,
        created_at_unix_ms: record.created_at_unix_ms,
    }
}

#[tauri::command]
fn get_history_snapshot(
    state: State<'_, AppState>,
    run_id: String,
) -> Result<Option<StoredEntity>, UiError> {
    let event_id = run_id.parse::<i64>().map_err(|error| UiError {
        kind: "invalid_history",
        message: format!("That history entry identifier is invalid: {error}"),
        expected: None,
        actual: None,
    })?;
    let mut session = lock_state(&state)?;
    active_store(&mut session)?
        .load_history_event_snapshot(event_id)
        .map_err(Into::into)
}

fn as_working_candidate(mut entity: StoredEntity) -> Result<StoredEntity, UiError> {
    match &mut entity {
        StoredEntity::CharacterCandidate(candidate) => {
            candidate.character.envelope.canon_state = CanonState::Candidate;
        }
        StoredEntity::QuestCandidate(candidate) => {
            candidate.quest.envelope.canon_state = CanonState::Candidate;
        }
        StoredEntity::LocationCandidate(candidate) => {
            candidate.location.envelope.canon_state = CanonState::Candidate;
        }
        _ => {
            return Err(UiError {
                kind: "invalid_history",
                message: "This saved snapshot cannot be edited as a candidate because its generation data is unavailable.".into(),
                expected: None,
                actual: None,
            });
        }
    }
    entity.validate().map_err(|error| UiError {
        kind: "invalid_history",
        message: format!("This saved snapshot cannot be used safely: {error}"),
        expected: None,
        actual: None,
    })?;
    Ok(entity)
}

#[tauri::command]
fn use_history_as_working_copy(
    state: State<'_, AppState>,
    run_id: String,
) -> Result<WorkingState, UiError> {
    let mut session = lock_state(&state)?;
    let event_id = run_id.parse::<i64>().map_err(|error| UiError {
        kind: "invalid_history",
        message: format!("That history entry identifier is invalid: {error}"),
        expected: None,
        actual: None,
    })?;
    let snapshot = active_store(&mut session)?
        .load_history_event_snapshot(event_id)?
        .ok_or_else(|| UiError {
            kind: "not_found",
            message: "That history snapshot is no longer available.".into(),
            expected: None,
            actual: None,
        })?;
    let entity = as_working_candidate(snapshot)?;
    let id = entity.envelope().id;
    session.write_version = active_store(&mut session)?
        .get_entity(id)?
        .map(|record| record.write_version);
    session.working = Some(entity.clone());
    session.history = Some(SessionHistory::new(entity));
    Ok(working_state(&session))
}

#[tauri::command]
fn get_canon_revisions(
    state: State<'_, AppState>,
    id: String,
) -> Result<Vec<RevisionView>, UiError> {
    let mut session = lock_state(&state)?;
    Ok(active_store(&mut session)?
        .list_revisions(parse_entity_id(&id)?)?
        .into_iter()
        .map(revision_view)
        .collect())
}

#[tauri::command]
fn use_canon_revision_as_working_copy(
    state: State<'_, AppState>,
    id: String,
    revision_number: i64,
) -> Result<WorkingState, UiError> {
    let mut session = lock_state(&state)?;
    let entity_id = parse_entity_id(&id)?;
    let revisions = active_store(&mut session)?.list_revisions(entity_id)?;
    let revision = revisions
        .into_iter()
        .find(|revision| revision.revision_number == revision_number)
        .ok_or_else(|| UiError {
            kind: "not_found",
            message: "That canon revision is no longer available.".into(),
            expected: None,
            actual: None,
        })?;
    let entity = as_working_candidate(revision.entity)?;
    session.write_version = active_store(&mut session)?
        .get_entity(entity_id)?
        .map(|record| record.write_version);
    session.working = Some(entity.clone());
    session.history = Some(SessionHistory::new(entity));
    Ok(working_state(&session))
}

fn revision_view(value: EntityRevision) -> RevisionView {
    RevisionView {
        revision_number: value.revision_number,
        reason: value.reason,
        entity: value.entity,
        created_at_unix_ms: value.created_at_unix_ms,
    }
}

fn parse_entity_id(value: &str) -> Result<EntityId, UiError> {
    value
        .parse::<forge_contracts::Uuid128>()
        .map(EntityId)
        .map_err(UiError::from)
}

fn character_field(value: &str) -> Result<CharacterField, UiError> {
    serde_json::from_value(Value::String(value.to_owned())).map_err(|_| UiError {
        kind: "validation",
        message: format!("Unknown Character field: {value}"),
        expected: None,
        actual: None,
    })
}
fn quest_field(value: &str) -> Result<QuestField, UiError> {
    serde_json::from_value(Value::String(value.to_owned())).map_err(|_| UiError {
        kind: "validation",
        message: format!("Unknown Quest field: {value}"),
        expected: None,
        actual: None,
    })
}
fn location_field(value: &str) -> Result<LocationField, UiError> {
    serde_json::from_value(Value::String(value.to_owned())).map_err(|_| UiError {
        kind: "validation",
        message: format!("Unknown Location field: {value}"),
        expected: None,
        actual: None,
    })
}
fn string_values(value: Value) -> Result<Vec<String>, UiError> {
    match value {
        Value::String(text) => Ok(text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect()),
        Value::Array(items) => items
            .into_iter()
            .map(|item| {
                item.as_str().map(str::to_owned).ok_or_else(|| UiError {
                    kind: "validation",
                    message: "List fields require text values.".into(),
                    expected: None,
                    actual: None,
                })
            })
            .collect(),
        _ => Err(UiError {
            kind: "validation",
            message: "Enter text for this field.".into(),
            expected: None,
            actual: None,
        }),
    }
}
fn scalar_value(value: Value) -> Result<String, UiError> {
    value.as_str().map(str::to_owned).ok_or_else(|| UiError {
        kind: "validation",
        message: "Enter text for this field.".into(),
        expected: None,
        actual: None,
    })
}
fn character_value(field: CharacterField, value: Value) -> Result<CharacterFieldValue, UiError> {
    if matches!(
        field,
        CharacterField::Personality
            | CharacterField::Skills
            | CharacterField::Abilities
            | CharacterField::StoryHooks
    ) {
        Ok(CharacterFieldValue::Texts(string_values(value)?))
    } else {
        Ok(CharacterFieldValue::Text(scalar_value(value)?))
    }
}
fn quest_value(field: QuestField, value: Value) -> Result<QuestFieldValue, UiError> {
    if matches!(
        field,
        QuestField::Complications
            | QuestField::Locations
            | QuestField::Rewards
            | QuestField::Failure
            | QuestField::FollowUp
    ) {
        Ok(QuestFieldValue::Texts(string_values(value)?))
    } else {
        Ok(QuestFieldValue::Text(scalar_value(value)?))
    }
}
fn location_value(field: LocationField, value: Value) -> Result<LocationFieldValue, UiError> {
    match field {
        LocationField::Profile => serde_json::from_value::<LocationProfile>(value)
            .map(LocationFieldValue::Profile)
            .map_err(|_| UiError {
                kind: "validation",
                message: "Choose a supported Location profile.".into(),
                expected: None,
                actual: None,
            }),
        LocationField::NotableFeatures | LocationField::StoryHooks => {
            Ok(LocationFieldValue::Texts(string_values(value)?))
        }
        _ => Ok(LocationFieldValue::Text(scalar_value(value)?)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            default_project_directory,
            get_user_settings,
            update_user_settings,
            recent_projects,
            forget_recent_project,
            create_project,
            open_project,
            export_project_command,
            create_backup_command,
            import_project_command,
            restore_backup_command,
            export_entity_command,
            close_project,
            get_project,
            forge_character,
            forge_quest,
            forge_location,
            set_field_lock,
            reroll_field,
            reroll_unlocked,
            edit_field,
            save_candidate,
            revise_canon,
            list_entities,
            get_entity,
            toggle_favorite,
            promote_to_canon,
            list_history,
            get_history_snapshot,
            get_canon_revisions,
            use_history_as_working_copy,
            use_canon_revision_as_working_copy,
            undo_session,
            redo_session
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Character Forge desktop shell");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn character_candidate(seed: u64) -> CharacterCandidate {
        generate_character_candidate(
            ProjectId::from_bytes([seed as u8; 16]),
            "dark-fantasy",
            seed,
            BTreeMap::new(),
            BTreeMap::new(),
        )
        .unwrap()
    }

    #[test]
    fn locked_name_survives_title_dependent_reroll_at_command_boundary() {
        let mut candidate = character_candidate(991);
        candidate.character.title = "Title Reroll Sentinel".into();
        let mut entity = StoredEntity::CharacterCandidate(candidate);
        edit_entity_field(
            &mut entity,
            "name",
            Value::String("The Operator's Locked Name".into()),
        )
        .unwrap();
        let name_key = CharacterField::Name.key().unwrap();
        let title_key = CharacterField::Title.key().unwrap();
        let StoredEntity::CharacterCandidate(candidate) = &entity else {
            panic!("Character edit returned the wrong entity type");
        };
        let locked_name = candidate.character.envelope.name.clone();
        let name_nonce = candidate.dna.recipe.field_controls[&name_key].nonce;
        let name_origin = candidate.field_origins[&name_key];
        let title_nonce = candidate.dna.recipe.field_controls[&title_key].nonce;

        set_entity_field_lock(&mut entity, "name", true).unwrap();
        reroll_entity_field(&mut entity, "archetype").unwrap();

        let StoredEntity::CharacterCandidate(candidate) = entity else {
            panic!("Character reroll returned the wrong entity type");
        };
        assert_eq!(candidate.character.envelope.name, locked_name);
        assert_eq!(
            candidate.dna.recipe.field_controls[&name_key].nonce,
            name_nonce
        );
        assert_eq!(candidate.field_origins[&name_key], name_origin);
        assert!(candidate.dna.recipe.field_controls[&name_key].locked);
        assert!(candidate.dna.recipe.field_controls[&title_key].nonce > title_nonce);
    }

    #[test]
    fn locked_name_survives_reroll_all_and_full_candidate_serialization() {
        let mut candidate = character_candidate(992);
        candidate.character.title = "Title Reroll Sentinel".into();
        let mut entity = StoredEntity::CharacterCandidate(candidate);
        edit_entity_field(
            &mut entity,
            "name",
            Value::String("A Name Held Fast".into()),
        )
        .unwrap();
        let name_key = CharacterField::Name.key().unwrap();
        let title_key = CharacterField::Title.key().unwrap();
        let StoredEntity::CharacterCandidate(candidate) = &entity else {
            panic!("Character edit returned the wrong entity type");
        };
        let locked_name = candidate.character.envelope.name.clone();
        let name_nonce = candidate.dna.recipe.field_controls[&name_key].nonce;
        let name_origin = candidate.field_origins[&name_key];
        let title_nonce = candidate.dna.recipe.field_controls[&title_key].nonce;

        set_entity_field_lock(&mut entity, "name", true).unwrap();
        let command_response: StoredEntity =
            serde_json::from_value(serde_json::to_value(&entity).unwrap()).unwrap();
        entity = command_response;
        reroll_entity_unlocked(&mut entity).unwrap();

        let StoredEntity::CharacterCandidate(candidate) = entity else {
            panic!("Character reroll returned the wrong entity type");
        };
        assert_eq!(candidate.character.envelope.name, locked_name);
        assert_eq!(
            candidate.dna.recipe.field_controls[&name_key].nonce,
            name_nonce
        );
        assert_eq!(candidate.field_origins[&name_key], name_origin);
        assert!(candidate.dna.recipe.field_controls[&name_key].locked);
        assert!(candidate.dna.recipe.field_controls[&title_key].nonce > title_nonce);
    }

    #[test]
    fn unlocking_name_restores_reroll_eligibility() {
        let mut candidate = character_candidate(993);
        candidate.character.envelope.name = "A Name Eligible to Change".into();
        let name_key = CharacterField::Name.key().unwrap();
        let original_name = candidate.character.envelope.name.clone();
        let original_nonce = candidate.dna.recipe.field_controls[&name_key].nonce;
        let mut entity = StoredEntity::CharacterCandidate(candidate);

        assert_eq!(character_field("name").unwrap(), CharacterField::Name);
        set_entity_field_lock(&mut entity, "name", true).unwrap();
        set_entity_field_lock(&mut entity, "name", false).unwrap();
        reroll_entity_field(&mut entity, "name").unwrap();

        let StoredEntity::CharacterCandidate(candidate) = entity else {
            panic!("Character reroll returned the wrong entity type");
        };
        assert_ne!(candidate.character.envelope.name, original_name);
        assert_eq!(
            candidate.dna.recipe.field_controls[&name_key].nonce.0,
            original_nonce.0 + 1
        );
        assert!(!candidate.dna.recipe.field_controls[&name_key].locked);
    }

    #[test]
    fn editing_title_does_not_replace_locked_name() {
        let mut candidate = character_candidate(994);
        candidate.character.envelope.name = "An Independently Kept Name".into();
        let name_key = CharacterField::Name.key().unwrap();
        let original_name = candidate.character.envelope.name.clone();
        let name_nonce = candidate.dna.recipe.field_controls[&name_key].nonce;
        let mut entity = StoredEntity::CharacterCandidate(candidate);

        set_entity_field_lock(&mut entity, "name", true).unwrap();
        edit_entity_field(
            &mut entity,
            "title",
            Value::String("A New Title Only".into()),
        )
        .unwrap();

        let StoredEntity::CharacterCandidate(candidate) = entity else {
            panic!("Character edit returned the wrong entity type");
        };
        assert_eq!(candidate.character.envelope.name, original_name);
        assert_eq!(candidate.character.title, "A New Title Only");
        assert_eq!(
            candidate.dna.recipe.field_controls[&name_key].nonce,
            name_nonce
        );
        assert!(candidate.dna.recipe.field_controls[&name_key].locked);
    }

    #[test]
    fn quest_lock_still_survives_the_shared_command_reroll_path() {
        let dna = QuestForge::new_dna(
            RootSeed::new(995),
            "occult",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        let candidate =
            QuestForge::generate_in_project(&dna, ProjectId::from_bytes([9; 16])).unwrap();
        let mut entity = StoredEntity::QuestCandidate(candidate);
        let objective_key = QuestField::Objective.key().unwrap();
        set_entity_field_lock(&mut entity, "objective", true).unwrap();
        let StoredEntity::QuestCandidate(before) = &entity else {
            panic!("Quest lock returned the wrong entity type");
        };
        let objective = before.quest.objective.clone();
        let nonce = before.dna.recipe.field_controls[&objective_key].nonce;

        reroll_entity_field(&mut entity, "title").unwrap();

        let StoredEntity::QuestCandidate(after) = entity else {
            panic!("Quest reroll returned the wrong entity type");
        };
        assert_eq!(after.quest.objective, objective);
        assert_eq!(after.dna.recipe.field_controls[&objective_key].nonce, nonce);
        assert!(after.dna.recipe.field_controls[&objective_key].locked);
    }

    #[test]
    fn character_generation_keeps_name_and_title_as_separate_core_fields() {
        let project_id = ProjectId::from_bytes([8; 16]);
        let axes = BTreeMap::new();
        let constraints = BTreeMap::new();
        let actual = generate_character_candidate(
            project_id,
            "dark-fantasy",
            996,
            axes.clone(),
            constraints.clone(),
        )
        .unwrap();
        let mut creative_axes = CreativeAxes { levels: axes };
        creative_axes.normalize().unwrap();
        let dna = CharacterForge::new_dna(
            RootSeed::new(996),
            "dark-fantasy",
            creative_axes,
            constraints,
        )
        .unwrap();
        let expected = CharacterForge::generate_in_project(&dna, project_id).unwrap();
        assert_eq!(actual, expected);
        assert_ne!(actual.character.envelope.name, actual.character.title);
    }

    #[test]
    fn quest_field_mapping_covers_v3_fields_and_rejects_unknown_values() {
        assert_eq!(quest_field("briefing").unwrap(), QuestField::Briefing);
        assert_eq!(quest_field("clue").unwrap(), QuestField::Clue);
        assert_eq!(quest_field("not-a-field").unwrap_err().kind, "validation");
    }

    #[test]
    fn stale_write_errors_have_a_structured_frontend_shape() {
        let error = UiError::from(StoreError::Conflict {
            expected: 2,
            actual: Some(4),
        });
        let value = serde_json::to_value(error).unwrap();
        assert_eq!(value["kind"], "stale_write");
        assert_eq!(value["expected"], 2);
        assert_eq!(value["actual"], 4);
    }

    #[test]
    fn recent_project_metadata_roundtrips_without_project_contents() {
        let root = std::env::temp_dir().join(format!(
            "character-forge-recent-test-{}-{}",
            std::process::id(),
            PROJECT_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        let settings_file = root.join(RECENT_PROJECTS_FILE);
        let saved = RecentProjectsFile {
            version: 1,
            projects: vec![RecentProject {
                name: "Ashen Coast".into(),
                path: root
                    .join("Ashen Coast.sqlite3")
                    .to_string_lossy()
                    .into_owned(),
                last_opened_at_unix_ms: 1234,
            }],
        };
        write_recent_projects(&settings_file, &saved).unwrap();
        let loaded = read_recent_projects(&settings_file).unwrap();
        assert_eq!(loaded.version, 1);
        assert_eq!(loaded.projects.len(), 1);
        assert_eq!(loaded.projects[0].name, "Ashen Coast");
        assert_eq!(loaded.projects[0].last_opened_at_unix_ms, 1234);
        assert!(!Path::new(&loaded.projects[0].path).exists());
        assert!(std::fs::metadata(&settings_file).unwrap().len() < 1024);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn application_settings_default_on_missing_or_malformed_file_and_persist_updates() {
        let root = std::env::temp_dir().join(format!(
            "character-forge-settings-test-{}-{}",
            std::process::id(),
            PROJECT_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        let settings_file = root.join(USER_SETTINGS_FILE);
        assert!(!read_user_settings(&settings_file).reduced_motion);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(&settings_file, b"{broken").unwrap();
        assert!(!read_user_settings(&settings_file).reduced_motion);
        let updated = UserSettings {
            version: 1,
            reduced_motion: true,
        };
        write_user_settings(&settings_file, &updated).unwrap();
        assert!(read_user_settings(&settings_file).reduced_motion);
        write_user_settings(&settings_file, &UserSettings::default()).unwrap();
        assert!(!read_user_settings(&settings_file).reduced_motion);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn session_history_restores_complete_candidate_snapshot_and_discards_redo_branch() {
        let original = StoredEntity::CharacterCandidate(character_candidate(91));
        let original_name = original.envelope().name.clone();
        let mut changed = original.clone();
        edit_entity_field(
            &mut changed,
            "name",
            Value::String("Manual sentinel".into()),
        )
        .unwrap();
        set_entity_field_lock(&mut changed, "name", true).unwrap();
        let mut history = SessionHistory::new(original.clone());
        history.push(changed.clone());
        assert!(history.can_undo());
        assert_eq!(history.undo(), Some(&original));
        assert_eq!(history.redo(), Some(&changed));
        let Some(StoredEntity::CharacterCandidate(restored)) = history.undo() else {
            panic!("undo lost the Character candidate wrapper");
        };
        assert_eq!(restored.character.envelope.name, original_name);
        let restored = history.redo().unwrap();
        let StoredEntity::CharacterCandidate(restored) = restored else {
            panic!("redo lost the Character candidate wrapper");
        };
        assert_eq!(restored.character.envelope.name, "Manual sentinel");
        let name_key = CharacterField::Name.key().unwrap();
        assert!(restored.dna.recipe.field_controls[&name_key].locked);
        assert_eq!(
            restored.field_origins[&name_key],
            forge_app_core::FieldValueOrigin::UserModified
        );
        history.undo();
        history.push(original);
        assert!(!history.can_redo());
    }

    #[test]
    fn project_creation_reservation_rejects_collisions_without_touching_existing_file() {
        let root = std::env::temp_dir().join(format!(
            "character-forge-create-test-{}-{}",
            std::process::id(),
            PROJECT_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&root).unwrap();
        let target = root.join("My World.sqlite3");
        std::fs::write(&target, b"keep the existing project").unwrap();
        let before = std::fs::read(&target).unwrap();
        let error = reserve_project_database(&target).unwrap_err();
        assert_eq!(error.kind, "project_exists");
        assert_eq!(std::fs::read(&target).unwrap(), before);
        std::fs::remove_dir_all(root).unwrap();
    }
}
