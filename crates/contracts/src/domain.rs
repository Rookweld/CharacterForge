use crate::{
    ContentVersion, EntityId, GenerationMetadata, GeneratorVersion, ProjectId, SchemaVersion,
    ValidationCode, ValidationError, ValidationErrors, ValidationResult,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Character,
    Quest,
    Location,
    Faction,
    Item,
    Creature,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    BornIn,
    MemberOf,
    Owns,
    OpposedBy,
}

impl RelationshipType {
    pub const fn key(self) -> &'static str {
        match self {
            Self::BornIn => "born_in",
            Self::MemberOf => "member_of",
            Self::Owns => "owns",
            Self::OpposedBy => "opposed_by",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipState {
    Candidate,
    Canon,
    Archived,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Relationship {
    pub id: crate::RelationshipId,
    pub project_id: ProjectId,
    pub relationship_type: RelationshipType,
    pub source_id: EntityId,
    pub target_id: EntityId,
    pub state: RelationshipState,
    pub origin: EntityOrigin,
    pub created_at_unix_ms: i64,
    pub updated_at_unix_ms: i64,
}

impl Relationship {
    pub fn validate(&self) -> ValidationResult<()> {
        self.id.validate("relationship.id")?;
        self.project_id.validate("relationship.project_id")?;
        self.source_id.validate("relationship.source_id")?;
        self.target_id.validate("relationship.target_id")?;
        if self.source_id == self.target_id {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::InvalidEntity,
                "relationship.endpoints",
                "source and target must be distinct entities",
            )));
        }
        if self.created_at_unix_ms < 0 || self.updated_at_unix_ms < self.created_at_unix_ms {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::InvalidTimestamp,
                "relationship.timestamps",
                "timestamps must be non-negative and updated_at must not precede created_at",
            )));
        }
        Ok(())
    }
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

#[cfg(test)]
mod substrate_contract_tests {
    use super::*;
    use crate::RelationshipId;

    #[test]
    fn entity_type_serialization_keeps_existing_and_new_keys_stable() {
        let expected = [
            (EntityType::Character, "\"character\""),
            (EntityType::Quest, "\"quest\""),
            (EntityType::Location, "\"location\""),
            (EntityType::Faction, "\"faction\""),
            (EntityType::Item, "\"item\""),
            (EntityType::Creature, "\"creature\""),
        ];
        for (kind, json) in expected {
            assert_eq!(serde_json::to_string(&kind).unwrap(), json);
            assert_eq!(serde_json::from_str::<EntityType>(json).unwrap(), kind);
        }
    }

    #[test]
    fn relationship_contract_rejects_nil_ids_and_has_stable_type_keys() {
        assert_eq!(RelationshipType::BornIn.key(), "born_in");
        assert_eq!(RelationshipType::MemberOf.key(), "member_of");
        assert_eq!(RelationshipType::Owns.key(), "owns");
        assert_eq!(RelationshipType::OpposedBy.key(), "opposed_by");
        assert!(
            RelationshipId::from_bytes([0; 16])
                .validate("relationship.id")
                .is_err()
        );
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocationProfile {
    Settlement,
    District,
    Building,
    Dungeon,
    Ruins,
    Wilderness,
    Landmark,
    Interior,
    Site,
    Other,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FactionProfile {
    Guild,
    Order,
    Cult,
    Government,
    Kingdom,
    Military,
    Corporation,
    Gang,
    Rebellion,
    ReligiousOrder,
    Tribe,
    SecretSociety,
    Other,
}

impl FactionProfile {
    pub const ALL: [Self; 13] = [
        Self::Guild,
        Self::Order,
        Self::Cult,
        Self::Government,
        Self::Kingdom,
        Self::Military,
        Self::Corporation,
        Self::Gang,
        Self::Rebellion,
        Self::ReligiousOrder,
        Self::Tribe,
        Self::SecretSociety,
        Self::Other,
    ];
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Faction {
    pub envelope: EntityEnvelope,
    pub faction_profile: FactionProfile,
    pub identity: String,
    pub ideology: String,
    pub primary_goal: String,
    pub methods: String,
    pub culture: String,
    pub resources: String,
    pub public_face: String,
    pub hidden_truth: String,
    pub symbols: String,
    pub reputation: String,
    pub story_hooks: Vec<String>,
}

impl Faction {
    pub fn validate(&self) -> ValidationResult<()> {
        self.envelope.validate(EntityType::Faction)?;
        for (path, value) in [
            ("faction.identity", &self.identity),
            ("faction.ideology", &self.ideology),
            ("faction.primary_goal", &self.primary_goal),
            ("faction.methods", &self.methods),
            ("faction.culture", &self.culture),
            ("faction.resources", &self.resources),
            ("faction.public_face", &self.public_face),
            ("faction.hidden_truth", &self.hidden_truth),
            ("faction.symbols", &self.symbols),
            ("faction.reputation", &self.reputation),
        ] {
            validate_text(path, value)?;
        }
        validate_text_list("faction.story_hooks", &self.story_hooks)?;
        if self.story_hooks.is_empty() {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::MissingField,
                "faction.story_hooks",
                "must contain at least one value",
            )));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ItemProfile {
    Weapon,
    Armor,
    Artifact,
    Consumable,
    Tool,
    KeyItem,
    Resource,
    Treasure,
    Book,
    QuestItem,
    Other,
}

impl ItemProfile {
    pub const ALL: [Self; 11] = [
        Self::Weapon,
        Self::Armor,
        Self::Artifact,
        Self::Consumable,
        Self::Tool,
        Self::KeyItem,
        Self::Resource,
        Self::Treasure,
        Self::Book,
        Self::QuestItem,
        Self::Other,
    ];
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub envelope: EntityEnvelope,
    pub item_profile: ItemProfile,
    pub appearance: String,
    pub purpose: String,
    pub properties: String,
    pub origin: String,
    pub history: String,
    pub power_or_function: String,
    pub cost_or_drawback: String,
    pub rarity: String,
    pub condition: String,
    pub secret: String,
    pub story_hooks: Vec<String>,
}

impl Item {
    pub fn validate(&self) -> ValidationResult<()> {
        self.envelope.validate(EntityType::Item)?;
        for (path, value) in [
            ("item.appearance", &self.appearance),
            ("item.purpose", &self.purpose),
            ("item.properties", &self.properties),
            ("item.origin", &self.origin),
            ("item.history", &self.history),
            ("item.power_or_function", &self.power_or_function),
            ("item.cost_or_drawback", &self.cost_or_drawback),
            ("item.rarity", &self.rarity),
            ("item.condition", &self.condition),
            ("item.secret", &self.secret),
        ] {
            validate_text(path, value)?;
        }
        validate_text_list("item.story_hooks", &self.story_hooks)?;
        if self.story_hooks.is_empty() {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::MissingField,
                "item.story_hooks",
                "must contain at least one value",
            )));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Location {
    pub envelope: EntityEnvelope,
    pub location_profile: LocationProfile,
    pub summary: String,
    pub visual_identity: String,
    pub atmosphere: String,
    pub purpose: String,
    pub history: String,
    pub inhabitants_summary: String,
    pub notable_features: Vec<String>,
    pub danger: String,
    pub secret: String,
    pub story_hooks: Vec<String>,
}

impl Location {
    pub fn validate(&self) -> ValidationResult<()> {
        self.envelope.validate(EntityType::Location)?;
        for (path, value) in [
            ("location.summary", &self.summary),
            ("location.visual_identity", &self.visual_identity),
            ("location.atmosphere", &self.atmosphere),
            ("location.purpose", &self.purpose),
            ("location.history", &self.history),
            ("location.inhabitants_summary", &self.inhabitants_summary),
            ("location.danger", &self.danger),
            ("location.secret", &self.secret),
        ] {
            validate_text(path, value)?;
        }
        for (path, values) in [
            ("location.notable_features", &self.notable_features),
            ("location.story_hooks", &self.story_hooks),
        ] {
            validate_text_list(path, values)?;
            if values.is_empty() {
                return Err(ValidationErrors::one(ValidationError::new(
                    ValidationCode::MissingField,
                    path,
                    "must contain at least one value",
                )));
            }
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
    fn location_contract_is_typed_validated_and_profile_keys_are_stable() {
        let mut location = Location {
            envelope: envelope(EntityType::Location),
            location_profile: LocationProfile::Ruins,
            summary: "A collapsed observatory watches the salt flats.".into(),
            visual_identity: "Copper ribs rise through pale stone.".into(),
            atmosphere: "Wind carries a low metallic note.".into(),
            purpose: "It once measured the returning tide.".into(),
            history: "Its keepers sealed the western stair.".into(),
            inhabitants_summary: "Three families shelter in the dry vault.".into(),
            notable_features: vec!["A cracked meridian lens".into()],
            danger: "The upper gallery sheds stone without warning.".into(),
            secret: "The tide charts were altered before the collapse.".into(),
            story_hooks: vec!["A fresh lamp burns below the sealed stair.".into()],
        };
        assert!(location.validate().is_ok());
        assert_eq!(
            serde_json::to_string(&location.location_profile).unwrap(),
            "\"ruins\""
        );
        location.notable_features.clear();
        assert!(location.validate().is_err());
        assert_eq!(
            serde_json::to_string(&LocationProfile::Other).unwrap(),
            "\"other\""
        );
    }

    #[test]
    fn faction_contract_is_typed_validated_and_profiles_have_stable_keys() {
        let faction = Faction {
            envelope: envelope(EntityType::Faction),
            faction_profile: FactionProfile::ReligiousOrder,
            identity: "A Unicode-safe identity — with purpose.".into(),
            ideology: "Belief".into(),
            primary_goal: "Goal".into(),
            methods: "Methods".into(),
            culture: "Culture".into(),
            resources: "Resources".into(),
            public_face: "Public face".into(),
            hidden_truth: "Hidden truth".into(),
            symbols: "Symbols".into(),
            reputation: "Reputation".into(),
            story_hooks: vec!["Hook one".into(), "Hook two".into(), "Hook three".into()],
        };
        faction.validate().unwrap();
        assert_eq!(FactionProfile::ALL.len(), 13);
        assert_eq!(
            serde_json::to_string(&FactionProfile::ReligiousOrder).unwrap(),
            "\"religious-order\""
        );
        assert_eq!(
            serde_json::from_str::<FactionProfile>("\"secret-society\"").unwrap(),
            FactionProfile::SecretSociety
        );
        let mut invalid = faction.clone();
        invalid.identity.clear();
        assert!(invalid.validate().is_err());
        assert!(serde_json::from_str::<Faction>("{\"unknown\": true}").is_err());
    }

    #[test]
    fn item_contract_is_typed_validated_and_profiles_have_stable_keys() {
        let item = Item {
            envelope: envelope(EntityType::Item),
            item_profile: ItemProfile::KeyItem,
            appearance: "A Unicode-safe appearance — with purpose.".into(),
            purpose: "Belief".into(),
            properties: "Goal".into(),
            origin: "Origin".into(),
            history: "History".into(),
            power_or_function: "PowerOrFunction".into(),
            cost_or_drawback: "Public face".into(),
            rarity: "Hidden truth".into(),
            condition: "Condition".into(),
            secret: "Secret".into(),
            story_hooks: vec!["Hook one".into(), "Hook two".into(), "Hook three".into()],
        };
        item.validate().unwrap();
        for (profile, key) in ItemProfile::ALL.into_iter().zip([
            "weapon",
            "armor",
            "artifact",
            "consumable",
            "tool",
            "key-item",
            "resource",
            "treasure",
            "book",
            "quest-item",
            "other",
        ]) {
            assert_eq!(serde_json::to_value(profile).unwrap(), key);
            assert_eq!(
                serde_json::from_value::<ItemProfile>(serde_json::json!(key)).unwrap(),
                profile
            );
        }
        let encoded = serde_json::to_value(&item).unwrap();
        for field in [
            "appearance",
            "purpose",
            "properties",
            "origin",
            "history",
            "power_or_function",
            "cost_or_drawback",
            "rarity",
            "condition",
            "secret",
        ] {
            let mut invalid = encoded.clone();
            invalid[field] = serde_json::json!("  ");
            assert!(
                serde_json::from_value::<Item>(invalid)
                    .unwrap()
                    .validate()
                    .is_err(),
                "{field}"
            );
        }
        let mut invalid = item.clone();
        invalid.envelope.entity_type = EntityType::Faction;
        assert!(invalid.validate().is_err());
        invalid = item.clone();
        invalid.story_hooks.clear();
        assert!(invalid.validate().is_err());
        let mut invalid = encoded.clone();
        invalid["owner_id"] = serde_json::json!("not-authorized");
        assert!(serde_json::from_value::<Item>(invalid).is_err());
        let mut invalid = encoded;
        invalid["item_profile"] = serde_json::json!("inventory-stat-pack");
        assert!(serde_json::from_value::<Item>(invalid).is_err());

        assert_eq!(ItemProfile::ALL.len(), 11);
        assert_eq!(
            serde_json::to_string(&ItemProfile::KeyItem).unwrap(),
            "\"key-item\""
        );
        assert_eq!(
            serde_json::from_str::<ItemProfile>("\"quest-item\"").unwrap(),
            ItemProfile::QuestItem
        );
        let mut invalid = item.clone();
        invalid.appearance.clear();
        assert!(invalid.validate().is_err());
        assert!(serde_json::from_str::<Item>("{\"unknown\": true}").is_err());
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
