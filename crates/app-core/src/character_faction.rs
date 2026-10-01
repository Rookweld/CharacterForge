//! One saved Character → one proposed Faction and member_of assertion.
//! Hometown's persisted v1 records and hashing remain unchanged.
use crate::forge_further::{derive_bytes, digest};
use crate::{
    CharacterFactionFacts, CharacterForge, FactionCandidate, FactionForge, ForgeActionDefinition,
    ForgeResultMapping, HometownSource, compile_hometown_context,
};
use forge_contracts::{
    CanonState, CreativeAxes, EntityId, EntityOrigin, EntityType, FactionProfile, ForgeDNA,
    PresetId, ProjectId, Relationship, RelationshipId, RelationshipState, RelationshipType,
    RootSeed, ValidationCode, ValidationError, ValidationErrors, ValidationResult,
};
use serde::{Deserialize, Serialize};

pub const FORGE_FACTION_ACTION: ForgeActionDefinition<Option<FactionProfile>> =
    ForgeActionDefinition {
        action_id: "character.forge-faction",
        action_version: 1,
        source_type: EntityType::Character,
        target_type: EntityType::Faction,
        target_profile: None,
        intent: "Character Faction",
        relationship_type: RelationshipType::MemberOf,
        context_recipe_id: "character-faction-v1",
    };
pub const FORGE_FACTION_RECIPE: (&str, u16) = ("character-faction-v1", 1);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FactionContextSnapshot {
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
    pub facts: CharacterFactionFacts,
    pub inherited_preset: PresetId,
    pub inherited_creative_axes: CreativeAxes,
    pub source_body_hash: String,
    pub context_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FactionFurtherRecipeRecord {
    pub format_version: u16,
    pub action_id: String,
    pub action_version: u16,
    pub context_recipe_id: String,
    pub context_recipe_version: u16,
    pub context_snapshot: FactionContextSnapshot,
    pub action_seed: u64,
    pub target_dna: ForgeDNA,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FactionResultBundle {
    pub primary: FactionCandidate,
    pub supporting: Vec<()>,
    pub relationship: Relationship,
    pub recipe: FactionFurtherRecipeRecord,
    pub result: ForgeResultMapping,
}

pub fn compile_faction_context(
    source: &HometownSource,
) -> ValidationResult<FactionContextSnapshot> {
    // Share exact persisted Character hashing and eligibility without changing B2 bytes.
    let base = compile_hometown_context(source)?;
    let dna = &source
        .character
        .envelope
        .generation_provenance
        .as_ref()
        .ok_or_else(|| {
            invalid(
                "source.provenance",
                "Character generation provenance is required",
            )
        })?
        .dna;
    CharacterForge::generate(dna)?; // Supported replay tuple/constraints, no source mutation.
    let c = &source.character;
    let facts = CharacterFactionFacts {
        name: c.envelope.name.clone(),
        title: c.title.clone(),
        archetype: c.archetype.clone(),
        role: c.role.clone(),
        personality: c.personality.clone(),
        desire: c.desire.clone(),
        fear: c.fear.clone(),
        flaw: c.flaw.clone(),
        faction_hint: c.faction_hint.clone(),
        relationship_hook: c.relationship_hook.clone(),
        backstory_hook: c.backstory_hook.clone(),
        secret: c.secret.clone(),
    };
    facts.validate()?;
    let mut snapshot = FactionContextSnapshot {
        format_version: 1,
        context_recipe_id: FORGE_FACTION_RECIPE.0.into(),
        context_recipe_version: 1,
        source_entity_id: base.source_entity_id,
        source_write_version: base.source_write_version,
        source_canon_state: base.source_canon_state,
        source_type: base.source_type,
        source_generation_kind: base.source_generation_kind,
        source_generator_version: base.source_generator_version,
        source_content_version: base.source_content_version,
        facts,
        inherited_preset: base.inherited_preset,
        inherited_creative_axes: base.inherited_creative_axes,
        source_body_hash: base.source_body_hash,
        context_hash: String::new(),
    };
    snapshot.context_hash = context_hash(&snapshot)?;
    Ok(snapshot)
}

fn context_hash(snapshot: &FactionContextSnapshot) -> ValidationResult<String> {
    let mut material = snapshot.clone();
    material.context_hash.clear();
    let bytes = serde_json::to_vec(&material)
        .map_err(|_| invalid("context", "could not serialize faction context"))?;
    Ok(digest(
        "character-forge/character-faction/context/v1",
        &bytes,
    ))
}

fn target_dna(snapshot: &FactionContextSnapshot, seed: u64) -> ValidationResult<ForgeDNA> {
    let bytes = derive_bytes(
        "character-forge/forge-action-seed/v1",
        &[
            FORGE_FACTION_ACTION.action_id.as_bytes(),
            &FORGE_FACTION_ACTION.action_version.to_le_bytes(),
            snapshot.context_hash.as_bytes(),
            &seed.to_le_bytes(),
        ],
    );
    let root = u64::from_le_bytes(bytes[..8].try_into().expect("eight-byte seed"));
    let mut dna = FactionForge::new_dna(
        RootSeed::new(root),
        snapshot.inherited_preset.as_str(),
        snapshot.inherited_creative_axes.clone(),
    )?;
    dna.recipe.constraints.insert(
        forge_contracts::ConstraintKey::new(crate::faction_context::CONTEXT_KEY)?,
        serde_json::to_string(&snapshot.facts)
            .map_err(|_| invalid("context", "could not serialize faction facts"))?,
    );
    dna.normalize()?;
    Ok(dna)
}

pub fn forge_character_faction(
    source: &HometownSource,
    action_seed: u64,
    project_id: ProjectId,
) -> ValidationResult<FactionResultBundle> {
    if source.character.envelope.project_id != project_id {
        return Err(invalid(
            "source.project_id",
            "source Character belongs to another project",
        ));
    }
    let snapshot = compile_faction_context(source)?;
    let dna = target_dna(&snapshot, action_seed)?;
    let primary = FactionForge::generate_in_project(&dna, project_id)?;
    let bytes = derive_bytes(
        "character-forge/character-faction/relationship-id/v1",
        &[
            snapshot.context_hash.as_bytes(),
            &action_seed.to_le_bytes(),
            primary.faction.envelope.id.uuid().as_bytes(),
        ],
    );
    let id = RelationshipId::from_bytes(bytes[..16].try_into().expect("relationship id"));
    let relationship = Relationship {
        id,
        project_id,
        relationship_type: RelationshipType::MemberOf,
        source_id: snapshot.source_entity_id,
        target_id: primary.faction.envelope.id,
        state: RelationshipState::Candidate,
        origin: EntityOrigin::Generated,
        created_at_unix_ms: 0,
        updated_at_unix_ms: 0,
    };
    let result = ForgeResultMapping {
        format_version: 1,
        primary_output_id: relationship.target_id,
        primary_output_type: EntityType::Faction,
        relationship_ids: vec![id],
        relationship_type: RelationshipType::MemberOf,
        source_entity_id: relationship.source_id,
        warnings: Vec::new(),
    };
    let recipe = FactionFurtherRecipeRecord {
        format_version: 1,
        action_id: FORGE_FACTION_ACTION.action_id.into(),
        action_version: 1,
        context_recipe_id: FORGE_FACTION_RECIPE.0.into(),
        context_recipe_version: 1,
        context_snapshot: snapshot,
        action_seed,
        target_dna: dna,
    };
    let bundle = FactionResultBundle {
        primary,
        supporting: Vec::new(),
        relationship,
        recipe,
        result,
    };
    validate_faction_bundle(&bundle)?;
    Ok(bundle)
}

pub fn validate_faction_bundle(bundle: &FactionResultBundle) -> ValidationResult<()> {
    bundle.primary.validate()?;
    bundle.relationship.validate()?;
    let r = &bundle.recipe;
    let c = &r.context_snapshot;
    let e = &bundle.primary.faction.envelope;
    let initial = target_dna(c, r.action_seed)?;
    let original_id = FactionForge::generate_in_project(&initial, e.project_id)?
        .faction
        .envelope
        .id;
    if r.format_version != 1
        || r.action_id != FORGE_FACTION_ACTION.action_id
        || r.action_version != 1
        || r.context_recipe_id != FORGE_FACTION_RECIPE.0
        || r.context_recipe_version != 1
        || c.format_version != 1
        || c.context_recipe_id != r.context_recipe_id
        || c.context_recipe_version != 1
        || c.context_hash != context_hash(c)?
        || c.source_type != EntityType::Character
        || c.source_generation_kind != "character"
        || c.source_write_version <= 0
        || !matches!(
            c.source_canon_state,
            CanonState::Candidate | CanonState::Canon
        )
        || e.id != original_id
        || e.entity_type != EntityType::Faction
        || e.canon_state != CanonState::Candidate
        || e.origin != EntityOrigin::Generated
        || !bundle.supporting.is_empty()
        || bundle.relationship.project_id != e.project_id
        || bundle.relationship.state != RelationshipState::Candidate
        || bundle.relationship.origin != EntityOrigin::Generated
        || bundle.relationship.relationship_type != RelationshipType::MemberOf
        || bundle.relationship.source_id != c.source_entity_id
        || bundle.relationship.target_id != e.id
        || bundle.result.format_version != 1
        || bundle.result.primary_output_id != e.id
        || bundle.result.primary_output_type != EntityType::Faction
        || bundle.result.source_entity_id != c.source_entity_id
        || bundle.result.relationship_type != RelationshipType::MemberOf
        || bundle.result.relationship_ids != [bundle.relationship.id]
        || r.target_dna != bundle.primary.dna
        || r.target_dna.recipe.root_seed != initial.recipe.root_seed
        || r.target_dna.recipe.universe_preset != initial.recipe.universe_preset
        || r.target_dna.recipe.creative_axes != initial.recipe.creative_axes
        || r.target_dna.recipe.constraints != initial.recipe.constraints
    {
        return Err(invalid(
            "bundle",
            "Faction bundle does not match its frozen action, context and output provenance",
        ));
    }
    let bytes = derive_bytes(
        "character-forge/character-faction/relationship-id/v1",
        &[
            c.context_hash.as_bytes(),
            &r.action_seed.to_le_bytes(),
            e.id.uuid().as_bytes(),
        ],
    );
    if bundle.relationship.id
        != RelationshipId::from_bytes(bytes[..16].try_into().expect("relationship id"))
    {
        return Err(invalid(
            "bundle.relationship.id",
            "proposed Relationship identity changed",
        ));
    }
    crate::validate_relationship_endpoints(
        &bundle.relationship,
        EntityType::Character,
        EntityType::Faction,
    )
}

pub fn finalize_faction_review(
    original: &FactionResultBundle,
    reviewed: FactionCandidate,
) -> ValidationResult<FactionResultBundle> {
    validate_faction_bundle(original)?;
    reviewed.validate()?;
    if reviewed.faction.envelope.id != original.primary.faction.envelope.id {
        return Err(invalid(
            "bundle.reviewed_primary",
            "reviewed Faction identity changed",
        ));
    }
    let mut finalized = original.clone();
    finalized.primary = reviewed;
    finalized.recipe.target_dna = finalized.primary.dna.clone();
    validate_faction_bundle(&finalized)?;
    Ok(finalized)
}

fn invalid(path: &str, message: &str) -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(
        ValidationCode::InvalidEntity,
        path,
        message,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FactionField, FactionFieldValue};
    use std::collections::BTreeMap;

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
    fn faction_action_context_and_replay_are_stable_read_only_and_inherited() {
        assert_eq!(FORGE_FACTION_ACTION.action_id, "character.forge-faction");
        assert_eq!(FORGE_FACTION_ACTION.action_version, 1);
        assert_eq!(
            FORGE_FACTION_ACTION.relationship_type,
            RelationshipType::MemberOf
        );
        assert_eq!(FORGE_FACTION_RECIPE, ("character-faction-v1", 1));
        let source = source();
        let before = source.clone();
        let project = source.character.envelope.project_id;
        let context = compile_faction_context(&source).unwrap();
        assert_eq!(
            serde_json::to_vec(&context).unwrap(),
            serde_json::to_vec(&compile_faction_context(&source).unwrap()).unwrap()
        );
        let first = forge_character_faction(&source, 123, project).unwrap();
        assert_eq!(
            first,
            forge_character_faction(&source, 123, project).unwrap()
        );
        assert_ne!(
            first.primary,
            forge_character_faction(&source, 124, project)
                .unwrap()
                .primary
        );
        assert_eq!(
            first.primary,
            FactionForge::generate_in_project(&first.recipe.target_dna, project).unwrap()
        );
        let dna = &source
            .character
            .envelope
            .generation_provenance
            .as_ref()
            .unwrap()
            .dna;
        assert_eq!(
            first.primary.dna.recipe.universe_preset,
            dna.recipe.universe_preset
        );
        assert_eq!(
            first.primary.dna.recipe.creative_axes,
            dna.recipe.creative_axes
        );
        assert_eq!(first.relationship.state, RelationshipState::Candidate);
        assert_eq!(first.relationship.origin, EntityOrigin::Generated);
        assert_eq!(source, before);
    }

    #[test]
    fn faction_context_captures_all_authored_facts_and_rejects_unsupported_sources() {
        let source = source();
        let original = compile_faction_context(&source).unwrap();
        let mut changed = source.clone();
        changed.character.faction_hint = "a trade guild".into();
        let context = compile_faction_context(&changed).unwrap();
        assert_ne!(context.context_hash, original.context_hash);
        assert_eq!(context.facts.personality, changed.character.personality);
        let faction =
            forge_character_faction(&changed, 9, changed.character.envelope.project_id).unwrap();
        assert_eq!(
            faction.primary.faction.faction_profile,
            FactionProfile::Guild
        );
        assert!(faction.primary.faction.identity.contains("a trade guild"));
        assert_ne!(
            faction.primary.faction.envelope.name,
            changed.character.faction_hint
        );
        for origin in [EntityOrigin::Manual, EntityOrigin::Imported] {
            changed = source.clone();
            changed.character.envelope.origin = origin;
            assert!(compile_faction_context(&changed).is_err());
        }
        changed = source.clone();
        changed.character.envelope.canon_state = CanonState::Archived;
        assert!(compile_faction_context(&changed).is_err());
        changed = source.clone();
        changed.character.secret = "x".repeat(40_000);
        assert!(compile_faction_context(&changed).is_err());
        changed = source.clone();
        changed
            .character
            .envelope
            .generation_provenance
            .as_mut()
            .unwrap()
            .dna
            .generator_version = forge_contracts::GeneratorVersion(99);
        assert!(compile_faction_context(&changed).is_err());
    }

    #[test]
    fn faction_review_finalizer_preserves_frozen_context_identity_and_exact_reviewed_dna() {
        let source = source();
        let bundle =
            forge_character_faction(&source, 123, source.character.envelope.project_id).unwrap();
        let mut working = bundle.primary.clone();
        FactionForge::lock(&mut working, FactionField::Name).unwrap();
        FactionForge::reroll_all_unlocked(&mut working).unwrap();
        FactionForge::reroll_field(&mut working, FactionField::HiddenTruth).unwrap();
        FactionForge::manual_edit(
            &mut working,
            FactionField::PrimaryGoal,
            FactionFieldValue::Text("Reviewed shared goal".into()),
        )
        .unwrap();
        let finalized = finalize_faction_review(&bundle, working.clone()).unwrap();
        assert_eq!(finalized.primary, working);
        assert_eq!(finalized.recipe.target_dna, working.dna);
        let mut expected = bundle.clone();
        expected.primary = working;
        expected.recipe.target_dna = expected.primary.dna.clone();
        assert_eq!(finalized, expected);
        assert_eq!(finalized.relationship, bundle.relationship);
    }

    #[test]
    fn faction_finalization_rejects_replaced_identity_mapping_action_or_provenance() {
        let source = source();
        let project = source.character.envelope.project_id;
        let bundle = forge_character_faction(&source, 123, project).unwrap();
        assert!(
            finalize_faction_review(
                &bundle,
                forge_character_faction(&source, 124, project)
                    .unwrap()
                    .primary
            )
            .is_err()
        );
        for case in 0..8 {
            let mut bad = bundle.clone();
            match case {
                0 => bad.relationship.target_id = source.character.envelope.id,
                1 => bad.relationship.relationship_type = RelationshipType::BornIn,
                2 => bad.recipe.context_snapshot.context_hash.push('x'),
                3 => bad.recipe.action_id = "other".into(),
                4 => bad.recipe.action_seed += 1,
                5 => bad.result.primary_output_type = EntityType::Location,
                6 => bad.primary.faction.envelope.origin = EntityOrigin::Manual,
                _ => {
                    bad.primary
                        .faction
                        .envelope
                        .generation_provenance
                        .as_mut()
                        .unwrap()
                        .dna
                        .recipe
                        .kind = forge_contracts::GenerationKind::Location
                }
            }
            assert!(
                finalize_faction_review(&bad, bad.primary.clone()).is_err(),
                "case {case}"
            );
        }
        assert_eq!(
            bundle,
            forge_character_faction(&source, 123, project).unwrap()
        );
    }
}
