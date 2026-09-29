use super::*;
use forge_app_core::{
    CharacterField, CharacterFieldValue, CharacterForge, FieldValueOrigin, LocationField,
    LocationFieldValue, LocationForge, QuestField, QuestFieldValue, QuestForge,
};
use forge_contracts::{
    ContentVersion, CreativeAxes, INITIAL_CONTENT_VERSION, INITIAL_GENERATOR_VERSION,
    RelationshipType, RootSeed, SchemaVersion,
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

#[test]
fn location_candidate_persists_favorites_promotes_revises_and_reopens() {
    let db = TestDb::new("location-lifecycle");
    let mut store = create_store(&db);
    let dna =
        LocationForge::new_dna(RootSeed::new(55), "steampunk", CreativeAxes::neutral()).unwrap();
    let candidate = LocationForge::generate_in_project(&dna, project_id()).unwrap();
    let id = candidate.location.envelope.id;
    assert_eq!(
        store
            .persist_candidate(StoredEntity::LocationCandidate(candidate.clone()))
            .unwrap(),
        1
    );
    assert_eq!(
        store.get_entity(id).unwrap().unwrap().entity,
        StoredEntity::LocationCandidate(candidate)
    );
    assert_eq!(store.toggle_favorite(id, true, 1).unwrap(), 2);
    assert_eq!(store.promote_candidate(id, 2).unwrap(), 3);
    let mut canon = match store.get_entity(id).unwrap().unwrap().entity {
        StoredEntity::LocationCandidate(value) => value,
        _ => panic!("Location candidate wrapper was not retained"),
    };
    LocationForge::manual_edit(
        &mut canon,
        LocationField::Purpose,
        LocationFieldValue::Text("The community maintains a public pressure archive.".into()),
    )
    .unwrap();
    assert_eq!(
        store
            .revise_canon(
                StoredEntity::LocationCandidate(canon),
                3,
                "clarified public purpose"
            )
            .unwrap(),
        4
    );
    assert_eq!(store.list_revisions(id).unwrap().len(), 2);
    drop(store);
    let reopened = ProjectStore::open(db.path()).unwrap();
    let current = reopened.get_entity(id).unwrap().unwrap();
    assert!(current.entity.envelope().favorite);
    assert_eq!(current.write_version, 4);
    assert_eq!(reopened.list_revisions(id).unwrap().len(), 2);
    let history = reopened.list_history(Some(id)).unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].action, "forge");
    assert_eq!(history[1].action, "manual_edit");
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
            schema_version: SchemaVersion(CURRENT_SCHEMA_VERSION as u16),
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

fn add_typed_endpoint(store: &ProjectStore, id: EntityId, entity_type: &str) {
    store.connection.execute(
        "INSERT INTO entities(id,project_id,entity_type,name,canon_state,origin,favorite,schema_version,body_json,generation_json,write_version,created_at,updated_at) VALUES (?1,?2,?3,'Typed endpoint','candidate','manual',0,1,'{}',NULL,1,10,10)",
        params![id_text(id.uuid()), id_text(project_id().uuid()), entity_type],
    ).unwrap();
}

fn born_in_relationship(source_id: EntityId, target_id: EntityId) -> Relationship {
    Relationship {
        id: RelationshipId::from_bytes([41; 16]),
        project_id: project_id(),
        relationship_type: RelationshipType::BornIn,
        source_id,
        target_id,
        state: RelationshipState::Candidate,
        origin: EntityOrigin::Manual,
        created_at_unix_ms: 10,
        updated_at_unix_ms: 10,
    }
}

#[test]
#[ignore = "one-time reproducible writer for the frozen schema-1 fixture"]
fn write_frozen_schema1_fixture_once() {
    let folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    fs::create_dir_all(&folder).unwrap();
    let path = folder.join("schema1-populated.sqlite3");
    let db = TestDb(path.clone());
    let mut store = ProjectStore::open(db.path()).unwrap();
    store.create_project(&new_project()).unwrap();

    let mut candidate = character_candidate(88001);
    candidate.character.envelope.tags = vec!["archive".into(), "café".into()];
    candidate.character.envelope.notes = Some("Δοκιμή — frozen schema-one prose".into());
    CharacterForge::lock(&mut candidate, CharacterField::Name).unwrap();
    candidate
        .manual_edit(
            CharacterField::Title,
            CharacterFieldValue::Text("The Keeper of Reykjavík".into()),
        )
        .unwrap();
    let candidate_id = candidate.character.envelope.id;
    store
        .persist_candidate(StoredEntity::CharacterCandidate(candidate))
        .unwrap();
    store.toggle_favorite(candidate_id, true, 1).unwrap();

    let canon_character = character_candidate(88002);
    let character_id = canon_character.character.envelope.id;
    store
        .persist_candidate(StoredEntity::CharacterCandidate(canon_character))
        .unwrap();
    store.promote_candidate(character_id, 1).unwrap();
    let mut revised_character = store.get_entity(character_id).unwrap().unwrap().entity;
    if let StoredEntity::CharacterCandidate(value) = &mut revised_character {
        value.character.title.push_str(" — δεύτερη μαρτυρία");
    }
    store
        .revise_canon(revised_character, 2, "fixture canon revision α")
        .unwrap();

    let mut quest_candidate_value = quest_candidate(88003, 3);
    QuestForge::lock(&mut quest_candidate_value, QuestField::Objective).unwrap();
    let quest_candidate_id = quest_candidate_value.quest.envelope.id;
    store
        .persist_candidate(StoredEntity::QuestCandidate(quest_candidate_value))
        .unwrap();
    let quest_canon = quest_candidate(88004, 3);
    let quest_canon_id = quest_canon.quest.envelope.id;
    store
        .persist_candidate(StoredEntity::QuestCandidate(quest_canon))
        .unwrap();
    store.promote_candidate(quest_canon_id, 1).unwrap();
    let mut revised_quest = store.get_entity(quest_canon_id).unwrap().unwrap().entity;
    if let StoredEntity::QuestCandidate(value) = &mut revised_quest {
        value.quest.objective.push_str(" — Δοκιμή");
    }
    store
        .revise_canon(revised_quest, 2, "quest canon revision β")
        .unwrap();
    let mut candidate_quest = store
        .get_entity(quest_candidate_id)
        .unwrap()
        .unwrap()
        .entity;
    if let StoredEntity::QuestCandidate(value) = &mut candidate_quest {
        value.quest.envelope.tags = vec!["candidate".into(), "世界".into()];
        value.quest.envelope.notes = Some("notes: naïve façade".into());
    }
    store.save_candidate(candidate_quest, 1).unwrap();
    store.set_setting("reduced_motion", "true").unwrap();
    store.checkpoint().unwrap();
    drop(store);

    let connection = Connection::open(db.path()).unwrap();
    connection
        .pragma_update(None, "foreign_keys", "OFF")
        .unwrap();
    connection.execute_batch(r#"
DROP TABLE relationship_revisions;
DROP TABLE relationships;
CREATE TABLE entities_v1 (
 id TEXT PRIMARY KEY NOT NULL, project_id TEXT NOT NULL,
 entity_type TEXT NOT NULL CHECK(entity_type IN ('character','quest')),
 name TEXT NOT NULL CHECK(length(trim(name)) > 0),
 canon_state TEXT NOT NULL CHECK(canon_state IN ('draft','candidate','canon','archived')),
 origin TEXT NOT NULL CHECK(origin IN ('manual','generated','imported')),
 favorite INTEGER NOT NULL DEFAULT 0 CHECK(favorite IN (0,1)), schema_version INTEGER NOT NULL CHECK(schema_version > 0),
 body_json TEXT NOT NULL CHECK(json_valid(body_json)), generation_json TEXT CHECK(generation_json IS NULL OR json_valid(generation_json)),
 write_version INTEGER NOT NULL CHECK(write_version > 0), created_at INTEGER NOT NULL CHECK(created_at >= 0), updated_at INTEGER NOT NULL CHECK(updated_at >= created_at),
 UNIQUE(project_id,id), FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE
);
INSERT INTO entities_v1 SELECT * FROM entities;
DROP TABLE entities;
ALTER TABLE entities_v1 RENAME TO entities;
CREATE INDEX idx_entities_project_state ON entities(project_id,canon_state,updated_at);
CREATE INDEX idx_entities_project_type ON entities(project_id,entity_type,updated_at);
UPDATE projects SET schema_version=1;
PRAGMA user_version=1;
"#).unwrap();
    connection
        .pragma_update(None, "foreign_keys", "ON")
        .unwrap();
    connection
        .execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
        .unwrap();
    std::mem::forget(db);
}

fn fixture_rows(connection: &Connection) -> Vec<String> {
    ["projects", "entities", "entity_revisions", "forge_runs", "settings"].into_iter().map(|table| {
        let sql = if table == "projects" { "SELECT id,name,generator_version,content_version,created_at,updated_at FROM projects ORDER BY rowid".to_string() } else { format!("SELECT * FROM {table} ORDER BY rowid") };
        let mut statement = connection.prepare(&sql).unwrap();
        let width = statement.column_count();
        statement.query_map([], |row| {
            let values = (0..width).map(|column| row.get::<_, rusqlite::types::Value>(column).map(|v| format!("{v:?}"))).collect::<rusqlite::Result<Vec<_>>>()?;
            Ok(values.join("|"))
        }).unwrap().collect::<rusqlite::Result<Vec<_>>>().unwrap().join("\n")
    }).collect()
}

fn fixture_replay_signatures(connection: &Connection) -> Vec<String> {
    let mut statement = connection
        .prepare("SELECT body_json FROM entities ORDER BY id")
        .unwrap();
    let bodies = statement
        .query_map([], |row| row.get::<_, String>(0))
        .unwrap()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();
    bodies
        .into_iter()
        .map(|body| match decode_stored_entity(&body).unwrap() {
            StoredEntity::CharacterCandidate(candidate) => serde_json::to_string(
                &CharacterForge::generate_in_project(
                    &candidate.dna,
                    candidate.character.envelope.project_id,
                )
                .unwrap(),
            )
            .unwrap(),
            StoredEntity::QuestCandidate(candidate) => {
                serde_json::to_string(&QuestForge::generate(&candidate.dna).unwrap()).unwrap()
            }
            other => serde_json::to_string(&other).unwrap(),
        })
        .collect()
}

fn legacy_replay_version_signatures() -> Vec<String> {
    let mut character_v1 = CharacterForge::new_dna(
        RootSeed::new(88010),
        "dark-fantasy",
        CreativeAxes::neutral(),
        Default::default(),
    )
    .unwrap();
    character_v1.content_version = ContentVersion(1);
    let character_v1_output =
        CharacterForge::generate_in_project(&character_v1, project_id()).unwrap();
    let character_v2_output = character_candidate(88011);
    let quest_v2_output = quest_candidate(88012, 2);
    let quest_v3_output = quest_candidate(88013, 3);
    vec![
        serde_json::to_string(&character_v1_output).unwrap(),
        serde_json::to_string(&character_v2_output).unwrap(),
        serde_json::to_string(&quest_v2_output).unwrap(),
        serde_json::to_string(&quest_v3_output).unwrap(),
    ]
}

#[test]
fn frozen_schema1_fixture_migrates_losslessly_and_failure_rolls_back_with_backup() {
    let fixture =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/schema1-populated.sqlite3");
    assert!(
        fixture.exists(),
        "frozen schema-1 fixture must be present in source control"
    );
    let success = TestDb::new("schema1-frozen-success");
    fs::copy(&fixture, success.path()).unwrap();
    let source = Connection::open(success.path()).unwrap();
    let before = fixture_rows(&source);
    let replay_before = fixture_replay_signatures(&source);
    let versioned_replay_before = legacy_replay_version_signatures();
    drop(source);
    let migrated = ProjectStore::open(success.path()).unwrap();
    assert_eq!(migrated.list_relationships().unwrap().len(), 0);
    assert_eq!(fixture_rows(&migrated.connection), before);
    assert_eq!(
        fixture_replay_signatures(&migrated.connection),
        replay_before
    );
    assert_eq!(legacy_replay_version_signatures(), versioned_replay_before);
    assert_eq!(
        migrated
            .load_project()
            .unwrap()
            .unwrap()
            .envelope
            .schema_version
            .0,
        2
    );
    let characters = migrated.list_entities().unwrap();
    let mut locked_character = characters
        .iter()
        .find_map(|record| match &record.entity {
            StoredEntity::CharacterCandidate(candidate)
                if candidate.character.envelope.notes.as_deref()
                    == Some("Δοκιμή — frozen schema-one prose") =>
            {
                Some(candidate.clone())
            }
            _ => None,
        })
        .expect("frozen locked Character Candidate");
    let locked_name = locked_character.character.envelope.name.clone();
    let name_key = CharacterField::Name.key().unwrap();
    let name_nonce = locked_character.dna.recipe.field_controls[&name_key].nonce;
    CharacterForge::reroll_field(&mut locked_character, CharacterField::Archetype).unwrap();
    assert_eq!(locked_character.character.envelope.name, locked_name);
    assert_eq!(
        locked_character.dna.recipe.field_controls[&name_key].nonce,
        name_nonce
    );
    let mut locked_quest = characters
        .iter()
        .find_map(|record| match &record.entity {
            StoredEntity::QuestCandidate(candidate)
                if candidate.clue.is_some()
                    && candidate
                        .quest
                        .envelope
                        .tags
                        .iter()
                        .any(|tag| tag == "candidate") =>
            {
                Some(candidate.clone())
            }
            _ => None,
        })
        .expect("frozen Quest v3 Candidate");
    let locked_objective = locked_quest.quest.objective.clone();
    let objective_key = QuestField::Objective.key().unwrap();
    let objective_nonce = locked_quest.dna.recipe.field_controls[&objective_key].nonce;
    QuestForge::reroll_field(&mut locked_quest, QuestField::Twist).unwrap();
    assert_eq!(locked_quest.quest.objective, locked_objective);
    assert_eq!(
        locked_quest.dna.recipe.field_controls[&objective_key].nonce,
        objective_nonce
    );
    migrated.integrity_check().unwrap();
    assert!(migration_backup_path(success.path()).exists());
    drop(migrated);
    let reopened = ProjectStore::open(success.path()).unwrap();
    assert_eq!(
        reopened.load_project().unwrap().unwrap().name,
        "Memory Test World"
    );

    let failure = TestDb::new("schema1-frozen-failure");
    fs::copy(&fixture, failure.path()).unwrap();
    let original_rows = fixture_rows(&Connection::open(failure.path()).unwrap());
    Connection::open(failure.path())
        .unwrap()
        .execute_batch("CREATE TABLE entities_v2(injected INTEGER);")
        .unwrap();
    assert!(matches!(
        ProjectStore::open(failure.path()),
        Err(StoreError::Sqlite(_))
    ));
    let original = Connection::open(failure.path()).unwrap();
    assert_eq!(
        original
            .query_row("PRAGMA user_version", [], |row| row.get::<_, u32>(0))
            .unwrap(),
        1
    );
    assert_eq!(fixture_rows(&original), original_rows);
    assert!(migration_backup_path(failure.path()).exists());
    assert_eq!(
        original
            .query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))
            .unwrap(),
        "ok"
    );
    assert_eq!(
        original
            .query_row("SELECT COUNT(*) FROM projects", [], |row| row
                .get::<_, i64>(0))
            .unwrap(),
        1
    );
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

#[derive(Clone)]
enum GauntletWorking {
    Character(CharacterCandidate),
    Quest(QuestCandidate),
}

impl GauntletWorking {
    fn stored(&self) -> StoredEntity {
        match self {
            Self::Character(value) => StoredEntity::CharacterCandidate(value.clone()),
            Self::Quest(value) => StoredEntity::QuestCandidate(value.clone()),
        }
    }

    fn validate(&self) -> ValidationResult<()> {
        match self {
            Self::Character(value) => value.validate(),
            Self::Quest(value) => value.validate(),
        }
    }
}

struct GauntletRng(u64);

impl GauntletRng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

#[test]
fn deterministic_session_state_gauntlet_runs_one_thousand_reproducible_sequences() {
    const META_SEED: u64 = 0x6a07_2026_cafe_1234;
    const PRESETS: [&str; 11] = [
        "dark-fantasy",
        "pirate",
        "steampunk",
        "cyberpunk",
        "nordic-myth",
        "post-apocalyptic",
        "occult",
        "monster-hunter",
        "rogue-assassin",
        "contemporary-weird",
        "chaos-blend",
    ];
    let db = TestDb::new("gauntlet-session");
    let mut store = create_store(&db);
    let mut total_operations = 0usize;

    for sequence in 0..1_000usize {
        let entity_type = if sequence % 2 == 0 {
            "Character"
        } else {
            "Quest"
        };
        let forge_seed = META_SEED.wrapping_add(sequence as u64 * 0x9e37_79b9);
        let preset = PRESETS[sequence % PRESETS.len()];
        let candidate = if sequence % 2 == 0 {
            let dna = CharacterForge::new_dna(
                RootSeed::new(forge_seed),
                preset,
                CreativeAxes::neutral(),
                Default::default(),
            )
            .unwrap();
            GauntletWorking::Character(
                CharacterForge::generate_in_project(&dna, project_id()).unwrap(),
            )
        } else {
            let dna = QuestForge::new_dna(
                RootSeed::new(forge_seed),
                preset,
                CreativeAxes::neutral(),
                Default::default(),
            )
            .unwrap();
            GauntletWorking::Quest(QuestForge::generate_in_project(&dna, project_id()).unwrap())
        };
        candidate.validate().unwrap();
        let id = candidate.stored().envelope().id;
        let mut version = store.persist_candidate(candidate.stored()).unwrap();
        let mut session = SessionHistory::new(candidate.stored());
        let mut oracle = vec![session.current().clone()];
        let mut cursor = 0usize;
        let mut save_count = 0usize;
        let mut rng = GauntletRng(META_SEED ^ (sequence as u64 + 1));
        let mut operations = Vec::<String>::new();

        for step in 0..32usize {
            let choice = rng.next() % 10;
            let before = session.current().clone();
            let mut next = before.clone();
            let operation = match (&mut next, choice) {
                (StoredEntity::CharacterCandidate(c), 0) => {
                    if c.dna.recipe.field_controls[&CharacterField::Name.key().unwrap()].locked {
                        CharacterForge::unlock(c, CharacterField::Name).unwrap();
                        "unlock-name"
                    } else {
                        CharacterForge::lock(c, CharacterField::Name).unwrap();
                        "lock-name"
                    }
                }
                (StoredEntity::CharacterCandidate(c), 1) => {
                    let fields = [
                        CharacterField::Title,
                        CharacterField::Fear,
                        CharacterField::Desire,
                        CharacterField::Appearance,
                        CharacterField::SignatureGear,
                    ];
                    let offset = rng.next() as usize % fields.len();
                    let field = (0..fields.len())
                        .map(|index| fields[(offset + index) % fields.len()])
                        .find(|field| !c.dna.recipe.field_controls[&field.key().unwrap()].locked)
                        .unwrap();
                    CharacterForge::reroll_field(c, field).unwrap();
                    "reroll-character-field"
                }
                (StoredEntity::CharacterCandidate(c), 2) => {
                    CharacterForge::reroll_all_unlocked(c).unwrap();
                    "reroll-character-all"
                }
                (StoredEntity::CharacterCandidate(c), 3) => {
                    c.manual_edit(
                        CharacterField::Name,
                        CharacterFieldValue::Text(format!("Sequence {sequence} step {step} — Å🜁")),
                    )
                    .unwrap();
                    "edit-character-name"
                }
                (StoredEntity::QuestCandidate(c), 0) => {
                    if c.dna.recipe.field_controls[&QuestField::Title.key().unwrap()].locked {
                        QuestForge::unlock(c, QuestField::Title).unwrap();
                        "unlock-quest-title"
                    } else {
                        QuestForge::lock(c, QuestField::Title).unwrap();
                        "lock-quest-title"
                    }
                }
                (StoredEntity::QuestCandidate(c), 1) => {
                    let fields = [
                        QuestField::Title,
                        QuestField::Premise,
                        QuestField::Patron,
                        QuestField::Objective,
                        QuestField::Stakes,
                        QuestField::Twist,
                        QuestField::MoralPressure,
                    ];
                    let offset = rng.next() as usize % fields.len();
                    let field = (0..fields.len())
                        .map(|index| fields[(offset + index) % fields.len()])
                        .find(|field| !c.dna.recipe.field_controls[&field.key().unwrap()].locked)
                        .unwrap();
                    QuestForge::reroll_field(c, field).unwrap();
                    "reroll-quest-field"
                }
                (StoredEntity::QuestCandidate(c), 2) => {
                    QuestForge::reroll_all(c).unwrap();
                    "reroll-quest-all"
                }
                (StoredEntity::QuestCandidate(c), 3) => {
                    c.manual_edit(
                        QuestField::Title,
                        QuestFieldValue::Text(format!("Sequence {sequence} step {step} — Ö🜁")),
                    )
                    .unwrap();
                    "edit-quest-title"
                }
                (_, 4) => {
                    let records = store.list_history(Some(id)).unwrap();
                    let snapshot = store
                        .load_history_event_snapshot(records[0].event_id)
                        .unwrap()
                        .unwrap();
                    operations.push("inspect-history-read-only".into());
                    assert_eq!(
                        store.list_history(Some(id)).unwrap().len(),
                        records.len(),
                        "history inspection mutated history"
                    );
                    let mut working = snapshot;
                    if let StoredEntity::CharacterCandidate(c) = &mut working {
                        c.validate().unwrap();
                    }
                    if let StoredEntity::QuestCandidate(c) = &mut working {
                        c.validate().unwrap();
                    }
                    session.push(working.clone());
                    oracle.truncate(cursor + 1);
                    oracle.push(working);
                    cursor += 1;
                    "use-history-snapshot-as-working-copy"
                }
                (_, 5) => {
                    let records = store.list_history(Some(id)).unwrap();
                    assert_eq!(
                        store.list_history(Some(id)).unwrap(),
                        records,
                        "history listing changed records"
                    );
                    operations.push("inspect-history-read-only".into());
                    "inspect-history"
                }
                (_, 6) => {
                    let saved = session.current().clone();
                    version = store.save_candidate(saved, version).unwrap();
                    save_count += 1;
                    operations.push("save".into());
                    assert_eq!(
                        store.list_history(Some(id)).unwrap().len(),
                        save_count + 1,
                        "save did not add exactly one history record"
                    );
                    "save"
                }
                (_, 7) => {
                    let actual = session.undo().cloned();
                    let expected = if cursor > 0 {
                        cursor -= 1;
                        Some(oracle[cursor].clone())
                    } else {
                        None
                    };
                    assert_eq!(actual, expected, "Undo oracle mismatch");
                    operations.push("undo".into());
                    "undo"
                }
                (_, 8) => {
                    let actual = session.redo().cloned();
                    let expected = if cursor + 1 < oracle.len() {
                        cursor += 1;
                        Some(oracle[cursor].clone())
                    } else {
                        None
                    };
                    assert_eq!(actual, expected, "Redo oracle mismatch");
                    operations.push("redo".into());
                    "redo"
                }
                (_, _) => {
                    let actual = session.current().clone();
                    assert_eq!(
                        actual, oracle[cursor],
                        "session current diverged from oracle"
                    );
                    operations.push("read-current".into());
                    "read-current"
                }
            };
            if matches!(
                operation,
                "lock-name"
                    | "unlock-name"
                    | "reroll-character-field"
                    | "reroll-character-all"
                    | "edit-character-name"
                    | "lock-quest-title"
                    | "unlock-quest-title"
                    | "reroll-quest-field"
                    | "reroll-quest-all"
                    | "edit-quest-title"
            ) {
                let after = next;
                after.validate().unwrap_or_else(|error| panic!(
                    "G07 sequence={sequence} meta_seed={META_SEED:#x} type={entity_type} preset={preset} forge_seed={forge_seed} operations={operations:?} op={operation}: {error}"
                ));
                if after != before {
                    session.push(after.clone());
                    oracle.truncate(cursor + 1);
                    oracle.push(after);
                    cursor += 1;
                }
            }
            let actual = session.current();
            assert_eq!(
                actual, &oracle[cursor],
                "G07 sequence={sequence} meta_seed={META_SEED:#x} type={entity_type} preset={preset} forge_seed={forge_seed} operations={operations:?} op={operation}: state oracle mismatch"
            );
            total_operations += 1;
        }
    }
    assert_eq!(total_operations, 32_000);
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
fn relationship_candidate_lifecycle_is_revisioned_and_stale_safe() {
    let db = TestDb::new("relationship-lifecycle");
    let mut store = create_store(&db);
    let character = character_candidate(120).character.envelope.id;
    store
        .persist_candidate(StoredEntity::CharacterCandidate(character_candidate(120)))
        .unwrap();
    let location = EntityId::from_bytes([42; 16]);
    add_typed_endpoint(&store, location, "location");
    let relationship = born_in_relationship(character, location);
    assert_eq!(store.create_relationship(relationship.clone()).unwrap(), 1);
    assert_eq!(
        store
            .get_relationship(relationship.id)
            .unwrap()
            .unwrap()
            .relationship,
        relationship
    );
    assert_eq!(store.list_relationships().unwrap().len(), 1);
    assert_eq!(
        store
            .list_relationships_for_entity(character)
            .unwrap()
            .len(),
        1
    );
    assert!(matches!(
        store.create_relationship(relationship.clone()),
        Err(StoreError::Sqlite(_))
    ));
    assert_eq!(store.promote_relationship(relationship.id, 1).unwrap(), 2);
    let promoted = store
        .get_relationship(relationship.id)
        .unwrap()
        .unwrap()
        .relationship;
    let revisions = store.list_relationship_revisions(relationship.id).unwrap();
    assert_eq!(revisions.len(), 1);
    assert_eq!(revisions[0].relationship, promoted);
    assert_eq!(promoted.state, RelationshipState::Canon);

    let mut proposed_b = promoted.clone();
    proposed_b.origin = EntityOrigin::Imported;
    assert_eq!(
        store
            .revise_relationship(proposed_b, 2, "clarified provenance")
            .unwrap(),
        3
    );
    let revisions = store.list_relationship_revisions(relationship.id).unwrap();
    assert_eq!(revisions.len(), 2);
    assert_eq!(revisions[0].relationship, promoted);
    let revised = revisions[1].relationship.clone();
    assert_eq!(revised.origin, EntityOrigin::Imported);
    assert_eq!(
        store
            .get_relationship(relationship.id)
            .unwrap()
            .unwrap()
            .relationship,
        revised
    );

    let mut stale = revised.clone();
    stale.origin = EntityOrigin::Generated;
    assert!(matches!(
        store.revise_relationship(stale, 2, "stale revision"),
        Err(StoreError::Conflict { .. })
    ));
    assert_eq!(
        store.list_relationship_revisions(relationship.id).unwrap(),
        revisions
    );
    assert_eq!(
        store
            .get_relationship(relationship.id)
            .unwrap()
            .unwrap()
            .relationship,
        revised
    );

    let mut proposed_c = revised.clone();
    proposed_c.origin = EntityOrigin::Generated;
    assert_eq!(
        store
            .revise_relationship(proposed_c, 3, "confirmed provenance")
            .unwrap(),
        4
    );
    let revisions = store.list_relationship_revisions(relationship.id).unwrap();
    assert_eq!(revisions.len(), 3);
    assert_eq!(revisions[0].relationship, promoted);
    assert_eq!(revisions[1].relationship, revised);
    let final_revision = revisions[2].relationship.clone();
    assert_eq!(final_revision.origin, EntityOrigin::Generated);
    assert_eq!(
        store
            .get_relationship(relationship.id)
            .unwrap()
            .unwrap()
            .relationship,
        final_revision
    );
    drop(store);

    let mut reopened = ProjectStore::open(db.path()).unwrap();
    let reopened_revisions = reopened
        .list_relationship_revisions(relationship.id)
        .unwrap();
    assert_eq!(reopened_revisions, revisions);
    let reopened_current = reopened.get_relationship(relationship.id).unwrap().unwrap();
    assert_eq!(reopened_current.relationship, final_revision);
    assert_eq!(reopened_current.write_version, 4);

    // Archive behavior remains unchanged: archive retains the prior Canon snapshot.
    assert_eq!(
        reopened.archive_relationship(relationship.id, 4).unwrap(),
        5
    );
    let archived_revisions = reopened
        .list_relationship_revisions(relationship.id)
        .unwrap();
    assert_eq!(archived_revisions.len(), 4);
    assert_eq!(archived_revisions[3].relationship, final_revision);
    assert!(matches!(
        reopened.promote_relationship(relationship.id, 5),
        Err(StoreError::InvalidOperation(_))
    ));
    assert_eq!(
        reopened
            .get_relationship(relationship.id)
            .unwrap()
            .unwrap()
            .write_version,
        5
    );
}

#[test]
fn relationship_rejects_wrong_endpoint_type_and_cross_project_identity() {
    let db = TestDb::new("relationship-invalid");
    let mut store = create_store(&db);
    let candidate = character_candidate(121);
    let character = candidate.character.envelope.id;
    store
        .persist_candidate(StoredEntity::CharacterCandidate(candidate))
        .unwrap();
    let quest = quest_candidate(122, 3);
    let quest_id = quest.quest.envelope.id;
    store
        .persist_candidate(StoredEntity::QuestCandidate(quest))
        .unwrap();
    let location = EntityId::from_bytes([43; 16]);
    add_typed_endpoint(&store, location, "location");
    assert!(matches!(
        store.create_relationship(born_in_relationship(quest_id, location)),
        Err(StoreError::Validation(_))
    ));
    let mut cross_project = born_in_relationship(character, location);
    cross_project.project_id = ProjectId::from_bytes([99; 16]);
    assert!(matches!(
        store.create_relationship(cross_project),
        Err(StoreError::InvalidOperation(_))
    ));
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
fn character_locked_name_value_nonce_and_origin_survive_close_reopen() {
    let db = TestDb::new("character-name-lock-reopen");
    let mut store = create_store(&db);
    let mut candidate = character_candidate(4005);
    candidate.character.envelope.name = "A Name Kept Across Reopen".into();
    CharacterForge::lock(&mut candidate, CharacterField::Name).unwrap();
    let name_key = CharacterField::Name.key().unwrap();
    let title_key = CharacterField::Title.key().unwrap();
    let original_name = candidate.character.envelope.name.clone();
    let name_nonce = candidate.dna.recipe.field_controls[&name_key].nonce;
    let name_origin = candidate.field_origins[&name_key];
    let id = candidate.character.envelope.id;
    store
        .persist_candidate(StoredEntity::CharacterCandidate(candidate))
        .unwrap();
    drop(store);

    let reopened = ProjectStore::open(db.path()).unwrap();
    let row = reopened.get_entity(id).unwrap().unwrap();
    let StoredEntity::CharacterCandidate(mut candidate) = row.entity else {
        panic!("Reopened record is not a Character candidate");
    };
    assert_eq!(candidate.character.envelope.name, original_name);
    assert_eq!(
        candidate.dna.recipe.field_controls[&name_key].nonce,
        name_nonce
    );
    assert_eq!(candidate.field_origins[&name_key], name_origin);
    assert!(candidate.dna.recipe.field_controls[&name_key].locked);

    let title_nonce = candidate.dna.recipe.field_controls[&title_key].nonce;
    CharacterForge::reroll_field(&mut candidate, CharacterField::Archetype).unwrap();
    assert_eq!(candidate.character.envelope.name, original_name);
    assert_eq!(
        candidate.dna.recipe.field_controls[&name_key].nonce,
        name_nonce
    );
    assert_eq!(candidate.field_origins[&name_key], name_origin);
    assert!(candidate.dna.recipe.field_controls[&title_key].nonce > title_nonce);
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
fn character_and_quest_history_working_copy_and_canon_chains_survive_reopen() {
    let db = TestDb::new("gauntlet-revision-chains");
    let mut store = create_store(&db);
    let mut expected_snapshots = Vec::new();

    let mut character = character_candidate(70_001);
    let character_id = character.character.envelope.id;
    let mut character_version = store
        .persist_candidate(StoredEntity::CharacterCandidate(character.clone()))
        .unwrap();
    for revision in 1..=4 {
        character
            .manual_edit(
                CharacterField::Title,
                CharacterFieldValue::Text(format!("Character history branch {revision} — Å🜁")),
            )
            .unwrap();
        CharacterForge::lock(&mut character, CharacterField::Name).unwrap();
        character_version = store
            .save_candidate(
                StoredEntity::CharacterCandidate(character.clone()),
                character_version,
            )
            .unwrap();
        let stored = store.get_entity(character_id).unwrap().unwrap().entity;
        let StoredEntity::CharacterCandidate(persisted) = stored else {
            panic!("save changed Character entity type");
        };
        character = persisted;
        let records = store.list_history(Some(character_id)).unwrap();
        expected_snapshots.push((
            records.last().unwrap().event_id,
            store
                .load_history_event_snapshot(records.last().unwrap().event_id)
                .unwrap()
                .unwrap(),
        ));
        drop(store);
        store = ProjectStore::open(db.path()).unwrap();
        store.integrity_check().unwrap();
        assert_eq!(
            store.get_entity(character_id).unwrap().unwrap().entity,
            StoredEntity::CharacterCandidate(character.clone())
        );
    }
    let history_before_restore = store.list_history(Some(character_id)).unwrap();
    let old_copy = store
        .load_history_event_snapshot(history_before_restore[1].event_id)
        .unwrap()
        .unwrap();
    let StoredEntity::CharacterCandidate(mut old_character) = old_copy else {
        panic!("history snapshot was not a Character candidate");
    };
    old_character.character.envelope.canon_state = CanonState::Candidate;
    old_character
        .manual_edit(
            CharacterField::Fear,
            CharacterFieldValue::Text("An edited old revision stays a new branch".into()),
        )
        .unwrap();
    character_version = store
        .save_candidate(
            StoredEntity::CharacterCandidate(old_character),
            character_version,
        )
        .unwrap();
    assert_eq!(
        store.list_history(Some(character_id)).unwrap().len(),
        history_before_restore.len() + 1
    );
    for (event_id, expected) in &expected_snapshots {
        assert_eq!(
            store
                .load_history_event_snapshot(*event_id)
                .unwrap()
                .as_ref(),
            Some(expected)
        );
    }
    assert_eq!(
        store
            .promote_candidate(character_id, character_version)
            .unwrap(),
        character_version + 1
    );
    for revision in 1..=3 {
        let record = store.get_entity(character_id).unwrap().unwrap();
        let mut revised = record.entity;
        if let StoredEntity::CharacterCandidate(value) = &mut revised {
            value
                .manual_edit(
                    CharacterField::Title,
                    CharacterFieldValue::Text(format!("Canon Character {revision}")),
                )
                .unwrap();
        }
        let version = store
            .get_entity(character_id)
            .unwrap()
            .unwrap()
            .write_version;
        store
            .revise_canon(
                revised,
                version,
                &format!("Character canon revision {revision}"),
            )
            .unwrap();
    }
    let canon_revisions = store.list_revisions(character_id).unwrap();
    assert_eq!(
        canon_revisions
            .iter()
            .map(|r| r.revision_number)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    drop(store);
    store = ProjectStore::open(db.path()).unwrap();
    assert_eq!(store.list_revisions(character_id).unwrap(), canon_revisions);

    let mut quest = quest_candidate(70_002, 3);
    let quest_id = quest.quest.envelope.id;
    let mut quest_version = store
        .persist_candidate(StoredEntity::QuestCandidate(quest.clone()))
        .unwrap();
    for revision in 1..=4 {
        quest
            .manual_edit(
                QuestField::Objective,
                QuestFieldValue::Text(format!("Quest objective branch {revision} — Ö🜁")),
            )
            .unwrap();
        QuestForge::lock(&mut quest, QuestField::Title).unwrap();
        quest_version = store
            .save_candidate(StoredEntity::QuestCandidate(quest.clone()), quest_version)
            .unwrap();
        let stored = store.get_entity(quest_id).unwrap().unwrap().entity;
        let StoredEntity::QuestCandidate(persisted) = stored else {
            panic!("save changed Quest entity type");
        };
        quest = persisted;
        let records = store.list_history(Some(quest_id)).unwrap();
        expected_snapshots.push((
            records.last().unwrap().event_id,
            store
                .load_history_event_snapshot(records.last().unwrap().event_id)
                .unwrap()
                .unwrap(),
        ));
        drop(store);
        store = ProjectStore::open(db.path()).unwrap();
        store.integrity_check().unwrap();
        assert_eq!(
            store.get_entity(quest_id).unwrap().unwrap().entity,
            StoredEntity::QuestCandidate(quest.clone())
        );
    }
    let quest_history = store.list_history(Some(quest_id)).unwrap();
    let old_copy = store
        .load_history_event_snapshot(quest_history[1].event_id)
        .unwrap()
        .unwrap();
    let StoredEntity::QuestCandidate(mut old_quest) = old_copy else {
        panic!("history snapshot was not a Quest candidate");
    };
    old_quest.quest.envelope.canon_state = CanonState::Candidate;
    old_quest
        .manual_edit(
            QuestField::Stakes,
            QuestFieldValue::Text("An old Quest snapshot restored and edited".into()),
        )
        .unwrap();
    quest_version = store
        .save_candidate(StoredEntity::QuestCandidate(old_quest), quest_version)
        .unwrap();
    assert_eq!(
        store.list_history(Some(quest_id)).unwrap().len(),
        quest_history.len() + 1
    );
    assert_eq!(
        store.promote_candidate(quest_id, quest_version).unwrap(),
        quest_version + 1
    );
    for revision in 1..=3 {
        let record = store.get_entity(quest_id).unwrap().unwrap();
        let mut revised = record.entity;
        if let StoredEntity::QuestCandidate(value) = &mut revised {
            value
                .manual_edit(
                    QuestField::Objective,
                    QuestFieldValue::Text(format!("Canon Quest revision {revision}")),
                )
                .unwrap();
        }
        let version = store.get_entity(quest_id).unwrap().unwrap().write_version;
        store
            .revise_canon(
                revised,
                version,
                &format!("Quest canon revision {revision}"),
            )
            .unwrap();
    }
    let quest_revisions = store.list_revisions(quest_id).unwrap();
    assert_eq!(
        quest_revisions
            .iter()
            .map(|r| r.revision_number)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    drop(store);
    store = ProjectStore::open(db.path()).unwrap();
    assert_eq!(store.list_revisions(quest_id).unwrap(), quest_revisions);
    for (event_id, expected) in expected_snapshots {
        assert_eq!(
            store
                .load_history_event_snapshot(event_id)
                .unwrap()
                .as_ref(),
            Some(&expected)
        );
    }
}

#[test]
fn historical_canon_working_copy_is_transient_until_explicit_revision_for_character_and_quest() {
    let db = TestDb::new("gauntlet-canon-working-copy-save");
    let mut store = create_store(&db);

    let character = character_candidate(71_001);
    let character_id = character.character.envelope.id;
    let character_version = store
        .persist_candidate(StoredEntity::CharacterCandidate(character))
        .unwrap();
    store
        .promote_candidate(character_id, character_version)
        .unwrap();
    for (title, reason) in [("Character B", "Canon B"), ("Character C", "Canon C")] {
        let mut entity = store.get_entity(character_id).unwrap().unwrap().entity;
        let StoredEntity::CharacterCandidate(value) = &mut entity else {
            panic!("Character canon should retain candidate provenance");
        };
        value
            .manual_edit(
                CharacterField::Title,
                CharacterFieldValue::Text(title.into()),
            )
            .unwrap();
        let version = store
            .get_entity(character_id)
            .unwrap()
            .unwrap()
            .write_version;
        store.revise_canon(entity, version, reason).unwrap();
    }
    let character_revisions_abc = store.list_revisions(character_id).unwrap();
    assert_eq!(character_revisions_abc.len(), 3);
    let character_c = store.get_entity(character_id).unwrap().unwrap();

    let mut character_a = character_revisions_abc[0].entity.clone();
    character_a.set_state(CanonState::Candidate);
    let mut character_working = character_a.clone();
    let StoredEntity::CharacterCandidate(value) = &mut character_working else {
        panic!("Character revision should retain candidate provenance");
    };
    value
        .manual_edit(
            CharacterField::Title,
            CharacterFieldValue::Text("Edited working copy from A".into()),
        )
        .unwrap();
    CharacterForge::lock(value, CharacterField::Name).unwrap();
    CharacterForge::reroll_field(value, CharacterField::Title).unwrap();
    let mut character_session = SessionHistory::new(character_a.clone());
    character_session.push(character_working.clone());
    assert_eq!(character_session.undo(), Some(&character_a));
    assert_eq!(character_session.redo(), Some(&character_working));
    // This is the canceled confirmation path: no persistence operation is invoked.
    assert_eq!(
        store.get_entity(character_id).unwrap().unwrap(),
        character_c
    );
    assert_eq!(
        store.list_revisions(character_id).unwrap(),
        character_revisions_abc
    );

    // Repeat loading A as a fresh working copy after the canceled save.
    let mut character_a_again = character_revisions_abc[0].entity.clone();
    character_a_again.set_state(CanonState::Candidate);
    let mut character_d = character_a_again;
    let StoredEntity::CharacterCandidate(value) = &mut character_d else {
        panic!("Character revision should retain candidate provenance");
    };
    value
        .manual_edit(
            CharacterField::Fear,
            CharacterFieldValue::Text("Confirmed copy of A as D".into()),
        )
        .unwrap();
    CharacterForge::lock(value, CharacterField::Name).unwrap();
    character_d.set_state(CanonState::Canon);
    let next_version = store
        .revise_canon(character_d, character_c.write_version, "Confirmed Canon D")
        .unwrap();
    let character_revisions_abcd = store.list_revisions(character_id).unwrap();
    assert_eq!(character_revisions_abcd.len(), 4);
    assert_eq!(
        &character_revisions_abcd[..3],
        character_revisions_abc.as_slice()
    );
    assert_eq!(character_revisions_abcd[3].revision_number, 4);
    assert_eq!(
        character_revisions_abcd[3].entity.envelope().id,
        character_id
    );
    let StoredEntity::CharacterCandidate(value) = &character_revisions_abcd[3].entity else {
        panic!("Character revision should retain candidate provenance");
    };
    assert!(value.dna.recipe.field_controls[&CharacterField::Name.key().unwrap()].locked);
    assert_eq!(
        value.field_origins[&CharacterField::Fear.key().unwrap()],
        FieldValueOrigin::UserModified
    );
    assert_eq!(
        store.get_entity(character_id).unwrap().unwrap().entity,
        character_revisions_abcd[3].entity
    );
    assert_eq!(
        store
            .get_entity(character_id)
            .unwrap()
            .unwrap()
            .write_version,
        next_version
    );
    assert_eq!(store.list_entities().unwrap().len(), 1);

    let quest = quest_candidate(71_002, 3);
    let quest_id = quest.quest.envelope.id;
    let quest_version = store
        .persist_candidate(StoredEntity::QuestCandidate(quest))
        .unwrap();
    store.promote_candidate(quest_id, quest_version).unwrap();
    for (objective, reason) in [("Quest B", "Canon B"), ("Quest C", "Canon C")] {
        let mut entity = store.get_entity(quest_id).unwrap().unwrap().entity;
        let StoredEntity::QuestCandidate(value) = &mut entity else {
            panic!("Quest canon should retain candidate provenance");
        };
        value
            .manual_edit(
                QuestField::Objective,
                QuestFieldValue::Text(objective.into()),
            )
            .unwrap();
        let version = store.get_entity(quest_id).unwrap().unwrap().write_version;
        store.revise_canon(entity, version, reason).unwrap();
    }
    let quest_revisions_abc = store.list_revisions(quest_id).unwrap();
    assert_eq!(quest_revisions_abc.len(), 3);
    let quest_c = store.get_entity(quest_id).unwrap().unwrap();

    let mut quest_a = quest_revisions_abc[0].entity.clone();
    quest_a.set_state(CanonState::Candidate);
    let mut quest_working = quest_a.clone();
    let StoredEntity::QuestCandidate(value) = &mut quest_working else {
        panic!("Quest revision should retain candidate provenance");
    };
    value
        .manual_edit(
            QuestField::Objective,
            QuestFieldValue::Text("Edited working copy from A".into()),
        )
        .unwrap();
    QuestForge::lock(value, QuestField::Title).unwrap();
    QuestForge::reroll_field(value, QuestField::Premise).unwrap();
    let mut quest_session = SessionHistory::new(quest_a.clone());
    quest_session.push(quest_working.clone());
    assert_eq!(quest_session.undo(), Some(&quest_a));
    assert_eq!(quest_session.redo(), Some(&quest_working));
    assert_eq!(store.get_entity(quest_id).unwrap().unwrap(), quest_c);
    assert_eq!(store.list_revisions(quest_id).unwrap(), quest_revisions_abc);

    let mut quest_a_again = quest_revisions_abc[0].entity.clone();
    quest_a_again.set_state(CanonState::Candidate);
    let mut quest_d = quest_a_again;
    let StoredEntity::QuestCandidate(value) = &mut quest_d else {
        panic!("Quest revision should retain candidate provenance");
    };
    value
        .manual_edit(
            QuestField::Stakes,
            QuestFieldValue::Text("Confirmed copy of A as D".into()),
        )
        .unwrap();
    QuestForge::lock(value, QuestField::Title).unwrap();
    quest_d.set_state(CanonState::Canon);
    store
        .revise_canon(quest_d, quest_c.write_version, "Confirmed Canon D")
        .unwrap();
    let quest_revisions_abcd = store.list_revisions(quest_id).unwrap();
    assert_eq!(quest_revisions_abcd.len(), 4);
    assert_eq!(&quest_revisions_abcd[..3], quest_revisions_abc.as_slice());
    assert_eq!(quest_revisions_abcd[3].revision_number, 4);
    assert_eq!(quest_revisions_abcd[3].entity.envelope().id, quest_id);
    let StoredEntity::QuestCandidate(value) = &quest_revisions_abcd[3].entity else {
        panic!("Quest revision should retain candidate provenance");
    };
    assert!(value.dna.recipe.field_controls[&QuestField::Title.key().unwrap()].locked);
    assert_eq!(
        serde_json::to_value(value.field_origins[&QuestField::Stakes.key().unwrap()]).unwrap(),
        "user_modified"
    );
    assert_eq!(
        store.get_entity(quest_id).unwrap().unwrap().entity,
        quest_revisions_abcd[3].entity
    );
    assert_eq!(store.list_entities().unwrap().len(), 2);

    drop(store);
    let reopened = ProjectStore::open(db.path()).unwrap();
    reopened.integrity_check().unwrap();
    assert_eq!(
        reopened.list_revisions(character_id).unwrap(),
        character_revisions_abcd
    );
    assert_eq!(
        reopened.list_revisions(quest_id).unwrap(),
        quest_revisions_abcd
    );
    assert_eq!(
        reopened.get_entity(character_id).unwrap().unwrap().entity,
        character_revisions_abcd[3].entity
    );
    assert_eq!(
        reopened.get_entity(quest_id).unwrap().unwrap().entity,
        quest_revisions_abcd[3].entity
    );
}

#[test]
fn foreign_key_corruption_is_reported_and_does_not_affect_an_independent_project() {
    let corrupt_db = TestDb::new("gauntlet-foreign-key-corrupt");
    let healthy_db = TestDb::new("gauntlet-foreign-key-healthy");
    let mut corrupt_store = create_store(&corrupt_db);
    let candidate = character_candidate(80_001);
    let entity_id = candidate.character.envelope.id;
    corrupt_store
        .persist_candidate(StoredEntity::CharacterCandidate(candidate))
        .unwrap();
    drop(corrupt_store);

    let raw = Connection::open(corrupt_db.path()).unwrap();
    raw.pragma_update(None, "foreign_keys", false).unwrap();
    raw.execute(
        "UPDATE entities SET project_id = ?1 WHERE id = ?2",
        rusqlite::params![
            "00000000-0000-0000-0000-000000000099",
            entity_id.uuid().to_string()
        ],
    )
    .unwrap();
    drop(raw);

    assert!(matches!(
        ProjectStore::open(corrupt_db.path()),
        Err(StoreError::Corrupt(message)) if message.contains("foreign-key violations")
    ));

    let healthy = create_store(&healthy_db);
    healthy.integrity_check().unwrap();
    assert_eq!(
        healthy.load_project().unwrap().unwrap().envelope.id,
        project_id()
    );
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
