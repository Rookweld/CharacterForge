use forge_app_core::{
    CharacterField, CharacterFieldValue, CharacterForge, QuestField, QuestFieldValue, QuestForge,
};
use forge_contracts::{
    CreativeAxes, CreativeAxis, EntityId, INITIAL_CONTENT_VERSION, INITIAL_GENERATOR_VERSION,
    INITIAL_SCHEMA_VERSION, ProjectEnvelope, ProjectId, RootSeed,
};
use forge_storage::{
    EntityRecord, EntityRevision, ForgeRunRecord, ProjectSession, ProjectStore, StoreError,
    StoredEntity, StoredProject,
};
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
use tauri::State;

#[derive(Default)]
struct AppSession {
    project: ProjectSession,
    working: Option<StoredEntity>,
    write_version: Option<i64>,
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EntityView {
    entity: StoredEntity,
    write_version: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HistoryView {
    run_id: EntityId,
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
    state: State<'_, AppState>,
    path: String,
    name: String,
) -> Result<ProjectInfo, UiError> {
    let path = PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        return Err(UiError {
            kind: "validation",
            message: "Choose a project database path.".into(),
            expected: None,
            actual: None,
        });
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| UiError {
            kind: "storage",
            message: error.to_string(),
            expected: None,
            actual: None,
        })?;
    }
    let mut session = lock_state(&state)?;
    session.project.open(&path)?;
    let store = active_store(&mut session)?;
    if store.load_project()?.is_some() {
        return Err(UiError {
            kind: "project_exists",
            message: "A project already exists at this location. Open it instead.".into(),
            expected: None,
            actual: None,
        });
    }
    let timestamp = now_ms()?;
    store.create_project(&StoredProject {
        envelope: ProjectEnvelope {
            id: new_project_id(),
            schema_version: INITIAL_SCHEMA_VERSION,
            generator_version: INITIAL_GENERATOR_VERSION,
            content_version: INITIAL_CONTENT_VERSION,
        },
        name: name.trim().to_owned(),
        created_at_unix_ms: timestamp,
        updated_at_unix_ms: timestamp,
    })?;
    session.working = None;
    session.write_version = None;
    project_info(&session)
}

#[tauri::command]
fn open_project(state: State<'_, AppState>, path: String) -> Result<ProjectInfo, UiError> {
    let mut session = lock_state(&state)?;
    let store = session.project.open(Path::new(path.trim()))?;
    if store.load_project()?.is_none() {
        session.project.close();
        return Err(UiError { kind: "invalid_project", message: "This database does not contain a project. Create a new project or choose another file.".into(), expected: None, actual: None });
    }
    session.working = None;
    session.write_version = None;
    project_info(&session)
}

#[tauri::command]
fn close_project(state: State<'_, AppState>) -> Result<(), UiError> {
    let mut session = lock_state(&state)?;
    session.project.close();
    session.working = None;
    session.write_version = None;
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
    let mut creative_axes = CreativeAxes { levels: axes };
    creative_axes.normalize()?;
    let dna = CharacterForge::new_dna(RootSeed::new(seed), &preset, creative_axes, constraints)?;
    let mut candidate = CharacterForge::generate_in_project(&dna, project.envelope.id)?;
    candidate.character.envelope.name = candidate.character.title.clone();
    let entity = StoredEntity::CharacterCandidate(candidate);
    session.working = Some(entity.clone());
    session.write_version = None;
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
    Ok(entity)
}

#[tauri::command]
fn set_field_lock(
    state: State<'_, AppState>,
    field: String,
    locked: bool,
) -> Result<StoredEntity, UiError> {
    let mut session = lock_state(&state)?;
    let entity = session.working.as_mut().ok_or_else(|| UiError {
        kind: "no_candidate",
        message: "Forge or open a candidate first.".into(),
        expected: None,
        actual: None,
    })?;
    match entity {
        StoredEntity::CharacterCandidate(candidate) => {
            let target = character_field(&field)?;
            if locked {
                CharacterForge::lock(candidate, target)?
            } else {
                CharacterForge::unlock(candidate, target)?
            }
        }
        StoredEntity::QuestCandidate(candidate) => {
            let target = quest_field(&field)?;
            if locked {
                QuestForge::lock(candidate, target)?
            } else {
                QuestForge::unlock(candidate, target)?
            }
        }
        _ => {
            return Err(UiError {
                kind: "invalid_operation",
                message: "Canon records cannot be edited as candidates.".into(),
                expected: None,
                actual: None,
            });
        }
    }
    Ok(entity.clone())
}

#[tauri::command]
fn reroll_field(state: State<'_, AppState>, field: String) -> Result<StoredEntity, UiError> {
    let mut session = lock_state(&state)?;
    let entity = session.working.as_mut().ok_or_else(|| UiError {
        kind: "no_candidate",
        message: "Forge or open a candidate first.".into(),
        expected: None,
        actual: None,
    })?;
    match entity {
        StoredEntity::CharacterCandidate(candidate) => {
            CharacterForge::reroll_field(candidate, character_field(&field)?)?;
            candidate.character.envelope.name = candidate.character.title.clone();
        }
        StoredEntity::QuestCandidate(candidate) => {
            QuestForge::reroll_field(candidate, quest_field(&field)?)?;
        }
        _ => {
            return Err(UiError {
                kind: "invalid_operation",
                message: "Canon records cannot be rerolled.".into(),
                expected: None,
                actual: None,
            });
        }
    }
    Ok(entity.clone())
}

#[tauri::command]
fn reroll_unlocked(state: State<'_, AppState>) -> Result<StoredEntity, UiError> {
    let mut session = lock_state(&state)?;
    let entity = session.working.as_mut().ok_or_else(|| UiError {
        kind: "no_candidate",
        message: "Forge or open a candidate first.".into(),
        expected: None,
        actual: None,
    })?;
    match entity {
        StoredEntity::CharacterCandidate(candidate) => {
            CharacterForge::reroll_all_unlocked(candidate)?;
            candidate.character.envelope.name = candidate.character.title.clone();
        }
        StoredEntity::QuestCandidate(candidate) => {
            QuestForge::reroll_all(candidate)?;
        }
        _ => {
            return Err(UiError {
                kind: "invalid_operation",
                message: "Canon records cannot be rerolled.".into(),
                expected: None,
                actual: None,
            });
        }
    }
    Ok(entity.clone())
}

#[tauri::command]
fn edit_field(
    state: State<'_, AppState>,
    field: String,
    value: Value,
) -> Result<StoredEntity, UiError> {
    let mut session = lock_state(&state)?;
    let entity = session.working.as_mut().ok_or_else(|| UiError {
        kind: "no_candidate",
        message: "Forge or open a candidate first.".into(),
        expected: None,
        actual: None,
    })?;
    match entity {
        StoredEntity::CharacterCandidate(candidate) => {
            let target = character_field(&field)?;
            let value = character_value(target, value)?;
            candidate.manual_edit(target, value)?;
            if target == CharacterField::Title {
                candidate.character.envelope.name = candidate.character.title.clone();
            }
        }
        StoredEntity::QuestCandidate(candidate) => {
            let target = quest_field(&field)?;
            let value = quest_value(target, value)?;
            candidate.manual_edit(target, value)?;
        }
        _ => {
            return Err(UiError {
                kind: "invalid_operation",
                message: "Canon records must be revised explicitly.".into(),
                expected: None,
                actual: None,
            });
        }
    }
    Ok(entity.clone())
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
        run_id: record.run_id,
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
    let mut session = lock_state(&state)?;
    active_store(&mut session)?
        .load_history_snapshot(parse_entity_id(&run_id)?)
        .map_err(Into::into)
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            create_project,
            open_project,
            close_project,
            get_project,
            forge_character,
            forge_quest,
            set_field_lock,
            reroll_field,
            reroll_unlocked,
            edit_field,
            save_candidate,
            list_entities,
            get_entity,
            toggle_favorite,
            promote_to_canon,
            list_history,
            get_history_snapshot,
            get_canon_revisions
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Character Forge desktop shell");
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
