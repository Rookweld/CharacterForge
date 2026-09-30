//! Standalone deterministic Faction Forge, with an isolated schema and field-stream contract.

use crate::{FieldStream, FieldValues, faction_content};
use forge_contracts::{
    CanonState, ContentVersion, CreativeAxes, CreativeAxis, EntityEnvelope, EntityId, EntityOrigin,
    EntityType, Faction, FactionProfile, FieldKey, ForgeDNA, GenerationAction, GenerationKind,
    GenerationMetadata, GenerationRecipe, GeneratorVersion, INITIAL_SCHEMA_VERSION, PresetId,
    ProjectId, RootSeed, ValidationCode, ValidationError, ValidationErrors, ValidationResult,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const FACTION_GENERATOR_VERSION: GeneratorVersion = GeneratorVersion(1);
pub const FACTION_CONTENT_VERSION: ContentVersion = ContentVersion(1);
pub const FACTION_FIELD_ORDER: [FactionField; 13] = [
    FactionField::Profile,
    FactionField::Name,
    FactionField::Identity,
    FactionField::Ideology,
    FactionField::PrimaryGoal,
    FactionField::Methods,
    FactionField::Culture,
    FactionField::Resources,
    FactionField::PublicFace,
    FactionField::HiddenTruth,
    FactionField::Symbols,
    FactionField::Reputation,
    FactionField::StoryHooks,
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
pub enum FactionField {
    Profile,
    Name,
    Identity,
    Ideology,
    PrimaryGoal,
    Methods,
    Culture,
    Resources,
    PublicFace,
    HiddenTruth,
    Symbols,
    Reputation,
    StoryHooks,
}

impl FactionField {
    pub const ALL: [Self; 13] = FACTION_FIELD_ORDER;

    pub fn key(self) -> ValidationResult<FieldKey> {
        FieldKey::new(match self {
            Self::Profile => "01-faction-profile",
            Self::Name => "02-name",
            Self::Identity => "03-identity",
            Self::Ideology => "04-ideology",
            Self::PrimaryGoal => "05-primary-goal",
            Self::Methods => "06-methods",
            Self::Culture => "07-culture",
            Self::Resources => "08-resources",
            Self::PublicFace => "09-public-face",
            Self::HiddenTruth => "10-hidden-truth",
            Self::Symbols => "11-symbols",
            Self::Reputation => "12-reputation",
            Self::StoryHooks => "13-story-hooks",
        })
    }

    pub const fn action_key(self) -> &'static str {
        match self {
            Self::Profile => "faction_profile",
            Self::Name => "name",
            Self::Identity => "identity",
            Self::Ideology => "ideology",
            Self::PrimaryGoal => "primary_goal",
            Self::Methods => "methods",
            Self::Culture => "culture",
            Self::Resources => "resources",
            Self::PublicFace => "public_face",
            Self::HiddenTruth => "hidden_truth",
            Self::Symbols => "symbols",
            Self::Reputation => "reputation",
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
pub enum FactionFieldValue {
    Profile(FactionProfile),
    Text(String),
    Texts(Vec<String>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactionOrigin {
    Generated,
    UserModified,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FactionCandidate {
    pub faction: Faction,
    pub dna: ForgeDNA,
    pub field_origins: BTreeMap<FieldKey, FactionOrigin>,
}

impl FactionCandidate {
    pub fn validate(&self) -> ValidationResult<()> {
        self.faction.validate()?;
        validate_faction_dna(&self.dna)?;
        if self
            .faction
            .envelope
            .generation_provenance
            .as_ref()
            .map(|p| &p.dna)
            != Some(&self.dna)
        {
            return Err(error(
                ValidationCode::InvalidEntity,
                "faction_candidate.provenance.dna",
                "candidate DNA and Faction provenance DNA must match",
            ));
        }
        let expected = FACTION_FIELD_ORDER
            .into_iter()
            .map(FactionField::key)
            .collect::<ValidationResult<BTreeSet<_>>>()?;
        if self.field_origins.keys().cloned().collect::<BTreeSet<_>>() != expected {
            return Err(error(
                ValidationCode::MissingField,
                "faction_candidate.field_origins",
                "candidate must record the origin of every generated Faction field",
            ));
        }
        Ok(())
    }

    pub fn manual_edit(
        &mut self,
        field: FactionField,
        value: FactionFieldValue,
    ) -> ValidationResult<()> {
        let mut next = self.clone();
        set_field(&mut next.faction, field, value)?;
        next.field_origins
            .insert(field.key()?, FactionOrigin::UserModified);
        update_provenance(&mut next, GenerationAction::ManualEdit)?;
        next.validate()?;
        *self = next;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FactionForge;

impl FactionForge {
    pub fn new_dna(seed: RootSeed, preset: &str, axes: CreativeAxes) -> ValidationResult<ForgeDNA> {
        let preset = PresetId::new(preset)?;
        if !PRESETS.contains(&preset.as_str()) {
            return Err(error(
                ValidationCode::UnsupportedPreset,
                "universe_preset",
                "unsupported Faction preset",
            ));
        }
        let mut recipe = GenerationRecipe::new(GenerationKind::Faction, seed, preset);
        recipe.creative_axes = axes;
        for field in FACTION_FIELD_ORDER {
            recipe.add_field(field.key()?)?;
        }
        recipe.set_dependencies(
            FactionField::Identity.key()?,
            [FactionField::Profile.key()?],
        )?;
        let mut dna = ForgeDNA::new(recipe);
        dna.generator_version = FACTION_GENERATOR_VERSION;
        dna.content_version = FACTION_CONTENT_VERSION;
        dna.normalize()?;
        validate_faction_dna(&dna)?;
        Ok(dna)
    }

    pub fn generate(dna: &ForgeDNA) -> ValidationResult<FactionCandidate> {
        Self::generate_in_project(dna, demo_project_id())
    }

    pub fn generate_in_project(
        dna: &ForgeDNA,
        project_id: ProjectId,
    ) -> ValidationResult<FactionCandidate> {
        let mut dna = dna.clone();
        dna.normalize()?;
        validate_faction_dna(&dna)?;
        project_id.validate("project_id")?;
        let values = generate_values(&dna)?;
        let id = deterministic_id(
            "character-forge/faction/entity-id/v1",
            &[&dna.canonical_bytes()?, project_id.uuid().as_bytes()],
        );
        let mut faction =
            faction_from_values(values, dna.clone(), EntityId::from_bytes(id), project_id)?;
        faction.envelope.generation_provenance = Some(GenerationMetadata {
            generation_id: EntityId::from_bytes(deterministic_id(
                "character-forge/faction/generation-id/v1",
                &[&dna.canonical_bytes()?, project_id.uuid().as_bytes()],
            )),
            generated_at_unix_ms: 0,
            action: GenerationAction::Forge,
            parent_generation_id: None,
            dna: dna.clone(),
        });
        let field_origins = FACTION_FIELD_ORDER
            .into_iter()
            .map(|field| Ok((field.key()?, FactionOrigin::Generated)))
            .collect::<ValidationResult<_>>()?;
        let candidate = FactionCandidate {
            faction,
            dna,
            field_origins,
        };
        candidate.validate()?;
        Ok(candidate)
    }

    pub fn lock(candidate: &mut FactionCandidate, field: FactionField) -> ValidationResult<()> {
        change_lock(candidate, field, true)
    }
    pub fn unlock(candidate: &mut FactionCandidate, field: FactionField) -> ValidationResult<()> {
        change_lock(candidate, field, false)
    }
    pub fn reroll_field(
        candidate: &mut FactionCandidate,
        field: FactionField,
    ) -> ValidationResult<Vec<FactionField>> {
        reroll(candidate, Some(field))
    }
    pub fn reroll_all_unlocked(
        candidate: &mut FactionCandidate,
    ) -> ValidationResult<Vec<FactionField>> {
        reroll(candidate, None)
    }
    pub fn manual_edit(
        candidate: &mut FactionCandidate,
        field: FactionField,
        value: FactionFieldValue,
    ) -> ValidationResult<()> {
        candidate.manual_edit(field, value)
    }
}

fn validate_faction_dna(dna: &ForgeDNA) -> ValidationResult<()> {
    dna.validate()?;
    if dna.recipe.kind != GenerationKind::Faction
        || dna.generator_version != FACTION_GENERATOR_VERSION
        || dna.content_version != FACTION_CONTENT_VERSION
    {
        return Err(error(
            ValidationCode::UnsupportedVersion,
            "faction.dna",
            "Faction Forge requires generation/content version 1 and Faction generation kind",
        ));
    }
    if !PRESETS.contains(&dna.recipe.universe_preset.as_str()) {
        return Err(error(
            ValidationCode::UnsupportedPreset,
            "universe_preset",
            "unsupported Faction preset",
        ));
    }
    if !dna.recipe.constraints.is_empty() {
        return Err(error(
            ValidationCode::InvalidConstraint,
            "constraints",
            "Faction Forge does not currently support generation constraints",
        ));
    }
    let expected = FACTION_FIELD_ORDER
        .into_iter()
        .map(FactionField::key)
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
            "Faction ForgeDNA must register exactly the canonical Faction fields",
        ));
    }
    let mut expected_dependencies = BTreeMap::new();
    expected_dependencies.insert(
        FactionField::Identity.key()?,
        BTreeSet::from([FactionField::Profile.key()?]),
    );
    if dna.recipe.dependencies != expected_dependencies {
        return Err(error(
            ValidationCode::InvalidDependency,
            "recipe.dependencies",
            "Faction ForgeDNA dependency graph does not match the versioned Faction graph",
        ));
    }
    Ok(())
}

fn generate_values(dna: &ForgeDNA) -> ValidationResult<FieldValues<FactionFieldValue>> {
    let mut values = FieldValues::new();
    for field in FACTION_FIELD_ORDER {
        let key = field.key()?;
        let source = chaos_source_for(dna, field)?;
        let mut stream = FieldStream::derive(dna, &key)?;
        let profile = values.get(&FactionField::Profile.key()?).and_then(|v| {
            if let FactionFieldValue::Profile(v) = v {
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
    field: FactionField,
    profile: Option<FactionProfile>,
    stream: &mut FieldStream,
) -> ValidationResult<FactionFieldValue> {
    let theme = faction_content::theme(source).ok_or_else(|| {
        error(
            ValidationCode::UnsupportedPreset,
            "universe_preset",
            "Faction corpus is missing for the selected preset",
        )
    })?;
    if field == FactionField::Profile {
        let index = stream.next_bounded(FactionProfile::ALL.len() as u64)? as usize;
        return Ok(FactionFieldValue::Profile(FactionProfile::ALL[index]));
    }
    let index = stream.next_bounded(6)? as usize;
    let profile = profile.unwrap_or(FactionProfile::Other);
    let axes = &dna.recipe.creative_axes;
    let text = match field {
        FactionField::Profile => unreachable!(),
        FactionField::Name => theme.names[index].to_owned(),
        FactionField::Identity => format!(
            "A faction formed by {}. {}",
            theme.identities[index],
            profile_structure(profile)
        ),
        FactionField::Ideology => {
            theme.ideologies[axis_index(index, axes, CreativeAxis::Mythic)].to_owned()
        }
        FactionField::PrimaryGoal => {
            theme.goals[axis_index(index, axes, CreativeAxis::Epic)].to_owned()
        }
        FactionField::Methods => theme.methods[index].to_owned(),
        FactionField::Culture => {
            theme.cultures[axis_index(index, axes, CreativeAxis::Ambiguity)].to_owned()
        }
        FactionField::Resources => theme.resources[index].to_owned(),
        FactionField::PublicFace => theme.public_faces[index].to_owned(),
        FactionField::HiddenTruth => {
            theme.hidden_truths[axis_index(index, axes, CreativeAxis::Bleak)].to_owned()
        }
        FactionField::Symbols => theme.symbols[index].to_owned(),
        FactionField::Reputation => theme.reputations[index].to_owned(),
        FactionField::StoryHooks => String::new(),
    };
    if field == FactionField::StoryHooks {
        let start = axis_index(index, axes, CreativeAxis::Chaos);
        let hooks = (0..3)
            .map(|offset| theme.hooks[(start + offset) % 6].to_owned())
            .collect();
        Ok(FactionFieldValue::Texts(hooks))
    } else {
        Ok(FactionFieldValue::Text(text))
    }
}

fn axis_index(index: usize, axes: &CreativeAxes, axis: CreativeAxis) -> usize {
    (index + (axes.levels.get(&axis).copied().unwrap_or(50) as usize / 20)) % 6
}

fn chaos_source_for(dna: &ForgeDNA, field: FactionField) -> ValidationResult<&'static str> {
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
                    "unsupported Faction preset",
                )
            });
    }
    let group = source_group(field);
    let mut hasher = blake3::Hasher::new_derive_key("character-forge/faction/chaos-source/v1");
    hasher.update(&dna.recipe.root_seed.value().to_le_bytes());
    hasher.update(&[group]);
    let digest = hasher.finalize();
    let index = u64::from_le_bytes(digest.as_bytes()[..8].try_into().expect("hash prefix")) % 10;
    Ok(PRESETS[..10][index as usize])
}

fn source_group(field: FactionField) -> u8 {
    match field {
        FactionField::Name
        | FactionField::Identity
        | FactionField::Ideology
        | FactionField::PrimaryGoal
        | FactionField::Methods
        | FactionField::Profile => 0u8,
        FactionField::Culture | FactionField::Resources | FactionField::PublicFace => 1,
        FactionField::HiddenTruth
        | FactionField::Symbols
        | FactionField::Reputation
        | FactionField::StoryHooks => 2,
    }
}

fn faction_from_values(
    values: FieldValues<FactionFieldValue>,
    dna: ForgeDNA,
    id: EntityId,
    project_id: ProjectId,
) -> ValidationResult<Faction> {
    let profile = match values.get(&FactionField::Profile.key()?) {
        Some(FactionFieldValue::Profile(v)) => *v,
        _ => {
            return Err(error(
                ValidationCode::MissingField,
                "faction.faction_profile",
                "generated profile is missing",
            ));
        }
    };
    let text = |field: FactionField| match values.get(&field.key().ok()?) {
        Some(FactionFieldValue::Text(value)) => Some(value.clone()),
        _ => None,
    };
    let list = |field: FactionField| match values.get(&field.key().ok()?) {
        Some(FactionFieldValue::Texts(value)) => Some(value.clone()),
        _ => None,
    };
    let name = text(FactionField::Name).ok_or_else(|| {
        error(
            ValidationCode::MissingField,
            "faction.name",
            "generated name is missing",
        )
    })?;
    Ok(Faction {
        envelope: EntityEnvelope {
            id,
            project_id,
            entity_type: EntityType::Faction,
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
        faction_profile: profile,
        identity: text(FactionField::Identity).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.identity",
                "generated identity is missing",
            )
        })?,
        ideology: text(FactionField::Ideology).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.ideology",
                "generated ideology is missing",
            )
        })?,
        primary_goal: text(FactionField::PrimaryGoal).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.primary_goal",
                "generated goal is missing",
            )
        })?,
        methods: text(FactionField::Methods).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.methods",
                "generated methods are missing",
            )
        })?,
        culture: text(FactionField::Culture).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.culture",
                "generated culture is missing",
            )
        })?,
        resources: text(FactionField::Resources).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.resources",
                "generated resources are missing",
            )
        })?,
        public_face: text(FactionField::PublicFace).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.public_face",
                "generated public face is missing",
            )
        })?,
        hidden_truth: text(FactionField::HiddenTruth).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.hidden_truth",
                "generated hidden truth is missing",
            )
        })?,
        symbols: text(FactionField::Symbols).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.symbols",
                "generated symbols are missing",
            )
        })?,
        reputation: text(FactionField::Reputation).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.reputation",
                "generated reputation is missing",
            )
        })?,
        story_hooks: list(FactionField::StoryHooks).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.story_hooks",
                "generated story hooks are missing",
            )
        })?,
    })
}

fn set_field(
    faction: &mut Faction,
    field: FactionField,
    value: FactionFieldValue,
) -> ValidationResult<()> {
    match (field, value) {
        (FactionField::Profile, FactionFieldValue::Profile(v)) => faction.faction_profile = v,
        (FactionField::Name, FactionFieldValue::Text(v)) => faction.envelope.name = v,
        (FactionField::Identity, FactionFieldValue::Text(v)) => faction.identity = v,
        (FactionField::Ideology, FactionFieldValue::Text(v)) => faction.ideology = v,
        (FactionField::PrimaryGoal, FactionFieldValue::Text(v)) => faction.primary_goal = v,
        (FactionField::Methods, FactionFieldValue::Text(v)) => faction.methods = v,
        (FactionField::Culture, FactionFieldValue::Text(v)) => faction.culture = v,
        (FactionField::Resources, FactionFieldValue::Text(v)) => faction.resources = v,
        (FactionField::PublicFace, FactionFieldValue::Text(v)) => faction.public_face = v,
        (FactionField::HiddenTruth, FactionFieldValue::Text(v)) => faction.hidden_truth = v,
        (FactionField::Symbols, FactionFieldValue::Text(v)) => faction.symbols = v,
        (FactionField::Reputation, FactionFieldValue::Text(v)) => faction.reputation = v,
        (FactionField::StoryHooks, FactionFieldValue::Texts(v)) => faction.story_hooks = v,
        _ => {
            return Err(error(
                ValidationCode::InvalidEntity,
                "faction.field",
                "field value has the wrong scalar/list/profile shape",
            ));
        }
    }
    Ok(())
}

fn candidate_values(faction: &Faction) -> ValidationResult<FieldValues<FactionFieldValue>> {
    let mut values = FieldValues::new();
    for (field, value) in [
        (
            FactionField::Profile,
            FactionFieldValue::Profile(faction.faction_profile),
        ),
        (
            FactionField::Name,
            FactionFieldValue::Text(faction.envelope.name.clone()),
        ),
        (
            FactionField::Identity,
            FactionFieldValue::Text(faction.identity.clone()),
        ),
        (
            FactionField::Ideology,
            FactionFieldValue::Text(faction.ideology.clone()),
        ),
        (
            FactionField::PrimaryGoal,
            FactionFieldValue::Text(faction.primary_goal.clone()),
        ),
        (
            FactionField::Methods,
            FactionFieldValue::Text(faction.methods.clone()),
        ),
        (
            FactionField::Culture,
            FactionFieldValue::Text(faction.culture.clone()),
        ),
        (
            FactionField::Resources,
            FactionFieldValue::Text(faction.resources.clone()),
        ),
        (
            FactionField::PublicFace,
            FactionFieldValue::Text(faction.public_face.clone()),
        ),
        (
            FactionField::HiddenTruth,
            FactionFieldValue::Text(faction.hidden_truth.clone()),
        ),
        (
            FactionField::Symbols,
            FactionFieldValue::Text(faction.symbols.clone()),
        ),
        (
            FactionField::Reputation,
            FactionFieldValue::Text(faction.reputation.clone()),
        ),
        (
            FactionField::StoryHooks,
            FactionFieldValue::Texts(faction.story_hooks.clone()),
        ),
    ] {
        values.insert(field.key()?, value);
    }
    Ok(values)
}

fn reroll(
    candidate: &mut FactionCandidate,
    requested: Option<FactionField>,
) -> ValidationResult<Vec<FactionField>> {
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
        if *origin == FactionOrigin::UserModified
            && requested.is_none_or(|field| field.key().is_ok_and(|k| k != *key))
        {
            dna.recipe
                .field_controls
                .get_mut(key)
                .expect("validated field control")
                .locked = true;
        }
    }
    let mut values = candidate_values(&next.faction)?;
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
        let field = FactionField::from_key(key).ok_or_else(|| {
            error(
                ValidationCode::UnknownField,
                "faction.field",
                "unknown Faction field key",
            )
        })?;
        let value = values.get(key).cloned().ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "faction.field",
                "rerolled field value is missing",
            )
        })?;
        set_field(&mut next.faction, field, value)?;
        next.field_origins
            .insert(key.clone(), FactionOrigin::Generated);
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
        .filter_map(|k| FactionField::from_key(&k))
        .collect())
}

fn generate_staged(
    dna: &ForgeDNA,
    key: &FieldKey,
    stream: &mut FieldStream,
    staged: &mut FieldValues<FactionFieldValue>,
) -> ValidationResult<FactionFieldValue> {
    let field = FactionField::from_key(key).ok_or_else(|| {
        error(
            ValidationCode::UnknownField,
            "faction.field",
            "unknown Faction field key",
        )
    })?;
    let source = chaos_source_for(dna, field)?;
    let profile = staged.get(&FactionField::Profile.key()?).and_then(|v| {
        if let FactionFieldValue::Profile(v) = v {
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
    candidate: &mut FactionCandidate,
    field: FactionField,
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
                "faction.field",
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
    candidate: &mut FactionCandidate,
    action: GenerationAction,
) -> ValidationResult<()> {
    let mut dna = candidate.dna.clone();
    dna.normalize()?;
    candidate.dna = dna.clone();
    candidate.faction.envelope.generation_provenance = Some(GenerationMetadata {
        generation_id: EntityId::from_bytes(deterministic_id(
            "character-forge/faction/generation-id/v1",
            &[
                &dna.canonical_bytes()?,
                candidate.faction.envelope.id.uuid().as_bytes(),
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

fn profile_structure(profile: FactionProfile) -> &'static str {
    match profile {
        FactionProfile::Guild => "Its members govern a craft or trade association.",
        FactionProfile::Order => "It follows a formal order with ranks and vows.",
        FactionProfile::Cult => "It gathers around a shared doctrine and initiated membership.",
        FactionProfile::Government => "It holds authority through offices, law, or civic mandate.",
        FactionProfile::Kingdom => "It claims sovereign territory under a royal house.",
        FactionProfile::Military => "It operates as a standing force under a chain of command.",
        FactionProfile::Corporation => {
            "It operates through owned assets, contracts, and executive control."
        }
        FactionProfile::Gang => "It is organized through local crews and mutual protection.",
        FactionProfile::Rebellion => "It coordinates opposition to an existing authority.",
        FactionProfile::ReligiousOrder => {
            "It combines religious vows with an established hierarchy."
        }
        FactionProfile::Tribe => "It is organized around kinship and communal stewardship.",
        FactionProfile::SecretSociety => "It restricts membership and conceals its deliberations.",
        FactionProfile::Other => {
            "It has a distinct structure outside the common institutional forms."
        }
    }
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
        FactionForge::new_dna(RootSeed::new(seed), preset, CreativeAxes::neutral()).unwrap()
    }

    fn content(candidate: &FactionCandidate) -> String {
        serde_json::to_string(&(
            &candidate.faction.envelope.name,
            candidate.faction.faction_profile,
            &candidate.faction.identity,
            &candidate.faction.ideology,
            &candidate.faction.primary_goal,
            &candidate.faction.methods,
            &candidate.faction.culture,
            &candidate.faction.resources,
            &candidate.faction.public_face,
            &candidate.faction.hidden_truth,
            &candidate.faction.symbols,
            &candidate.faction.reputation,
            &candidate.faction.story_hooks,
        ))
        .unwrap()
    }

    #[test]
    fn preset_sweep_is_valid_deterministic_and_clone_free() {
        for preset in PRESETS {
            let mut seen = BTreeSet::new();
            for seed in 0..256 {
                let source = dna(seed, preset);
                let original = source.clone();
                let first = FactionForge::generate(&source).unwrap();
                let replay = FactionForge::generate(&source).unwrap();
                first.validate().unwrap();
                assert_eq!(first, replay, "replay changed for {preset} seed {seed}");
                assert_eq!(source, original, "generation mutated ForgeDNA");
                assert_eq!(first.faction.story_hooks.len(), 3);
                assert!(seen.insert(content(&first)), "clone in {preset} at {seed}");
            }
        }
    }

    #[test]
    fn chaos_blend_has_broad_source_and_profile_distribution() {
        let mut sources: [BTreeSet<&str>; 3] = std::array::from_fn(|_| BTreeSet::new());
        let mut profiles = BTreeSet::new();
        let mut contents = BTreeSet::new();
        for seed in 0..1024 {
            let generation_dna = dna(seed, "chaos-blend");
            let candidate = FactionForge::generate(&generation_dna).unwrap();
            candidate.validate().unwrap();
            assert_eq!(candidate, FactionForge::generate(&generation_dna).unwrap());
            for field in FACTION_FIELD_ORDER {
                sources[source_group(field) as usize]
                    .insert(chaos_source_for(&generation_dna, field).unwrap());
            }
            profiles.insert(candidate.faction.faction_profile);
            assert!(
                contents.insert(content(&candidate)),
                "Chaos clone at {seed}"
            );
        }
        assert!(sources.iter().all(|group| group.len() == 10));
        assert_eq!(profiles.len(), FactionProfile::ALL.len());
    }

    #[test]
    fn action_keys_locks_manual_edit_and_reroll_are_typed() {
        assert_eq!(
            FactionField::from_action_key("name"),
            Some(FactionField::Name)
        );
        assert_eq!(
            FactionField::from_action_key("faction_profile"),
            Some(FactionField::Profile)
        );
        let mut candidate = FactionForge::generate(&dna(9, "nordic-myth")).unwrap();
        FactionForge::lock(&mut candidate, FactionField::Name).unwrap();
        FactionForge::lock(&mut candidate, FactionField::Profile).unwrap();
        let name = candidate.faction.envelope.name.clone();
        let profile = candidate.faction.faction_profile;
        let name_control = candidate.dna.recipe.field_controls[&FactionField::Name.key().unwrap()];
        let profile_control =
            candidate.dna.recipe.field_controls[&FactionField::Profile.key().unwrap()];
        FactionForge::reroll_all_unlocked(&mut candidate).unwrap();
        assert_eq!(candidate.faction.envelope.name, name);
        assert_eq!(candidate.faction.faction_profile, profile);
        assert_eq!(
            candidate.dna.recipe.field_controls[&FactionField::Name.key().unwrap()],
            name_control
        );
        assert_eq!(
            candidate.dna.recipe.field_controls[&FactionField::Profile.key().unwrap()],
            profile_control
        );
        FactionForge::manual_edit(
            &mut candidate,
            FactionField::Identity,
            FactionFieldValue::Text("The æther-keepers of Åland".into()),
        )
        .unwrap();
        assert_eq!(
            candidate.field_origins[&FactionField::Identity.key().unwrap()],
            FactionOrigin::UserModified
        );
        FactionForge::reroll_field(&mut candidate, FactionField::Identity).unwrap();
        assert_eq!(
            candidate.field_origins[&FactionField::Identity.key().unwrap()],
            FactionOrigin::Generated
        );
    }

    #[test]
    fn profile_serialization_and_schema_boundaries_are_stable() {
        assert_eq!(
            serde_json::to_string(&FactionProfile::ReligiousOrder).unwrap(),
            "\"religious-order\""
        );
        assert_eq!(
            serde_json::to_string(&GenerationKind::Faction).unwrap(),
            "\"faction\""
        );
        let mut invalid_version = dna(1, "pirate");
        invalid_version.generator_version = GeneratorVersion(2);
        assert!(FactionForge::generate(&invalid_version).is_err());
        assert!(
            FactionForge::new_dna(RootSeed::new(1), "unknown", CreativeAxes::neutral()).is_err()
        );
    }

    #[test]
    fn creative_axis_extremes_are_valid_and_can_influence_output() {
        let neutral = FactionForge::generate(&dna(74, "dark-fantasy")).unwrap();
        let mut mythic_low = CreativeAxes::neutral();
        mythic_low.levels.insert(CreativeAxis::Mythic, 0);
        let mut mythic_high = CreativeAxes::neutral();
        mythic_high.levels.insert(CreativeAxis::Mythic, 100);
        let low = FactionForge::generate(
            &FactionForge::new_dna(RootSeed::new(74), "dark-fantasy", mythic_low).unwrap(),
        )
        .unwrap();
        let high = FactionForge::generate(
            &FactionForge::new_dna(RootSeed::new(74), "dark-fantasy", mythic_high).unwrap(),
        )
        .unwrap();
        neutral.validate().unwrap();
        low.validate().unwrap();
        high.validate().unwrap();
        assert_ne!(low.faction.ideology, high.faction.ideology);
    }
}
