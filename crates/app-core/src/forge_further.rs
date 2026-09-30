//! Bounded, deterministic Forge Further proof slice: saved Character → Hometown.

use crate::{LocationCandidate, LocationForge};
use forge_contracts::{
    CanonState, Character, CreativeAxes, EntityId, EntityOrigin, EntityType, FieldKey, ForgeDNA,
    LocationProfile, PresetId, ProjectId, Relationship, RelationshipId, RelationshipState,
    RelationshipType, RootSeed, ValidationCode, ValidationError, ValidationErrors,
    ValidationResult,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const FORGE_HOMETOWN_ACTION: ForgeActionDefinition = ForgeActionDefinition {
    action_id: "character.forge-hometown",
    action_version: 1,
    source_type: EntityType::Character,
    target_type: EntityType::Location,
    target_profile: LocationProfile::Settlement,
    intent: "hometown",
    relationship_type: RelationshipType::BornIn,
    context_recipe_id: "character-hometown-v1",
};

pub const FORGE_HOMETOWN_RECIPE: (&str, u16) = ("character-hometown-v1", 1);
const CONTEXT_FORMAT_VERSION: u16 = 1;
const RECIPE_RECORD_FORMAT_VERSION: u16 = 1;
const RESULT_RECORD_FORMAT_VERSION: u16 = 1;
const PROFILE_CONSTRAINT: &str = "location.profile";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeActionDefinition {
    pub action_id: &'static str,
    pub action_version: u16,
    pub source_type: EntityType,
    pub target_type: EntityType,
    pub target_profile: LocationProfile,
    pub intent: &'static str,
    pub relationship_type: RelationshipType,
    pub context_recipe_id: &'static str,
}

/// The exact persisted source representation needed by the read-only compiler.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HometownSource {
    pub character: Character,
    pub field_origins: BTreeMap<FieldKey, crate::FieldValueOrigin>,
    pub write_version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HometownContextFacts {
    pub name: String,
    pub title: String,
    pub archetype: String,
    pub role: String,
    pub faction_hint: String,
    pub backstory_hook: String,
    pub desire: String,
    pub secret: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HometownContextSnapshot {
    pub format_version: u16,
    pub context_recipe_id: String,
    pub context_recipe_version: u16,
    pub source_entity_id: EntityId,
    pub source_write_version: i64,
    pub source_canon_state: CanonState,
    pub source_type: EntityType,
    pub source_generation_kind: String,
    pub source_generator_version: u16,
    pub source_content_version: u16,
    pub facts: HometownContextFacts,
    pub inherited_preset: PresetId,
    pub inherited_creative_axes: CreativeAxes,
    pub source_body_hash: String,
    pub context_hash: String,
}

#[derive(Serialize)]
struct SourceBody<'a> {
    character: &'a Character,
    field_origins: &'a BTreeMap<FieldKey, crate::FieldValueOrigin>,
    write_version: i64,
}

#[derive(Serialize)]
struct ContextMaterial<'a> {
    format_version: u16,
    context_recipe_id: &'a str,
    context_recipe_version: u16,
    source_entity_id: EntityId,
    source_write_version: i64,
    source_canon_state: CanonState,
    source_type: EntityType,
    source_generation_kind: &'a str,
    source_generator_version: u16,
    source_content_version: u16,
    facts: &'a HometownContextFacts,
    inherited_preset: &'a PresetId,
    inherited_creative_axes: &'a CreativeAxes,
    source_body_hash: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForgeFurtherRecipeRecord {
    pub format_version: u16,
    pub action_id: String,
    pub action_version: u16,
    pub context_recipe_id: String,
    pub context_recipe_version: u16,
    pub context_snapshot: HometownContextSnapshot,
    pub action_seed: u64,
    pub location_dna: ForgeDNA,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForgeResultMapping {
    pub format_version: u16,
    pub primary_output_id: EntityId,
    pub primary_output_type: EntityType,
    pub relationship_ids: Vec<RelationshipId>,
    pub relationship_type: RelationshipType,
    pub source_entity_id: EntityId,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ForgeResultBundle {
    pub primary: LocationCandidate,
    pub supporting: Vec<()>,
    pub relationship: Relationship,
    pub recipe: ForgeFurtherRecipeRecord,
    pub result: ForgeResultMapping,
}

/// Finalize a reviewed Hometown candidate without changing its frozen source context.
/// The recipe DNA follows the exact candidate that will be persisted.
pub fn finalize_hometown_review(
    original: &ForgeResultBundle,
    reviewed_primary: LocationCandidate,
) -> ValidationResult<ForgeResultBundle> {
    reviewed_primary.validate()?;
    let id = reviewed_primary.location.envelope.id;
    if id != original.primary.location.envelope.id
        || id != original.result.primary_output_id
        || id != original.relationship.target_id
        || original.result.primary_output_type != EntityType::Location
        || original.relationship.relationship_type != RelationshipType::BornIn
        || original.result.relationship_type != RelationshipType::BornIn
        || original.result.relationship_ids != [original.relationship.id]
        || original.result.source_entity_id != original.recipe.context_snapshot.source_entity_id
        || original.relationship.source_id != original.recipe.context_snapshot.source_entity_id
    {
        return Err(validation(
            "bundle.reviewed_primary",
            "reviewed Location identity or relationship mapping does not match the frozen Hometown bundle",
        ));
    }
    let mut finalized = original.clone();
    finalized.primary = reviewed_primary;
    finalized.recipe.location_dna = finalized.primary.dna.clone();
    Ok(finalized)
}

pub fn compile_hometown_context(
    source: &HometownSource,
) -> ValidationResult<HometownContextSnapshot> {
    source.character.validate()?;
    if source.write_version <= 0 {
        return Err(validation(
            "source.write_version",
            "saved source version must be positive",
        ));
    }
    let envelope = &source.character.envelope;
    if envelope.entity_type != EntityType::Character {
        return Err(validation(
            "source.entity_type",
            "Forge Hometown requires a Character",
        ));
    }
    if !matches!(
        envelope.canon_state,
        CanonState::Candidate | CanonState::Canon
    ) || envelope.origin != EntityOrigin::Generated
    {
        return Err(validation(
            "source.state",
            "Forge Hometown requires a saved generated Candidate or Canon Character.",
        ));
    }
    let provenance = envelope.generation_provenance.as_ref().ok_or_else(|| {
        validation(
            "source.provenance",
            "This Character has no supported generation provenance.",
        )
    })?;
    let dna = &provenance.dna;
    if dna.recipe.kind != forge_contracts::GenerationKind::Character
        || dna.validate().is_err()
        || dna.recipe.universe_preset.as_str().is_empty()
    {
        return Err(validation(
            "source.provenance",
            "This Character's generation provenance is unsupported.",
        ));
    }
    let body = serde_json::to_vec(&SourceBody {
        character: &source.character,
        field_origins: &source.field_origins,
        write_version: source.write_version,
    })
    .map_err(|error| {
        validation(
            "source.body",
            &format!("could not serialize source: {error}"),
        )
    })?;
    let source_body_hash = digest("character-forge/source-character/v1", &body);
    let facts = HometownContextFacts {
        name: envelope.name.clone(),
        title: source.character.title.clone(),
        archetype: source.character.archetype.clone(),
        role: source.character.role.clone(),
        faction_hint: source.character.faction_hint.clone(),
        backstory_hook: source.character.backstory_hook.clone(),
        desire: source.character.desire.clone(),
        secret: source.character.secret.clone(),
    };
    let mut snapshot = HometownContextSnapshot {
        format_version: CONTEXT_FORMAT_VERSION,
        context_recipe_id: FORGE_HOMETOWN_RECIPE.0.into(),
        context_recipe_version: FORGE_HOMETOWN_RECIPE.1,
        source_entity_id: envelope.id,
        source_write_version: source.write_version,
        source_canon_state: envelope.canon_state,
        source_type: EntityType::Character,
        source_generation_kind: "character".into(),
        source_generator_version: dna.generator_version.0,
        source_content_version: dna.content_version.0,
        facts,
        inherited_preset: dna.recipe.universe_preset.clone(),
        inherited_creative_axes: dna.recipe.creative_axes.clone(),
        source_body_hash,
        context_hash: String::new(),
    };
    snapshot.context_hash = hash_context(&snapshot)?;
    Ok(snapshot)
}

pub fn forge_hometown(
    source: &HometownSource,
    action_seed: u64,
    project_id: ProjectId,
) -> ValidationResult<ForgeResultBundle> {
    let snapshot = compile_hometown_context(source)?;
    if source.character.envelope.project_id != project_id {
        return Err(validation(
            "source.project_id",
            "source Character belongs to another project",
        ));
    }
    let seed = action_root_seed(&snapshot.context_hash, action_seed);
    let mut dna = LocationForge::new_dna(
        RootSeed::new(seed),
        snapshot.inherited_preset.as_str(),
        snapshot.inherited_creative_axes.clone(),
    )?;
    let constraint = forge_contracts::ConstraintKey::new(PROFILE_CONSTRAINT)?;
    dna.recipe
        .constraints
        .insert(constraint, "settlement".into());
    dna.normalize()?;
    let primary = LocationForge::generate_in_project(&dna, project_id)?;
    let relationship_hash = derive_bytes(
        "character-forge/forge-hometown/relationship-id/v1",
        &[
            snapshot.context_hash.as_bytes(),
            &action_seed.to_le_bytes(),
            primary.location.envelope.id.uuid().as_bytes(),
        ],
    );
    let mut relationship_id_bytes = [0_u8; 16];
    relationship_id_bytes.copy_from_slice(&relationship_hash[..16]);
    let relationship_id = RelationshipId::from_bytes(relationship_id_bytes);
    let relationship = Relationship {
        id: relationship_id,
        project_id,
        relationship_type: RelationshipType::BornIn,
        source_id: source.character.envelope.id,
        target_id: primary.location.envelope.id,
        state: RelationshipState::Candidate,
        origin: EntityOrigin::Generated,
        created_at_unix_ms: 0,
        updated_at_unix_ms: 0,
    };
    let recipe = ForgeFurtherRecipeRecord {
        format_version: RECIPE_RECORD_FORMAT_VERSION,
        action_id: FORGE_HOMETOWN_ACTION.action_id.into(),
        action_version: FORGE_HOMETOWN_ACTION.action_version,
        context_recipe_id: FORGE_HOMETOWN_RECIPE.0.into(),
        context_recipe_version: FORGE_HOMETOWN_RECIPE.1,
        context_snapshot: snapshot.clone(),
        action_seed,
        location_dna: dna,
    };
    let result = ForgeResultMapping {
        format_version: RESULT_RECORD_FORMAT_VERSION,
        primary_output_id: primary.location.envelope.id,
        primary_output_type: EntityType::Location,
        relationship_ids: vec![relationship.id],
        relationship_type: RelationshipType::BornIn,
        source_entity_id: source.character.envelope.id,
        warnings: Vec::new(),
    };
    Ok(ForgeResultBundle {
        primary,
        supporting: Vec::new(),
        relationship,
        recipe,
        result,
    })
}

fn hash_context(snapshot: &HometownContextSnapshot) -> ValidationResult<String> {
    let material = ContextMaterial {
        format_version: snapshot.format_version,
        context_recipe_id: &snapshot.context_recipe_id,
        context_recipe_version: snapshot.context_recipe_version,
        source_entity_id: snapshot.source_entity_id,
        source_write_version: snapshot.source_write_version,
        source_canon_state: snapshot.source_canon_state,
        source_type: snapshot.source_type,
        source_generation_kind: &snapshot.source_generation_kind,
        source_generator_version: snapshot.source_generator_version,
        source_content_version: snapshot.source_content_version,
        facts: &snapshot.facts,
        inherited_preset: &snapshot.inherited_preset,
        inherited_creative_axes: &snapshot.inherited_creative_axes,
        source_body_hash: &snapshot.source_body_hash,
    };
    let bytes = serde_json::to_vec(&material)
        .map_err(|error| validation("context", &format!("could not serialize context: {error}")))?;
    Ok(digest("character-forge/context-snapshot/v1", &bytes))
}

fn action_root_seed(context_hash: &str, action_seed: u64) -> u64 {
    let bytes = derive_bytes(
        "character-forge/forge-action-seed/v1",
        &[
            FORGE_HOMETOWN_ACTION.action_id.as_bytes(),
            &FORGE_HOMETOWN_ACTION.action_version.to_le_bytes(),
            context_hash.as_bytes(),
            &action_seed.to_le_bytes(),
        ],
    );
    u64::from_le_bytes(bytes[..8].try_into().expect("eight-byte seed"))
}

fn derive_bytes(domain: &str, parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(&[0]);
    for part in parts {
        hasher.update(&(part.len() as u64).to_le_bytes());
        hasher.update(part);
    }
    *hasher.finalize().as_bytes()
}

fn digest(domain: &str, bytes: &[u8]) -> String {
    let hash = derive_bytes(domain, &[bytes]);
    hash.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn validation(path: &str, message: &str) -> forge_contracts::ValidationErrors {
    ValidationErrors::one(ValidationError::new(
        ValidationCode::InvalidEntity,
        path,
        message,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CharacterForge;
    use forge_contracts::{CreativeAxes, RootSeed};

    fn source() -> HometownSource {
        let dna = CharacterForge::new_dna(
            RootSeed::new(57),
            "nordic-myth",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        let candidate = CharacterForge::generate(&dna).unwrap();
        HometownSource {
            character: candidate.character,
            field_origins: candidate.field_origins,
            write_version: 4,
        }
    }

    #[test]
    fn action_and_recipe_are_stable_and_versioned() {
        assert_eq!(FORGE_HOMETOWN_ACTION.action_id, "character.forge-hometown");
        assert_eq!(FORGE_HOMETOWN_ACTION.action_version, 1);
        assert_eq!(FORGE_HOMETOWN_ACTION.source_type, EntityType::Character);
        assert_eq!(FORGE_HOMETOWN_ACTION.target_type, EntityType::Location);
        assert_eq!(
            FORGE_HOMETOWN_ACTION.target_profile,
            LocationProfile::Settlement
        );
        assert_eq!(
            FORGE_HOMETOWN_ACTION.relationship_type,
            RelationshipType::BornIn
        );
        assert_eq!(FORGE_HOMETOWN_RECIPE, ("character-hometown-v1", 1));
    }

    #[test]
    fn context_is_deterministic_bounded_and_sensitive_to_source_changes() {
        let source = source();
        let first = compile_hometown_context(&source).unwrap();
        let again = compile_hometown_context(&source).unwrap();
        assert_eq!(
            serde_json::to_vec(&first).unwrap(),
            serde_json::to_vec(&again).unwrap()
        );
        assert_eq!(first.context_hash, again.context_hash);
        let mut changed = source;
        changed.character.secret.push_str(" revised");
        assert_ne!(
            first.context_hash,
            compile_hometown_context(&changed).unwrap().context_hash
        );
        assert_eq!(first.facts.name, first.facts.name.trim());
    }

    #[test]
    fn manual_or_non_candidate_source_is_rejected_without_fabricated_inputs() {
        let mut manual = source();
        manual.character.envelope.origin = EntityOrigin::Manual;
        assert!(compile_hometown_context(&manual).is_err());

        let mut archived = source();
        archived.character.envelope.canon_state = CanonState::Archived;
        assert!(compile_hometown_context(&archived).is_err());
    }

    #[test]
    fn hometown_inherits_inputs_and_proposes_candidate_relationship_without_source_mutation() {
        let source = source();
        let original = source.clone();
        let project_id = source.character.envelope.project_id;
        let first = forge_hometown(&source, 123, project_id).unwrap();
        let again = forge_hometown(&source, 123, project_id).unwrap();
        assert_eq!(first.primary, again.primary);
        assert_ne!(
            first.primary,
            forge_hometown(&source, 124, project_id).unwrap().primary
        );
        assert_eq!(
            first.primary.location.location_profile,
            LocationProfile::Settlement
        );
        let source_dna = &source
            .character
            .envelope
            .generation_provenance
            .as_ref()
            .unwrap()
            .dna;
        assert_eq!(
            first.primary.dna.recipe.universe_preset,
            source_dna.recipe.universe_preset
        );
        assert_eq!(
            first.primary.dna.recipe.creative_axes,
            source_dna.recipe.creative_axes
        );
        assert_eq!(first.relationship.state, RelationshipState::Candidate);
        assert_eq!(first.relationship.origin, EntityOrigin::Generated);
        assert_eq!(
            first.relationship.relationship_type,
            RelationshipType::BornIn
        );
        assert_eq!(source, original);
    }

    #[test]
    fn reviewed_finalizer_preserves_frozen_provenance_and_rejects_identity_replacement() {
        let source = source();
        let project_id = source.character.envelope.project_id;
        let bundle = forge_hometown(&source, 123, project_id).unwrap();
        let mut reviewed = bundle.primary.clone();
        LocationForge::lock(&mut reviewed, crate::LocationField::Name).unwrap();
        let finalized = finalize_hometown_review(&bundle, reviewed.clone()).unwrap();
        assert_eq!(finalized.primary, reviewed);
        assert_eq!(finalized.recipe.location_dna, reviewed.dna);
        assert_eq!(
            finalized.recipe.context_snapshot,
            bundle.recipe.context_snapshot
        );
        assert_eq!(finalized.recipe.action_id, bundle.recipe.action_id);
        assert_eq!(
            finalized.recipe.action_version,
            bundle.recipe.action_version
        );
        assert_eq!(finalized.recipe.action_seed, bundle.recipe.action_seed);

        let replacement = forge_hometown(&source, 124, project_id).unwrap().primary;
        assert!(finalize_hometown_review(&bundle, replacement).is_err());
    }
}
