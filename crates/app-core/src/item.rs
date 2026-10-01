//! Standalone deterministic Item Forge, with an isolated schema and field-stream contract.

use crate::{FieldStream, FieldValues, item_content};
use forge_contracts::{
    CanonState, ContentVersion, CreativeAxes, CreativeAxis, EntityEnvelope, EntityId, EntityOrigin,
    EntityType, FieldKey, ForgeDNA, GenerationAction, GenerationKind, GenerationMetadata,
    GenerationRecipe, GeneratorVersion, INITIAL_SCHEMA_VERSION, Item, ItemProfile, PresetId,
    ProjectId, RootSeed, ValidationCode, ValidationError, ValidationErrors, ValidationResult,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const ITEM_GENERATOR_VERSION: GeneratorVersion = GeneratorVersion(1);
pub const ITEM_CONTENT_VERSION: ContentVersion = ContentVersion(1);
pub const ITEM_FIELD_ORDER: [ItemField; 13] = [
    ItemField::Profile,
    ItemField::Name,
    ItemField::Appearance,
    ItemField::Purpose,
    ItemField::Properties,
    ItemField::Origin,
    ItemField::History,
    ItemField::PowerOrFunction,
    ItemField::CostOrDrawback,
    ItemField::Rarity,
    ItemField::Condition,
    ItemField::Secret,
    ItemField::StoryHooks,
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
pub enum ItemField {
    Profile,
    Name,
    Appearance,
    Purpose,
    Properties,
    Origin,
    History,
    PowerOrFunction,
    CostOrDrawback,
    Rarity,
    Condition,
    Secret,
    StoryHooks,
}

impl ItemField {
    pub const ALL: [Self; 13] = ITEM_FIELD_ORDER;

    pub fn key(self) -> ValidationResult<FieldKey> {
        FieldKey::new(match self {
            Self::Profile => "01-item-profile",
            Self::Name => "02-name",
            Self::Appearance => "03-appearance",
            Self::Purpose => "04-purpose",
            Self::Properties => "05-properties",
            Self::Origin => "06-origin",
            Self::History => "07-history",
            Self::PowerOrFunction => "08-power-or-function",
            Self::CostOrDrawback => "09-cost-or-drawback",
            Self::Rarity => "10-rarity",
            Self::Condition => "11-condition",
            Self::Secret => "12-secret",
            Self::StoryHooks => "13-story-hooks",
        })
    }

    pub const fn action_key(self) -> &'static str {
        match self {
            Self::Profile => "item_profile",
            Self::Name => "name",
            Self::Appearance => "appearance",
            Self::Purpose => "purpose",
            Self::Properties => "properties",
            Self::Origin => "origin",
            Self::History => "history",
            Self::PowerOrFunction => "power_or_function",
            Self::CostOrDrawback => "cost_or_drawback",
            Self::Rarity => "rarity",
            Self::Condition => "condition",
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
pub enum ItemFieldValue {
    Profile(ItemProfile),
    Text(String),
    Texts(Vec<String>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemOrigin {
    Generated,
    UserModified,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemCandidate {
    pub item: Item,
    pub dna: ForgeDNA,
    pub field_origins: BTreeMap<FieldKey, ItemOrigin>,
}

impl ItemCandidate {
    pub fn validate(&self) -> ValidationResult<()> {
        self.item.validate()?;
        validate_item_dna(&self.dna)?;
        if self
            .item
            .envelope
            .generation_provenance
            .as_ref()
            .map(|p| &p.dna)
            != Some(&self.dna)
        {
            return Err(error(
                ValidationCode::InvalidEntity,
                "item_candidate.provenance.dna",
                "candidate DNA and Item provenance DNA must match",
            ));
        }
        let expected = ITEM_FIELD_ORDER
            .into_iter()
            .map(ItemField::key)
            .collect::<ValidationResult<BTreeSet<_>>>()?;
        if self.field_origins.keys().cloned().collect::<BTreeSet<_>>() != expected {
            return Err(error(
                ValidationCode::MissingField,
                "item_candidate.field_origins",
                "candidate must record the origin of every generated Item field",
            ));
        }
        Ok(())
    }

    pub fn manual_edit(&mut self, field: ItemField, value: ItemFieldValue) -> ValidationResult<()> {
        let mut next = self.clone();
        set_field(&mut next.item, field, value)?;
        next.field_origins
            .insert(field.key()?, ItemOrigin::UserModified);
        update_provenance(&mut next, GenerationAction::ManualEdit)?;
        next.validate()?;
        *self = next;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ItemForge;

impl ItemForge {
    pub fn new_dna(seed: RootSeed, preset: &str, axes: CreativeAxes) -> ValidationResult<ForgeDNA> {
        let preset = PresetId::new(preset)?;
        if !PRESETS.contains(&preset.as_str()) {
            return Err(error(
                ValidationCode::UnsupportedPreset,
                "universe_preset",
                "unsupported Item preset",
            ));
        }
        let mut recipe = GenerationRecipe::new(GenerationKind::Item, seed, preset);
        recipe.creative_axes = axes;
        for field in ITEM_FIELD_ORDER {
            recipe.add_field(field.key()?)?;
        }
        recipe.set_dependencies(ItemField::Appearance.key()?, [ItemField::Profile.key()?])?;
        recipe.set_dependencies(ItemField::Purpose.key()?, [ItemField::Profile.key()?])?;
        recipe.set_dependencies(ItemField::Properties.key()?, [ItemField::Profile.key()?])?;
        recipe.set_dependencies(
            ItemField::PowerOrFunction.key()?,
            [ItemField::Profile.key()?],
        )?;
        recipe.set_dependencies(
            ItemField::CostOrDrawback.key()?,
            [ItemField::Profile.key()?],
        )?;
        let mut dna = ForgeDNA::new(recipe);
        dna.generator_version = ITEM_GENERATOR_VERSION;
        dna.content_version = ITEM_CONTENT_VERSION;
        dna.normalize()?;
        validate_item_dna(&dna)?;
        Ok(dna)
    }

    pub fn generate(dna: &ForgeDNA) -> ValidationResult<ItemCandidate> {
        Self::generate_in_project(dna, demo_project_id())
    }

    pub fn generate_in_project(
        dna: &ForgeDNA,
        project_id: ProjectId,
    ) -> ValidationResult<ItemCandidate> {
        let mut dna = dna.clone();
        dna.normalize()?;
        validate_item_dna(&dna)?;
        project_id.validate("project_id")?;
        let values = generate_values(&dna)?;
        let id = deterministic_id(
            "character-forge/item/entity-id/v1",
            &[&dna.canonical_bytes()?, project_id.uuid().as_bytes()],
        );
        let mut item = item_from_values(values, dna.clone(), EntityId::from_bytes(id), project_id)?;
        item.envelope.generation_provenance = Some(GenerationMetadata {
            generation_id: EntityId::from_bytes(deterministic_id(
                "character-forge/item/generation-id/v1",
                &[&dna.canonical_bytes()?, project_id.uuid().as_bytes()],
            )),
            generated_at_unix_ms: 0,
            action: GenerationAction::Forge,
            parent_generation_id: None,
            dna: dna.clone(),
        });
        let field_origins = ITEM_FIELD_ORDER
            .into_iter()
            .map(|field| Ok((field.key()?, ItemOrigin::Generated)))
            .collect::<ValidationResult<_>>()?;
        let candidate = ItemCandidate {
            item,
            dna,
            field_origins,
        };
        candidate.validate()?;
        Ok(candidate)
    }

    pub fn lock(candidate: &mut ItemCandidate, field: ItemField) -> ValidationResult<()> {
        change_lock(candidate, field, true)
    }
    pub fn unlock(candidate: &mut ItemCandidate, field: ItemField) -> ValidationResult<()> {
        change_lock(candidate, field, false)
    }
    pub fn reroll_field(
        candidate: &mut ItemCandidate,
        field: ItemField,
    ) -> ValidationResult<Vec<ItemField>> {
        reroll(candidate, Some(field))
    }
    pub fn reroll_all_unlocked(candidate: &mut ItemCandidate) -> ValidationResult<Vec<ItemField>> {
        reroll(candidate, None)
    }
    pub fn manual_edit(
        candidate: &mut ItemCandidate,
        field: ItemField,
        value: ItemFieldValue,
    ) -> ValidationResult<()> {
        candidate.manual_edit(field, value)
    }
}

fn validate_item_dna(dna: &ForgeDNA) -> ValidationResult<()> {
    dna.validate()?;
    if dna.recipe.kind != GenerationKind::Item
        || dna.generator_version != ITEM_GENERATOR_VERSION
        || dna.content_version != ITEM_CONTENT_VERSION
    {
        return Err(error(
            ValidationCode::UnsupportedVersion,
            "item.dna",
            "Item Forge requires generation/content version 1 and Item generation kind",
        ));
    }
    if !PRESETS.contains(&dna.recipe.universe_preset.as_str()) {
        return Err(error(
            ValidationCode::UnsupportedPreset,
            "universe_preset",
            "unsupported Item preset",
        ));
    }
    crate::item_context::from_dna(dna)?;
    let expected = ITEM_FIELD_ORDER
        .into_iter()
        .map(ItemField::key)
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
            "Item ForgeDNA must register exactly the canonical Item fields",
        ));
    }
    let mut expected_dependencies = BTreeMap::new();
    expected_dependencies.insert(
        ItemField::Appearance.key()?,
        BTreeSet::from([ItemField::Profile.key()?]),
    );
    expected_dependencies.insert(
        ItemField::Purpose.key()?,
        BTreeSet::from([ItemField::Profile.key()?]),
    );
    expected_dependencies.insert(
        ItemField::Properties.key()?,
        BTreeSet::from([ItemField::Profile.key()?]),
    );
    expected_dependencies.insert(
        ItemField::PowerOrFunction.key()?,
        BTreeSet::from([ItemField::Profile.key()?]),
    );
    expected_dependencies.insert(
        ItemField::CostOrDrawback.key()?,
        BTreeSet::from([ItemField::Profile.key()?]),
    );
    if dna.recipe.dependencies != expected_dependencies {
        return Err(error(
            ValidationCode::InvalidDependency,
            "recipe.dependencies",
            "Item ForgeDNA dependency graph does not match the versioned Item graph",
        ));
    }
    Ok(())
}

fn generate_values(dna: &ForgeDNA) -> ValidationResult<FieldValues<ItemFieldValue>> {
    let mut values = FieldValues::new();
    for field in ITEM_FIELD_ORDER {
        let key = field.key()?;
        let source = chaos_source_for(dna, field)?;
        let mut stream = FieldStream::derive(dna, &key)?;
        let profile = values.get(&ItemField::Profile.key()?).and_then(|v| {
            if let ItemFieldValue::Profile(v) = v {
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
    field: ItemField,
    profile: Option<ItemProfile>,
    stream: &mut FieldStream,
) -> ValidationResult<ItemFieldValue> {
    let base = generate_base_field(dna, source, field, profile, stream)?;
    match crate::item_context::from_dna(dna)? {
        None => Ok(base),
        Some(facts) => crate::item_context::adapt(&facts, dna, source, field, base, stream),
    }
}

fn generate_base_field(
    dna: &ForgeDNA,
    source: &str,
    field: ItemField,
    profile: Option<ItemProfile>,
    stream: &mut FieldStream,
) -> ValidationResult<ItemFieldValue> {
    let theme = item_content::theme(source).ok_or_else(|| {
        error(
            ValidationCode::UnsupportedPreset,
            "universe_preset",
            "missing Item corpus",
        )
    })?;
    if field == ItemField::Profile {
        return Ok(ItemFieldValue::Profile(
            ItemProfile::ALL[stream.next_bounded(11)? as usize],
        ));
    }
    let index = stream.next_bounded(6)? as usize;
    let axes = &dna.recipe.creative_axes;
    let shape = item_content::profile(profile.unwrap_or(ItemProfile::Other));
    let text = match field {
        ItemField::Profile => unreachable!(),
        ItemField::Name => theme.names[index].to_owned(),
        ItemField::Appearance => format!(
            "{} {}",
            shape.forms[index],
            theme.surfaces[axis_index(index, axes, CreativeAxis::Technology)]
        ),
        ItemField::Purpose => format!(
            "{} {}",
            shape.purposes[index],
            theme.uses[axis_index(index, axes, CreativeAxis::Epic)]
        ),
        ItemField::Properties => format!(
            "{} {}",
            shape.properties[index],
            theme.traits[axis_index(index, axes, CreativeAxis::Mysticism)]
        ),
        ItemField::Origin => theme.origins[index].to_owned(),
        ItemField::History => {
            theme.histories[axis_index(index, axes, CreativeAxis::Ambiguity)].to_owned()
        }
        ItemField::PowerOrFunction => {
            shape.functions[axis_index(index, axes, CreativeAxis::Mythic)].to_owned()
        }
        ItemField::CostOrDrawback => {
            shape.drawbacks[axis_index(index, axes, CreativeAxis::Bleak)].to_owned()
        }
        ItemField::Rarity => item_content::RARITIES[index].to_owned(),
        ItemField::Condition => item_content::CONDITIONS[index].to_owned(),
        ItemField::Secret => {
            theme.secrets[axis_index(index, axes, CreativeAxis::Surreal)].to_owned()
        }
        ItemField::StoryHooks => String::new(),
    };
    if field == ItemField::StoryHooks {
        let start = axis_index(index, axes, CreativeAxis::Chaos);
        Ok(ItemFieldValue::Texts(
            (0..3)
                .map(|o| theme.hooks[(start + o) % 6].to_owned())
                .collect(),
        ))
    } else {
        Ok(ItemFieldValue::Text(text))
    }
}

fn axis_index(index: usize, axes: &CreativeAxes, axis: CreativeAxis) -> usize {
    (index + (axes.levels.get(&axis).copied().unwrap_or(50) as usize / 20)) % 6
}

fn chaos_source_for(dna: &ForgeDNA, field: ItemField) -> ValidationResult<&'static str> {
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
                    "unsupported Item preset",
                )
            });
    }
    let group = source_group(field);
    let mut hasher = blake3::Hasher::new_derive_key("character-forge/item/chaos-source/v1");
    hasher.update(&dna.recipe.root_seed.value().to_le_bytes());
    hasher.update(&[group]);
    let digest = hasher.finalize();
    let index = u64::from_le_bytes(digest.as_bytes()[..8].try_into().expect("hash prefix")) % 10;
    Ok(PRESETS[..10][index as usize])
}

fn source_group(field: ItemField) -> u8 {
    match field {
        ItemField::Profile
        | ItemField::Name
        | ItemField::Appearance
        | ItemField::Purpose
        | ItemField::Properties => 0,
        ItemField::Origin
        | ItemField::History
        | ItemField::PowerOrFunction
        | ItemField::CostOrDrawback => 1,
        ItemField::Rarity | ItemField::Condition | ItemField::Secret | ItemField::StoryHooks => 2,
    }
}

fn item_from_values(
    values: FieldValues<ItemFieldValue>,
    dna: ForgeDNA,
    id: EntityId,
    project_id: ProjectId,
) -> ValidationResult<Item> {
    let profile = match values.get(&ItemField::Profile.key()?) {
        Some(ItemFieldValue::Profile(v)) => *v,
        _ => {
            return Err(error(
                ValidationCode::MissingField,
                "item.item_profile",
                "generated profile is missing",
            ));
        }
    };
    let text = |field: ItemField| match values.get(&field.key().ok()?) {
        Some(ItemFieldValue::Text(value)) => Some(value.clone()),
        _ => None,
    };
    let list = |field: ItemField| match values.get(&field.key().ok()?) {
        Some(ItemFieldValue::Texts(value)) => Some(value.clone()),
        _ => None,
    };
    let name = text(ItemField::Name).ok_or_else(|| {
        error(
            ValidationCode::MissingField,
            "item.name",
            "generated name is missing",
        )
    })?;
    Ok(Item {
        envelope: EntityEnvelope {
            id,
            project_id,
            entity_type: EntityType::Item,
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
        item_profile: profile,
        appearance: text(ItemField::Appearance).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.appearance",
                "generated appearance is missing",
            )
        })?,
        purpose: text(ItemField::Purpose).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.purpose",
                "generated purpose is missing",
            )
        })?,
        properties: text(ItemField::Properties).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.properties",
                "generated goal is missing",
            )
        })?,
        origin: text(ItemField::Origin).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.origin",
                "generated origin are missing",
            )
        })?,
        history: text(ItemField::History).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.history",
                "generated history is missing",
            )
        })?,
        power_or_function: text(ItemField::PowerOrFunction).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.power_or_function",
                "generated power_or_function are missing",
            )
        })?,
        cost_or_drawback: text(ItemField::CostOrDrawback).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.cost_or_drawback",
                "generated public face is missing",
            )
        })?,
        rarity: text(ItemField::Rarity).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.rarity",
                "generated hidden truth is missing",
            )
        })?,
        condition: text(ItemField::Condition).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.condition",
                "generated condition are missing",
            )
        })?,
        secret: text(ItemField::Secret).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.secret",
                "generated secret is missing",
            )
        })?,
        story_hooks: list(ItemField::StoryHooks).ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.story_hooks",
                "generated story hooks are missing",
            )
        })?,
    })
}

fn set_field(item: &mut Item, field: ItemField, value: ItemFieldValue) -> ValidationResult<()> {
    match (field, value) {
        (ItemField::Profile, ItemFieldValue::Profile(v)) => item.item_profile = v,
        (ItemField::Name, ItemFieldValue::Text(v)) => item.envelope.name = v,
        (ItemField::Appearance, ItemFieldValue::Text(v)) => item.appearance = v,
        (ItemField::Purpose, ItemFieldValue::Text(v)) => item.purpose = v,
        (ItemField::Properties, ItemFieldValue::Text(v)) => item.properties = v,
        (ItemField::Origin, ItemFieldValue::Text(v)) => item.origin = v,
        (ItemField::History, ItemFieldValue::Text(v)) => item.history = v,
        (ItemField::PowerOrFunction, ItemFieldValue::Text(v)) => item.power_or_function = v,
        (ItemField::CostOrDrawback, ItemFieldValue::Text(v)) => item.cost_or_drawback = v,
        (ItemField::Rarity, ItemFieldValue::Text(v)) => item.rarity = v,
        (ItemField::Condition, ItemFieldValue::Text(v)) => item.condition = v,
        (ItemField::Secret, ItemFieldValue::Text(v)) => item.secret = v,
        (ItemField::StoryHooks, ItemFieldValue::Texts(v)) => item.story_hooks = v,
        _ => {
            return Err(error(
                ValidationCode::InvalidEntity,
                "item.field",
                "field value has the wrong scalar/list/profile shape",
            ));
        }
    }
    Ok(())
}

fn candidate_values(item: &Item) -> ValidationResult<FieldValues<ItemFieldValue>> {
    let mut values = FieldValues::new();
    for (field, value) in [
        (
            ItemField::Profile,
            ItemFieldValue::Profile(item.item_profile),
        ),
        (
            ItemField::Name,
            ItemFieldValue::Text(item.envelope.name.clone()),
        ),
        (
            ItemField::Appearance,
            ItemFieldValue::Text(item.appearance.clone()),
        ),
        (
            ItemField::Purpose,
            ItemFieldValue::Text(item.purpose.clone()),
        ),
        (
            ItemField::Properties,
            ItemFieldValue::Text(item.properties.clone()),
        ),
        (ItemField::Origin, ItemFieldValue::Text(item.origin.clone())),
        (
            ItemField::History,
            ItemFieldValue::Text(item.history.clone()),
        ),
        (
            ItemField::PowerOrFunction,
            ItemFieldValue::Text(item.power_or_function.clone()),
        ),
        (
            ItemField::CostOrDrawback,
            ItemFieldValue::Text(item.cost_or_drawback.clone()),
        ),
        (ItemField::Rarity, ItemFieldValue::Text(item.rarity.clone())),
        (
            ItemField::Condition,
            ItemFieldValue::Text(item.condition.clone()),
        ),
        (ItemField::Secret, ItemFieldValue::Text(item.secret.clone())),
        (
            ItemField::StoryHooks,
            ItemFieldValue::Texts(item.story_hooks.clone()),
        ),
    ] {
        values.insert(field.key()?, value);
    }
    Ok(values)
}

fn reroll(
    candidate: &mut ItemCandidate,
    requested: Option<ItemField>,
) -> ValidationResult<Vec<ItemField>> {
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
        if *origin == ItemOrigin::UserModified
            && requested.is_none_or(|field| field.key().is_ok_and(|k| k != *key))
        {
            dna.recipe
                .field_controls
                .get_mut(key)
                .expect("validated field control")
                .locked = true;
        }
    }
    let mut values = candidate_values(&next.item)?;
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
        let field = ItemField::from_key(key).ok_or_else(|| {
            error(
                ValidationCode::UnknownField,
                "item.field",
                "unknown Item field key",
            )
        })?;
        let value = values.get(key).cloned().ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "item.field",
                "rerolled field value is missing",
            )
        })?;
        set_field(&mut next.item, field, value)?;
        next.field_origins
            .insert(key.clone(), ItemOrigin::Generated);
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
        .filter_map(|k| ItemField::from_key(&k))
        .collect())
}

fn generate_staged(
    dna: &ForgeDNA,
    key: &FieldKey,
    stream: &mut FieldStream,
    staged: &mut FieldValues<ItemFieldValue>,
) -> ValidationResult<ItemFieldValue> {
    let field = ItemField::from_key(key).ok_or_else(|| {
        error(
            ValidationCode::UnknownField,
            "item.field",
            "unknown Item field key",
        )
    })?;
    let source = chaos_source_for(dna, field)?;
    let profile = staged.get(&ItemField::Profile.key()?).and_then(|v| {
        if let ItemFieldValue::Profile(v) = v {
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
    candidate: &mut ItemCandidate,
    field: ItemField,
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
                "item.field",
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
    candidate: &mut ItemCandidate,
    action: GenerationAction,
) -> ValidationResult<()> {
    let mut dna = candidate.dna.clone();
    dna.normalize()?;
    candidate.dna = dna.clone();
    candidate.item.envelope.generation_provenance = Some(GenerationMetadata {
        generation_id: EntityId::from_bytes(deterministic_id(
            "character-forge/item/generation-id/v1",
            &[
                &dna.canonical_bytes()?,
                candidate.item.envelope.id.uuid().as_bytes(),
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
        ItemForge::new_dna(RootSeed::new(seed), preset, CreativeAxes::neutral()).unwrap()
    }

    fn content(candidate: &ItemCandidate) -> String {
        serde_json::to_string(&(
            &candidate.item.envelope.name,
            candidate.item.item_profile,
            &candidate.item.appearance,
            &candidate.item.purpose,
            &candidate.item.properties,
            &candidate.item.origin,
            &candidate.item.history,
            &candidate.item.power_or_function,
            &candidate.item.cost_or_drawback,
            &candidate.item.rarity,
            &candidate.item.condition,
            &candidate.item.secret,
            &candidate.item.story_hooks,
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
                let first = ItemForge::generate(&source).unwrap();
                let replay = ItemForge::generate(&source).unwrap();
                first.validate().unwrap();
                assert_eq!(first, replay, "replay changed for {preset} seed {seed}");
                assert_eq!(source, original, "generation mutated ForgeDNA");
                assert_eq!(first.item.story_hooks.len(), 3);
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
            let candidate = ItemForge::generate(&generation_dna).unwrap();
            candidate.validate().unwrap();
            assert_eq!(candidate, ItemForge::generate(&generation_dna).unwrap());
            for field in ITEM_FIELD_ORDER {
                sources[source_group(field) as usize]
                    .insert(chaos_source_for(&generation_dna, field).unwrap());
            }
            let pattern = [ItemField::Name, ItemField::Origin, ItemField::Secret]
                .map(|field| chaos_source_for(&generation_dna, field).unwrap());
            mixed += usize::from(pattern.iter().collect::<BTreeSet<_>>().len() > 1);
            patterns.insert(pattern);
            for (group, source) in pattern.into_iter().enumerate() {
                *source_counts[group].entry(source).or_insert(0) += 1;
            }
            *profile_counts
                .entry(candidate.item.item_profile)
                .or_insert(0) += 1;
            profiles.insert(candidate.item.item_profile);
            assert!(
                contents.insert(content(&candidate)),
                "Chaos clone at {seed}"
            );
        }
        assert!(sources.iter().all(|group| group.len() == 10));
        assert_eq!(profiles.len(), ItemProfile::ALL.len());
        assert!(patterns.len() > 500);
        assert!(mixed > 950);
        assert!(
            source_counts
                .iter()
                .all(|counts| counts.values().all(|count| *count < 160))
        );
        println!(
            "Item Chaos: profile counts {profile_counts:?}; source counts {source_counts:?}; patterns {}; mixed {mixed}/1024; exact duplicates 0",
            patterns.len()
        );
    }

    #[test]
    fn action_keys_locks_manual_edit_and_reroll_are_typed() {
        assert_eq!(ItemField::from_action_key("name"), Some(ItemField::Name));
        assert_eq!(
            ItemField::from_action_key("item_profile"),
            Some(ItemField::Profile)
        );
        let mut candidate = ItemForge::generate(&dna(9, "nordic-myth")).unwrap();
        ItemForge::lock(&mut candidate, ItemField::Name).unwrap();
        ItemForge::lock(&mut candidate, ItemField::Profile).unwrap();
        let name = candidate.item.envelope.name.clone();
        let profile = candidate.item.item_profile;
        let name_control = candidate.dna.recipe.field_controls[&ItemField::Name.key().unwrap()];
        let profile_control =
            candidate.dna.recipe.field_controls[&ItemField::Profile.key().unwrap()];
        ItemForge::reroll_all_unlocked(&mut candidate).unwrap();
        assert_eq!(candidate.item.envelope.name, name);
        assert_eq!(candidate.item.item_profile, profile);
        assert_eq!(
            candidate.dna.recipe.field_controls[&ItemField::Name.key().unwrap()],
            name_control
        );
        assert_eq!(
            candidate.dna.recipe.field_controls[&ItemField::Profile.key().unwrap()],
            profile_control
        );
        ItemForge::manual_edit(
            &mut candidate,
            ItemField::Appearance,
            ItemFieldValue::Text("The æther-keepers of Åland".into()),
        )
        .unwrap();
        assert_eq!(
            candidate.field_origins[&ItemField::Appearance.key().unwrap()],
            ItemOrigin::UserModified
        );
        ItemForge::reroll_field(&mut candidate, ItemField::Appearance).unwrap();
        assert_eq!(
            candidate.field_origins[&ItemField::Appearance.key().unwrap()],
            ItemOrigin::Generated
        );
    }

    #[test]
    fn profile_serialization_and_schema_boundaries_are_stable() {
        assert_eq!(
            serde_json::to_string(&ItemProfile::KeyItem).unwrap(),
            "\"key-item\""
        );
        assert_eq!(
            serde_json::to_string(&GenerationKind::Item).unwrap(),
            "\"item\""
        );
        let mut invalid_version = dna(1, "pirate");
        invalid_version.generator_version = GeneratorVersion(2);
        assert!(ItemForge::generate(&invalid_version).is_err());
        assert!(ItemForge::new_dna(RootSeed::new(1), "unknown", CreativeAxes::neutral()).is_err());
    }

    #[test]
    fn creative_axis_extremes_are_valid_and_can_influence_output() {
        let neutral = ItemForge::generate(&dna(74, "dark-fantasy")).unwrap();
        let mut mythic_low = CreativeAxes::neutral();
        mythic_low.levels.insert(CreativeAxis::Mythic, 0);
        let mut mythic_high = CreativeAxes::neutral();
        mythic_high.levels.insert(CreativeAxis::Mythic, 100);
        let low = ItemForge::generate(
            &ItemForge::new_dna(RootSeed::new(74), "dark-fantasy", mythic_low).unwrap(),
        )
        .unwrap();
        let high = ItemForge::generate(
            &ItemForge::new_dna(RootSeed::new(74), "dark-fantasy", mythic_high).unwrap(),
        )
        .unwrap();
        neutral.validate().unwrap();
        low.validate().unwrap();
        high.validate().unwrap();
        assert_ne!(low.item.power_or_function, high.item.power_or_function);
    }

    #[test]
    fn literal_field_keys_and_profile_dependency_firewall_are_stable() {
        let expected = [
            "01-item-profile",
            "02-name",
            "03-appearance",
            "04-purpose",
            "05-properties",
            "06-origin",
            "07-history",
            "08-power-or-function",
            "09-cost-or-drawback",
            "10-rarity",
            "11-condition",
            "12-secret",
            "13-story-hooks",
        ];
        assert_eq!(
            ItemField::ALL.map(|f| f.key().unwrap().to_string()),
            expected
        );
        let mut candidate = ItemForge::generate(&dna(308, "pirate")).unwrap();
        ItemForge::lock(&mut candidate, ItemField::Appearance).unwrap();
        let before = candidate.clone();
        let changed = ItemForge::reroll_field(&mut candidate, ItemField::Profile).unwrap();
        assert_eq!(
            changed,
            vec![
                ItemField::Profile,
                ItemField::Purpose,
                ItemField::Properties,
                ItemField::PowerOrFunction,
                ItemField::CostOrDrawback
            ]
        );
        assert_eq!(candidate.item.appearance, before.item.appearance);
        for field in ItemField::ALL {
            let key = field.key().unwrap();
            assert_eq!(
                candidate.dna.recipe.field_controls[&key].nonce.0,
                before.dna.recipe.field_controls[&key].nonce.0
                    + u64::from(changed.contains(&field))
            );
        }
        assert_eq!(candidate.item.envelope.id, before.item.envelope.id);
        assert_eq!(candidate.item.secret, before.item.secret);
        let before = candidate.clone();
        let changed = ItemForge::reroll_all_unlocked(&mut candidate).unwrap();
        assert_eq!(changed.len(), 12);
        for field in ItemField::ALL {
            let key = field.key().unwrap();
            assert_eq!(
                candidate.dna.recipe.field_controls[&key].nonce.0,
                before.dna.recipe.field_controls[&key].nonce.0
                    + u64::from(field != ItemField::Appearance)
            );
        }
        let mut cycle = dna(1, "pirate");
        cycle.recipe.dependencies.insert(
            ItemField::Profile.key().unwrap(),
            BTreeSet::from([ItemField::Appearance.key().unwrap()]),
        );
        assert!(ItemForge::generate(&cycle).is_err());
    }

    #[test]
    fn invalid_shapes_versions_and_manual_edits_are_atomic() {
        let candidate = ItemForge::generate(&dna(81, "occult")).unwrap();
        let mut edited = candidate.clone();
        assert!(
            edited
                .manual_edit(ItemField::Profile, ItemFieldValue::Text("weapon".into()))
                .is_err()
        );
        assert!(
            edited
                .manual_edit(ItemField::Secret, ItemFieldValue::Text("  ".into()))
                .is_err()
        );
        assert!(
            edited
                .manual_edit(ItemField::StoryHooks, ItemFieldValue::Texts(vec![]))
                .is_err()
        );
        assert_eq!(candidate, edited);
        edited
            .manual_edit(
                ItemField::Origin,
                ItemFieldValue::Text("Forged near Åland — 東京 🜁".into()),
            )
            .unwrap();
        let nonce = edited.dna.recipe.field_controls[&ItemField::Origin.key().unwrap()].nonce;
        ItemForge::reroll_all_unlocked(&mut edited).unwrap();
        assert_eq!(edited.item.origin, "Forged near Åland — 東京 🜁");
        assert_eq!(
            edited.dna.recipe.field_controls[&ItemField::Origin.key().unwrap()].nonce,
            nonce
        );
        assert_eq!(
            edited.field_origins[&ItemField::Origin.key().unwrap()],
            ItemOrigin::UserModified
        );
        assert_eq!(
            serde_json::from_str::<ItemCandidate>(&serde_json::to_string(&edited).unwrap())
                .unwrap(),
            edited
        );
        let mut unsupported = dna(1, "pirate");
        unsupported.content_version = ContentVersion(2);
        assert!(ItemForge::generate(&unsupported).is_err());
        let mut bad = candidate.clone();
        bad.field_origins.clear();
        assert!(bad.validate().is_err());
        let mut bad = candidate.clone();
        bad.dna
            .recipe
            .field_controls
            .get_mut(&ItemField::Name.key().unwrap())
            .unwrap()
            .locked = true;
        assert!(bad.validate().is_err());
    }

    #[test]
    fn all_profiles_and_axes_produce_valid_distinct_structures() {
        for preset in &PRESETS[..10] {
            let theme = item_content::theme(preset).unwrap();
            for choices in [
                theme.names,
                theme.surfaces,
                theme.uses,
                theme.traits,
                theme.origins,
                theme.histories,
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
        for profile in ItemProfile::ALL {
            let source = item_content::profile(profile);
            assert!(forms.insert(source.forms[0]));
            for choices in [
                source.forms,
                source.purposes,
                source.properties,
                source.functions,
                source.drawbacks,
            ] {
                assert_eq!(choices.into_iter().collect::<BTreeSet<_>>().len(), 6);
            }
        }
        for axis in CreativeAxis::ALL {
            for level in [0, 100] {
                let mut axes = CreativeAxes::neutral();
                axes.levels.insert(axis, level);
                for preset in PRESETS {
                    ItemForge::generate(
                        &ItemForge::new_dna(RootSeed::new(999), preset, axes.clone()).unwrap(),
                    )
                    .unwrap()
                    .validate()
                    .unwrap();
                }
            }
        }
    }
}
