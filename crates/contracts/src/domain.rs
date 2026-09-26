use crate::{
    ContentVersion, EntityId, GenerationMetadata, GeneratorVersion, ProjectId, SchemaVersion,
    ValidationCode, ValidationError, ValidationErrors, ValidationResult,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Character,
    Quest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonState {
    Draft,
    Candidate,
    Canon,
    Archived,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityOrigin {
    Manual,
    Generated,
    Imported,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectEnvelope {
    pub id: ProjectId,
    pub schema_version: SchemaVersion,
    pub generator_version: GeneratorVersion,
    pub content_version: ContentVersion,
}

impl ProjectEnvelope {
    pub fn validate(&self) -> ValidationResult<()> {
        self.id.validate("project.id")?;
        validate_version(self.schema_version.0, "project.schema_version")?;
        validate_version(self.generator_version.0, "project.generator_version")?;
        validate_version(self.content_version.0, "project.content_version")
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityEnvelope {
    pub id: EntityId,
    pub project_id: ProjectId,
    pub entity_type: EntityType,
    pub name: String,
    pub schema_version: SchemaVersion,
    pub canon_state: CanonState,
    pub origin: EntityOrigin,
    pub favorite: bool,
    pub created_at_unix_ms: i64,
    pub updated_at_unix_ms: i64,
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub generation_provenance: Option<GenerationMetadata>,
}

impl EntityEnvelope {
    pub fn validate(&self, expected_type: EntityType) -> ValidationResult<()> {
        self.id.validate("entity.id")?;
        self.project_id.validate("entity.project_id")?;
        validate_version(self.schema_version.0, "entity.schema_version")?;
        if self.entity_type != expected_type {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::InvalidEntity,
                "entity.entity_type",
                "entity envelope type does not match its payload",
            )));
        }
        validate_text("entity.name", &self.name)?;
        if self.created_at_unix_ms < 0 || self.updated_at_unix_ms < self.created_at_unix_ms {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::InvalidTimestamp,
                "entity.timestamps",
                "timestamps must be non-negative and updated_at must not precede created_at",
            )));
        }
        for (index, tag) in self.tags.iter().enumerate() {
            validate_text(&format!("entity.tags[{index}]"), tag)?;
        }
        if let Some(provenance) = &self.generation_provenance {
            provenance
                .generation_id
                .validate("generation.generation_id")?;
            if provenance.generated_at_unix_ms < 0 {
                return Err(ValidationErrors::one(ValidationError::new(
                    ValidationCode::InvalidTimestamp,
                    "generation.generated_at_unix_ms",
                    "generation timestamp must not be negative",
                )));
            }
            if let Some(parent) = provenance.parent_generation_id {
                parent.validate("generation.parent_generation_id")?;
            }
            provenance.dna.validate()?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Character {
    pub envelope: EntityEnvelope,
    pub aliases: Vec<String>,
    pub title: String,
    pub archetype: String,
    pub role: String,
    pub appearance: String,
    pub faction_hint: String,
    pub signature_gear: String,
    pub personality: Vec<String>,
    pub desire: String,
    pub fear: String,
    pub flaw: String,
    pub secret: String,
    pub skills: Vec<String>,
    pub abilities: Vec<String>,
    pub gear_links: Vec<EntityId>,
    pub faction_links: Vec<EntityId>,
    pub relationship_links: Vec<EntityId>,
    pub relationship_hook: String,
    pub backstory_hook: String,
    pub story_hooks: Vec<String>,
}

impl Character {
    pub fn validate(&self) -> ValidationResult<()> {
        self.envelope.validate(EntityType::Character)?;
        for (path, value) in [
            ("character.title", &self.title),
            ("character.archetype", &self.archetype),
            ("character.role", &self.role),
            ("character.appearance", &self.appearance),
            ("character.faction_hint", &self.faction_hint),
            ("character.signature_gear", &self.signature_gear),
            ("character.desire", &self.desire),
            ("character.fear", &self.fear),
            ("character.flaw", &self.flaw),
            ("character.secret", &self.secret),
            ("character.relationship_hook", &self.relationship_hook),
            ("character.backstory_hook", &self.backstory_hook),
        ] {
            validate_text(path, value)?;
        }
        for (path, values) in [
            ("character.aliases", &self.aliases),
            ("character.personality", &self.personality),
            ("character.skills", &self.skills),
            ("character.abilities", &self.abilities),
            ("character.story_hooks", &self.story_hooks),
        ] {
            validate_text_list(path, values)?;
        }
        for (path, values, minimum) in [
            ("character.personality", &self.personality, 2),
            ("character.skills", &self.skills, 2),
            ("character.abilities", &self.abilities, 1),
            ("character.story_hooks", &self.story_hooks, 3),
        ] {
            if values.len() < minimum {
                return Err(ValidationErrors::one(ValidationError::new(
                    ValidationCode::MissingField,
                    path,
                    format!("must contain at least {minimum} values"),
                )));
            }
        }
        validate_id_list("character.gear_links", &self.gear_links)?;
        validate_id_list("character.faction_links", &self.faction_links)?;
        validate_id_list("character.relationship_links", &self.relationship_links)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Quest {
    pub envelope: EntityEnvelope,
    pub quest_type: String,
    pub premise: String,
    pub patron: String,
    pub objective: String,
    pub stakes: String,
    pub opposition: String,
    pub complications: Vec<String>,
    pub twist: String,
    pub moral_pressure: String,
    pub locations: Vec<String>,
    pub rewards: Vec<String>,
    pub failure_consequences: Vec<String>,
    pub follow_up_hooks: Vec<String>,
}

impl Quest {
    pub fn validate(&self) -> ValidationResult<()> {
        self.envelope.validate(EntityType::Quest)?;
        for (path, value) in [
            ("quest.quest_type", &self.quest_type),
            ("quest.premise", &self.premise),
            ("quest.patron", &self.patron),
            ("quest.objective", &self.objective),
            ("quest.stakes", &self.stakes),
            ("quest.opposition", &self.opposition),
            ("quest.twist", &self.twist),
            ("quest.moral_pressure", &self.moral_pressure),
        ] {
            validate_text(path, value)?;
        }
        for (path, values) in [
            ("quest.complications", &self.complications),
            ("quest.locations", &self.locations),
            ("quest.rewards", &self.rewards),
            ("quest.failure_consequences", &self.failure_consequences),
            ("quest.follow_up_hooks", &self.follow_up_hooks),
        ] {
            validate_text_list(path, values)?;
        }
        Ok(())
    }
}

fn validate_version(version: u16, path: &str) -> ValidationResult<()> {
    if version == 0 {
        return Err(ValidationErrors::one(ValidationError::new(
            ValidationCode::UnsupportedVersion,
            path,
            "version 0 is reserved for uninitialized data",
        )));
    }
    Ok(())
}

fn validate_text(path: &str, value: &str) -> ValidationResult<()> {
    if value.trim().is_empty() {
        return Err(ValidationErrors::one(ValidationError::new(
            ValidationCode::EmptyValue,
            path,
            "value must contain non-whitespace text",
        )));
    }
    Ok(())
}

fn validate_text_list(path: &str, values: &[String]) -> ValidationResult<()> {
    for (index, value) in values.iter().enumerate() {
        validate_text(&format!("{path}[{index}]"), value)?;
    }
    Ok(())
}

fn validate_id_list(path: &str, values: &[EntityId]) -> ValidationResult<()> {
    for (index, value) in values.iter().enumerate() {
        value.validate(&format!("{path}[{index}]"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CreativeAxes, FieldKey, ForgeDNA, GenerationKind, GenerationRecipe,
        INITIAL_CONTENT_VERSION, INITIAL_GENERATOR_VERSION, INITIAL_SCHEMA_VERSION, PresetId,
        RootSeed,
    };

    fn id(byte: u8) -> EntityId {
        let mut bytes = [0; 16];
        bytes[15] = byte;
        EntityId::from_bytes(bytes)
    }

    fn envelope(entity_type: EntityType) -> EntityEnvelope {
        EntityEnvelope {
            id: id(1),
            project_id: ProjectId::from_bytes({
                let mut bytes = [0; 16];
                bytes[15] = 2;
                bytes
            }),
            entity_type,
            name: "The Glass Cartographer".to_owned(),
            schema_version: INITIAL_SCHEMA_VERSION,
            canon_state: CanonState::Candidate,
            origin: EntityOrigin::Generated,
            favorite: false,
            created_at_unix_ms: 1,
            updated_at_unix_ms: 1,
            tags: vec!["traveler".to_owned()],
            notes: None,
            generation_provenance: None,
        }
    }

    #[test]
    fn project_and_entity_contracts_validate_strong_identity_and_versions() {
        let project = ProjectEnvelope {
            id: ProjectId::from_bytes({
                let mut bytes = [0; 16];
                bytes[15] = 2;
                bytes
            }),
            schema_version: INITIAL_SCHEMA_VERSION,
            generator_version: INITIAL_GENERATOR_VERSION,
            content_version: INITIAL_CONTENT_VERSION,
        };
        assert!(project.validate().is_ok());
        let mut invalid = envelope(EntityType::Character);
        invalid.updated_at_unix_ms = 0;
        invalid.created_at_unix_ms = 2;
        assert_eq!(
            invalid.validate(EntityType::Character).unwrap_err().errors[0].code,
            ValidationCode::InvalidTimestamp
        );
        assert_eq!(
            envelope(EntityType::Quest)
                .validate(EntityType::Character)
                .unwrap_err()
                .errors[0]
                .code,
            ValidationCode::InvalidEntity
        );
    }

    #[test]
    fn character_and_quest_are_explicit_typed_contracts() {
        let character = Character {
            envelope: envelope(EntityType::Character),
            aliases: vec![],
            title: "The Glass Cartographer".into(),
            archetype: "cartographer".into(),
            role: "guide".into(),
            appearance: "ink-stained coat".into(),
            faction_hint: "Harbor Surveyors".into(),
            signature_gear: "brass compass".into(),
            personality: vec!["patient".into(), "observant".into()],
            desire: "map the shifting coast".into(),
            fear: "losing the route".into(),
            flaw: "overconfident".into(),
            secret: "the charts are forged".into(),
            skills: vec!["navigation".into(), "surveying".into()],
            abilities: vec!["can read a moving coastline".into()],
            gear_links: vec![],
            faction_links: vec![],
            relationship_links: vec![],
            relationship_hook: "A rival knows the true shoreline".into(),
            backstory_hook: "The map records a drowned town".into(),
            story_hooks: vec![
                "the coast moves".into(),
                "a chart is forged".into(),
                "a rival returns".into(),
            ],
        };
        assert!(character.validate().is_ok());
        let mut quest = Quest {
            envelope: envelope(EntityType::Quest),
            quest_type: "recovery".into(),
            premise: "find the chart".into(),
            patron: "the harbor council".into(),
            objective: "recover the original".into(),
            stakes: "routes close".into(),
            opposition: "a rival surveyor".into(),
            complications: vec![],
            twist: "the chart is current".into(),
            moral_pressure: "the rival needs it".into(),
            locations: vec![],
            rewards: vec![],
            failure_consequences: vec![],
            follow_up_hooks: vec![],
        };
        assert!(quest.validate().is_ok());
        quest.objective.clear();
        assert_eq!(
            quest.validate().unwrap_err().errors[0].code,
            ValidationCode::EmptyValue
        );
    }

    #[test]
    fn generated_provenance_carries_versioned_dna() {
        let mut recipe = GenerationRecipe::new(
            GenerationKind::Character,
            RootSeed::new(7),
            PresetId::new("core").unwrap(),
        );
        recipe.add_field(FieldKey::new("name").unwrap()).unwrap();
        let metadata = GenerationMetadata {
            generation_id: id(3),
            generated_at_unix_ms: 10,
            action: crate::GenerationAction::Forge,
            parent_generation_id: None,
            dna: ForgeDNA::new(recipe),
        };
        let mut character = Character {
            envelope: envelope(EntityType::Character),
            aliases: vec![],
            title: "The Glass Cartographer".into(),
            archetype: "scout".into(),
            role: "guide".into(),
            appearance: "blue cloak".into(),
            faction_hint: "Harbor Surveyors".into(),
            signature_gear: "brass compass".into(),
            personality: vec!["patient".into(), "guarded".into()],
            desire: "find home".into(),
            fear: "dark".into(),
            flaw: "proud".into(),
            secret: "none".into(),
            skills: vec!["navigation".into(), "tracking".into()],
            abilities: vec!["can hear a route change".into()],
            gear_links: vec![],
            faction_links: vec![],
            relationship_links: vec![],
            relationship_hook: "A rival knows the true shoreline".into(),
            backstory_hook: "The map records a drowned town".into(),
            story_hooks: vec![
                "the coast moves".into(),
                "a chart is forged".into(),
                "a rival returns".into(),
            ],
        };
        character.envelope.generation_provenance = Some(metadata);
        assert!(character.validate().is_ok());
        assert_eq!(CreativeAxes::default().levels.len(), 8);
    }
}
