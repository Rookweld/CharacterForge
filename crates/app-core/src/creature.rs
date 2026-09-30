//! Standalone deterministic Creature Forge, with an isolated schema and field-stream contract.

use crate::{FieldStream, FieldValues, creature_content};
use forge_contracts::{
    CanonState, ContentVersion, CreativeAxes, CreativeAxis, Creature, CreatureProfile,
    EntityEnvelope, EntityId, EntityOrigin, EntityType, FieldKey, ForgeDNA, GenerationAction,
    GenerationKind, GenerationMetadata, GenerationRecipe, GeneratorVersion, INITIAL_SCHEMA_VERSION,
    PresetId, ProjectId, RootSeed, ValidationCode, ValidationError, ValidationErrors,
    ValidationResult,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const CREATURE_GENERATOR_VERSION: GeneratorVersion = GeneratorVersion(1);
pub const CREATURE_CONTENT_VERSION: ContentVersion = ContentVersion(1);
pub const CREATURE_FIELD_ORDER: [CreatureField; 14] = [
    CreatureField::Profile,
    CreatureField::Name,
    CreatureField::Appearance,
    CreatureField::Habitat,
    CreatureField::Behavior,
    CreatureField::Ecology,
    CreatureField::Threat,
    CreatureField::AbilitiesSummary,
    CreatureField::Weakness,
    CreatureField::Origin,
    CreatureField::SignsOfPresence,
    CreatureField::LootOrResource,
    CreatureField::Secret,
    CreatureField::StoryHooks,
];

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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatureField {
    Profile,
    Name,
    Appearance,
    Habitat,
    Behavior,
    Ecology,
    Threat,
    AbilitiesSummary,
    Weakness,
    Origin,
    SignsOfPresence,
    LootOrResource,
    Secret,
    StoryHooks,
}

impl CreatureField {
    pub const ALL: [Self; 14] = CREATURE_FIELD_ORDER;

    pub fn key(self) -> ValidationResult<FieldKey> {
        FieldKey::new(match self {
            Self::Profile => "01-creature-profile",
            Self::Name => "02-name",
            Self::Appearance => "03-appearance",
            Self::Habitat => "04-habitat",
            Self::Behavior => "05-behavior",
            Self::Ecology => "06-ecology",
            Self::Threat => "07-threat",
            Self::AbilitiesSummary => "08-abilities-summary",
            Self::Weakness => "09-weakness",
            Self::Origin => "10-origin",
            Self::SignsOfPresence => "11-signs-of-presence",
            Self::LootOrResource => "12-loot-or-resource",
            Self::Secret => "13-secret",
            Self::StoryHooks => "14-story-hooks",
        })
    }

    pub const fn action_key(self) -> &'static str {
        match self {
            Self::Profile => "creature_profile",
            Self::Name => "name",
            Self::Appearance => "appearance",
            Self::Habitat => "habitat",
            Self::Behavior => "behavior",
            Self::Ecology => "ecology",
            Self::Threat => "threat",
            Self::AbilitiesSummary => "abilities_summary",
            Self::Weakness => "weakness",
            Self::Origin => "origin",
            Self::SignsOfPresence => "signs_of_presence",
            Self::LootOrResource => "loot_or_resource",
            Self::Secret => "secret",
            Self::StoryHooks => "story_hooks",
        }
    }

    fn from_key(key: &FieldKey) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|field| field.key().is_ok_and(|v| v == *key))
    }

    pub fn from_action_key(key: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|field| field.action_key() == key)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum CreatureFieldValue {
    Profile(CreatureProfile),
    Text(String),
    Texts(Vec<String>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatureOrigin {
    Generated,
    UserModified,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreatureCandidate {
    pub creature: Creature,
    pub dna: ForgeDNA,
    pub field_origins: BTreeMap<FieldKey, CreatureOrigin>,
}

impl CreatureCandidate {
    pub fn validate(&self) -> ValidationResult<()> {
        self.creature.validate()?;
        validate_creature_dna(&self.dna)?;
        if self
            .creature
            .envelope
            .generation_provenance
            .as_ref()
            .map(|p| &p.dna)
            != Some(&self.dna)
        {
            return Err(error(
                ValidationCode::InvalidEntity,
                "creature_candidate.provenance.dna",
                "candidate DNA and Creature provenance DNA must match",
            ));
        }
        let expected = CREATURE_FIELD_ORDER
            .into_iter()
            .map(CreatureField::key)
            .collect::<ValidationResult<BTreeSet<_>>>()?;
        if self.field_origins.keys().cloned().collect::<BTreeSet<_>>() != expected {
            return Err(error(
                ValidationCode::MissingField,
                "creature_candidate.field_origins",
                "candidate must record the origin of every generated Creature field",
            ));
        }
        Ok(())
    }

    pub fn manual_edit(
        &mut self,
        field: CreatureField,
        value: CreatureFieldValue,
    ) -> ValidationResult<()> {
        let mut next = self.clone();
        set_field(&mut next.creature, field, value)?;
        next.field_origins
            .insert(field.key()?, CreatureOrigin::UserModified);
        update_provenance(&mut next, GenerationAction::ManualEdit)?;
        next.validate()?;
        *self = next;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CreatureForge;

impl CreatureForge {
    pub fn new_dna(seed: RootSeed, preset: &str, axes: CreativeAxes) -> ValidationResult<ForgeDNA> {
        let preset = PresetId::new(preset)?;
        if !PRESETS.contains(&preset.as_str()) {
            return Err(error(
                ValidationCode::UnsupportedPreset,
                "universe_preset",
                "unsupported Creature preset",
            ));
        }
        let mut recipe = GenerationRecipe::new(GenerationKind::Creature, seed, preset);
        recipe.creative_axes = axes;
        for field in CREATURE_FIELD_ORDER {
            recipe.add_field(field.key()?)?;
        }
        recipe.set_dependencies(
            CreatureField::Appearance.key()?,
            [CreatureField::Profile.key()?],
        )?;
        recipe.set_dependencies(
            CreatureField::Habitat.key()?,
            [CreatureField::Profile.key()?],
        )?;
        recipe.set_dependencies(
            CreatureField::Behavior.key()?,
            [CreatureField::Profile.key()?],
        )?;
        recipe.set_dependencies(
            CreatureField::Ecology.key()?,
            [CreatureField::Profile.key()?],
        )?;
        recipe.set_dependencies(
            CreatureField::Threat.key()?,
            [CreatureField::Profile.key()?],
        )?;
        recipe.set_dependencies(
            CreatureField::AbilitiesSummary.key()?,
            [CreatureField::Profile.key()?],
        )?;
        recipe.set_dependencies(
            CreatureField::Weakness.key()?,
            [CreatureField::Profile.key()?],
        )?;
        recipe.set_dependencies(
            CreatureField::Origin.key()?,
            [CreatureField::Profile.key()?],
        )?;
        recipe.set_dependencies(
            CreatureField::SignsOfPresence.key()?,
            [CreatureField::Profile.key()?],
        )?;
        recipe.set_dependencies(
            CreatureField::LootOrResource.key()?,
            [CreatureField::Profile.key()?],
        )?;
        let mut dna = ForgeDNA::new(recipe);
        dna.generator_version = CREATURE_GENERATOR_VERSION;
        dna.content_version = CREATURE_CONTENT_VERSION;
        dna.normalize()?;
        validate_creature_dna(&dna)?;
        Ok(dna)
    }

    pub fn generate(dna: &ForgeDNA) -> ValidationResult<CreatureCandidate> {
        Self::generate_in_project(dna, demo_project_id())
    }

    pub fn generate_in_project(
        dna: &ForgeDNA,
        project_id: ProjectId,
    ) -> ValidationResult<CreatureCandidate> {
        let mut dna = dna.clone();
        dna.normalize()?;
        validate_creature_dna(&dna)?;
        project_id.validate("project_id")?;
        let values = generate_values(&dna)?;
        let id = deterministic_id(
            "character-forge/creature/entity-id/v1",
            &[&dna.canonical_bytes()?, project_id.uuid().as_bytes()],
        );
        let mut creature =
            creature_from_values(values, dna.clone(), EntityId::from_bytes(id), project_id)?;
        creature.envelope.generation_provenance = Some(GenerationMetadata {
            generation_id: EntityId::from_bytes(deterministic_id(
                "character-forge/creature/generation-id/v1",
                &[&dna.canonical_bytes()?, project_id.uuid().as_bytes()],
            )),
            generated_at_unix_ms: 0,
            action: GenerationAction::Forge,
            parent_generation_id: None,
            dna: dna.clone(),
        });
        let field_origins = CREATURE_FIELD_ORDER
            .into_iter()
            .map(|field| Ok((field.key()?, CreatureOrigin::Generated)))
            .collect::<ValidationResult<_>>()?;
        let candidate = CreatureCandidate {
            creature,
            dna,
            field_origins,
        };
        candidate.validate()?;
        Ok(candidate)
    }

    pub fn lock(candidate: &mut CreatureCandidate, field: CreatureField) -> ValidationResult<()> {
        change_lock(candidate, field, true)
    }
    pub fn unlock(candidate: &mut CreatureCandidate, field: CreatureField) -> ValidationResult<()> {
        change_lock(candidate, field, false)
    }
    pub fn reroll_field(
        candidate: &mut CreatureCandidate,
        field: CreatureField,
    ) -> ValidationResult<Vec<CreatureField>> {
        reroll(candidate, Some(field))
    }
    pub fn reroll_all_unlocked(
        candidate: &mut CreatureCandidate,
    ) -> ValidationResult<Vec<CreatureField>> {
        reroll(candidate, None)
    }
    pub fn manual_edit(
        candidate: &mut CreatureCandidate,
        field: CreatureField,
        value: CreatureFieldValue,
    ) -> ValidationResult<()> {
        candidate.manual_edit(field, value)
    }
}

fn validate_creature_dna(dna: &ForgeDNA) -> ValidationResult<()> {
    dna.validate()?;
    if dna.recipe.kind != GenerationKind::Creature
        || dna.generator_version != CREATURE_GENERATOR_VERSION
        || dna.content_version != CREATURE_CONTENT_VERSION
    {
        return Err(error(
            ValidationCode::UnsupportedVersion,
            "creature.dna",
            "Creature Forge requires generation/content version 1 and Creature generation kind",
        ));
    }
    if !PRESETS.contains(&dna.recipe.universe_preset.as_str()) {
        return Err(error(
            ValidationCode::UnsupportedPreset,
            "universe_preset",
            "unsupported Creature preset",
        ));
    }
    if !dna.recipe.constraints.is_empty() {
        return Err(error(
            ValidationCode::InvalidConstraint,
            "constraints",
            "Creature Forge does not currently support generation constraints",
        ));
    }
    let expected = CREATURE_FIELD_ORDER
        .into_iter()
        .map(CreatureField::key)
        .collect::<ValidationResult<BTreeSet<_>>>()?;
    if dna
        .recipe
        .field_controls
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        != expected
    {
        return Err(error(
            ValidationCode::InvalidDependency,
            "recipe.field_controls",
            "Creature ForgeDNA must register exactly the canonical Creature fields",
        ));
    }
    let mut expected_dependencies = BTreeMap::new();
    expected_dependencies.insert(
        CreatureField::Appearance.key()?,
        BTreeSet::from([CreatureField::Profile.key()?]),
    );
    expected_dependencies.insert(
        CreatureField::Habitat.key()?,
        BTreeSet::from([CreatureField::Profile.key()?]),
    );
    expected_dependencies.insert(
        CreatureField::Behavior.key()?,
        BTreeSet::from([CreatureField::Profile.key()?]),
    );
    expected_dependencies.insert(
        CreatureField::Ecology.key()?,
        BTreeSet::from([CreatureField::Profile.key()?]),
    );
    expected_dependencies.insert(
        CreatureField::Threat.key()?,
        BTreeSet::from([CreatureField::Profile.key()?]),
    );
    expected_dependencies.insert(
        CreatureField::AbilitiesSummary.key()?,
        BTreeSet::from([CreatureField::Profile.key()?]),
    );
    expected_dependencies.insert(
        CreatureField::Weakness.key()?,
        BTreeSet::from([CreatureField::Profile.key()?]),
    );
    expected_dependencies.insert(
        CreatureField::Origin.key()?,
        BTreeSet::from([CreatureField::Profile.key()?]),
    );
    expected_dependencies.insert(
        CreatureField::SignsOfPresence.key()?,
        BTreeSet::from([CreatureField::Profile.key()?]),
    );
    expected_dependencies.insert(
        CreatureField::LootOrResource.key()?,
        BTreeSet::from([CreatureField::Profile.key()?]),
    );
    if dna.recipe.dependencies != expected_dependencies {
        return Err(error(
            ValidationCode::InvalidDependency,
            "recipe.dependencies",
            "Creature ForgeDNA dependency graph does not match the versioned Creature graph",
        ));
    }
    Ok(())
}

fn generate_values(dna: &ForgeDNA) -> ValidationResult<FieldValues<CreatureFieldValue>> {
    let mut values = FieldValues::new();
    for field in CREATURE_FIELD_ORDER {
        let key = field.key()?;
        let source = chaos_source_for(dna, field)?;
        let mut stream = FieldStream::derive(dna, &key)?;
        let profile = values.get(&CreatureField::Profile.key()?).and_then(|v| {
            if let CreatureFieldValue::Profile(v) = v {
                Some(*v)
            } else {
                None
            }
        });
        values.insert(
            key,
            generate_field(dna, source, field, profile, &mut stream)?,
        );
    }
    Ok(values)
}

fn generate_field(
    dna: &ForgeDNA,
    source: &str,
    field: CreatureField,
    profile: Option<CreatureProfile>,
    stream: &mut FieldStream,
) -> ValidationResult<CreatureFieldValue> {
    let theme = creature_content::theme(source).ok_or_else(|| {
        error(
            ValidationCode::UnsupportedPreset,
            "universe_preset",
            "missing Creature corpus",
        )
    })?;
    if field == CreatureField::Profile {
        return Ok(CreatureFieldValue::Profile(
            CreatureProfile::ALL[stream.next_bounded(11)? as usize],
        ));
    }
    let index = stream.next_bounded(6)? as usize;
    let axes = &dna.recipe.creative_axes;
    if field == CreatureField::StoryHooks {
        let start = axis_index(index, axes, CreativeAxis::Chaos);
        return Ok(CreatureFieldValue::Texts(
            (0..3)
                .map(|o| theme.hooks[(start + o) % 6].to_owned())
                .collect(),
        ));
    }
    let shape = creature_content::profile(profile.unwrap_or(CreatureProfile::Other));
    let text = match field {
        CreatureField::Profile | CreatureField::StoryHooks => unreachable!(),
        CreatureField::Name => theme.names[index].to_owned(),
        CreatureField::Appearance => format!(
            "{} {}",
            shape.appearance[index],
            theme.appearance[axis_index(index, axes, CreativeAxis::Technology)]
        ),
        CreatureField::Habitat => format!("{} {}", shape.habitat[index], theme.habitat[index]),
        CreatureField::Behavior => format!(
            "{} {}",
            shape.behavior[index],
            theme.behavior[axis_index(index, axes, CreativeAxis::Ambiguity)]
        ),
        CreatureField::Ecology => format!("{} {}", shape.ecology[index], theme.ecology[index]),
        CreatureField::Threat => format!(
            "{} {}",
            shape.threat[axis_index(index, axes, CreativeAxis::Bleak)],
            theme.threat[axis_index(index, axes, CreativeAxis::Epic)]
        ),
        CreatureField::AbilitiesSummary => format!(
            "{} {}",
            shape.abilities[axis_index(index, axes, CreativeAxis::Mythic)],
            theme.abilities[axis_index(index, axes, CreativeAxis::Mysticism)]
        ),
        CreatureField::Weakness => format!("{} {}", shape.weakness[index], theme.weakness[index]),
        CreatureField::Origin => format!("{} {}", shape.origin[index], theme.origin[index]),
        CreatureField::SignsOfPresence => format!("{} {}", shape.signs[index], theme.signs[index]),
        CreatureField::LootOrResource => format!("{} {}", shape.loot[index], theme.loot[index]),
        CreatureField::Secret => {
            theme.secrets[axis_index(index, axes, CreativeAxis::Surreal)].to_owned()
        }
    };
    Ok(CreatureFieldValue::Text(text))
}

fn axis_index(index: usize, axes: &CreativeAxes, axis: CreativeAxis) -> usize {
    (index + (axes.levels.get(&axis).copied().unwrap_or(50) as usize / 20)) % 6
}

fn chaos_source_for(dna: &ForgeDNA, field: CreatureField) -> ValidationResult<&'static str> {
    let preset = dna.recipe.universe_preset.as_str();
    if preset != "chaos-blend" {
        return PRESETS[..10]
            .iter()
            .copied()
            .find(|candidate| *candidate == preset)
            .ok_or_else(|| {
                error(
                    ValidationCode::UnsupportedPreset,
                    "universe_preset",
                    "unsupported Creature preset",
                )
            });
    }
    let group = source_group(field);
    let mut hasher = blake3::Hasher::new_derive_key("character-forge/creature/chaos-source/v1");
    hasher.update(&dna.recipe.root_seed.value().to_le_bytes());
    hasher.update(&[group]);
    let digest = hasher.finalize();
    let index = u64::from_le_bytes(digest.as_bytes()[..8].try_into().expect("hash prefix")) % 10;
    Ok(PRESETS[..10][index as usize])
}

fn source_group(field: CreatureField) -> u8 {
    match field {
        CreatureField::Profile
        | CreatureField::Name
        | CreatureField::Appearance
        | CreatureField::Habitat
        | CreatureField::Behavior
        | CreatureField::Ecology
        | CreatureField::SignsOfPresence => 0,
        CreatureField::Threat
        | CreatureField::AbilitiesSummary
        | CreatureField::Weakness
        | CreatureField::LootOrResource => 1,
        CreatureField::Origin | CreatureField::Secret | CreatureField::StoryHooks => 2,
    }
}

fn creature_from_values(
    values: FieldValues<CreatureFieldValue>,
    dna: ForgeDNA,
    id: EntityId,
    project_id: ProjectId,
) -> ValidationResult<Creature> {
    let profile = match values.get(&CreatureField::Profile.key()?) {
        Some(CreatureFieldValue::Profile(v)) => *v,
        _ => {
            return Err(error(
                ValidationCode::MissingField,
                "creature.creature_profile",
                "generated profile is missing",
            ));
        }
    };
    let text = |field: CreatureField| match values.get(&field.key().ok()?) {
        Some(CreatureFieldValue::Text(value)) => Some(value.clone()),
        _ => None,
    };
    let list = |field: CreatureField| match values.get(&field.key().ok()?) {
        Some(CreatureFieldValue::Texts(value)) => Some(value.clone()),
        _ => None,
    };
    let name = text(CreatureField::Name).ok_or_else(|| {
        error(
            ValidationCode::MissingField,
            "creature.name",
            "generated name is missing",
        )
    })?;
    Ok(Creature {
        envelope: EntityEnvelope {
            id,
            project_id,
            entity_type: EntityType::Creature,
            name,
            schema_version: INITIAL_SCHEMA_VERSION,
            canon_state: CanonState::Candidate,
            origin: EntityOrigin::Generated,
            favorite: false,
            created_at_unix_ms: 0,
            updated_at_unix_ms: 0,
            tags: vec![format!("preset:{}", dna.recipe.universe_preset)],
            notes: None,
            generation_provenance: None,
        },
        creature_profile: profile,
        appearance: text(CreatureField::Appearance).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.appearance",
                "generated appearance is missing",
            )
        })?,
        habitat: text(CreatureField::Habitat).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.habitat",
                "generated habitat is missing",
            )
        })?,
        behavior: text(CreatureField::Behavior).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.behavior",
                "generated behavior is missing",
            )
        })?,
        ecology: text(CreatureField::Ecology).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.ecology",
                "generated ecology is missing",
            )
        })?,
        threat: text(CreatureField::Threat).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.threat",
                "generated threat is missing",
            )
        })?,
        abilities_summary: text(CreatureField::AbilitiesSummary).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.abilities_summary",
                "generated abilities summary is missing",
            )
        })?,
        weakness: text(CreatureField::Weakness).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.weakness",
                "generated weakness is missing",
            )
        })?,
        origin: text(CreatureField::Origin).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.origin",
                "generated origin is missing",
            )
        })?,
        signs_of_presence: text(CreatureField::SignsOfPresence).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.signs_of_presence",
                "generated signs of presence are missing",
            )
        })?,
        loot_or_resource: text(CreatureField::LootOrResource).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.loot_or_resource",
                "missing resource",
            )
        })?,
        secret: text(CreatureField::Secret).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.secret",
                "generated secret is missing",
            )
        })?,
        story_hooks: list(CreatureField::StoryHooks).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.story_hooks",
                "generated story hooks are missing",
            )
        })?,
    })
}

fn set_field(
    creature: &mut Creature,
    field: CreatureField,
    value: CreatureFieldValue,
) -> ValidationResult<()> {
    match (field, value) {
        (CreatureField::Profile, CreatureFieldValue::Profile(v)) => creature.creature_profile = v,
        (CreatureField::Name, CreatureFieldValue::Text(v)) => creature.envelope.name = v,
        (CreatureField::Appearance, CreatureFieldValue::Text(v)) => creature.appearance = v,
        (CreatureField::Habitat, CreatureFieldValue::Text(v)) => creature.habitat = v,
        (CreatureField::Behavior, CreatureFieldValue::Text(v)) => creature.behavior = v,
        (CreatureField::Ecology, CreatureFieldValue::Text(v)) => creature.ecology = v,
        (CreatureField::Threat, CreatureFieldValue::Text(v)) => creature.threat = v,
        (CreatureField::AbilitiesSummary, CreatureFieldValue::Text(v)) => {
            creature.abilities_summary = v
        }
        (CreatureField::Weakness, CreatureFieldValue::Text(v)) => creature.weakness = v,
        (CreatureField::Origin, CreatureFieldValue::Text(v)) => creature.origin = v,
        (CreatureField::SignsOfPresence, CreatureFieldValue::Text(v)) => {
            creature.signs_of_presence = v
        }
        (CreatureField::LootOrResource, CreatureFieldValue::Text(v)) => {
            creature.loot_or_resource = v
        }
        (CreatureField::Secret, CreatureFieldValue::Text(v)) => creature.secret = v,
        (CreatureField::StoryHooks, CreatureFieldValue::Texts(v)) => creature.story_hooks = v,
        _ => {
            return Err(error(
                ValidationCode::InvalidEntity,
                "creature.field",
                "field value has the wrong scalar/list/profile shape",
            ));
        }
    }
    Ok(())
}

fn candidate_values(creature: &Creature) -> ValidationResult<FieldValues<CreatureFieldValue>> {
    let mut values = FieldValues::new();
    for (field, value) in [
        (
            CreatureField::Profile,
            CreatureFieldValue::Profile(creature.creature_profile),
        ),
        (
            CreatureField::Name,
            CreatureFieldValue::Text(creature.envelope.name.clone()),
        ),
        (
            CreatureField::Appearance,
            CreatureFieldValue::Text(creature.appearance.clone()),
        ),
        (
            CreatureField::Habitat,
            CreatureFieldValue::Text(creature.habitat.clone()),
        ),
        (
            CreatureField::Behavior,
            CreatureFieldValue::Text(creature.behavior.clone()),
        ),
        (
            CreatureField::Ecology,
            CreatureFieldValue::Text(creature.ecology.clone()),
        ),
        (
            CreatureField::Threat,
            CreatureFieldValue::Text(creature.threat.clone()),
        ),
        (
            CreatureField::AbilitiesSummary,
            CreatureFieldValue::Text(creature.abilities_summary.clone()),
        ),
        (
            CreatureField::Weakness,
            CreatureFieldValue::Text(creature.weakness.clone()),
        ),
        (
            CreatureField::Origin,
            CreatureFieldValue::Text(creature.origin.clone()),
        ),
        (
            CreatureField::SignsOfPresence,
            CreatureFieldValue::Text(creature.signs_of_presence.clone()),
        ),
        (
            CreatureField::LootOrResource,
            CreatureFieldValue::Text(creature.loot_or_resource.clone()),
        ),
        (
            CreatureField::Secret,
            CreatureFieldValue::Text(creature.secret.clone()),
        ),
        (
            CreatureField::StoryHooks,
            CreatureFieldValue::Texts(creature.story_hooks.clone()),
        ),
    ] {
        values.insert(field.key()?, value);
    }
    Ok(values)
}

fn reroll(
    candidate: &mut CreatureCandidate,
    requested: Option<CreatureField>,
) -> ValidationResult<Vec<CreatureField>> {
    let mut next = candidate.clone();
    next.validate()?;
    let mut dna = next.dna.clone();
    let prior_locks: BTreeMap<_, _> = dna
        .recipe
        .field_controls
        .iter()
        .map(|(k, v)| (k.clone(), v.locked))
        .collect();
    for (key, origin) in &next.field_origins {
        if *origin == CreatureOrigin::UserModified
            && requested.is_none_or(|field| field.key().is_ok_and(|k| k != *key))
        {
            dna.recipe
                .field_controls
                .get_mut(key)
                .expect("validated field control")
                .locked = true;
        }
    }
    let mut values = candidate_values(&next.creature)?;
    let mut staged = values.clone();
    let recipe_dna = dna.clone();
    let result = if let Some(field) = requested {
        crate::reroll_field(&mut dna, &mut values, &field.key()?, |key, stream| {
            generate_staged(&recipe_dna, key, stream, &mut staged)
        })?
    } else {
        crate::reroll_all_unlocked(&mut dna, &mut values, |key, stream| {
            generate_staged(&recipe_dna, key, stream, &mut staged)
        })?
    };
    for (key, locked) in prior_locks {
        dna.recipe
            .field_controls
            .get_mut(&key)
            .expect("validated field control")
            .locked = locked;
    }
    for key in &result.changed_fields {
        let field = CreatureField::from_key(key).ok_or_else(|| {
            error(
                ValidationCode::UnknownField,
                "creature.field",
                "unknown Creature field key",
            )
        })?;
        let value = values.get(key).cloned().ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "creature.field",
                "rerolled field value is missing",
            )
        })?;
        set_field(&mut next.creature, field, value)?;
        next.field_origins
            .insert(key.clone(), CreatureOrigin::Generated);
    }
    next.dna = dna;
    update_provenance(
        &mut next,
        if requested.is_some() {
            GenerationAction::RerollField
        } else {
            GenerationAction::RerollUnlocked
        },
    )?;
    next.validate()?;
    *candidate = next;
    Ok(result
        .changed_fields
        .into_iter()
        .filter_map(|k| CreatureField::from_key(&k))
        .collect())
}

fn generate_staged(
    dna: &ForgeDNA,
    key: &FieldKey,
    stream: &mut FieldStream,
    staged: &mut FieldValues<CreatureFieldValue>,
) -> ValidationResult<CreatureFieldValue> {
    let field = CreatureField::from_key(key).ok_or_else(|| {
        error(
            ValidationCode::UnknownField,
            "creature.field",
            "unknown Creature field key",
        )
    })?;
    let source = chaos_source_for(dna, field)?;
    let profile = staged.get(&CreatureField::Profile.key()?).and_then(|v| {
        if let CreatureFieldValue::Profile(v) = v {
            Some(*v)
        } else {
            None
        }
    });
    let value = generate_field(dna, source, field, profile, stream)?;
    staged.insert(key.clone(), value.clone());
    Ok(value)
}

fn change_lock(
    candidate: &mut CreatureCandidate,
    field: CreatureField,
    locked: bool,
) -> ValidationResult<()> {
    let mut next = candidate.clone();
    next.validate()?;
    next.dna
        .recipe
        .field_controls
        .get_mut(&field.key()?)
        .ok_or_else(|| {
            error(
                ValidationCode::UnknownField,
                "creature.field",
                "field is not registered",
            )
        })?
        .locked = locked;
    update_provenance(&mut next, GenerationAction::ManualEdit)?;
    next.validate()?;
    *candidate = next;
    Ok(())
}

fn update_provenance(
    candidate: &mut CreatureCandidate,
    action: GenerationAction,
) -> ValidationResult<()> {
    let mut dna = candidate.dna.clone();
    dna.normalize()?;
    candidate.dna = dna.clone();
    candidate.creature.envelope.generation_provenance = Some(GenerationMetadata {
        generation_id: EntityId::from_bytes(deterministic_id(
            "character-forge/creature/generation-id/v1",
            &[
                &dna.canonical_bytes()?,
                candidate.creature.envelope.id.uuid().as_bytes(),
                &[action as u8],
            ],
        )),
        generated_at_unix_ms: 0,
        action,
        parent_generation_id: None,
        dna,
    });
    Ok(())
}

fn deterministic_id(context: &str, parts: &[&[u8]]) -> [u8; 16] {
    let mut hasher = blake3::Hasher::new_derive_key(context);
    for part in parts {
        hasher.update(&(part.len() as u64).to_le_bytes());
        hasher.update(part);
    }
    let mut id = [0; 16];
    id.copy_from_slice(&hasher.finalize().as_bytes()[..16]);
    id[6] = (id[6] & 0x0f) | 0x50;
    id[8] = (id[8] & 0x3f) | 0x80;
    id
}

fn demo_project_id() -> ProjectId {
    let mut id = [0; 16];
    id[15] = 1;
    ProjectId::from_bytes(id)
}

fn error(code: ValidationCode, path: &str, message: &str) -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(code, path, message))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dna(seed: u64, preset: &str) -> ForgeDNA {
        CreatureForge::new_dna(RootSeed::new(seed), preset, CreativeAxes::neutral()).unwrap()
    }

    fn content(candidate: &CreatureCandidate) -> String {
        serde_json::to_string(&(
            &candidate.creature.envelope.name,
            candidate.creature.creature_profile,
            &candidate.creature.appearance,
            &candidate.creature.habitat,
            &candidate.creature.behavior,
            &candidate.creature.ecology,
            &candidate.creature.threat,
            &candidate.creature.abilities_summary,
            &candidate.creature.weakness,
            &candidate.creature.origin,
            &candidate.creature.signs_of_presence,
            &candidate.creature.loot_or_resource,
            &candidate.creature.secret,
            &candidate.creature.story_hooks,
        ))
        .unwrap()
    }

    #[test]
    fn preset_sweep_is_valid_deterministic_and_clone_free() {
        for preset in PRESETS {
            let mut seen = BTreeSet::new();
            for seed in 0..256 {
                let source = dna(seed, preset);
                let ecologyal = source.clone();
                let first = CreatureForge::generate(&source).unwrap();
                let replay = CreatureForge::generate(&source).unwrap();
                first.validate().unwrap();
                assert_eq!(first, replay, "replay changed for {preset} seed {seed}");
                assert_eq!(source, ecologyal, "generation mutated ForgeDNA");
                assert_eq!(first.creature.story_hooks.len(), 3);
                assert!(seen.insert(content(&first)), "clone in {preset} at {seed}");
            }
        }
    }

    #[test]
    fn chaos_blend_has_broad_source_and_profile_distribution() {
        let mut sources: [BTreeSet<&str>; 3] = std::array::from_fn(|_| BTreeSet::new());
        let mut source_counts: [BTreeMap<&str, usize>; 3] =
            std::array::from_fn(|_| BTreeMap::new());
        let mut profile_counts = BTreeMap::new();
        let mut patterns = BTreeSet::new();
        let mut mixed = 0;
        let mut profiles = BTreeSet::new();
        let mut contents = BTreeSet::new();
        for seed in 0..1024 {
            let generation_dna = dna(seed, "chaos-blend");
            let candidate = CreatureForge::generate(&generation_dna).unwrap();
            candidate.validate().unwrap();
            assert_eq!(candidate, CreatureForge::generate(&generation_dna).unwrap());
            for field in CREATURE_FIELD_ORDER {
                sources[source_group(field) as usize]
                    .insert(chaos_source_for(&generation_dna, field).unwrap());
            }
            let pattern = [
                CreatureField::Name,
                CreatureField::Threat,
                CreatureField::Secret,
            ]
            .map(|field| chaos_source_for(&generation_dna, field).unwrap());
            mixed += usize::from(pattern.iter().collect::<BTreeSet<_>>().len() > 1);
            patterns.insert(pattern);
            for (group, source) in pattern.into_iter().enumerate() {
                *source_counts[group].entry(source).or_insert(0) += 1;
            }
            *profile_counts
                .entry(candidate.creature.creature_profile)
                .or_insert(0) += 1;
            profiles.insert(candidate.creature.creature_profile);
            assert!(
                contents.insert(content(&candidate)),
                "Chaos clone at {seed}"
            );
        }
        assert!(sources.iter().all(|group| group.len() == 10));
        assert_eq!(profiles.len(), CreatureProfile::ALL.len());
        assert!(patterns.len() > 500);
        assert!(mixed > 950);
        assert!(
            source_counts
                .iter()
                .all(|counts| counts.values().all(|count| *count < 160))
        );
        println!(
            "Creature Chaos: profile counts {profile_counts:?}; source counts {source_counts:?}; patterns {}; mixed {mixed}/1024; exact duplicates 0",
            patterns.len()
        );
    }

    #[test]
    fn action_keys_locks_manual_edit_and_reroll_are_typed() {
        assert_eq!(
            CreatureField::from_action_key("name"),
            Some(CreatureField::Name)
        );
        assert_eq!(
            CreatureField::from_action_key("creature_profile"),
            Some(CreatureField::Profile)
        );
        let mut candidate = CreatureForge::generate(&dna(9, "nordic-myth")).unwrap();
        CreatureForge::lock(&mut candidate, CreatureField::Name).unwrap();
        CreatureForge::lock(&mut candidate, CreatureField::Profile).unwrap();
        let name = candidate.creature.envelope.name.clone();
        let profile = candidate.creature.creature_profile;
        let name_control = candidate.dna.recipe.field_controls[&CreatureField::Name.key().unwrap()];
        let profile_control =
            candidate.dna.recipe.field_controls[&CreatureField::Profile.key().unwrap()];
        CreatureForge::reroll_all_unlocked(&mut candidate).unwrap();
        assert_eq!(candidate.creature.envelope.name, name);
        assert_eq!(candidate.creature.creature_profile, profile);
        assert_eq!(
            candidate.dna.recipe.field_controls[&CreatureField::Name.key().unwrap()],
            name_control
        );
        assert_eq!(
            candidate.dna.recipe.field_controls[&CreatureField::Profile.key().unwrap()],
            profile_control
        );
        CreatureForge::manual_edit(
            &mut candidate,
            CreatureField::Appearance,
            CreatureFieldValue::Text("The æther-keepers of Åland".into()),
        )
        .unwrap();
        assert_eq!(
            candidate.field_origins[&CreatureField::Appearance.key().unwrap()],
            CreatureOrigin::UserModified
        );
        CreatureForge::reroll_field(&mut candidate, CreatureField::Appearance).unwrap();
        assert_eq!(
            candidate.field_origins[&CreatureField::Appearance.key().unwrap()],
            CreatureOrigin::Generated
        );
    }

    #[test]
    fn profile_serialization_and_schema_boundaries_are_stable() {
        assert_eq!(
            serde_json::to_string(&CreatureProfile::HumanoidCreature).unwrap(),
            "\"humanoid-creature\""
        );
        assert_eq!(
            serde_json::to_string(&GenerationKind::Creature).unwrap(),
            "\"creature\""
        );
        let mut invalid_version = dna(1, "pirate");
        invalid_version.generator_version = GeneratorVersion(2);
        assert!(CreatureForge::generate(&invalid_version).is_err());
        assert!(
            CreatureForge::new_dna(RootSeed::new(1), "unknown", CreativeAxes::neutral()).is_err()
        );
    }

    #[test]
    fn creative_axis_extremes_are_valid_and_can_influence_output() {
        let neutral = CreatureForge::generate(&dna(74, "dark-fantasy")).unwrap();
        let mut mythic_low = CreativeAxes::neutral();
        mythic_low.levels.insert(CreativeAxis::Mythic, 0);
        let mut mythic_high = CreativeAxes::neutral();
        mythic_high.levels.insert(CreativeAxis::Mythic, 100);
        let low = CreatureForge::generate(
            &CreatureForge::new_dna(RootSeed::new(74), "dark-fantasy", mythic_low).unwrap(),
        )
        .unwrap();
        let high = CreatureForge::generate(
            &CreatureForge::new_dna(RootSeed::new(74), "dark-fantasy", mythic_high).unwrap(),
        )
        .unwrap();
        neutral.validate().unwrap();
        low.validate().unwrap();
        high.validate().unwrap();
        assert_ne!(
            low.creature.abilities_summary,
            high.creature.abilities_summary
        );
    }

    #[test]
    fn literal_field_keys_and_profile_dependency_firewall_are_stable() {
        let expected = [
            "01-creature-profile",
            "02-name",
            "03-appearance",
            "04-habitat",
            "05-behavior",
            "06-ecology",
            "07-threat",
            "08-abilities-summary",
            "09-weakness",
            "10-origin",
            "11-signs-of-presence",
            "12-loot-or-resource",
            "13-secret",
            "14-story-hooks",
        ];
        assert_eq!(
            CreatureField::ALL.map(|f| f.key().unwrap().to_string()),
            expected
        );
        let mut candidate = CreatureForge::generate(&dna(308, "pirate")).unwrap();
        CreatureForge::lock(&mut candidate, CreatureField::Appearance).unwrap();
        let before = candidate.clone();
        let changed = CreatureForge::reroll_field(&mut candidate, CreatureField::Profile).unwrap();
        assert_eq!(
            changed,
            vec![
                CreatureField::Profile,
                CreatureField::Habitat,
                CreatureField::Behavior,
                CreatureField::Ecology,
                CreatureField::Threat,
                CreatureField::AbilitiesSummary,
                CreatureField::Weakness,
                CreatureField::Origin,
                CreatureField::SignsOfPresence,
                CreatureField::LootOrResource
            ]
        );
        assert_eq!(candidate.creature.appearance, before.creature.appearance);
        for field in CreatureField::ALL {
            let key = field.key().unwrap();
            assert_eq!(
                candidate.dna.recipe.field_controls[&key].nonce.0,
                before.dna.recipe.field_controls[&key].nonce.0
                    + u64::from(changed.contains(&field))
            );
        }
        assert_eq!(candidate.creature.envelope.id, before.creature.envelope.id);
        assert_eq!(candidate.creature.secret, before.creature.secret);
        let before = candidate.clone();
        let changed = CreatureForge::reroll_all_unlocked(&mut candidate).unwrap();
        assert_eq!(changed.len(), 13);
        for field in CreatureField::ALL {
            let key = field.key().unwrap();
            assert_eq!(
                candidate.dna.recipe.field_controls[&key].nonce.0,
                before.dna.recipe.field_controls[&key].nonce.0
                    + u64::from(field != CreatureField::Appearance)
            );
        }
        let mut cycle = dna(1, "pirate");
        cycle.recipe.dependencies.insert(
            CreatureField::Profile.key().unwrap(),
            BTreeSet::from([CreatureField::Appearance.key().unwrap()]),
        );
        assert!(CreatureForge::generate(&cycle).is_err());
    }

    #[test]
    fn invalid_shapes_versions_and_manual_edits_are_atomic() {
        let candidate = CreatureForge::generate(&dna(81, "occult")).unwrap();
        let mut edited = candidate.clone();
        assert!(
            edited
                .manual_edit(
                    CreatureField::Profile,
                    CreatureFieldValue::Text("beast".into())
                )
                .is_err()
        );
        assert!(
            edited
                .manual_edit(CreatureField::Secret, CreatureFieldValue::Text("  ".into()))
                .is_err()
        );
        assert!(
            edited
                .manual_edit(CreatureField::StoryHooks, CreatureFieldValue::Texts(vec![]))
                .is_err()
        );
        assert_eq!(candidate, edited);
        edited
            .manual_edit(
                CreatureField::Ecology,
                CreatureFieldValue::Text("Forged near Åland — 東京 🜁".into()),
            )
            .unwrap();
        let nonce = edited.dna.recipe.field_controls[&CreatureField::Ecology.key().unwrap()].nonce;
        CreatureForge::reroll_all_unlocked(&mut edited).unwrap();
        assert_eq!(edited.creature.ecology, "Forged near Åland — 東京 🜁");
        assert_eq!(
            edited.dna.recipe.field_controls[&CreatureField::Ecology.key().unwrap()].nonce,
            nonce
        );
        assert_eq!(
            edited.field_origins[&CreatureField::Ecology.key().unwrap()],
            CreatureOrigin::UserModified
        );
        assert_eq!(
            serde_json::from_str::<CreatureCandidate>(&serde_json::to_string(&edited).unwrap())
                .unwrap(),
            edited
        );
        let mut unsupported = dna(1, "pirate");
        unsupported.content_version = ContentVersion(2);
        assert!(CreatureForge::generate(&unsupported).is_err());
        let mut bad = candidate.clone();
        bad.field_origins.clear();
        assert!(bad.validate().is_err());
        let mut bad = candidate.clone();
        bad.dna
            .recipe
            .field_controls
            .get_mut(&CreatureField::Name.key().unwrap())
            .unwrap()
            .locked = true;
        assert!(bad.validate().is_err());
    }

    #[test]
    fn every_locked_field_and_control_survives_all_reroll_and_provenance_matches() {
        let mut candidate = CreatureForge::generate(&dna(517, "chaos-blend")).unwrap();
        for field in CreatureField::ALL {
            CreatureForge::lock(&mut candidate, field).unwrap();
        }
        let before = candidate.clone();
        assert!(
            CreatureForge::reroll_all_unlocked(&mut candidate)
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            candidate_values(&candidate.creature).unwrap(),
            candidate_values(&before.creature).unwrap()
        );
        assert_eq!(candidate.dna, before.dna);
        assert_eq!(candidate.field_origins, before.field_origins);
        assert_eq!(
            candidate
                .creature
                .envelope
                .generation_provenance
                .as_ref()
                .unwrap()
                .dna,
            candidate.dna
        );
        for field in CreatureField::ALL {
            CreatureForge::unlock(&mut candidate, field).unwrap();
        }
        let before = candidate.clone();
        let changed = CreatureForge::reroll_all_unlocked(&mut candidate).unwrap();
        assert_eq!(changed.len(), 14);
        for field in CreatureField::ALL {
            let key = field.key().unwrap();
            assert_eq!(
                candidate.dna.recipe.field_controls[&key].nonce.0,
                before.dna.recipe.field_controls[&key].nonce.0 + 1
            );
        }
        candidate.validate().unwrap();
    }

    #[test]
    fn all_profiles_and_axes_produce_valid_distinct_structures() {
        for preset in &PRESETS[..10] {
            let theme = creature_content::theme(preset).unwrap();
            for choices in [
                theme.names,
                theme.appearance,
                theme.habitat,
                theme.behavior,
                theme.origin,
                theme.ecology,
                theme.threat,
                theme.abilities,
                theme.weakness,
                theme.signs,
                theme.loot,
                theme.secrets,
                theme.hooks,
            ] {
                assert_eq!(
                    choices.into_iter().collect::<BTreeSet<_>>().len(),
                    6,
                    "{preset}"
                );
                assert!(choices.into_iter().all(|text| !text.trim().is_empty()));
            }
        }
        let mut forms = BTreeSet::new();
        for profile in CreatureProfile::ALL {
            let source = creature_content::profile(profile);
            assert!(forms.insert(source.appearance[0]));
            for choices in [
                source.appearance,
                source.habitat,
                source.behavior,
                source.abilities,
                source.weakness,
                source.ecology,
                source.threat,
                source.origin,
                source.signs,
                source.loot,
            ] {
                assert_eq!(choices.into_iter().collect::<BTreeSet<_>>().len(), 6);
            }
        }
        for axis in CreativeAxis::ALL {
            for level in [0, 100] {
                let mut axes = CreativeAxes::neutral();
                axes.levels.insert(axis, level);
                for preset in PRESETS {
                    CreatureForge::generate(
                        &CreatureForge::new_dna(RootSeed::new(999), preset, axes.clone()).unwrap(),
                    )
                    .unwrap()
                    .validate()
                    .unwrap();
                }
            }
        }
    }
}
