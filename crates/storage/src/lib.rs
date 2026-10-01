//! SQLite persistence for projects, generated candidates, canon and durable history.
//!
//! This crate depends on domain contracts and candidate types, never on Tauri or UI code.

use forge_app_core::{
    CharacterCandidate, CreatureCandidate, FactionCandidate, FactionResultBundle,
    ForgeResultBundle, HometownSource, ItemCandidate, LocationCandidate, QuestCandidate,
    RelationshipDefinition, compile_faction_context, compile_hometown_context,
    relationship_definition, validate_faction_bundle, validate_relationship_endpoints,
};
use forge_contracts::{
    CanonState, Character, Creature, EntityEnvelope, EntityId, EntityOrigin, EntityType, Faction,
    GenerationAction, GenerationMetadata, Item, Location, ProjectEnvelope, ProjectId, Quest,
    Relationship, RelationshipId, RelationshipState, SchemaVersion, Uuid128, ValidationCode,
    ValidationError, ValidationErrors, ValidationResult,
};
use rusqlite::{Connection, OptionalExtension, Transaction, params};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub const CURRENT_SCHEMA_VERSION: u32 = 2;
const BUSY_TIMEOUT_MS: u64 = 5_000;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoredProject {
    pub envelope: ProjectEnvelope,
    pub name: String,
    pub created_at_unix_ms: i64,
    pub updated_at_unix_ms: i64,
}

impl StoredProject {
    pub fn validate(&self) -> ValidationResult<()> {
        self.envelope.validate()?;
        if self.name.trim().is_empty() {
            return Err(validation_error(
                ValidationCode::EmptyValue,
                "project.name",
                "value must contain non-whitespace text",
            ));
        }
        if self.created_at_unix_ms < 0 || self.updated_at_unix_ms < self.created_at_unix_ms {
            return Err(validation_error(
                ValidationCode::InvalidTimestamp,
                "project.timestamps",
                "timestamps must be non-negative and updated_at must not precede created_at",
            ));
        }
        Ok(())
    }
}

/// Complete persisted representation. Candidate wrappers retain per-field manual-edit origins,
/// including Quest v3 clue/briefing data, while canonical entities retain their full contract.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum StoredEntity {
    CharacterCandidate(CharacterCandidate),
    QuestCandidate(QuestCandidate),
    LocationCandidate(LocationCandidate),
    FactionCandidate(FactionCandidate),
    ItemCandidate(ItemCandidate),
    CreatureCandidate(CreatureCandidate),
    Character(Character),
    Quest(Quest),
    Location(Location),
    Faction(Faction),
    Item(Item),
    Creature(Creature),
}

impl StoredEntity {
    pub fn envelope(&self) -> &EntityEnvelope {
        match self {
            Self::CharacterCandidate(value) => &value.character.envelope,
            Self::QuestCandidate(value) => &value.quest.envelope,
            Self::LocationCandidate(value) => &value.location.envelope,
            Self::FactionCandidate(value) => &value.faction.envelope,
            Self::ItemCandidate(value) => &value.item.envelope,
            Self::CreatureCandidate(value) => &value.creature.envelope,
            Self::Character(value) => &value.envelope,
            Self::Quest(value) => &value.envelope,
            Self::Location(value) => &value.envelope,
            Self::Faction(value) => &value.envelope,
            Self::Item(value) => &value.envelope,
            Self::Creature(value) => &value.envelope,
        }
    }

    pub fn validate(&self) -> ValidationResult<()> {
        match self {
            Self::CharacterCandidate(value) => value.validate(),
            Self::QuestCandidate(value) => value.validate(),
            Self::LocationCandidate(value) => value.validate(),
            Self::FactionCandidate(value) => value.validate(),
            Self::ItemCandidate(value) => value.validate(),
            Self::CreatureCandidate(value) => value.validate(),
            Self::Character(value) => value.validate(),
            Self::Quest(value) => value.validate(),
            Self::Location(value) => value.validate(),
            Self::Faction(value) => value.validate(),
            Self::Item(value) => value.validate(),
            Self::Creature(value) => value.validate(),
        }
    }

    fn set_state(&mut self, state: CanonState) {
        match self {
            Self::CharacterCandidate(value) => value.character.envelope.canon_state = state,
            Self::QuestCandidate(value) => value.quest.envelope.canon_state = state,
            Self::LocationCandidate(value) => value.location.envelope.canon_state = state,
            Self::FactionCandidate(value) => value.faction.envelope.canon_state = state,
            Self::ItemCandidate(value) => value.item.envelope.canon_state = state,
            Self::CreatureCandidate(value) => value.creature.envelope.canon_state = state,
            Self::Character(value) => value.envelope.canon_state = state,
            Self::Quest(value) => value.envelope.canon_state = state,
            Self::Location(value) => value.envelope.canon_state = state,
            Self::Faction(value) => value.envelope.canon_state = state,
            Self::Item(value) => value.envelope.canon_state = state,
            Self::Creature(value) => value.envelope.canon_state = state,
        }
    }

    fn set_favorite(&mut self, favorite: bool) {
        match self {
            Self::CharacterCandidate(value) => value.character.envelope.favorite = favorite,
            Self::QuestCandidate(value) => value.quest.envelope.favorite = favorite,
            Self::LocationCandidate(value) => value.location.envelope.favorite = favorite,
            Self::FactionCandidate(value) => value.faction.envelope.favorite = favorite,
            Self::ItemCandidate(value) => value.item.envelope.favorite = favorite,
            Self::CreatureCandidate(value) => value.creature.envelope.favorite = favorite,
            Self::Character(value) => value.envelope.favorite = favorite,
            Self::Quest(value) => value.envelope.favorite = favorite,
            Self::Location(value) => value.envelope.favorite = favorite,
            Self::Faction(value) => value.envelope.favorite = favorite,
            Self::Item(value) => value.envelope.favorite = favorite,
            Self::Creature(value) => value.envelope.favorite = favorite,
        }
    }

    fn set_updated_at(&mut self, updated_at: i64) {
        match self {
            Self::CharacterCandidate(value) => {
                value.character.envelope.updated_at_unix_ms = updated_at
            }
            Self::QuestCandidate(value) => value.quest.envelope.updated_at_unix_ms = updated_at,
            Self::LocationCandidate(value) => {
                value.location.envelope.updated_at_unix_ms = updated_at
            }
            Self::FactionCandidate(value) => value.faction.envelope.updated_at_unix_ms = updated_at,
            Self::ItemCandidate(value) => value.item.envelope.updated_at_unix_ms = updated_at,
            Self::CreatureCandidate(value) => {
                value.creature.envelope.updated_at_unix_ms = updated_at
            }
            Self::Character(value) => value.envelope.updated_at_unix_ms = updated_at,
            Self::Quest(value) => value.envelope.updated_at_unix_ms = updated_at,
            Self::Location(value) => value.envelope.updated_at_unix_ms = updated_at,
            Self::Faction(value) => value.envelope.updated_at_unix_ms = updated_at,
            Self::Item(value) => value.envelope.updated_at_unix_ms = updated_at,
            Self::Creature(value) => value.envelope.updated_at_unix_ms = updated_at,
        }
    }

    fn generation(&self) -> Option<&GenerationMetadata> {
        self.envelope().generation_provenance.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntityRecord {
    pub entity: StoredEntity,
    pub write_version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeRunRecord {
    pub event_id: i64,
    pub run_id: EntityId,
    pub entity_id: Option<EntityId>,
    pub action: String,
    pub parent_run_id: Option<EntityId>,
    pub recipe_json: String,
    pub result_json: String,
    pub created_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeBundleSaveReceipt {
    pub location_write_version: i64,
    pub relationship_write_version: i64,
    pub forge_event_id: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FactionBundleSaveReceipt {
    pub faction_write_version: i64,
    pub relationship_write_version: i64,
    pub forge_event_id: i64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FactionBundleFailpoint {
    Primary,
    Relationship,
    Event,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ForgeBundleFailpoint {
    AfterLocation,
    AfterRelationship,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForgeBundleHistoryResult {
    pub format_version: u16,
    pub mapping: forge_app_core::ForgeResultMapping,
    pub primary_snapshot: StoredEntity,
}

/// Rust application-facing interface for a single local project database.
pub trait ProjectRepository {
    fn create_project(&mut self, project: &StoredProject) -> StoreResult<()>;
    fn load_project(&self) -> StoreResult<Option<StoredProject>>;
    fn rename_project(&mut self, name: &str, expected_updated_at_unix_ms: i64) -> StoreResult<i64>;
    fn set_setting(&mut self, key: &str, value: &str) -> StoreResult<()>;
    fn get_setting(&self, key: &str) -> StoreResult<Option<String>>;
}

/// Rust application-facing interface for candidate/canon and persistent history operations.
pub trait EntityRepository {
    fn persist_candidate(&mut self, entity: StoredEntity) -> StoreResult<i64>;
    fn save_candidate(&mut self, entity: StoredEntity, expected_version: i64) -> StoreResult<i64>;
    fn promote_candidate(&mut self, id: EntityId, expected_version: i64) -> StoreResult<i64>;
    fn revise_canon(
        &mut self,
        entity: StoredEntity,
        expected_version: i64,
        reason: &str,
    ) -> StoreResult<i64>;
    fn get_entity(&self, id: EntityId) -> StoreResult<Option<EntityRecord>>;
    fn list_entities(&self) -> StoreResult<Vec<EntityRecord>>;
    fn toggle_favorite(
        &mut self,
        id: EntityId,
        favorite: bool,
        expected_version: i64,
    ) -> StoreResult<i64>;
    fn list_history(&self, id: Option<EntityId>) -> StoreResult<Vec<ForgeRunRecord>>;
    fn load_history_snapshot(&self, run_id: EntityId) -> StoreResult<Option<StoredEntity>>;
    fn load_history_event_snapshot(&self, event_id: i64) -> StoreResult<Option<StoredEntity>>;
    fn list_revisions(&self, id: EntityId) -> StoreResult<Vec<EntityRevision>>;
}

/// Persistence boundary for explicit first-class Relationship assertions.
pub trait RelationshipRepository {
    fn create_relationship(&mut self, relationship: Relationship) -> StoreResult<i64>;
    fn get_relationship(&self, id: RelationshipId) -> StoreResult<Option<RelationshipRecord>>;
    fn list_relationships(&self) -> StoreResult<Vec<RelationshipRecord>>;
    fn list_relationships_for_entity(&self, id: EntityId) -> StoreResult<Vec<RelationshipRecord>>;
    fn promote_relationship(
        &mut self,
        id: RelationshipId,
        expected_version: i64,
    ) -> StoreResult<i64>;
    fn revise_relationship(
        &mut self,
        relationship: Relationship,
        expected_version: i64,
        reason: &str,
    ) -> StoreResult<i64>;
    fn archive_relationship(
        &mut self,
        id: RelationshipId,
        expected_version: i64,
    ) -> StoreResult<i64>;
    fn list_relationship_revisions(
        &self,
        id: RelationshipId,
    ) -> StoreResult<Vec<RelationshipRevision>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntityRevision {
    pub revision_number: i64,
    pub reason: String,
    pub entity: StoredEntity,
    pub created_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationshipRecord {
    pub relationship: Relationship,
    pub write_version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipRevision {
    pub revision_number: i64,
    pub reason: String,
    pub relationship: Relationship,
    pub created_at_unix_ms: i64,
}

/// Presentation of one persisted assertion from either endpoint; never an inverse row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionRole {
    Source,
    Target,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedAvailability {
    Available,
    Archived,
    Missing,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionView {
    pub relationship: Relationship,
    pub write_version: i64,
    pub role: ConnectionRole,
    pub related_id: EntityId,
    pub related_type: EntityType,
    pub related_name: Option<String>,
    pub related_availability: RelatedAvailability,
    pub definition: RelationshipDefinition,
}

#[derive(Debug)]
pub enum StoreError {
    Sqlite(rusqlite::Error),
    Io(std::io::Error),
    Serialization(String),
    Validation(ValidationErrors),
    Corrupt(String),
    UnsupportedSchema(u32),
    NotFound(String),
    Conflict { expected: i64, actual: Option<i64> },
    InvalidOperation(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => write!(f, "SQLite operation failed: {error}"),
            Self::Io(error) => write!(f, "storage file operation failed: {error}"),
            Self::Serialization(error) => write!(f, "stored data could not be decoded: {error}"),
            Self::Validation(error) => write!(f, "stored entity failed validation: {error}"),
            Self::Corrupt(error) => write!(f, "project database is corrupt: {error}"),
            Self::UnsupportedSchema(version) => {
                write!(f, "database schema version {version} is not supported")
            }
            Self::NotFound(value) => write!(f, "not found: {value}"),
            Self::Conflict { expected, actual } => write!(
                f,
                "stale write: expected version {expected}, current version is {actual:?}"
            ),
            Self::InvalidOperation(error) => f.write_str(error),
        }
    }
}

impl Error for StoreError {}
impl From<rusqlite::Error> for StoreError {
    fn from(value: rusqlite::Error) -> Self {
        if matches!(
            value,
            rusqlite::Error::SqliteFailure(ref code, _)
                if matches!(code.code, rusqlite::ErrorCode::NotADatabase | rusqlite::ErrorCode::DatabaseCorrupt)
        ) {
            Self::Corrupt(value.to_string())
        } else {
            Self::Sqlite(value)
        }
    }
}
impl From<std::io::Error> for StoreError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<ValidationErrors> for StoreError {
    fn from(value: ValidationErrors) -> Self {
        Self::Validation(value)
    }
}

pub type StoreResult<T> = Result<T, StoreError>;

pub struct ProjectStore {
    path: PathBuf,
    connection: Connection,
}

impl ProjectStore {
    /// Opens or creates an on-disk project database. Future schema versions are never rewritten.
    pub fn open(path: impl AsRef<Path>) -> StoreResult<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut connection = Connection::open(&path)?;
        connection.busy_timeout(std::time::Duration::from_millis(BUSY_TIMEOUT_MS))?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        migrate(&mut connection, &path)?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.pragma_update(None, "synchronous", "FULL")?;
        connection.pragma_update(None, "wal_autocheckpoint", 1000)?;
        let store = Self { path, connection };
        store.integrity_check()?;
        Ok(store)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Stores a new Candidate assertion. Endpoints must be typed entities in this project.
    pub fn create_relationship(&mut self, relationship: Relationship) -> StoreResult<i64> {
        relationship.validate()?;
        if relationship.state != RelationshipState::Candidate {
            return Err(StoreError::InvalidOperation(
                "new relationships must begin as Candidate".into(),
            ));
        }
        let tx = self.connection.transaction()?;
        insert_candidate_relationship(&tx, &relationship)?;
        tx.commit()?;
        Ok(1)
    }

    pub fn get_relationship(&self, id: RelationshipId) -> StoreResult<Option<RelationshipRecord>> {
        self.connection
            .query_row(
                "SELECT body_json, write_version FROM relationships WHERE id=?1",
                [id_text(id.uuid())],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)),
            )
            .optional()?
            .map(|(body, version)| {
                Ok(RelationshipRecord {
                    relationship: decode_relationship(&body)?,
                    write_version: version,
                })
            })
            .transpose()
    }

    pub fn list_relationships(&self) -> StoreResult<Vec<RelationshipRecord>> {
        let mut statement = self
            .connection
            .prepare("SELECT body_json,write_version FROM relationships ORDER BY created_at,id")?;
        let rows = statement.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;
        rows.map(|row| {
            let (body, write_version) = row?;
            Ok(RelationshipRecord {
                relationship: decode_relationship(&body)?,
                write_version,
            })
        })
        .collect()
    }

    pub fn list_relationships_for_entity(
        &self,
        id: EntityId,
    ) -> StoreResult<Vec<RelationshipRecord>> {
        let mut statement = self.connection.prepare("SELECT body_json,write_version FROM relationships WHERE source_id=?1 OR target_id=?1 ORDER BY created_at,id")?;
        let rows = statement.query_map([id_text(id.uuid())], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;
        rows.map(|row| {
            let (body, write_version) = row?;
            Ok(RelationshipRecord {
                relationship: decode_relationship(&body)?,
                write_version,
            })
        })
        .collect()
    }

    /// Resolves only this project's saved endpoints, including archived assertions.
    pub fn list_connection_views(&self, id: EntityId) -> StoreResult<Vec<ConnectionView>> {
        let viewed = self
            .get_entity(id)?
            .ok_or_else(|| StoreError::NotFound(id_text(id.uuid())))?;
        let project_id = viewed.entity.envelope().project_id;
        self.list_relationships_for_entity(id)?
            .into_iter()
            .map(|record| {
                let relationship = record.relationship;
                if relationship.project_id != project_id {
                    return Err(StoreError::Corrupt(
                        "connection crosses project boundary".into(),
                    ));
                }
                let definition = *relationship_definition(relationship.relationship_type);
                let (role, related_id, related_type, viewed_type) = if relationship.source_id == id
                {
                    (
                        ConnectionRole::Source,
                        relationship.target_id,
                        definition.target,
                        definition.source,
                    )
                } else {
                    (
                        ConnectionRole::Target,
                        relationship.source_id,
                        definition.source,
                        definition.target,
                    )
                };
                if viewed.entity.envelope().entity_type != viewed_type {
                    return Err(StoreError::Corrupt(
                        "connection endpoint type disagrees with definition".into(),
                    ));
                }
                let related = self.get_entity(related_id)?;
                let (related_name, related_availability) = match related {
                    None => (None, RelatedAvailability::Missing),
                    Some(record) => {
                        let envelope = record.entity.envelope();
                        if envelope.project_id != project_id || envelope.entity_type != related_type
                        {
                            return Err(StoreError::Corrupt(
                                "related endpoint crosses project or type boundary".into(),
                            ));
                        }
                        (
                            Some(envelope.name.clone()),
                            if envelope.canon_state == CanonState::Archived {
                                RelatedAvailability::Archived
                            } else {
                                RelatedAvailability::Available
                            },
                        )
                    }
                };
                Ok(ConnectionView {
                    relationship,
                    write_version: record.write_version,
                    role,
                    related_id,
                    related_type,
                    related_name,
                    related_availability,
                    definition,
                })
            })
            .collect()
    }

    pub fn promote_relationship(&mut self, id: RelationshipId, expected: i64) -> StoreResult<i64> {
        self.mutate_relationship(
            id,
            expected,
            Some("promoted to canon"),
            RelationshipState::Canon,
        )
    }

    pub fn revise_relationship(
        &mut self,
        mut relationship: Relationship,
        expected: i64,
        reason: &str,
    ) -> StoreResult<i64> {
        validate_text("relationship.revision_reason", reason)?;
        let prior = self
            .get_relationship(relationship.id)?
            .ok_or_else(|| StoreError::NotFound(id_text(relationship.id.uuid())))?;
        if prior.relationship.state != RelationshipState::Canon
            || relationship.state != RelationshipState::Canon
        {
            return Err(StoreError::InvalidOperation(
                "only Canon relationships may be explicitly revised".into(),
            ));
        }
        if prior.relationship.project_id != relationship.project_id
            || prior.relationship.relationship_type != relationship.relationship_type
            || prior.relationship.source_id != relationship.source_id
            || prior.relationship.target_id != relationship.target_id
        {
            return Err(StoreError::InvalidOperation(
                "relationship identity, project, type, and endpoints are immutable".into(),
            ));
        }
        relationship.created_at_unix_ms = prior.relationship.created_at_unix_ms;
        relationship.updated_at_unix_ms = now_unix_ms()?.max(prior.relationship.updated_at_unix_ms);
        relationship.validate()?;
        let tx = self.connection.transaction()?;
        let current = relationship_version_tx(&tx, relationship.id)?;
        check_version(expected, current)?;
        let revision_number: i64 = tx.query_row("SELECT COALESCE(MAX(revision_number),0)+1 FROM relationship_revisions WHERE relationship_id=?1", [id_text(relationship.id.uuid())], |row| row.get(0))?;
        let body = encode_json(&relationship)?;
        tx.execute("INSERT INTO relationship_revisions(relationship_id,revision_number,reason,body_json,created_at) VALUES (?1,?2,?3,?4,?5)", params![id_text(relationship.id.uuid()), revision_number, reason, body, relationship.updated_at_unix_ms])?;
        let changed = tx.execute("UPDATE relationships SET origin=?1,body_json=?2,updated_at=?3,write_version=write_version+1 WHERE id=?4 AND write_version=?5", params![origin(relationship.origin), body, relationship.updated_at_unix_ms, id_text(relationship.id.uuid()), expected])?;
        if changed != 1 {
            return Err(StoreError::Conflict {
                expected,
                actual: relationship_version_tx(&tx, relationship.id)?,
            });
        }
        tx.commit()?;
        Ok(expected + 1)
    }

    pub fn archive_relationship(&mut self, id: RelationshipId, expected: i64) -> StoreResult<i64> {
        self.mutate_relationship(
            id,
            expected,
            Some("archive relationship"),
            RelationshipState::Archived,
        )
    }

    pub fn list_relationship_revisions(
        &self,
        id: RelationshipId,
    ) -> StoreResult<Vec<RelationshipRevision>> {
        let mut statement = self.connection.prepare("SELECT revision_number,reason,body_json,created_at FROM relationship_revisions WHERE relationship_id=?1 ORDER BY revision_number")?;
        let rows = statement.query_map([id_text(id.uuid())], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })?;
        rows.map(|row| {
            let (revision_number, reason, body, created_at_unix_ms) = row?;
            Ok(RelationshipRevision {
                revision_number,
                reason,
                relationship: decode_relationship(&body)?,
                created_at_unix_ms,
            })
        })
        .collect()
    }

    fn mutate_relationship(
        &mut self,
        id: RelationshipId,
        expected: i64,
        reason: Option<&str>,
        state: RelationshipState,
    ) -> StoreResult<i64> {
        let tx = self.connection.transaction()?;
        let (body, current) = tx
            .query_row(
                "SELECT body_json,write_version FROM relationships WHERE id=?1",
                [id_text(id.uuid())],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)),
            )
            .optional()?
            .ok_or_else(|| StoreError::NotFound(id_text(id.uuid())))?;
        check_version(expected, Some(current))?;
        let mut relationship = decode_relationship(&body)?;
        // One timestamp for the mutation and promotion snapshot; separate clock reads
        // previously allowed current Canon and its revision to differ by 1 ms.
        let mutation_time = now_unix_ms()?.max(relationship.updated_at_unix_ms);
        if relationship.state == RelationshipState::Archived {
            return Err(StoreError::InvalidOperation(
                "archived relationship is immutable".into(),
            ));
        }
        if state == RelationshipState::Canon && relationship.state != RelationshipState::Candidate {
            return Err(StoreError::InvalidOperation(
                "only Candidate relationships can be promoted".into(),
            ));
        }
        if state == RelationshipState::Canon {
            relationship.state = RelationshipState::Canon;
            relationship.updated_at_unix_ms = mutation_time;
        }
        if let Some(reason) = reason {
            let revision_number: i64 = tx.query_row("SELECT COALESCE(MAX(revision_number),0)+1 FROM relationship_revisions WHERE relationship_id=?1", [id_text(id.uuid())], |row| row.get(0))?;
            let snapshot = if state == RelationshipState::Canon {
                encode_json(&relationship)?
            } else {
                body
            };
            tx.execute("INSERT INTO relationship_revisions(relationship_id,revision_number,reason,body_json,created_at) VALUES (?1,?2,?3,?4,?5)", params![id_text(id.uuid()), revision_number, reason, snapshot, mutation_time])?;
        }
        relationship.state = state;
        relationship.updated_at_unix_ms = mutation_time;
        let new_body = encode_json(&relationship)?;
        let changed = tx.execute("UPDATE relationships SET state=?1,body_json=?2,updated_at=?3,write_version=write_version+1 WHERE id=?4 AND write_version=?5", params![relationship_state(state), new_body, relationship.updated_at_unix_ms, id_text(id.uuid()), expected])?;
        if changed != 1 {
            return Err(StoreError::Conflict {
                expected,
                actual: relationship_version_tx(&tx, id)?,
            });
        }
        tx.commit()?;
        Ok(expected + 1)
    }

    pub fn create_project(&mut self, project: &StoredProject) -> StoreResult<()> {
        project.validate()?;
        let existing: i64 =
            self.connection
                .query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))?;
        if existing != 0 {
            return Err(StoreError::InvalidOperation(
                "a project database can contain exactly one project".into(),
            ));
        }
        self.connection.execute(
            "INSERT INTO projects(id, name, schema_version, generator_version, content_version, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                id_text(project.envelope.id.uuid()), project.name,
                CURRENT_SCHEMA_VERSION as u16, project.envelope.generator_version.0,
                project.envelope.content_version.0, project.created_at_unix_ms,
                project.updated_at_unix_ms
            ],
        )?;
        Ok(())
    }

    pub fn load_project(&self) -> StoreResult<Option<StoredProject>> {
        let row = self.connection.query_row(
            "SELECT id, name, schema_version, generator_version, content_version, created_at, updated_at FROM projects LIMIT 1",
            [],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, u16>(2)?, row.get::<_, u16>(3)?, row.get::<_, u16>(4)?, row.get::<_, i64>(5)?, row.get::<_, i64>(6)?)),
        ).optional()?;
        let project = row
            .map(|(id, name, schema, generator, content, created, updated)| {
                Ok::<StoredProject, StoreError>(StoredProject {
                    envelope: ProjectEnvelope {
                        id: ProjectId(id.parse::<Uuid128>().map_err(StoreError::Validation)?),
                        schema_version: SchemaVersion(schema),
                        generator_version: forge_contracts::GeneratorVersion(generator),
                        content_version: forge_contracts::ContentVersion(content),
                    },
                    name,
                    created_at_unix_ms: created,
                    updated_at_unix_ms: updated,
                })
            })
            .transpose()?;
        if let Some(project) = &project {
            project
                .validate()
                .map_err(|error| StoreError::Corrupt(error.to_string()))?;
        }
        Ok(project)
    }

    pub fn rename_project(
        &mut self,
        name: &str,
        expected_updated_at_unix_ms: i64,
    ) -> StoreResult<i64> {
        validate_text("project.name", name)?;
        let tx = self.connection.transaction()?;
        let current: Option<i64> = tx
            .query_row("SELECT updated_at FROM projects LIMIT 1", [], |row| {
                row.get(0)
            })
            .optional()?;
        check_version(expected_updated_at_unix_ms, current)?;
        let updated_at = now_unix_ms()?.max(expected_updated_at_unix_ms.saturating_add(1));
        tx.execute(
            "UPDATE projects SET name=?1, updated_at=?2 WHERE updated_at=?3",
            params![name, updated_at, expected_updated_at_unix_ms],
        )?;
        tx.commit()?;
        Ok(updated_at)
    }

    /// Inserts a new candidate and its persistent generation-history record atomically.
    pub fn persist_candidate(&mut self, entity: StoredEntity) -> StoreResult<i64> {
        entity.validate()?;
        if !matches!(
            entity.envelope().canon_state,
            CanonState::Candidate | CanonState::Draft
        ) {
            return Err(StoreError::InvalidOperation(
                "generated work must be persisted as a candidate or draft".into(),
            ));
        }
        let tx = self.connection.transaction()?;
        insert_entity(&tx, &entity)?;
        insert_generation_run(&tx, &entity)?;
        tx.commit()?;
        Ok(1)
    }

    /// Atomically saves a reviewed Forge Further bundle after revalidating its exact source version.
    pub fn save_forge_bundle(
        &mut self,
        source_id: EntityId,
        expected_source_version: i64,
        bundle: &ForgeResultBundle,
    ) -> StoreResult<ForgeBundleSaveReceipt> {
        self.save_forge_bundle_inner(source_id, expected_source_version, bundle, None)
    }

    fn save_forge_bundle_inner(
        &mut self,
        source_id: EntityId,
        expected_source_version: i64,
        bundle: &ForgeResultBundle,
        fail_at: Option<ForgeBundleFailpoint>,
    ) -> StoreResult<ForgeBundleSaveReceipt> {
        bundle.primary.validate()?;
        bundle.relationship.validate()?;
        let location = StoredEntity::LocationCandidate(bundle.primary.clone());
        if !bundle.supporting.is_empty()
            || bundle.relationship.state != RelationshipState::Candidate
            || bundle.relationship.origin != EntityOrigin::Generated
            || bundle.relationship.source_id != source_id
            || bundle.relationship.target_id != location.envelope().id
            || bundle.result.source_entity_id != source_id
            || bundle.result.primary_output_id != location.envelope().id
            || bundle.result.relationship_ids != [bundle.relationship.id]
            || bundle.result.relationship_type != bundle.relationship.relationship_type
            || bundle.recipe.context_snapshot.source_entity_id != source_id
            || bundle.recipe.context_snapshot.source_write_version != expected_source_version
            || bundle.recipe.location_dna != bundle.primary.dna
        {
            return Err(StoreError::InvalidOperation(
                "Forge Hometown bundle does not match its frozen source and output mapping".into(),
            ));
        }
        let tx = self.connection.transaction()?;
        let source = get_entity_tx(&tx, source_id)?
            .ok_or_else(|| StoreError::NotFound(id_text(source_id.uuid())))?;
        check_version(expected_source_version, Some(source.write_version))?;
        let source_snapshot = hometown_source(&source)?;
        let compiled =
            compile_hometown_context(&source_snapshot).map_err(StoreError::Validation)?;
        if compiled != bundle.recipe.context_snapshot {
            return Err(StoreError::InvalidOperation(
                "Forge Hometown context no longer matches the persisted source".into(),
            ));
        }
        if source.entity.envelope().project_id != location.envelope().project_id
            || source.entity.envelope().project_id != bundle.relationship.project_id
        {
            return Err(StoreError::InvalidOperation(
                "Forge Hometown bundle endpoints must belong to the active project".into(),
            ));
        }
        insert_entity(&tx, &location)?;
        if fail_at == Some(ForgeBundleFailpoint::AfterLocation) {
            return Err(StoreError::InvalidOperation(
                "injected Forge Bundle failure after Location insert".into(),
            ));
        }
        insert_candidate_relationship(&tx, &bundle.relationship)?;
        if fail_at == Some(ForgeBundleFailpoint::AfterRelationship) {
            return Err(StoreError::InvalidOperation(
                "injected Forge Bundle failure after Relationship insert".into(),
            ));
        }
        let generation = location.generation().ok_or_else(|| {
            StoreError::InvalidOperation(
                "Hometown Location must retain generation provenance".into(),
            )
        })?;
        let recipe_json = encode_json(&bundle.recipe)?;
        let result_json = encode_json(&ForgeBundleHistoryResult {
            format_version: 1,
            mapping: bundle.result.clone(),
            primary_snapshot: location.clone(),
        })?;
        tx.execute(
            "INSERT INTO forge_runs(run_id,project_id,entity_id,action,parent_run_id,recipe_json,result_json,created_at) VALUES (?1,?2,?3,'forge_hometown',NULL,?4,?5,?6)",
            params![
                id_text(generation.generation_id.uuid()),
                id_text(location.envelope().project_id.uuid()),
                id_text(location.envelope().id.uuid()),
                recipe_json,
                result_json,
                generation.generated_at_unix_ms,
            ],
        )?;
        let forge_event_id = tx.last_insert_rowid();
        tx.commit()?;
        Ok(ForgeBundleSaveReceipt {
            location_write_version: 1,
            relationship_write_version: 1,
            forge_event_id,
        })
    }

    pub fn save_faction_bundle(
        &mut self,
        source_id: EntityId,
        expected_source_version: i64,
        bundle: &FactionResultBundle,
    ) -> StoreResult<FactionBundleSaveReceipt> {
        self.save_faction_bundle_inner(source_id, expected_source_version, bundle, None)
    }

    fn save_faction_bundle_inner(
        &mut self,
        source_id: EntityId,
        expected_source_version: i64,
        bundle: &FactionResultBundle,
        fail_at: Option<FactionBundleFailpoint>,
    ) -> StoreResult<FactionBundleSaveReceipt> {
        validate_faction_bundle(bundle)?;
        if bundle.recipe.context_snapshot.source_entity_id != source_id
            || bundle.recipe.context_snapshot.source_write_version != expected_source_version
        {
            return Err(StoreError::InvalidOperation(
                "Faction bundle source/version does not match the reviewed context".into(),
            ));
        }
        let primary = StoredEntity::FactionCandidate(bundle.primary.clone());
        let tx = self.connection.transaction()?;
        let source = get_entity_tx(&tx, source_id)?
            .ok_or_else(|| StoreError::NotFound(id_text(source_id.uuid())))?;
        check_version(expected_source_version, Some(source.write_version))?;
        let snapshot = compile_faction_context(&hometown_source(&source)?)?;
        if snapshot != bundle.recipe.context_snapshot
            || source.entity.envelope().project_id != primary.envelope().project_id
        {
            return Err(StoreError::InvalidOperation(
                "Faction bundle no longer matches its persisted source context/project".into(),
            ));
        }
        insert_entity(&tx, &primary)?;
        if fail_at == Some(FactionBundleFailpoint::Primary) {
            return Err(StoreError::InvalidOperation(
                "injected Faction bundle failure after primary".into(),
            ));
        }
        insert_candidate_relationship(&tx, &bundle.relationship)?;
        if fail_at == Some(FactionBundleFailpoint::Relationship) {
            return Err(StoreError::InvalidOperation(
                "injected Faction bundle failure after relationship".into(),
            ));
        }
        let generation = primary.generation().ok_or_else(|| {
            StoreError::InvalidOperation("Faction must retain generation provenance".into())
        })?;
        let recipe_json = encode_json(&bundle.recipe)?;
        let result_json = encode_json(&ForgeBundleHistoryResult {
            format_version: 1,
            mapping: bundle.result.clone(),
            primary_snapshot: primary.clone(),
        })?;
        tx.execute("INSERT INTO forge_runs(run_id,project_id,entity_id,action,parent_run_id,recipe_json,result_json,created_at) VALUES (?1,?2,?3,'forge_character_faction',NULL,?4,?5,?6)",
            params![id_text(generation.generation_id.uuid()), id_text(primary.envelope().project_id.uuid()), id_text(primary.envelope().id.uuid()), recipe_json, result_json, generation.generated_at_unix_ms])?;
        let forge_event_id = tx.last_insert_rowid();
        if fail_at == Some(FactionBundleFailpoint::Event) {
            return Err(StoreError::InvalidOperation(
                "injected Faction bundle failure after event".into(),
            ));
        }
        tx.commit()?;
        Ok(FactionBundleSaveReceipt {
            faction_write_version: 1,
            relationship_write_version: 1,
            forge_event_id,
        })
    }

    /// Applies an explicit candidate/manual edit. An entity must be saved with its current version.
    pub fn save_candidate(
        &mut self,
        entity: StoredEntity,
        expected_version: i64,
    ) -> StoreResult<i64> {
        entity.validate()?;
        if entity.envelope().canon_state == CanonState::Canon {
            return Err(StoreError::InvalidOperation(
                "canon changes require revise_canon with an explicit reason".into(),
            ));
        }
        let tx = self.connection.transaction()?;
        let existing = get_entity_tx(&tx, entity.envelope().id)?
            .ok_or_else(|| StoreError::NotFound(id_text(entity.envelope().id.uuid())))?;
        if existing.entity.envelope().canon_state == CanonState::Canon {
            return Err(StoreError::InvalidOperation(
                "canon entities can only change through revise_canon".into(),
            ));
        }
        let updated = update_entity(&tx, &entity, expected_version)?;
        insert_generation_run(&tx, &updated)?;
        tx.commit()?;
        Ok(expected_version + 1)
    }

    /// Explicitly promotes a candidate to canon, snapshotting revision 1 and preserving its ID.
    pub fn promote_candidate(&mut self, id: EntityId, expected_version: i64) -> StoreResult<i64> {
        let tx = self.connection.transaction()?;
        let mut record =
            get_entity_tx(&tx, id)?.ok_or_else(|| StoreError::NotFound(id_text(id.uuid())))?;
        check_version(expected_version, Some(record.write_version))?;
        if !matches!(
            record.entity.envelope().canon_state,
            CanonState::Candidate | CanonState::Draft
        ) {
            return Err(StoreError::InvalidOperation(
                "only a candidate or draft may be promoted".into(),
            ));
        }
        record.entity.set_state(CanonState::Canon);
        let promoted = update_entity(&tx, &record.entity, expected_version)?;
        insert_revision(&tx, id, 1, "promoted to canon", &promoted)?;
        tx.commit()?;
        Ok(expected_version + 1)
    }

    /// Explicitly revises a canon record. The previous canon snapshot is stored first in the same
    /// transaction; stale writes and any failure leave both current state and revisions untouched.
    pub fn revise_canon(
        &mut self,
        entity: StoredEntity,
        expected_version: i64,
        reason: &str,
    ) -> StoreResult<i64> {
        validate_text("revision.reason", reason)?;
        if entity.envelope().canon_state != CanonState::Canon {
            return Err(StoreError::InvalidOperation(
                "revision target must remain canon".into(),
            ));
        }
        entity.validate()?;
        let id = entity.envelope().id;
        let tx = self.connection.transaction()?;
        let previous =
            get_entity_tx(&tx, id)?.ok_or_else(|| StoreError::NotFound(id_text(id.uuid())))?;
        check_version(expected_version, Some(previous.write_version))?;
        if previous.entity.envelope().canon_state != CanonState::Canon {
            return Err(StoreError::InvalidOperation(
                "only canon entities can receive canon revisions".into(),
            ));
        }
        let next_revision: i64 = tx.query_row(
            "SELECT COALESCE(MAX(revision_number), 0) + 1 FROM entity_revisions WHERE entity_id = ?1",
            [id_text(id.uuid())], |row| row.get(0),
        )?;
        let updated = update_entity(&tx, &entity, expected_version)?;
        insert_revision(&tx, id, next_revision, reason, &updated)?;
        insert_generation_run(&tx, &updated)?;
        tx.commit()?;
        Ok(expected_version + 1)
    }

    pub fn get_entity(&self, id: EntityId) -> StoreResult<Option<EntityRecord>> {
        get_entity_connection(&self.connection, id)
    }

    pub fn list_entities(&self) -> StoreResult<Vec<EntityRecord>> {
        let mut statement = self
            .connection
            .prepare("SELECT body_json, write_version, entity_type, name, canon_state, origin, favorite, schema_version, project_id FROM entities ORDER BY created_at, id")?;
        let rows = statement.query_map([], raw_entity_row)?;
        let rows = rows.collect::<Result<Vec<_>, _>>()?;
        rows.into_iter().map(decode_entity_row).collect()
    }

    pub fn toggle_favorite(
        &mut self,
        id: EntityId,
        favorite: bool,
        expected_version: i64,
    ) -> StoreResult<i64> {
        let tx = self.connection.transaction()?;
        let mut record =
            get_entity_tx(&tx, id)?.ok_or_else(|| StoreError::NotFound(id_text(id.uuid())))?;
        check_version(expected_version, Some(record.write_version))?;
        record.entity.set_favorite(favorite);
        update_entity(&tx, &record.entity, expected_version)?;
        tx.commit()?;
        Ok(expected_version + 1)
    }

    pub fn list_history(&self, id: Option<EntityId>) -> StoreResult<Vec<ForgeRunRecord>> {
        let sql = if id.is_some() {
            "SELECT event_id, run_id, entity_id, action, parent_run_id, recipe_json, result_json, created_at FROM forge_runs WHERE entity_id = ?1 ORDER BY event_id"
        } else {
            "SELECT event_id, run_id, entity_id, action, parent_run_id, recipe_json, result_json, created_at FROM forge_runs ORDER BY event_id"
        };
        let mut statement = self.connection.prepare(sql)?;
        let mut records = Vec::new();
        let collect = |row: &rusqlite::Row<'_>| -> rusqlite::Result<ForgeRunRecord> {
            Ok(ForgeRunRecord {
                event_id: row.get(0)?,
                run_id: parse_id(row.get::<_, String>(1)?).map_err(to_sql_conversion_error)?,
                entity_id: row
                    .get::<_, Option<String>>(2)?
                    .map(|v| parse_id(v).map_err(to_sql_conversion_error))
                    .transpose()?,
                action: row.get(3)?,
                parent_run_id: row
                    .get::<_, Option<String>>(4)?
                    .map(|v| parse_id(v).map_err(to_sql_conversion_error))
                    .transpose()?,
                recipe_json: row.get(5)?,
                result_json: row.get(6)?,
                created_at_unix_ms: row.get(7)?,
            })
        };
        if let Some(id) = id {
            let rows = statement.query_map([id_text(id.uuid())], |row| collect(row))?;
            for row in rows {
                records.push(row?);
            }
        } else {
            let rows = statement.query_map([], |row| collect(row))?;
            for row in rows {
                records.push(row?);
            }
        }
        Ok(records)
    }

    /// Returns an immutable generated snapshot for restoration into the editing session.
    /// Saving the restored state remains a new generation-history action.
    pub fn load_history_snapshot(&self, run_id: EntityId) -> StoreResult<Option<StoredEntity>> {
        let result = self
            .connection
            .query_row(
                "SELECT result_json FROM forge_runs WHERE run_id=?1 ORDER BY created_at DESC, rowid DESC LIMIT 1",
                [id_text(run_id.uuid())],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        result
            .map(|json| decode_history_snapshot(&json))
            .transpose()
    }

    /// Loads the exact immutable snapshot represented by a history row.
    pub fn load_history_event_snapshot(&self, event_id: i64) -> StoreResult<Option<StoredEntity>> {
        let result = self
            .connection
            .query_row(
                "SELECT result_json FROM forge_runs WHERE event_id=?1",
                [event_id],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        result
            .map(|json| decode_history_snapshot(&json))
            .transpose()
    }

    pub fn list_revisions(&self, id: EntityId) -> StoreResult<Vec<EntityRevision>> {
        let mut statement = self.connection.prepare(
            "SELECT revision_number, reason, body_json, created_at FROM entity_revisions WHERE entity_id = ?1 ORDER BY revision_number",
        )?;
        let rows = statement.query_map([id_text(id.uuid())], |row| {
            let body: String = row.get(2)?;
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                body,
                row.get::<_, i64>(3)?,
            ))
        })?;
        rows.map(|row| {
            let (revision_number, reason, body, created_at_unix_ms) = row?;
            Ok(EntityRevision {
                revision_number,
                reason,
                entity: decode_stored_entity(&body)?,
                created_at_unix_ms,
            })
        })
        .collect()
    }

    pub fn set_setting(&mut self, key: &str, value: &str) -> StoreResult<()> {
        validate_text("setting.key", key)?;
        validate_text("setting.value", value)?;
        serde_json::from_str::<serde_json::Value>(value)
            .map_err(|error| StoreError::Serialization(error.to_string()))?;
        self.connection.execute(
            "INSERT INTO settings(key, value_json, updated_at) VALUES (?1, ?2, ?3) ON CONFLICT(key) DO UPDATE SET value_json=excluded.value_json, updated_at=excluded.updated_at",
            params![key, value, now_unix_ms()?],
        )?;
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> StoreResult<Option<String>> {
        self.connection
            .query_row(
                "SELECT value_json FROM settings WHERE key = ?1",
                [key],
                |row| row.get(0),
            )
            .optional()
            .map_err(StoreError::from)
    }

    pub fn integrity_check(&self) -> StoreResult<()> {
        let result: String = self
            .connection
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
        if result != "ok" {
            return Err(StoreError::Corrupt(result));
        }
        let violations: i64 = self.connection.query_row(
            "SELECT COUNT(*) FROM pragma_foreign_key_check",
            [],
            |row| row.get(0),
        )?;
        if violations != 0 {
            return Err(StoreError::Corrupt(format!(
                "{violations} foreign-key violations"
            )));
        }
        let mut statement = self.connection.prepare("SELECT r.id,r.project_id,r.relationship_type,r.source_id,r.target_id,r.state,r.origin,r.body_json,r.created_at,r.updated_at,s.entity_type,t.entity_type FROM relationships r JOIN entities s ON s.project_id=r.project_id AND s.id=r.source_id JOIN entities t ON t.project_id=r.project_id AND t.id=r.target_id")?;
        let rows = statement.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, i64>(8)?,
                row.get::<_, i64>(9)?,
                row.get::<_, String>(10)?,
                row.get::<_, String>(11)?,
            ))
        })?;
        for row in rows {
            let (
                id,
                project,
                kind,
                source,
                target,
                state,
                origin_text,
                body,
                created,
                updated,
                source_type,
                target_type,
            ) = row?;
            let relation = decode_relationship(&body)?;
            if id != id_text(relation.id.uuid())
                || project != id_text(relation.project_id.uuid())
                || kind != relation.relationship_type.key()
                || source != id_text(relation.source_id.uuid())
                || target != id_text(relation.target_id.uuid())
                || state != relationship_state(relation.state)
                || origin_text != origin(relation.origin)
                || created != relation.created_at_unix_ms
                || updated != relation.updated_at_unix_ms
            {
                return Err(StoreError::Corrupt(
                    "relationship columns disagree with the stored typed contract".into(),
                ));
            }
            validate_relationship_endpoints(
                &relation,
                entity_type_from_db(source_type)?,
                entity_type_from_db(target_type)?,
            )
            .map_err(|error| StoreError::Corrupt(error.to_string()))?;
        }
        Ok(())
    }

    pub fn checkpoint(&self) -> StoreResult<()> {
        self.connection
            .query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |row| {
                row.get::<_, i64>(0)
            })
            .map(|_| ())
            .map_err(StoreError::from)
    }

    /// Creates a standalone SQLite snapshot including committed WAL pages without closing this store.
    pub fn snapshot_into(&self, destination: impl AsRef<Path>) -> StoreResult<()> {
        let destination = destination.as_ref();
        if destination.exists() {
            return Err(StoreError::InvalidOperation(
                "snapshot destination already exists".into(),
            ));
        }
        let destination_text = destination.to_string_lossy();
        self.connection
            .execute("VACUUM INTO ?1", [destination_text.as_ref()])?;
        Ok(())
    }
}

impl ProjectRepository for ProjectStore {
    fn create_project(&mut self, project: &StoredProject) -> StoreResult<()> {
        ProjectStore::create_project(self, project)
    }
    fn load_project(&self) -> StoreResult<Option<StoredProject>> {
        ProjectStore::load_project(self)
    }
    fn rename_project(&mut self, name: &str, expected_updated_at_unix_ms: i64) -> StoreResult<i64> {
        ProjectStore::rename_project(self, name, expected_updated_at_unix_ms)
    }
    fn set_setting(&mut self, key: &str, value: &str) -> StoreResult<()> {
        ProjectStore::set_setting(self, key, value)
    }
    fn get_setting(&self, key: &str) -> StoreResult<Option<String>> {
        ProjectStore::get_setting(self, key)
    }
}

impl EntityRepository for ProjectStore {
    fn persist_candidate(&mut self, entity: StoredEntity) -> StoreResult<i64> {
        ProjectStore::persist_candidate(self, entity)
    }
    fn save_candidate(&mut self, entity: StoredEntity, expected_version: i64) -> StoreResult<i64> {
        ProjectStore::save_candidate(self, entity, expected_version)
    }
    fn promote_candidate(&mut self, id: EntityId, expected_version: i64) -> StoreResult<i64> {
        ProjectStore::promote_candidate(self, id, expected_version)
    }
    fn revise_canon(
        &mut self,
        entity: StoredEntity,
        expected_version: i64,
        reason: &str,
    ) -> StoreResult<i64> {
        ProjectStore::revise_canon(self, entity, expected_version, reason)
    }
    fn get_entity(&self, id: EntityId) -> StoreResult<Option<EntityRecord>> {
        ProjectStore::get_entity(self, id)
    }
    fn list_entities(&self) -> StoreResult<Vec<EntityRecord>> {
        ProjectStore::list_entities(self)
    }
    fn toggle_favorite(
        &mut self,
        id: EntityId,
        favorite: bool,
        expected_version: i64,
    ) -> StoreResult<i64> {
        ProjectStore::toggle_favorite(self, id, favorite, expected_version)
    }
    fn list_history(&self, id: Option<EntityId>) -> StoreResult<Vec<ForgeRunRecord>> {
        ProjectStore::list_history(self, id)
    }
    fn load_history_snapshot(&self, run_id: EntityId) -> StoreResult<Option<StoredEntity>> {
        ProjectStore::load_history_snapshot(self, run_id)
    }

    fn load_history_event_snapshot(&self, event_id: i64) -> StoreResult<Option<StoredEntity>> {
        ProjectStore::load_history_event_snapshot(self, event_id)
    }
    fn list_revisions(&self, id: EntityId) -> StoreResult<Vec<EntityRevision>> {
        ProjectStore::list_revisions(self, id)
    }
}

impl RelationshipRepository for ProjectStore {
    fn create_relationship(&mut self, relationship: Relationship) -> StoreResult<i64> {
        ProjectStore::create_relationship(self, relationship)
    }
    fn get_relationship(&self, id: RelationshipId) -> StoreResult<Option<RelationshipRecord>> {
        ProjectStore::get_relationship(self, id)
    }
    fn list_relationships(&self) -> StoreResult<Vec<RelationshipRecord>> {
        ProjectStore::list_relationships(self)
    }
    fn list_relationships_for_entity(&self, id: EntityId) -> StoreResult<Vec<RelationshipRecord>> {
        ProjectStore::list_relationships_for_entity(self, id)
    }
    fn promote_relationship(
        &mut self,
        id: RelationshipId,
        expected_version: i64,
    ) -> StoreResult<i64> {
        ProjectStore::promote_relationship(self, id, expected_version)
    }
    fn revise_relationship(
        &mut self,
        relationship: Relationship,
        expected_version: i64,
        reason: &str,
    ) -> StoreResult<i64> {
        ProjectStore::revise_relationship(self, relationship, expected_version, reason)
    }
    fn archive_relationship(
        &mut self,
        id: RelationshipId,
        expected_version: i64,
    ) -> StoreResult<i64> {
        ProjectStore::archive_relationship(self, id, expected_version)
    }
    fn list_relationship_revisions(
        &self,
        id: RelationshipId,
    ) -> StoreResult<Vec<RelationshipRevision>> {
        ProjectStore::list_relationship_revisions(self, id)
    }
}

/// Owns the currently-open project handle. A requested switch opens and validates the new
/// database before dropping the active one, so a bad path never loses the current session.
#[derive(Default)]
pub struct ProjectSession {
    active: Option<ProjectStore>,
}

impl ProjectSession {
    pub fn open(&mut self, path: impl AsRef<Path>) -> StoreResult<&ProjectStore> {
        let next = ProjectStore::open(path)?;
        next.integrity_check()?;
        self.active = Some(next);
        Ok(self.active.as_ref().expect("just assigned"))
    }
    pub fn close(&mut self) {
        self.active = None;
    }
    pub fn active(&self) -> Option<&ProjectStore> {
        self.active.as_ref()
    }
    pub fn active_mut(&mut self) -> Option<&mut ProjectStore> {
        self.active.as_mut()
    }
}

/// Session-only undo/redo; persistent forge history and canon revisions are independent.
#[derive(Clone, Debug)]
pub struct SessionHistory<T> {
    undo: Vec<T>,
    redo: Vec<T>,
}

impl<T: Clone> SessionHistory<T> {
    pub fn new(initial: T) -> Self {
        Self {
            undo: vec![initial],
            redo: Vec::new(),
        }
    }
    pub fn current(&self) -> &T {
        self.undo.last().expect("history always has initial state")
    }
    pub fn can_undo(&self) -> bool {
        self.undo.len() > 1
    }
    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
    pub fn push(&mut self, next: T) {
        self.undo.push(next);
        self.redo.clear();
    }
    pub fn undo(&mut self) -> Option<&T> {
        if self.undo.len() <= 1 {
            return None;
        }
        self.redo.push(self.undo.pop().expect("length checked"));
        self.undo.last()
    }
    pub fn redo(&mut self) -> Option<&T> {
        let next = self.redo.pop()?;
        self.undo.push(next);
        self.undo.last()
    }
}

fn migrate(connection: &mut Connection, path: &Path) -> StoreResult<()> {
    let version: u32 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    if version > CURRENT_SCHEMA_VERSION {
        return Err(StoreError::UnsupportedSchema(version));
    }
    if version == CURRENT_SCHEMA_VERSION {
        return Ok(());
    }
    if version == 1 {
        checkpoint_for_migration(connection)?;
        let backup = migration_backup_path(path);
        if backup.exists() {
            return Err(StoreError::InvalidOperation(format!(
                "refusing to overwrite existing migration recovery file {}",
                backup.display()
            )));
        }
        fs::copy(path, &backup)?;
        connection.pragma_update(None, "foreign_keys", "OFF")?;
        let result = (|| -> StoreResult<()> {
            let before_entities: i64 =
                connection.query_row("SELECT COUNT(*) FROM entities", [], |row| row.get(0))?;
            let before_revisions: i64 =
                connection.query_row("SELECT COUNT(*) FROM entity_revisions", [], |row| {
                    row.get(0)
                })?;
            let before_runs: i64 =
                connection.query_row("SELECT COUNT(*) FROM forge_runs", [], |row| row.get(0))?;
            let before_settings: i64 =
                connection.query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0))?;
            let tx = connection.transaction()?;
            tx.execute_batch(r#"
CREATE TABLE entities_v2 (
 id TEXT PRIMARY KEY NOT NULL, project_id TEXT NOT NULL,
 entity_type TEXT NOT NULL CHECK(entity_type IN ('character','quest','location','faction','item','creature')),
 name TEXT NOT NULL CHECK(length(trim(name)) > 0),
 canon_state TEXT NOT NULL CHECK(canon_state IN ('draft','candidate','canon','archived')),
 origin TEXT NOT NULL CHECK(origin IN ('manual','generated','imported')),
 favorite INTEGER NOT NULL DEFAULT 0 CHECK(favorite IN (0,1)),
 schema_version INTEGER NOT NULL CHECK(schema_version > 0),
 body_json TEXT NOT NULL CHECK(json_valid(body_json)),
 generation_json TEXT CHECK(generation_json IS NULL OR json_valid(generation_json)),
 write_version INTEGER NOT NULL CHECK(write_version > 0),
 created_at INTEGER NOT NULL CHECK(created_at >= 0), updated_at INTEGER NOT NULL CHECK(updated_at >= created_at),
 UNIQUE(project_id,id), FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE
);
INSERT INTO entities_v2 SELECT * FROM entities;
DROP TABLE entities;
ALTER TABLE entities_v2 RENAME TO entities;
CREATE INDEX idx_entities_project_state ON entities(project_id,canon_state,updated_at);
CREATE INDEX idx_entities_project_type ON entities(project_id,entity_type,updated_at);
CREATE TABLE relationships (
 id TEXT PRIMARY KEY NOT NULL, project_id TEXT NOT NULL, relationship_type TEXT NOT NULL CHECK(relationship_type IN ('born_in','member_of','owns','opposed_by')),
 source_id TEXT NOT NULL, target_id TEXT NOT NULL, state TEXT NOT NULL CHECK(state IN ('candidate','canon','archived')),
 origin TEXT NOT NULL CHECK(origin IN ('manual','generated','imported')), body_json TEXT NOT NULL CHECK(json_valid(body_json)),
 write_version INTEGER NOT NULL CHECK(write_version > 0), created_at INTEGER NOT NULL CHECK(created_at >= 0), updated_at INTEGER NOT NULL CHECK(updated_at >= created_at),
 UNIQUE(project_id,relationship_type,source_id,target_id), UNIQUE(project_id,id),
 FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE,
 FOREIGN KEY(project_id,source_id) REFERENCES entities(project_id,id),
 FOREIGN KEY(project_id,target_id) REFERENCES entities(project_id,id)
);
CREATE INDEX idx_relationships_project_state ON relationships(project_id,state,updated_at);
CREATE INDEX idx_relationships_source ON relationships(project_id,source_id);
CREATE INDEX idx_relationships_target ON relationships(project_id,target_id);
CREATE TABLE relationship_revisions (
 revision_id INTEGER PRIMARY KEY AUTOINCREMENT, relationship_id TEXT NOT NULL,
 revision_number INTEGER NOT NULL CHECK(revision_number > 0), reason TEXT NOT NULL CHECK(length(trim(reason)) > 0),
 body_json TEXT NOT NULL CHECK(json_valid(body_json)), created_at INTEGER NOT NULL CHECK(created_at >= 0),
 UNIQUE(relationship_id,revision_number), FOREIGN KEY(relationship_id) REFERENCES relationships(id) ON DELETE RESTRICT
);
"#)?;
            let bad: i64 =
                tx.query_row("SELECT COUNT(*) FROM pragma_foreign_key_check", [], |row| {
                    row.get(0)
                })?;
            if bad != 0 {
                return Err(StoreError::Corrupt(format!(
                    "schema-2 migration produced {bad} foreign-key violations"
                )));
            }
            let after_entities: i64 =
                tx.query_row("SELECT COUNT(*) FROM entities", [], |row| row.get(0))?;
            let after_revisions: i64 =
                tx.query_row("SELECT COUNT(*) FROM entity_revisions", [], |row| {
                    row.get(0)
                })?;
            let after_runs: i64 =
                tx.query_row("SELECT COUNT(*) FROM forge_runs", [], |row| row.get(0))?;
            let after_settings: i64 =
                tx.query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0))?;
            if (
                before_entities,
                before_revisions,
                before_runs,
                before_settings,
            ) != (after_entities, after_revisions, after_runs, after_settings)
            {
                return Err(StoreError::Corrupt(
                    "schema-2 migration changed legacy row counts".into(),
                ));
            }
            tx.pragma_update(None, "user_version", CURRENT_SCHEMA_VERSION)?;
            tx.execute(
                "UPDATE projects SET schema_version=?1",
                [CURRENT_SCHEMA_VERSION],
            )?;
            tx.commit()?;
            Ok(())
        })();
        connection.pragma_update(None, "foreign_keys", "ON")?;
        result?;
        return Ok(());
    }
    let has_legacy: bool = connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='legacy_projects')",
        [],
        |row| row.get(0),
    )?;
    let has_tables: bool = connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%')",
        [], |row| row.get(0),
    )?;
    if has_tables && !has_legacy {
        return Err(StoreError::Corrupt(
            "unversioned database has unknown tables".into(),
        ));
    }
    if has_tables {
        connection.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |row| {
            row.get::<_, i64>(0)
        })?;
        let backup = migration_backup_path(path);
        if backup.exists() {
            return Err(StoreError::InvalidOperation(format!(
                "refusing to overwrite existing migration recovery file {}",
                backup.display()
            )));
        }
        fs::copy(path, backup)?;
    }
    let tx = connection.transaction()?;
    tx.execute_batch(SCHEMA_V2)?;
    if has_legacy {
        let legacy_count: i64 =
            tx.query_row("SELECT COUNT(*) FROM legacy_projects", [], |row| row.get(0))?;
        if legacy_count > 1 {
            return Err(StoreError::Corrupt(
                "schema-0 fixture contains more than one project".into(),
            ));
        }
        tx.execute(
            "INSERT INTO projects(id, name, schema_version, generator_version, content_version, created_at, updated_at)
             SELECT id, name, ?1, 1, 1, created_at, updated_at FROM legacy_projects",
            [CURRENT_SCHEMA_VERSION],
        )?;
        tx.execute_batch("DROP TABLE legacy_projects;")?;
    }
    tx.pragma_update(None, "user_version", CURRENT_SCHEMA_VERSION)?;
    tx.commit()?;
    Ok(())
}

fn checkpoint_for_migration(connection: &Connection) -> StoreResult<()> {
    let (busy, log_frames, checkpointed): (i64, i64, i64) =
        connection.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
    if busy != 0 || (log_frames >= 0 && checkpointed != log_frames) {
        return Err(StoreError::InvalidOperation(format!(
            "WAL checkpoint incomplete before migration (busy={busy}, frames={checkpointed}/{log_frames})"
        )));
    }
    Ok(())
}

fn migration_backup_path(path: &Path) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(".pre-migration.bak");
    path.with_file_name(name)
}

const SCHEMA_V2: &str = r#"
CREATE TABLE IF NOT EXISTS projects (
 id TEXT PRIMARY KEY NOT NULL,
 name TEXT NOT NULL CHECK(length(trim(name)) > 0),
 schema_version INTEGER NOT NULL CHECK(schema_version > 0),
 generator_version INTEGER NOT NULL CHECK(generator_version > 0),
 content_version INTEGER NOT NULL CHECK(content_version > 0),
 created_at INTEGER NOT NULL CHECK(created_at >= 0),
 updated_at INTEGER NOT NULL CHECK(updated_at >= created_at)
);
CREATE TABLE IF NOT EXISTS entities (
 id TEXT PRIMARY KEY NOT NULL,
 project_id TEXT NOT NULL,
 entity_type TEXT NOT NULL CHECK(entity_type IN ('character','quest','location','faction','item','creature')),
 name TEXT NOT NULL CHECK(length(trim(name)) > 0),
 canon_state TEXT NOT NULL CHECK(canon_state IN ('draft','candidate','canon','archived')),
 origin TEXT NOT NULL CHECK(origin IN ('manual','generated','imported')),
 favorite INTEGER NOT NULL DEFAULT 0 CHECK(favorite IN (0,1)),
 schema_version INTEGER NOT NULL CHECK(schema_version > 0),
 body_json TEXT NOT NULL CHECK(json_valid(body_json)),
 generation_json TEXT CHECK(generation_json IS NULL OR json_valid(generation_json)),
 write_version INTEGER NOT NULL CHECK(write_version > 0),
 created_at INTEGER NOT NULL CHECK(created_at >= 0),
 updated_at INTEGER NOT NULL CHECK(updated_at >= created_at),
 UNIQUE(project_id, id),
 FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_entities_project_state ON entities(project_id, canon_state, updated_at);
CREATE INDEX IF NOT EXISTS idx_entities_project_type ON entities(project_id, entity_type, updated_at);
CREATE TABLE IF NOT EXISTS entity_revisions (
 revision_id INTEGER PRIMARY KEY AUTOINCREMENT,
 entity_id TEXT NOT NULL,
 revision_number INTEGER NOT NULL CHECK(revision_number > 0),
 reason TEXT NOT NULL CHECK(length(trim(reason)) > 0),
 body_json TEXT NOT NULL CHECK(json_valid(body_json)),
 generation_json TEXT CHECK(generation_json IS NULL OR json_valid(generation_json)),
 created_at INTEGER NOT NULL CHECK(created_at >= 0),
 UNIQUE(entity_id, revision_number),
 FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS forge_runs (
 event_id INTEGER PRIMARY KEY AUTOINCREMENT,
 run_id TEXT NOT NULL,
 project_id TEXT NOT NULL,
 entity_id TEXT,
 action TEXT NOT NULL,
 parent_run_id TEXT,
 recipe_json TEXT NOT NULL CHECK(json_valid(recipe_json)),
 result_json TEXT NOT NULL CHECK(json_valid(result_json)),
 created_at INTEGER NOT NULL CHECK(created_at >= 0),
 FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE,
 FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
);
CREATE INDEX IF NOT EXISTS idx_forge_runs_run ON forge_runs(run_id, created_at);
CREATE INDEX IF NOT EXISTS idx_forge_runs_project_time ON forge_runs(project_id, created_at);
CREATE INDEX IF NOT EXISTS idx_forge_runs_entity_time ON forge_runs(entity_id, created_at);
CREATE TABLE IF NOT EXISTS settings (
 key TEXT PRIMARY KEY NOT NULL,
 value_json TEXT NOT NULL CHECK(json_valid(value_json)),
 updated_at INTEGER NOT NULL CHECK(updated_at >= 0)
);
CREATE TABLE IF NOT EXISTS relationships (
 id TEXT PRIMARY KEY NOT NULL, project_id TEXT NOT NULL, relationship_type TEXT NOT NULL CHECK(relationship_type IN ('born_in','member_of','owns','opposed_by')),
 source_id TEXT NOT NULL, target_id TEXT NOT NULL, state TEXT NOT NULL CHECK(state IN ('candidate','canon','archived')),
 origin TEXT NOT NULL CHECK(origin IN ('manual','generated','imported')), body_json TEXT NOT NULL CHECK(json_valid(body_json)),
 write_version INTEGER NOT NULL CHECK(write_version > 0), created_at INTEGER NOT NULL CHECK(created_at >= 0), updated_at INTEGER NOT NULL CHECK(updated_at >= created_at),
 UNIQUE(project_id,relationship_type,source_id,target_id), UNIQUE(project_id,id),
 FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE,
 FOREIGN KEY(project_id,source_id) REFERENCES entities(project_id,id),
 FOREIGN KEY(project_id,target_id) REFERENCES entities(project_id,id)
);
CREATE INDEX IF NOT EXISTS idx_relationships_project_state ON relationships(project_id,state,updated_at);
CREATE INDEX IF NOT EXISTS idx_relationships_source ON relationships(project_id,source_id);
CREATE INDEX IF NOT EXISTS idx_relationships_target ON relationships(project_id,target_id);
CREATE TABLE IF NOT EXISTS relationship_revisions (
 revision_id INTEGER PRIMARY KEY AUTOINCREMENT, relationship_id TEXT NOT NULL,
 revision_number INTEGER NOT NULL CHECK(revision_number > 0), reason TEXT NOT NULL CHECK(length(trim(reason)) > 0),
 body_json TEXT NOT NULL CHECK(json_valid(body_json)), created_at INTEGER NOT NULL CHECK(created_at >= 0),
 UNIQUE(relationship_id,revision_number), FOREIGN KEY(relationship_id) REFERENCES relationships(id) ON DELETE RESTRICT
);
"#;

fn insert_entity(tx: &Transaction<'_>, entity: &StoredEntity) -> StoreResult<()> {
    let envelope = entity.envelope();
    let body = encode_json(entity)?;
    let generation = entity.generation().map(encode_json).transpose()?;
    tx.execute(
        "INSERT INTO entities(id, project_id, entity_type, name, canon_state, origin, favorite, schema_version, body_json, generation_json, write_version, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 1, ?11, ?12)",
        params![id_text(envelope.id.uuid()), id_text(envelope.project_id.uuid()), entity_type(envelope.entity_type), envelope.name, canon_state(envelope.canon_state), origin(envelope.origin), envelope.favorite, envelope.schema_version.0, body, generation, envelope.created_at_unix_ms, envelope.updated_at_unix_ms],
    )?;
    Ok(())
}

fn update_entity(
    tx: &Transaction<'_>,
    entity: &StoredEntity,
    expected: i64,
) -> StoreResult<StoredEntity> {
    let mut updated = entity.clone();
    updated.set_updated_at(now_unix_ms()?);
    updated.validate()?;
    let envelope = updated.envelope();
    let current = current_version(tx, envelope.id)?;
    check_version(expected, current)?;
    let current_entity = get_entity_tx(tx, envelope.id)?
        .ok_or_else(|| StoreError::NotFound(id_text(envelope.id.uuid())))?;
    let old = current_entity.entity.envelope();
    if old.project_id != envelope.project_id || old.entity_type != envelope.entity_type {
        return Err(StoreError::InvalidOperation(
            "entity identity, project, and type are immutable".into(),
        ));
    }
    let body = encode_json(&updated)?;
    let generation = updated.generation().map(encode_json).transpose()?;
    let changed = tx.execute(
        "UPDATE entities SET name=?1, canon_state=?2, origin=?3, favorite=?4, schema_version=?5, body_json=?6, generation_json=?7, updated_at=MAX(updated_at, ?8), write_version=write_version+1 WHERE id=?9 AND project_id=?10 AND write_version=?11",
        params![envelope.name, canon_state(envelope.canon_state), origin(envelope.origin), envelope.favorite, envelope.schema_version.0, body, generation, envelope.updated_at_unix_ms, id_text(envelope.id.uuid()), id_text(envelope.project_id.uuid()), expected],
    )?;
    if changed != 1 {
        return Err(StoreError::Conflict {
            expected,
            actual: current_version(tx, envelope.id)?,
        });
    }
    Ok(updated)
}

fn insert_generation_run(tx: &Transaction<'_>, entity: &StoredEntity) -> StoreResult<()> {
    let Some(generation) = entity.generation() else {
        return Ok(());
    };
    let recipe_json = generation
        .dna
        .to_canonical_json()
        .map_err(StoreError::Validation)?;
    let result_json = encode_json(entity)?;
    let action = match generation.action {
        GenerationAction::Forge => "forge",
        GenerationAction::RerollField => "reroll_field",
        GenerationAction::RerollUnlocked => "reroll_all_unlocked",
        GenerationAction::ManualEdit => "manual_edit",
        GenerationAction::RestoreHistory => "restore_history",
    };
    tx.execute(
        "INSERT INTO forge_runs(run_id, project_id, entity_id, action, parent_run_id, recipe_json, result_json, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![id_text(generation.generation_id.uuid()), id_text(entity.envelope().project_id.uuid()), id_text(entity.envelope().id.uuid()), action, generation.parent_generation_id.map(|id| id_text(id.uuid())), recipe_json, result_json, generation.generated_at_unix_ms],
    )?;
    Ok(())
}

fn insert_revision(
    tx: &Transaction<'_>,
    id: EntityId,
    number: i64,
    reason: &str,
    entity: &StoredEntity,
) -> StoreResult<()> {
    let body = encode_json(entity)?;
    let generation = entity.generation().map(encode_json).transpose()?;
    tx.execute(
        "INSERT INTO entity_revisions(entity_id, revision_number, reason, body_json, generation_json, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id_text(id.uuid()), number, reason, body, generation, now_unix_ms()?],
    )?;
    Ok(())
}

fn get_entity_connection(
    connection: &Connection,
    id: EntityId,
) -> StoreResult<Option<EntityRecord>> {
    let row = connection
        .query_row(
            "SELECT body_json, write_version, entity_type, name, canon_state, origin, favorite, schema_version, project_id FROM entities WHERE id=?1",
            [id_text(id.uuid())],
            raw_entity_row,
        )
        .optional()?;
    row.map(decode_entity_row).transpose()
}
fn get_entity_tx(tx: &Transaction<'_>, id: EntityId) -> StoreResult<Option<EntityRecord>> {
    let row = tx
        .query_row(
            "SELECT body_json, write_version, entity_type, name, canon_state, origin, favorite, schema_version, project_id FROM entities WHERE id=?1",
            [id_text(id.uuid())],
            raw_entity_row,
        )
        .optional()?;
    row.map(decode_entity_row).transpose()
}

fn validate_relationship_entities(
    tx: &Transaction<'_>,
    relationship: &Relationship,
) -> StoreResult<()> {
    let project = id_text(relationship.project_id.uuid());
    for endpoint in [relationship.source_id, relationship.target_id] {
        let owner: Option<String> = tx
            .query_row(
                "SELECT project_id FROM entities WHERE id=?1",
                [id_text(endpoint.uuid())],
                |row| row.get(0),
            )
            .optional()?;
        match owner {
            None => return Err(StoreError::NotFound(id_text(endpoint.uuid()))),
            Some(owner) if owner != project => {
                return Err(StoreError::InvalidOperation(
                    "relationships cannot cross project boundaries".into(),
                ));
            }
            Some(_) => {}
        }
    }
    Ok(())
}

fn insert_candidate_relationship(
    tx: &Transaction<'_>,
    relationship: &Relationship,
) -> StoreResult<()> {
    validate_relationship_entities(tx, relationship)?;
    let source = entity_type_from_db(entity_type_for_id(tx, relationship.source_id)?)?;
    let target = entity_type_from_db(entity_type_for_id(tx, relationship.target_id)?)?;
    validate_relationship_endpoints(relationship, source, target)?;
    let body = encode_json(relationship)?;
    tx.execute(
        "INSERT INTO relationships(id,project_id,relationship_type,source_id,target_id,state,origin,body_json,write_version,created_at,updated_at) VALUES (?1,?2,?3,?4,?5,'candidate',?6,?7,1,?8,?9)",
        params![
            id_text(relationship.id.uuid()),
            id_text(relationship.project_id.uuid()),
            relationship.relationship_type.key(),
            id_text(relationship.source_id.uuid()),
            id_text(relationship.target_id.uuid()),
            origin(relationship.origin),
            body,
            relationship.created_at_unix_ms,
            relationship.updated_at_unix_ms,
        ],
    )?;
    Ok(())
}

fn hometown_source(record: &EntityRecord) -> StoreResult<HometownSource> {
    match &record.entity {
        StoredEntity::CharacterCandidate(candidate) => Ok(HometownSource {
            character: candidate.character.clone(),
            field_origins: candidate.field_origins.clone(),
            write_version: record.write_version,
        }),
        StoredEntity::Character(character) => Ok(HometownSource {
            character: character.clone(),
            field_origins: BTreeMap::new(),
            write_version: record.write_version,
        }),
        _ => Err(StoreError::InvalidOperation(
            "Forge Hometown requires a persisted Character source".into(),
        )),
    }
}

fn entity_type_for_id(tx: &Transaction<'_>, endpoint: EntityId) -> StoreResult<String> {
    tx.query_row(
        "SELECT entity_type FROM entities WHERE id=?1",
        [id_text(endpoint.uuid())],
        |row| row.get(0),
    )
    .optional()?
    .ok_or_else(|| StoreError::NotFound(id_text(endpoint.uuid())))
}

fn entity_type_from_db(value: String) -> StoreResult<EntityType> {
    match value.as_str() {
        "character" => Ok(EntityType::Character),
        "quest" => Ok(EntityType::Quest),
        "location" => Ok(EntityType::Location),
        "faction" => Ok(EntityType::Faction),
        "item" => Ok(EntityType::Item),
        "creature" => Ok(EntityType::Creature),
        _ => Err(StoreError::Corrupt(format!("unknown entity type {value}"))),
    }
}

fn relationship_version_tx(tx: &Transaction<'_>, id: RelationshipId) -> StoreResult<Option<i64>> {
    tx.query_row(
        "SELECT write_version FROM relationships WHERE id=?1",
        [id_text(id.uuid())],
        |row| row.get(0),
    )
    .optional()
    .map_err(StoreError::from)
}

fn decode_relationship(body: &str) -> StoreResult<Relationship> {
    let relationship: Relationship = decode_json(body)?;
    relationship
        .validate()
        .map_err(|error| StoreError::Corrupt(error.to_string()))?;
    Ok(relationship)
}
fn current_version(tx: &Transaction<'_>, id: EntityId) -> StoreResult<Option<i64>> {
    tx.query_row(
        "SELECT write_version FROM entities WHERE id=?1",
        [id_text(id.uuid())],
        |row| row.get(0),
    )
    .optional()
    .map_err(StoreError::from)
}
fn check_version(expected: i64, actual: Option<i64>) -> StoreResult<()> {
    if actual != Some(expected) {
        return Err(StoreError::Conflict { expected, actual });
    }
    Ok(())
}
type RawEntityRow = (
    String,
    i64,
    String,
    String,
    String,
    String,
    bool,
    u16,
    String,
);
fn raw_entity_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<RawEntityRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
    ))
}
fn decode_entity_row(row: RawEntityRow) -> StoreResult<EntityRecord> {
    let entity = decode_stored_entity(&row.0)?;
    let envelope = entity.envelope();
    if entity_type(envelope.entity_type) != row.2
        || envelope.name != row.3
        || canon_state(envelope.canon_state) != row.4
        || origin(envelope.origin) != row.5
        || envelope.favorite != row.6
        || envelope.schema_version.0 != row.7
        || id_text(envelope.project_id.uuid()) != row.8
    {
        return Err(StoreError::Corrupt(
            "entity columns disagree with the stored versioned payload".into(),
        ));
    }
    Ok(EntityRecord {
        entity,
        write_version: row.1,
    })
}
fn encode_json<T: Serialize>(value: &T) -> StoreResult<String> {
    serde_json::to_string(value).map_err(|error| StoreError::Serialization(error.to_string()))
}
fn decode_json<T: for<'de> Deserialize<'de>>(value: &str) -> StoreResult<T> {
    serde_json::from_str(value).map_err(|error| StoreError::Corrupt(error.to_string()))
}
fn decode_stored_entity(value: &str) -> StoreResult<StoredEntity> {
    let entity: StoredEntity = decode_json(value)?;
    entity
        .validate()
        .map_err(|error| StoreError::Corrupt(error.to_string()))?;
    Ok(entity)
}

fn decode_history_snapshot(value: &str) -> StoreResult<StoredEntity> {
    if let Ok(entity) = decode_stored_entity(value) {
        return Ok(entity);
    }
    let result: ForgeBundleHistoryResult = decode_json(value)?;
    if result.format_version != 1 {
        return Err(StoreError::Serialization(
            "unsupported Forge Bundle history result version".into(),
        ));
    }
    Ok(result.primary_snapshot)
}
fn parse_id(value: String) -> StoreResult<EntityId> {
    value
        .parse::<Uuid128>()
        .map(EntityId)
        .map_err(StoreError::Validation)
}
fn id_text(value: Uuid128) -> String {
    value.to_string()
}
fn to_sql_conversion_error(error: StoreError) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error))
}
fn entity_type(value: EntityType) -> &'static str {
    match value {
        EntityType::Character => "character",
        EntityType::Quest => "quest",
        EntityType::Location => "location",
        EntityType::Faction => "faction",
        EntityType::Item => "item",
        EntityType::Creature => "creature",
    }
}
fn relationship_state(value: RelationshipState) -> &'static str {
    match value {
        RelationshipState::Candidate => "candidate",
        RelationshipState::Canon => "canon",
        RelationshipState::Archived => "archived",
    }
}
fn canon_state(value: CanonState) -> &'static str {
    match value {
        CanonState::Draft => "draft",
        CanonState::Candidate => "candidate",
        CanonState::Canon => "canon",
        CanonState::Archived => "archived",
    }
}
fn origin(value: EntityOrigin) -> &'static str {
    match value {
        EntityOrigin::Manual => "manual",
        EntityOrigin::Generated => "generated",
        EntityOrigin::Imported => "imported",
    }
}
fn validate_text(path: &str, value: &str) -> StoreResult<()> {
    if value.trim().is_empty() {
        return Err(StoreError::Validation(ValidationErrors::one(
            ValidationError::new(
                ValidationCode::EmptyValue,
                path,
                "value must contain non-whitespace text",
            ),
        )));
    }
    Ok(())
}
fn validation_error(code: ValidationCode, path: &str, message: &str) -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(code, path, message))
}
fn now_unix_ms() -> StoreResult<i64> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| StoreError::InvalidOperation(error.to_string()))?
        .as_millis();
    i64::try_from(millis)
        .map_err(|_| StoreError::InvalidOperation("system time is outside supported range".into()))
}

#[cfg(test)]
mod tests;
