use super::*;
use forge_app_core::{
    CharacterField, CharacterFieldValue, CharacterForge, FieldValueOrigin, QuestForge,
};
use forge_contracts::{
    ContentVersion, CreativeAxes, INITIAL_CONTENT_VERSION, INITIAL_GENERATOR_VERSION,
    INITIAL_SCHEMA_VERSION, RootSeed,
};
use rusqlite::Connection;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

struct TestDb(PathBuf);
impl TestDb {
    fn new(label: &str) -> Self {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self(std::env::temp_dir().join(format!(
            "character-forge-{label}-{}-{stamp}.sqlite3",
            std::process::id()
        )))
    }
    fn path(&self) -> &Path {
        &self.0
    }
}
impl Drop for TestDb {
    fn drop(&mut self) {
        for suffix in ["", "-wal", "-shm", ".pre-migration.bak"] {
            let mut path = self.0.as_os_str().to_os_string();
            path.push(suffix);
            let _ = fs::remove_file(PathBuf::from(path));
        }
    }
}

fn project_id() -> ProjectId {
    ProjectId::from_bytes([7; 16])
}
fn new_project() -> StoredProject {
    StoredProject {
        envelope: ProjectEnvelope {
            id: project_id(),
            schema_version: INITIAL_SCHEMA_VERSION,
            generator_version: INITIAL_GENERATOR_VERSION,
            content_version: INITIAL_CONTENT_VERSION,
        },
        name: "Memory Test World".into(),
        created_at_unix_ms: 10,
        updated_at_unix_ms: 10,
    }
}
fn create_store(db: &TestDb) -> ProjectStore {
    let mut store = ProjectStore::open(db.path()).unwrap();
    store.create_project(&new_project()).unwrap();
    store
}
fn character_candidate(seed: u64) -> CharacterCandidate {
    let dna = CharacterForge::new_dna(
        RootSeed::new(seed),
        "dark-fantasy",
        CreativeAxes::default(),
        Default::default(),
    )
    .unwrap();
    CharacterForge::generate_in_project(&dna, project_id()).unwrap()
}
fn quest_candidate(seed: u64, version: u16) -> QuestCandidate {
    let mut dna = QuestForge::new_dna(
        RootSeed::new(seed),
        "occult",
        CreativeAxes::default(),
        Default::default(),
    )
    .unwrap();
    if version == 2 {
        dna.content_version = ContentVersion(2);
        let clue = forge_contracts::FieldKey::new("16-clue").unwrap();
        let briefing = forge_contracts::FieldKey::new("17-briefing").unwrap();
        dna.recipe.field_controls.remove(&clue);
        dna.recipe.field_controls.remove(&briefing);
        dna.recipe.dependencies.remove(&clue);
        dna.recipe.dependencies.remove(&briefing);
    }
    let mut candidate = QuestForge::generate(&dna).unwrap();
    candidate.quest.envelope.project_id = project_id();
    candidate.validate().unwrap();
    candidate
}
#[test]
fn fresh_database_has_versioned_schema_and_enforces_foreign_keys() {
    let db = TestDb::new("fresh");
    let store = ProjectStore::open(db.path()).unwrap();
    assert_eq!(
        store
            .connection
            .query_row("PRAGMA user_version", [], |row| row.get::<_, u32>(0))
            .unwrap(),
        CURRENT_SCHEMA_VERSION
    );
    assert_eq!(
        store
            .connection
            .query_row("PRAGMA foreign_keys", [], |row| row.get::<_, u32>(0))
            .unwrap(),
        1
    );
    assert!(store.load_project().unwrap().is_none());
    store.integrity_check().unwrap();
}

#[test]
fn project_and_entity_crud_isolated_to_the_project_database() {
    let db = TestDb::new("crud");
    let mut store = create_store(&db);
    assert_eq!(store.load_project().unwrap().unwrap(), new_project());
    let renamed_at = store.rename_project("Witness Archive", 10).unwrap();
    assert!(renamed_at > 10);
    assert!(matches!(
        store.rename_project("Lost Update", 10),
        Err(StoreError::Conflict { .. })
    ));
    assert_eq!(
        store.load_project().unwrap().unwrap().name,
        "Witness Archive"
    );
    let mut candidate = character_candidate(22);
    let id = candidate.character.envelope.id;
    assert_eq!(
        store
            .persist_candidate(StoredEntity::CharacterCandidate(candidate.clone()))
            .unwrap(),
        1
    );
    let row = store.get_entity(id).unwrap().unwrap();
    assert_eq!(
        row.entity,
        StoredEntity::CharacterCandidate(candidate.clone())
    );
    assert_eq!(row.write_version, 1);
    assert_eq!(store.list_entities().unwrap().len(), 1);

    candidate.character.envelope.notes = Some("field notes after the first pass".into());
    let next = store
        .save_candidate(StoredEntity::CharacterCandidate(candidate), 1)
        .unwrap();
    assert_eq!(next, 2);
    assert_eq!(store.get_entity(id).unwrap().unwrap().write_version, 2);

    let mut wrong_project = character_candidate(23);
    wrong_project.character.envelope.project_id = ProjectId::from_bytes([8; 16]);
    assert!(matches!(
        store.persist_candidate(StoredEntity::CharacterCandidate(wrong_project)),
        Err(StoreError::Sqlite(_))
    ));
    assert_eq!(store.list_entities().unwrap().len(), 1);
}

#[test]
fn candidate_history_and_all_generator_metadata_survive_real_close_reopen() {
    let db = TestDb::new("reopen");
    let mut candidate = character_candidate(555);
    CharacterForge::lock(&mut candidate, CharacterField::Title).unwrap();
    let id = candidate.character.envelope.id;
    let expected_dna = candidate.dna.to_canonical_json().unwrap();
    let mut store = create_store(&db);
    store
        .persist_candidate(StoredEntity::CharacterCandidate(candidate.clone()))
        .unwrap();
    assert_eq!(store.list_history(Some(id)).unwrap().len(), 1);
    store.checkpoint().unwrap();
    drop(store);

    let store = ProjectStore::open(db.path()).unwrap();
    store.integrity_check().unwrap();
    let row = store.get_entity(id).unwrap().unwrap();
    let StoredEntity::CharacterCandidate(reopened) = row.entity else {
        panic!("candidate kind changed")
    };
    assert_eq!(reopened, candidate);
    assert_eq!(reopened.dna.to_canonical_json().unwrap(), expected_dna);
    assert_eq!(
        store.list_history(Some(id)).unwrap()[0].recipe_json,
        expected_dna
    );
    let run_id = candidate
        .character
        .envelope
        .generation_provenance
        .as_ref()
        .unwrap()
        .generation_id;
    assert_eq!(
        store.load_history_snapshot(run_id).unwrap(),
        Some(StoredEntity::CharacterCandidate(candidate))
    );
}

#[test]
fn repeated_identical_forge_actions_are_each_retained_in_history() {
    let db = TestDb::new("repeat-history");
    let mut store = create_store(&db);
    let candidate = character_candidate(12);
    let id = candidate.character.envelope.id;
    store
        .persist_candidate(StoredEntity::CharacterCandidate(candidate.clone()))
        .unwrap();
    store
        .save_candidate(StoredEntity::CharacterCandidate(candidate), 1)
        .unwrap();
    let history = store.list_history(Some(id)).unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].run_id, history[1].run_id);
}

#[test]
fn project_settings_are_parameterized_json_values() {
    let db = TestDb::new("settings");
    let mut store = create_store(&db);
    store.set_setting("active-preset", "\"occult\"").unwrap();
    assert_eq!(
        store.get_setting("active-preset").unwrap().as_deref(),
        Some("\"occult\"")
    );
    assert!(matches!(
        store.set_setting("malformed", "not-json"),
        Err(StoreError::Serialization(_))
    ));
    assert!(store.get_setting("malformed").unwrap().is_none());
}

#[test]
fn quest_v2_and_v3_metadata_and_character_manual_origin_roundtrip() {
    let db = TestDb::new("versions");
    let mut store = create_store(&db);
    let old_quest = quest_candidate(2002, 2);
    let old_id = old_quest.quest.envelope.id;
    assert_eq!(old_quest.dna.content_version, ContentVersion(2));
    store
        .persist_candidate(StoredEntity::QuestCandidate(old_quest.clone()))
        .unwrap();

    let new_quest = quest_candidate(3003, 3);
    let new_id = new_quest.quest.envelope.id;
    assert!(new_quest.clue.is_some() && new_quest.briefing.is_some());
    store
        .persist_candidate(StoredEntity::QuestCandidate(new_quest.clone()))
        .unwrap();

    let mut character = character_candidate(4004);
    character
        .manual_edit(
            CharacterField::Title,
            CharacterFieldValue::Text("The Witness of Seven Bells".into()),
        )
        .unwrap();
    assert!(
        character
            .field_origins
            .values()
            .any(|origin| *origin == FieldValueOrigin::UserModified)
    );
    let character_id = character.character.envelope.id;
    character.validate().unwrap();
    store
        .persist_candidate(StoredEntity::CharacterCandidate(character.clone()))
        .unwrap();

    assert_eq!(
        store.get_entity(old_id).unwrap().unwrap().entity,
        StoredEntity::QuestCandidate(old_quest)
    );
    assert_eq!(
        store.get_entity(new_id).unwrap().unwrap().entity,
        StoredEntity::QuestCandidate(new_quest)
    );
    assert_eq!(
        store.get_entity(character_id).unwrap().unwrap().entity,
        StoredEntity::CharacterCandidate(character)
    );
}

#[test]
fn promoting_and_revising_canon_preserves_inspectable_prior_snapshots() {
    let db = TestDb::new("canon");
    let mut store = create_store(&db);
    let candidate = quest_candidate(808, 3);
    let id = candidate.quest.envelope.id;
    store
        .persist_candidate(StoredEntity::QuestCandidate(candidate))
        .unwrap();
    assert_eq!(store.promote_candidate(id, 1).unwrap(), 2);
    let prior = store.get_entity(id).unwrap().unwrap();
    assert_eq!(prior.entity.envelope().canon_state, CanonState::Canon);

    let mut revised = prior.entity.clone();
    if let StoredEntity::QuestCandidate(candidate) = &mut revised {
        candidate
            .quest
            .objective
            .push_str(" while preserving the witness's testimony");
    }
    assert_eq!(
        store
            .revise_canon(revised, 2, "clarified canon objective")
            .unwrap(),
        3
    );
    let revisions = store.list_revisions(id).unwrap();
    assert_eq!(revisions.len(), 2);
    assert_eq!(revisions[0].revision_number, 1);
    assert_eq!(revisions[1].revision_number, 2);
    assert_eq!(revisions[0].entity, prior.entity);
    assert_eq!(
        revisions[1].entity,
        store.get_entity(id).unwrap().unwrap().entity
    );
    assert_eq!(store.get_entity(id).unwrap().unwrap().write_version, 3);

    let mut demoted = store.get_entity(id).unwrap().unwrap().entity;
    demoted.set_state(CanonState::Draft);
    assert!(matches!(
        store.save_candidate(demoted, 3),
        Err(StoreError::InvalidOperation(_))
    ));
    assert_eq!(
        store
            .get_entity(id)
            .unwrap()
            .unwrap()
            .entity
            .envelope()
            .canon_state,
        CanonState::Canon
    );
    let current_canon = store.get_entity(id).unwrap().unwrap();
    store.checkpoint().unwrap();
    drop(store);
    let reopened = ProjectStore::open(db.path()).unwrap();
    assert_eq!(reopened.get_entity(id).unwrap().unwrap(), current_canon);
    assert_eq!(reopened.list_revisions(id).unwrap(), revisions);
}

#[test]
fn stale_writes_are_structured_conflicts_and_favorites_stay_in_sync() {
    let db = TestDb::new("stale");
    let mut store = create_store(&db);
    let candidate = character_candidate(90);
    let id = candidate.character.envelope.id;
    store
        .persist_candidate(StoredEntity::CharacterCandidate(candidate))
        .unwrap();
    assert_eq!(store.toggle_favorite(id, true, 1).unwrap(), 2);
    let favorited = store.get_entity(id).unwrap().unwrap();
    assert!(favorited.entity.envelope().favorite);
    assert!(matches!(
        store.toggle_favorite(id, false, 1),
        Err(StoreError::Conflict {
            expected: 1,
            actual: Some(2)
        })
    ));
    assert!(
        store
            .get_entity(id)
            .unwrap()
            .unwrap()
            .entity
            .envelope()
            .favorite
    );
}

#[test]
fn failed_multi_record_write_rolls_back_entity_and_history() {
    let db = TestDb::new("rollback");
    let mut store = create_store(&db);
    let mut candidate = character_candidate(91);
    let id = candidate.character.envelope.id;
    store
        .persist_candidate(StoredEntity::CharacterCandidate(candidate.clone()))
        .unwrap();
    let before = store.get_entity(id).unwrap().unwrap();
    candidate.character.envelope.notes = Some("must roll back".into());
    store.connection.execute_batch("CREATE TRIGGER reject_history BEFORE INSERT ON forge_runs BEGIN SELECT RAISE(ABORT, 'injected history failure'); END;").unwrap();
    assert!(matches!(
        store.save_candidate(StoredEntity::CharacterCandidate(candidate), 1),
        Err(StoreError::Sqlite(_))
    ));
    assert_eq!(store.get_entity(id).unwrap().unwrap(), before);
    assert_eq!(store.list_history(Some(id)).unwrap().len(), 1);
}

#[test]
fn populated_schema_zero_migrates_and_keeps_a_recovery_copy() {
    let db = TestDb::new("migration-success");
    let connection = Connection::open(db.path()).unwrap();
    connection.execute_batch("CREATE TABLE legacy_projects(id TEXT PRIMARY KEY, name TEXT NOT NULL, created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL); INSERT INTO legacy_projects VALUES('07070707-0707-0707-0707-070707070707','Legacy World',10,11);").unwrap();
    drop(connection);
    let store = ProjectStore::open(db.path()).unwrap();
    let project = store.load_project().unwrap().unwrap();
    assert_eq!(project.name, "Legacy World");
    assert_eq!(project.updated_at_unix_ms, 11);
    assert!(migration_backup_path(db.path()).exists());
    assert_eq!(
        Connection::open(migration_backup_path(db.path()))
            .unwrap()
            .query_row("SELECT COUNT(*) FROM legacy_projects", [], |row| row
                .get::<_, i64>(0))
            .unwrap(),
        1
    );
}

#[test]
fn failed_schema_zero_migration_rolls_back_and_preserves_backup() {
    let db = TestDb::new("migration-failure");
    let connection = Connection::open(db.path()).unwrap();
    connection.execute_batch("CREATE TABLE legacy_projects(id TEXT PRIMARY KEY, name TEXT NOT NULL, created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL); INSERT INTO legacy_projects VALUES('07070707-0707-0707-0707-070707070707','One',10,10); INSERT INTO legacy_projects VALUES('08080808-0808-0808-0808-080808080808','Two',10,10);").unwrap();
    drop(connection);
    assert!(matches!(
        ProjectStore::open(db.path()),
        Err(StoreError::Corrupt(_))
    ));
    let backup = migration_backup_path(db.path());
    assert!(backup.exists());
    let original = Connection::open(db.path()).unwrap();
    assert_eq!(
        original
            .query_row("PRAGMA user_version", [], |row| row.get::<_, u32>(0))
            .unwrap(),
        0
    );
    assert_eq!(
        original
            .query_row("SELECT COUNT(*) FROM legacy_projects", [], |row| row
                .get::<_, i64>(0))
            .unwrap(),
        2
    );
    let saved = Connection::open(backup).unwrap();
    assert_eq!(
        saved
            .query_row("SELECT COUNT(*) FROM legacy_projects", [], |row| row
                .get::<_, i64>(0))
            .unwrap(),
        2
    );
}

#[test]
fn unsupported_future_schema_and_corrupt_database_fail_safely() {
    let future = TestDb::new("future");
    Connection::open(future.path())
        .unwrap()
        .pragma_update(None, "user_version", 99)
        .unwrap();
    assert!(matches!(
        ProjectStore::open(future.path()),
        Err(StoreError::UnsupportedSchema(99))
    ));

    let corrupt = TestDb::new("corrupt");
    fs::write(corrupt.path(), b"not a SQLite database").unwrap();
    assert!(matches!(
        ProjectStore::open(corrupt.path()),
        Err(StoreError::Corrupt(_))
    ));
}

#[test]
fn corrupted_entity_payload_returns_a_structured_corruption_error() {
    let db = TestDb::new("corrupt-row");
    let mut store = create_store(&db);
    let candidate = character_candidate(44);
    let id = candidate.character.envelope.id;
    store
        .persist_candidate(StoredEntity::CharacterCandidate(candidate))
        .unwrap();
    store
        .connection
        .execute(
            "UPDATE entities SET body_json='{}' WHERE id=?1",
            [id_text(id.uuid())],
        )
        .unwrap();
    assert!(matches!(store.get_entity(id), Err(StoreError::Corrupt(_))));
}

#[test]
fn session_undo_redo_is_transient_and_new_branch_clears_redo() {
    let mut history = SessionHistory::new("initial".to_string());
    history.push("edited".into());
    assert_eq!(history.undo().map(String::as_str), Some("initial"));
    assert_eq!(history.redo().map(String::as_str), Some("edited"));
    history.undo();
    history.push("new branch".into());
    assert!(history.redo().is_none());
    assert_eq!(history.current(), "new branch");
}

#[test]
fn project_session_keeps_current_project_if_switch_target_cannot_open() {
    let db = TestDb::new("session");
    let mut session = ProjectSession::default();
    session.open(db.path()).unwrap();
    let before = session.active().unwrap().path().to_path_buf();
    let invalid = TestDb::new("session-invalid");
    fs::write(invalid.path(), b"bad sqlite").unwrap();
    assert!(session.open(invalid.path()).is_err());
    assert_eq!(session.active().unwrap().path(), before);
    session.close();
    assert!(session.active().is_none());
}
