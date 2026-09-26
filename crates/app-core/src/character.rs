//! Deterministic Character Forge pipeline built on Mission 01 field streams.

use crate::character_content::{Choice, ContentCatalog, PresetContent, validate_choice_axes};
use crate::{FieldStream, FieldValues};
use forge_contracts::{
    CanonState, Character, ContentVersion, CreativeAxes, EntityEnvelope, EntityId, EntityOrigin,
    EntityType, FieldKey, ForgeDNA, GenerationAction, GenerationKind, GenerationMetadata,
    GenerationRecipe, PresetId, ProjectId, RootSeed, ValidationCode, ValidationError,
    ValidationErrors, ValidationResult,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

const CHARACTER_FIELD_ORDER: [CharacterField; 17] = [
    CharacterField::Archetype,
    CharacterField::Role,
    CharacterField::Name,
    CharacterField::Title,
    CharacterField::Appearance,
    CharacterField::Personality,
    CharacterField::Desire,
    CharacterField::Fear,
    CharacterField::Flaw,
    CharacterField::Secret,
    CharacterField::Skills,
    CharacterField::Abilities,
    CharacterField::FactionHint,
    CharacterField::SignatureGear,
    CharacterField::RelationshipHook,
    CharacterField::BackstoryHook,
    CharacterField::StoryHooks,
];
const CURRENT_CHARACTER_CONTENT_VERSION: ContentVersion = ContentVersion(2);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharacterField {
    Archetype,
    Role,
    Name,
    Title,
    Appearance,
    Personality,
    Desire,
    Fear,
    Flaw,
    Secret,
    Skills,
    Abilities,
    FactionHint,
    SignatureGear,
    RelationshipHook,
    BackstoryHook,
    StoryHooks,
}

impl CharacterField {
    pub fn key(self) -> ValidationResult<FieldKey> {
        let value = match self {
            Self::Archetype => "01-archetype",
            Self::Role => "02-role",
            Self::Name => "03-name",
            Self::Title => "04-title",
            Self::Appearance => "05-appearance",
            Self::Personality => "06-personality",
            Self::Desire => "07-desire",
            Self::Fear => "08-fear",
            Self::Flaw => "09-flaw",
            Self::Secret => "10-secret",
            Self::Skills => "11-skills",
            Self::Abilities => "12-abilities",
            Self::FactionHint => "13-faction-hint",
            Self::SignatureGear => "14-signature-gear",
            Self::RelationshipHook => "15-relationship-hook",
            Self::BackstoryHook => "16-backstory-hook",
            Self::StoryHooks => "17-story-hooks",
        };
        FieldKey::new(value)
    }

    fn from_key(key: &FieldKey) -> Option<Self> {
        CHARACTER_FIELD_ORDER
            .into_iter()
            .find(|field| field.key().is_ok_and(|candidate| candidate == *key))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum CharacterFieldValue {
    Text(String),
    Texts(Vec<String>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldValueOrigin {
    Generated,
    UserModified,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharacterCandidate {
    pub character: Character,
    pub dna: ForgeDNA,
    /// Records which current values were directly changed by the user.
    pub field_origins: BTreeMap<FieldKey, FieldValueOrigin>,
}

impl CharacterCandidate {
    pub fn validate(&self) -> ValidationResult<()> {
        self.character.validate()?;
        validate_character_dna(&self.dna)?;
        let expected: BTreeSet<_> = CHARACTER_FIELD_ORDER
            .into_iter()
            .map(CharacterField::key)
            .collect::<ValidationResult<_>>()?;
        let actual: BTreeSet<_> = self.field_origins.keys().cloned().collect();
        if expected != actual {
            return Err(error(
                ValidationCode::MissingField,
                "character_candidate.field_origins",
                "candidate must record the origin of every generated Character field",
            ));
        }
        let provenance = self
            .character
            .envelope
            .generation_provenance
            .as_ref()
            .ok_or_else(|| {
                ValidationErrors::one(ValidationError::new(
                    ValidationCode::MissingField,
                    "character_candidate.provenance",
                    "generated Character candidate must retain generation provenance",
                ))
            })?;
        if provenance.dna != self.dna {
            return Err(error(
                ValidationCode::InvalidEntity,
                "character_candidate.provenance.dna",
                "candidate DNA and Character provenance DNA must match",
            ));
        }
        Ok(())
    }

    pub fn manual_edit(
        &mut self,
        field: CharacterField,
        value: CharacterFieldValue,
    ) -> ValidationResult<()> {
        let mut updated = self.clone();
        set_character_field(&mut updated.character, field, value.clone())?;
        refresh_character_tags(&mut updated.character, &updated.dna)?;
        updated
            .field_origins
            .insert(field.key()?, FieldValueOrigin::UserModified);
        update_provenance(&mut updated, GenerationAction::ManualEdit, Some(value))?;
        updated.validate()?;
        *self = updated;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CharacterForge;

impl CharacterForge {
    /// Creates a complete canonical DNA recipe for one of the eleven v1 Character presets.
    pub fn new_dna(
        root_seed: RootSeed,
        preset: &str,
        creative_axes: CreativeAxes,
        constraints: forge_contracts::Constraints,
    ) -> ValidationResult<ForgeDNA> {
        let preset = PresetId::new(preset)?;
        let catalog = ContentCatalog::load(CURRENT_CHARACTER_CONTENT_VERSION)?;
        catalog.preset(&preset)?;
        let mut recipe = GenerationRecipe::new(GenerationKind::Character, root_seed, preset);
        recipe.creative_axes = creative_axes;
        recipe.constraints = constraints;
        for field in CHARACTER_FIELD_ORDER {
            recipe.add_field(field.key()?)?;
        }
        for field in CHARACTER_FIELD_ORDER {
            if field == CharacterField::Archetype {
                continue;
            }
            let mut dependencies = vec![CharacterField::Archetype.key()?];
            if matches!(
                field,
                CharacterField::RelationshipHook
                    | CharacterField::BackstoryHook
                    | CharacterField::StoryHooks
            ) {
                dependencies.push(CharacterField::Desire.key()?);
            }
            recipe.set_dependencies(field.key()?, dependencies)?;
        }
        let mut dna = ForgeDNA::new(recipe);
        dna.content_version = CURRENT_CHARACTER_CONTENT_VERSION;
        dna.normalize()?;
        validate_constraints(&dna.recipe.constraints)?;
        validate_character_dna(&dna)?;
        Ok(dna)
    }

    /// Generates a deterministic candidate without mutating the caller's ForgeDNA.
    pub fn generate(dna: &ForgeDNA) -> ValidationResult<CharacterCandidate> {
        Self::generate_in_project(dna, demo_project_id()?)
    }

    pub fn generate_in_project(
        dna: &ForgeDNA,
        project_id: ProjectId,
    ) -> ValidationResult<CharacterCandidate> {
        let mut canonical_dna = dna.clone();
        canonical_dna.normalize()?;
        validate_character_dna(&canonical_dna)?;
        project_id.validate("project_id")?;

        let catalog = ContentCatalog::load(canonical_dna.content_version)?;
        let preset = catalog.preset(&canonical_dna.recipe.universe_preset)?;
        let constraints = ConstraintSet::parse(&canonical_dna.recipe.constraints)?;
        let values = generate_all_fields(&canonical_dna, preset, &constraints)?;
        let character_id = EntityId::from_bytes(deterministic_id(
            "character-forge/entity-id/v1",
            &[
                &canonical_dna.canonical_bytes()?,
                project_id.uuid().as_bytes(),
            ],
        ));
        let mut character =
            character_from_values(values, canonical_dna.clone(), character_id, project_id)?;
        refresh_character_tags(&mut character, &canonical_dna)?;
        let generation_id = generation_id(&canonical_dna, None, GenerationAction::Forge, None)?;
        character.envelope.generation_provenance = Some(GenerationMetadata {
            generation_id,
            generated_at_unix_ms: 0,
            action: GenerationAction::Forge,
            parent_generation_id: None,
            dna: canonical_dna.clone(),
        });
        let field_origins = CHARACTER_FIELD_ORDER
            .into_iter()
            .map(|field| Ok((field.key()?, FieldValueOrigin::Generated)))
            .collect::<ValidationResult<_>>()?;
        let candidate = CharacterCandidate {
            character,
            dna: canonical_dna,
            field_origins,
        };
        candidate.validate()?;
        Ok(candidate)
    }

    pub fn lock(candidate: &mut CharacterCandidate, field: CharacterField) -> ValidationResult<()> {
        change_lock(candidate, field, true)
    }

    pub fn unlock(
        candidate: &mut CharacterCandidate,
        field: CharacterField,
    ) -> ValidationResult<()> {
        change_lock(candidate, field, false)
    }

    pub fn reroll_field(
        candidate: &mut CharacterCandidate,
        field: CharacterField,
    ) -> ValidationResult<Vec<CharacterField>> {
        let mut updated = candidate.clone();
        updated.validate()?;
        let catalog = ContentCatalog::load(updated.dna.content_version)?;
        let preset = catalog.preset(&updated.dna.recipe.universe_preset)?;
        let constraints = ConstraintSet::parse(&updated.dna.recipe.constraints)?;
        let mut dna = updated.dna.clone();
        let prior_locks: BTreeMap<_, _> = dna
            .recipe
            .field_controls
            .iter()
            .map(|(key, control)| (key.clone(), control.locked))
            .collect();
        for (key, origin) in &updated.field_origins {
            if *origin == FieldValueOrigin::UserModified
                && key != &field.key()?
                && let Some(control) = dna.recipe.field_controls.get_mut(key)
            {
                control.locked = true;
            }
        }
        let mut values = character_to_values(&updated.character)?;
        let mut staged = values.clone();
        let axes = dna.recipe.creative_axes.clone();
        let changed = crate::reroll_field(&mut dna, &mut values, &field.key()?, |key, stream| {
            let field = CharacterField::from_key(key).ok_or_else(|| {
                error(
                    ValidationCode::UnknownField,
                    format!("field_controls.{key}"),
                    "unknown Character field key",
                )
            })?;
            let value = generate_field(field, &staged, preset, &constraints, &axes, stream)?;
            staged.insert(key.clone(), value.clone());
            Ok(value)
        })?;
        restore_locks(&mut dna, &prior_locks);
        apply_changed_values(&mut updated.character, &values, &changed.changed_fields)?;
        updated.dna = dna;
        for changed_key in &changed.changed_fields {
            updated
                .field_origins
                .insert(changed_key.clone(), FieldValueOrigin::Generated);
        }
        refresh_character_tags(&mut updated.character, &updated.dna)?;
        update_provenance(&mut updated, GenerationAction::RerollField, None)?;
        let changed_fields = changed
            .changed_fields
            .into_iter()
            .filter_map(|key| CharacterField::from_key(&key))
            .collect();
        updated.validate()?;
        *candidate = updated;
        Ok(changed_fields)
    }

    pub fn reroll_all_unlocked(
        candidate: &mut CharacterCandidate,
    ) -> ValidationResult<Vec<CharacterField>> {
        let mut updated = candidate.clone();
        updated.validate()?;
        let catalog = ContentCatalog::load(updated.dna.content_version)?;
        let preset = catalog.preset(&updated.dna.recipe.universe_preset)?;
        let constraints = ConstraintSet::parse(&updated.dna.recipe.constraints)?;
        let mut dna = updated.dna.clone();
        let prior_locks: BTreeMap<_, _> = dna
            .recipe
            .field_controls
            .iter()
            .map(|(key, control)| (key.clone(), control.locked))
            .collect();
        for (key, origin) in &updated.field_origins {
            if *origin == FieldValueOrigin::UserModified
                && let Some(control) = dna.recipe.field_controls.get_mut(key)
            {
                control.locked = true;
            }
        }
        let mut values = character_to_values(&updated.character)?;
        let mut staged = values.clone();
        let axes = dna.recipe.creative_axes.clone();
        let changed = crate::reroll_all_unlocked(&mut dna, &mut values, |key, stream| {
            let field = CharacterField::from_key(key).ok_or_else(|| {
                error(
                    ValidationCode::UnknownField,
                    format!("field_controls.{key}"),
                    "unknown Character field key",
                )
            })?;
            let value = generate_field(field, &staged, preset, &constraints, &axes, stream)?;
            staged.insert(key.clone(), value.clone());
            Ok(value)
        })?;
        restore_locks(&mut dna, &prior_locks);
        apply_changed_values(&mut updated.character, &values, &changed.changed_fields)?;
        updated.dna = dna;
        for changed_key in &changed.changed_fields {
            updated
                .field_origins
                .insert(changed_key.clone(), FieldValueOrigin::Generated);
        }
        refresh_character_tags(&mut updated.character, &updated.dna)?;
        update_provenance(&mut updated, GenerationAction::RerollUnlocked, None)?;
        let changed_fields = changed
            .changed_fields
            .into_iter()
            .filter_map(|key| CharacterField::from_key(&key))
            .collect();
        updated.validate()?;
        *candidate = updated;
        Ok(changed_fields)
    }
}

#[derive(Default)]
struct ConstraintSet {
    must_be_non_human: bool,
    no_magic: bool,
    no_firearms: bool,
    urban_only: bool,
}

impl ConstraintSet {
    fn parse(constraints: &forge_contracts::Constraints) -> ValidationResult<Self> {
        validate_constraints(constraints)?;
        let mut result = Self::default();
        for (key, value) in constraints {
            match key.as_str() {
                "must_be_non_human" => result.must_be_non_human = parse_bool(key.as_str(), value)?,
                "no_magic" => result.no_magic = parse_bool(key.as_str(), value)?,
                "no_firearms" => result.no_firearms = parse_bool(key.as_str(), value)?,
                "urban_only" => result.urban_only = parse_bool(key.as_str(), value)?,
                "inspiration" => {}
                _ => {
                    return Err(error(
                        ValidationCode::InvalidConstraint,
                        format!("constraints.{key}"),
                        "unsupported Character constraint",
                    ));
                }
            }
        }
        Ok(result)
    }
}

fn validate_constraints(constraints: &forge_contracts::Constraints) -> ValidationResult<()> {
    for (key, value) in constraints {
        match key.as_str() {
            "must_be_non_human" | "no_magic" | "no_firearms" | "urban_only" => {
                parse_bool(key.as_str(), value)?;
            }
            "inspiration" if !value.trim().is_empty() && value.chars().count() <= 240 => {}
            "inspiration" => {
                return Err(error(
                    ValidationCode::InvalidConstraint,
                    format!("constraints.{key}"),
                    "inspiration must contain 1..=240 Unicode characters",
                ));
            }
            _ => {
                return Err(error(
                    ValidationCode::InvalidConstraint,
                    format!("constraints.{key}"),
                    "unsupported Character constraint; use must_be_non_human, no_magic, no_firearms, urban_only, or inspiration",
                ));
            }
        }
    }
    Ok(())
}

fn parse_bool(key: &str, value: &str) -> ValidationResult<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(error(
            ValidationCode::InvalidConstraint,
            format!("constraints.{key}"),
            "boolean Character constraints must be exactly true or false",
        )),
    }
}

fn validate_character_dna(dna: &ForgeDNA) -> ValidationResult<()> {
    dna.validate()?;
    if dna.recipe.kind != GenerationKind::Character {
        return Err(error(
            ValidationCode::InvalidEntity,
            "recipe.kind",
            "Character Forge requires a Character generation recipe",
        ));
    }
    let expected_keys: BTreeSet<_> = CHARACTER_FIELD_ORDER
        .into_iter()
        .map(CharacterField::key)
        .collect::<ValidationResult<_>>()?;
    let actual_keys: BTreeSet<_> = dna.recipe.field_controls.keys().cloned().collect();
    if expected_keys != actual_keys {
        return Err(error(
            ValidationCode::InvalidDependency,
            "recipe.field_controls",
            "Character ForgeDNA must register exactly the canonical Character fields",
        ));
    }
    let archetype_key = CharacterField::Archetype.key()?;
    let desire_key = CharacterField::Desire.key()?;
    let mut expected_dependencies = BTreeMap::new();
    for field in CHARACTER_FIELD_ORDER {
        if field == CharacterField::Archetype {
            continue;
        }
        let mut dependencies = BTreeSet::from([archetype_key.clone()]);
        if matches!(
            field,
            CharacterField::RelationshipHook
                | CharacterField::BackstoryHook
                | CharacterField::StoryHooks
        ) {
            dependencies.insert(desire_key.clone());
        }
        expected_dependencies.insert(field.key()?, dependencies);
    }
    if dna.recipe.dependencies != expected_dependencies {
        return Err(error(
            ValidationCode::InvalidDependency,
            "recipe.dependencies",
            "Character field dependency graph does not match the canonical Mission 02 graph",
        ));
    }
    validate_constraints(&dna.recipe.constraints)
}

fn generate_all_fields(
    dna: &ForgeDNA,
    preset: &PresetContent,
    constraints: &ConstraintSet,
) -> ValidationResult<FieldValues<CharacterFieldValue>> {
    let mut values = FieldValues::new();
    for field in CHARACTER_FIELD_ORDER {
        let key = field.key()?;
        let mut stream = FieldStream::derive(dna, &key)?;
        let value = generate_field(
            field,
            &values,
            preset,
            constraints,
            &dna.recipe.creative_axes,
            &mut stream,
        )?;
        values.insert(key, value);
    }
    Ok(values)
}

fn generate_field(
    field: CharacterField,
    current: &FieldValues<CharacterFieldValue>,
    preset: &PresetContent,
    constraints: &ConstraintSet,
    axes: &CreativeAxes,
    stream: &mut FieldStream,
) -> ValidationResult<CharacterFieldValue> {
    match field {
        CharacterField::Archetype => {
            choose(&preset.archetypes, axes, stream, "archetypes", |choice| {
                (!constraints.must_be_non_human || choice.tags.iter().any(|tag| tag == "non_human"))
                    && (!constraints.urban_only || choice.tags.iter().any(|tag| tag == "urban"))
            })
            .map(|choice| CharacterFieldValue::Text(choice.value.clone()))
        }
        CharacterField::Role => choose_text(&preset.roles, axes, stream, "roles"),
        CharacterField::Name => {
            let given = choose(&preset.given_names, axes, stream, "given_names", |_| true)?;
            let family = choose(&preset.family_names, axes, stream, "family_names", |_| true)?;
            Ok(CharacterFieldValue::Text(format!(
                "{} {}",
                given.value, family.value
            )))
        }
        CharacterField::Title => choose_text(&preset.titles, axes, stream, "titles"),
        CharacterField::Appearance => choose_text(&preset.appearances, axes, stream, "appearances"),
        CharacterField::Personality => {
            choose_texts(&preset.personalities, axes, stream, "personalities", 2)
        }
        CharacterField::Desire => choose_text(&preset.desires, axes, stream, "desires"),
        CharacterField::Fear => choose_text(&preset.fears, axes, stream, "fears"),
        CharacterField::Flaw => choose_text(&preset.flaws, axes, stream, "flaws"),
        CharacterField::Secret => choose_text(&preset.secrets, axes, stream, "secrets"),
        CharacterField::Skills => choose_texts(&preset.skills, axes, stream, "skills", 2),
        CharacterField::Abilities => {
            let choices = choose(&preset.abilities, axes, stream, "abilities", |choice| {
                !constraints.no_magic || !choice.tags.iter().any(|tag| tag == "magic")
            })?;
            Ok(CharacterFieldValue::Texts(vec![choices.value.clone()]))
        }
        CharacterField::FactionHint => choose_text(&preset.factions, axes, stream, "factions"),
        CharacterField::SignatureGear => {
            let choice = choose(&preset.gear, axes, stream, "gear", |choice| {
                !constraints.no_firearms || !choice.tags.iter().any(|tag| tag == "firearm")
            })?;
            Ok(CharacterFieldValue::Text(choice.value.clone()))
        }
        CharacterField::RelationshipHook => {
            let _desire = text_value(current, CharacterField::Desire)?;
            choose_text(&preset.relationships, axes, stream, "relationships")
        }
        CharacterField::BackstoryHook => {
            let _desire = text_value(current, CharacterField::Desire)?;
            choose_text(&preset.backstories, axes, stream, "backstories")
        }
        CharacterField::StoryHooks => {
            let _desire = text_value(current, CharacterField::Desire)?;
            choose_texts(&preset.hooks, axes, stream, "hooks", 3)
        }
    }
}

fn choose_text(
    choices: &[Choice],
    axes: &CreativeAxes,
    stream: &mut FieldStream,
    path: &str,
) -> ValidationResult<CharacterFieldValue> {
    choose(choices, axes, stream, path, |_| true)
        .map(|choice| CharacterFieldValue::Text(choice.value.clone()))
}

fn choose_texts(
    choices: &[Choice],
    axes: &CreativeAxes,
    stream: &mut FieldStream,
    path: &str,
    count: usize,
) -> ValidationResult<CharacterFieldValue> {
    if choices.len() < count {
        return Err(error(
            ValidationCode::MissingField,
            format!("character_content.{path}"),
            "content family has too few distinct choices",
        ));
    }
    let mut remaining = choices.to_vec();
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let choice = choose(&remaining, axes, stream, path, |_| true)?.clone();
        let index = remaining
            .iter()
            .position(|candidate| candidate.value == choice.value)
            .ok_or_else(|| {
                error(
                    ValidationCode::MissingField,
                    format!("character_content.{path}"),
                    "selected choice disappeared from its semantic family",
                )
            })?;
        remaining.remove(index);
        values.push(choice.value);
    }
    Ok(CharacterFieldValue::Texts(values))
}

fn choose<'a, F>(
    choices: &'a [Choice],
    axes: &CreativeAxes,
    stream: &mut FieldStream,
    path: &str,
    include: F,
) -> ValidationResult<&'a Choice>
where
    F: Fn(&Choice) -> bool,
{
    let weighted: Vec<_> = choices
        .iter()
        .filter(|choice| include(choice))
        .map(|choice| (choice, validate_choice_axes(axes, choice) as u64))
        .collect();
    let total = weighted
        .iter()
        .try_fold(0_u64, |total, (_, weight)| total.checked_add(*weight));
    let total = total.ok_or_else(|| {
        error(
            ValidationCode::InvalidRange,
            format!("character_content.{path}"),
            "weighted content total exceeds the supported deterministic range",
        )
    })?;
    if total == 0 {
        return Err(error(
            ValidationCode::InvalidConstraint,
            format!("character_content.{path}"),
            "constraints leave no compatible content choices",
        ));
    }
    let mut roll = stream.next_bounded(total)?;
    for (choice, weight) in weighted {
        if roll < weight {
            return Ok(choice);
        }
        roll -= weight;
    }
    Err(error(
        ValidationCode::InvalidRange,
        format!("character_content.{path}"),
        "weighted selection exceeded its validated range",
    ))
}

fn text_value(
    values: &FieldValues<CharacterFieldValue>,
    field: CharacterField,
) -> ValidationResult<&str> {
    match values.get(&field.key()?) {
        Some(CharacterFieldValue::Text(value)) => Ok(value),
        _ => Err(error(
            ValidationCode::MissingField,
            format!("character.{}", field_key_name(field)),
            "required Character context field is missing or has the wrong value type",
        )),
    }
}

fn character_from_values(
    values: FieldValues<CharacterFieldValue>,
    dna: ForgeDNA,
    id: EntityId,
    project_id: ProjectId,
) -> ValidationResult<Character> {
    let mut character = Character {
        envelope: EntityEnvelope {
            id,
            project_id,
            entity_type: EntityType::Character,
            name: String::new(),
            schema_version: dna.schema_version,
            canon_state: CanonState::Candidate,
            origin: EntityOrigin::Generated,
            favorite: false,
            created_at_unix_ms: 0,
            updated_at_unix_ms: 0,
            tags: vec![format!("preset:{}", dna.recipe.universe_preset)],
            notes: None,
            generation_provenance: None,
        },
        aliases: Vec::new(),
        title: String::new(),
        archetype: String::new(),
        role: String::new(),
        appearance: String::new(),
        faction_hint: String::new(),
        signature_gear: String::new(),
        personality: Vec::new(),
        desire: String::new(),
        fear: String::new(),
        flaw: String::new(),
        secret: String::new(),
        skills: Vec::new(),
        abilities: Vec::new(),
        gear_links: Vec::new(),
        faction_links: Vec::new(),
        relationship_links: Vec::new(),
        relationship_hook: String::new(),
        backstory_hook: String::new(),
        story_hooks: Vec::new(),
    };
    for field in CHARACTER_FIELD_ORDER {
        let value = values.get(&field.key()?).cloned().ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                format!("character.{}", field_key_name(field)),
                "generated Character field is missing",
            )
        })?;
        set_character_field(&mut character, field, value)?;
    }
    Ok(character)
}

fn character_to_values(
    character: &Character,
) -> ValidationResult<FieldValues<CharacterFieldValue>> {
    let mut values = FieldValues::new();
    for field in CHARACTER_FIELD_ORDER {
        let value = match field {
            CharacterField::Archetype => CharacterFieldValue::Text(character.archetype.clone()),
            CharacterField::Role => CharacterFieldValue::Text(character.role.clone()),
            CharacterField::Name => CharacterFieldValue::Text(character.envelope.name.clone()),
            CharacterField::Title => CharacterFieldValue::Text(character.title.clone()),
            CharacterField::Appearance => CharacterFieldValue::Text(character.appearance.clone()),
            CharacterField::Personality => {
                CharacterFieldValue::Texts(character.personality.clone())
            }
            CharacterField::Desire => CharacterFieldValue::Text(character.desire.clone()),
            CharacterField::Fear => CharacterFieldValue::Text(character.fear.clone()),
            CharacterField::Flaw => CharacterFieldValue::Text(character.flaw.clone()),
            CharacterField::Secret => CharacterFieldValue::Text(character.secret.clone()),
            CharacterField::Skills => CharacterFieldValue::Texts(character.skills.clone()),
            CharacterField::Abilities => CharacterFieldValue::Texts(character.abilities.clone()),
            CharacterField::FactionHint => {
                CharacterFieldValue::Text(character.faction_hint.clone())
            }
            CharacterField::SignatureGear => {
                CharacterFieldValue::Text(character.signature_gear.clone())
            }
            CharacterField::RelationshipHook => {
                CharacterFieldValue::Text(character.relationship_hook.clone())
            }
            CharacterField::BackstoryHook => {
                CharacterFieldValue::Text(character.backstory_hook.clone())
            }
            CharacterField::StoryHooks => CharacterFieldValue::Texts(character.story_hooks.clone()),
        };
        values.insert(field.key()?, value);
    }
    Ok(values)
}

fn set_character_field(
    character: &mut Character,
    field: CharacterField,
    value: CharacterFieldValue,
) -> ValidationResult<()> {
    let type_error = || {
        error(
            ValidationCode::InvalidEntity,
            format!("character.{}", field_key_name(field)),
            "field value has the wrong scalar/list shape",
        )
    };
    match (field, value) {
        (CharacterField::Archetype, CharacterFieldValue::Text(value)) => {
            character.archetype = value
        }
        (CharacterField::Role, CharacterFieldValue::Text(value)) => character.role = value,
        (CharacterField::Name, CharacterFieldValue::Text(value)) => character.envelope.name = value,
        (CharacterField::Title, CharacterFieldValue::Text(value)) => character.title = value,
        (CharacterField::Appearance, CharacterFieldValue::Text(value)) => {
            character.appearance = value
        }
        (CharacterField::Personality, CharacterFieldValue::Texts(value)) => {
            character.personality = value
        }
        (CharacterField::Desire, CharacterFieldValue::Text(value)) => character.desire = value,
        (CharacterField::Fear, CharacterFieldValue::Text(value)) => character.fear = value,
        (CharacterField::Flaw, CharacterFieldValue::Text(value)) => character.flaw = value,
        (CharacterField::Secret, CharacterFieldValue::Text(value)) => character.secret = value,
        (CharacterField::Skills, CharacterFieldValue::Texts(value)) => character.skills = value,
        (CharacterField::Abilities, CharacterFieldValue::Texts(value)) => {
            character.abilities = value
        }
        (CharacterField::FactionHint, CharacterFieldValue::Text(value)) => {
            character.faction_hint = value
        }
        (CharacterField::SignatureGear, CharacterFieldValue::Text(value)) => {
            character.signature_gear = value
        }
        (CharacterField::RelationshipHook, CharacterFieldValue::Text(value)) => {
            character.relationship_hook = value
        }
        (CharacterField::BackstoryHook, CharacterFieldValue::Text(value)) => {
            character.backstory_hook = value
        }
        (CharacterField::StoryHooks, CharacterFieldValue::Texts(value)) => {
            character.story_hooks = value
        }
        _ => return Err(type_error()),
    }
    Ok(())
}

fn apply_changed_values(
    character: &mut Character,
    values: &FieldValues<CharacterFieldValue>,
    changed: &[FieldKey],
) -> ValidationResult<()> {
    for key in changed {
        let field = CharacterField::from_key(key).ok_or_else(|| {
            error(
                ValidationCode::UnknownField,
                format!("field_values.{key}"),
                "unknown Character field key",
            )
        })?;
        let value = values.get(key).cloned().ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                format!("field_values.{key}"),
                "rerolled Character value is missing",
            )
        })?;
        set_character_field(character, field, value)?;
    }
    character.validate()
}

fn refresh_character_tags(character: &mut Character, dna: &ForgeDNA) -> ValidationResult<()> {
    let catalog = ContentCatalog::load(dna.content_version)?;
    let preset = catalog.preset(&dna.recipe.universe_preset)?;
    let mut tags = vec![format!("preset:{}", preset.id)];
    if let Some(choice) = preset
        .archetypes
        .iter()
        .find(|choice| choice.value == character.archetype)
    {
        if choice.tags.iter().any(|tag| tag == "non_human") {
            tags.push("non_human".into());
        }
        if choice.tags.iter().any(|tag| tag == "urban") {
            tags.push("urban".into());
        }
    }
    character.envelope.tags = tags;
    Ok(())
}

fn change_lock(
    candidate: &mut CharacterCandidate,
    field: CharacterField,
    locked: bool,
) -> ValidationResult<()> {
    let mut updated = candidate.clone();
    updated.validate()?;
    if locked {
        crate::lock_field(&mut updated.dna, &field.key()?)?;
    } else {
        crate::unlock_field(&mut updated.dna, &field.key()?)?;
    }
    sync_provenance_dna(&mut updated)?;
    updated.validate()?;
    *candidate = updated;
    Ok(())
}

fn restore_locks(dna: &mut ForgeDNA, prior_locks: &BTreeMap<FieldKey, bool>) {
    for (key, locked) in prior_locks {
        if let Some(control) = dna.recipe.field_controls.get_mut(key) {
            control.locked = *locked;
        }
    }
}

fn update_provenance(
    candidate: &mut CharacterCandidate,
    action: GenerationAction,
    edit_value: Option<CharacterFieldValue>,
) -> ValidationResult<()> {
    let old = generation_metadata(&candidate.character)?.clone();
    let parent = Some(old.generation_id);
    let generation_id = generation_id(&candidate.dna, parent, action, edit_value)?;
    candidate.character.envelope.generation_provenance = Some(GenerationMetadata {
        generation_id,
        generated_at_unix_ms: 0,
        action,
        parent_generation_id: parent,
        dna: candidate.dna.clone(),
    });
    Ok(())
}

fn sync_provenance_dna(candidate: &mut CharacterCandidate) -> ValidationResult<()> {
    let metadata = candidate
        .character
        .envelope
        .generation_provenance
        .as_mut()
        .ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "character.generation_provenance",
                "candidate provenance is missing",
            )
        })?;
    metadata.dna = candidate.dna.clone();
    Ok(())
}

fn generation_metadata(character: &Character) -> ValidationResult<&GenerationMetadata> {
    character
        .envelope
        .generation_provenance
        .as_ref()
        .ok_or_else(|| {
            error(
                ValidationCode::MissingField,
                "character.generation_provenance",
                "candidate provenance is missing",
            )
        })
}

fn generation_id(
    dna: &ForgeDNA,
    parent: Option<EntityId>,
    action: GenerationAction,
    extra: Option<CharacterFieldValue>,
) -> ValidationResult<EntityId> {
    let mut material = dna.canonical_bytes()?;
    if let Some(parent) = parent {
        material.extend_from_slice(parent.uuid().as_bytes());
    }
    material.push(match action {
        GenerationAction::Forge => 0,
        GenerationAction::RerollField => 1,
        GenerationAction::RerollUnlocked => 2,
        GenerationAction::ManualEdit => 3,
        GenerationAction::RestoreHistory => 4,
    });
    if let Some(extra) = extra {
        serde_json::to_writer(&mut material, &extra).map_err(|error| {
            ValidationErrors::one(ValidationError::new(
                ValidationCode::Serialization,
                "character.generation_provenance",
                format!("could not serialize generation identity input: {error}"),
            ))
        })?;
    }
    Ok(EntityId::from_bytes(deterministic_id(
        "character-forge/generation-id/v1",
        &[&material],
    )))
}

fn deterministic_id(context: &str, parts: &[&[u8]]) -> [u8; 16] {
    let mut hasher = blake3::Hasher::new_derive_key(context);
    for part in parts {
        hasher.update(&(part.len() as u64).to_le_bytes());
        hasher.update(part);
    }
    let hash = hasher.finalize();
    let mut bytes = [0; 16];
    bytes.copy_from_slice(&hash.as_bytes()[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x50;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    bytes
}

fn demo_project_id() -> ValidationResult<ProjectId> {
    Ok(ProjectId::from_bytes(deterministic_id(
        "character-forge/demo-project-id/v1",
        &[b"character-forge-engine-harness"],
    )))
}

fn field_key_name(field: CharacterField) -> &'static str {
    match field {
        CharacterField::Archetype => "archetype",
        CharacterField::Role => "role",
        CharacterField::Name => "name",
        CharacterField::Title => "title",
        CharacterField::Appearance => "appearance",
        CharacterField::Personality => "personality",
        CharacterField::Desire => "desire",
        CharacterField::Fear => "fear",
        CharacterField::Flaw => "flaw",
        CharacterField::Secret => "secret",
        CharacterField::Skills => "skills",
        CharacterField::Abilities => "abilities",
        CharacterField::FactionHint => "faction_hint",
        CharacterField::SignatureGear => "signature_gear",
        CharacterField::RelationshipHook => "relationship_hook",
        CharacterField::BackstoryHook => "backstory_hook",
        CharacterField::StoryHooks => "story_hooks",
    }
}

fn error(
    code: ValidationCode,
    path: impl Into<String>,
    message: impl Into<String>,
) -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(code, path, message))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character_content::CORE_PRESETS;
    use forge_contracts::{ConstraintKey, CreativeAxis, RootSeed};

    fn dna(seed: u64, preset: &str, axes: CreativeAxes) -> ForgeDNA {
        CharacterForge::new_dna(
            RootSeed::new(seed),
            preset,
            axes,
            forge_contracts::Constraints::new(),
        )
        .unwrap()
    }

    fn legacy_v1_dna(seed: u64, preset: &str) -> ForgeDNA {
        let mut dna = dna(seed, preset, CreativeAxes::neutral());
        dna.content_version = ContentVersion(1);
        dna.normalize().unwrap();
        dna
    }

    #[test]
    fn canonical_core_preset_list_matches_embedded_content_catalog() {
        let catalog = ContentCatalog::load(CURRENT_CHARACTER_CONTENT_VERSION).unwrap();
        let actual: Vec<_> = catalog.presets().map(|preset| preset.id.as_str()).collect();
        assert_eq!(actual.len(), CORE_PRESETS.len());
        for id in CORE_PRESETS {
            assert!(actual.contains(&id), "missing preset {id}");
        }
    }

    #[test]
    fn v2_semantic_choice_families_are_expanded_and_unique() {
        let v1 = ContentCatalog::load(ContentVersion(1)).unwrap();
        let v2 = ContentCatalog::load(ContentVersion(2)).unwrap();
        for id in CORE_PRESETS {
            let old = v1.preset(&PresetId::new(id).unwrap()).unwrap();
            let expanded = v2.preset(&PresetId::new(id).unwrap()).unwrap();
            assert!(expanded.roles.len() > old.roles.len(), "roles for {id}");
            assert!(
                expanded.personalities.len() > old.personalities.len(),
                "personalities for {id}"
            );
            assert!(expanded.hooks.len() > old.hooks.len(), "hooks for {id}");
            expanded.validate_unique_choices().unwrap();
        }
    }

    #[test]
    fn generated_candidates_are_reproducible_valid_and_do_not_mutate_dna() {
        let dna = dna(0x1234, "cyberpunk", CreativeAxes::neutral());
        let before = dna.canonical_bytes().unwrap();
        let first = CharacterForge::generate(&dna).unwrap();
        let second = CharacterForge::generate(&dna).unwrap();
        assert_eq!(first, second);
        assert_eq!(before, dna.canonical_bytes().unwrap());
        first.validate().unwrap();
        assert_eq!(first.character.story_hooks.len(), 3);
        assert_eq!(first.character.skills.len(), 2);
        assert_eq!(first.character.personality.len(), 2);
        assert_eq!(first.character.abilities.len(), 1);
        assert!(first.character.envelope.generation_provenance.is_some());
    }

    #[test]
    fn golden_seed_matrix_generates_valid_candidates_for_every_core_preset() {
        for (index, preset) in CORE_PRESETS.into_iter().enumerate() {
            let candidate =
                CharacterForge::generate(&dna(index as u64 + 1, preset, CreativeAxes::neutral()))
                    .unwrap_or_else(|error| panic!("{preset}: {error}"));
            candidate.validate().unwrap();
            assert_eq!(
                candidate.dna.content_version,
                CURRENT_CHARACTER_CONTENT_VERSION
            );
            assert!(
                candidate
                    .character
                    .envelope
                    .tags
                    .iter()
                    .any(|tag| tag == &format!("preset:{preset}"))
            );
        }
    }

    #[test]
    fn golden_seed_42_preserves_v1_core_preset_archetype_outputs() {
        let expected = [
            ("dark-fantasy", "Lantern Warden"),
            ("pirate", "Mutiny-Script Quartermaster"),
            ("steampunk", "Pressure-Ledger Inspector"),
            ("cyberpunk", "Civic Algorithm Auditor"),
            ("nordic-myth", "Boundary-Stone Keeper"),
            ("post-apocalyptic", "Water-Ration Mediator"),
            ("occult", "Missing-Persons Archivist"),
            ("monster-hunter", "Molt-Pattern Tracker"),
            ("rogue-assassin", "Ledger-Page Infiltrator"),
            ("contemporary-weird", "Municipal Oddity Clerk"),
            ("chaos-blend", "Tide-Court Signal Witch"),
        ];
        for (preset, archetype) in expected {
            let candidate = CharacterForge::generate(&legacy_v1_dna(42, preset)).unwrap();
            assert_eq!(
                candidate.character.archetype, archetype,
                "golden preset {preset}"
            );
        }
    }

    #[test]
    fn golden_seed_42_records_v2_core_preset_archetype_outputs() {
        let expected = [
            ("dark-fantasy", "Lantern Warden"),
            ("pirate", "Tide-Court Interpreter"),
            ("steampunk", "Pressure-Ledger Inspector"),
            ("cyberpunk", "Memory-Bank Salvager"),
            ("nordic-myth", "Oath-Weight Bearer"),
            ("post-apocalyptic", "Water-Ration Mediator"),
            ("occult", "Threshold Locksmith"),
            ("monster-hunter", "Molt-Pattern Tracker"),
            ("rogue-assassin", "Witness-Sparing Cutpurse"),
            ("contemporary-weird", "Substitute for an Unlisted Neighbor"),
            ("chaos-blend", "Tide-Court Signal Witch"),
        ];
        for (preset, archetype) in expected {
            let candidate =
                CharacterForge::generate(&dna(42, preset, CreativeAxes::neutral())).unwrap();
            assert_eq!(
                candidate.character.archetype, archetype,
                "golden preset {preset}"
            );
        }
    }

    #[test]
    fn content_v1_remains_generatable_from_its_frozen_corpus() {
        let candidate = CharacterForge::generate(&legacy_v1_dna(42, "pirate")).unwrap();
        assert_eq!(candidate.dna.content_version, ContentVersion(1));
        assert_eq!(candidate.character.archetype, "Mutiny-Script Quartermaster");
        assert!(candidate.validate().is_ok());
    }

    #[test]
    fn seed_changes_candidates_and_serialization_is_stable() {
        let one = CharacterForge::generate(&dna(1, "pirate", CreativeAxes::neutral())).unwrap();
        let two = CharacterForge::generate(&dna(2, "pirate", CreativeAxes::neutral())).unwrap();
        assert_ne!(one.character, two.character);
        assert_eq!(
            serde_json::to_vec(&one).unwrap(),
            serde_json::to_vec(
                &CharacterForge::generate(&dna(1, "pirate", CreativeAxes::neutral())).unwrap()
            )
            .unwrap()
        );
    }

    #[test]
    fn seed_sweep_has_low_duplicate_rate_for_names_and_archetypes() {
        for preset in CORE_PRESETS {
            let candidates: Vec<_> = (0..64)
                .map(|seed| {
                    CharacterForge::generate(&dna(seed, preset, CreativeAxes::neutral())).unwrap()
                })
                .collect();
            let names: BTreeSet<_> = candidates
                .iter()
                .map(|candidate| candidate.character.envelope.name.as_str())
                .collect();
            let identities: BTreeSet<_> = candidates
                .iter()
                .map(|candidate| {
                    (
                        candidate.character.envelope.name.as_str(),
                        candidate.character.archetype.as_str(),
                    )
                })
                .collect();
            assert!(
                names.len() >= 24,
                "{preset} produced only {} distinct names",
                names.len()
            );
            assert!(
                identities.len() >= 40,
                "{preset} produced only {} distinct name/archetype pairs",
                identities.len()
            );
        }
    }

    #[test]
    fn extreme_axes_and_chaos_blend_produce_valid_distinct_samples() {
        for (axis, level) in [
            (CreativeAxis::Chaos, 0),
            (CreativeAxis::Chaos, 100),
            (CreativeAxis::Technology, 100),
            (CreativeAxis::Mysticism, 100),
        ] {
            let mut axes = CreativeAxes::neutral();
            axes.levels.insert(axis, level);
            for seed in 0..32 {
                let candidate =
                    CharacterForge::generate(&dna(seed, "chaos-blend", axes.clone())).unwrap();
                candidate.validate().unwrap();
            }
        }
        let samples: BTreeSet<_> = (0..32)
            .map(|seed| {
                CharacterForge::generate(&dna(seed, "chaos-blend", CreativeAxes::neutral()))
                    .unwrap()
                    .character
                    .archetype
            })
            .collect();
        assert!(
            samples.len() >= 3,
            "Chaos Blend should vary across seed sweep"
        );
    }

    #[test]
    fn creative_axis_extremes_shift_weighted_content_distribution() {
        let sample = |level| {
            let mut axes = CreativeAxes::neutral();
            axes.levels.insert(CreativeAxis::Technology, level);
            (0..256)
                .map(|seed| {
                    CharacterForge::generate(&dna(seed, "cyberpunk", axes.clone()))
                        .unwrap()
                        .character
                        .archetype
                })
                .filter(|archetype| archetype.contains("Memory-Bank Salvager"))
                .count()
        };
        assert!(sample(100) > sample(0));
    }

    #[test]
    fn controlled_creative_axis_comparison_keeps_seed_and_other_axes_fixed() {
        let mut low_axes = CreativeAxes::neutral();
        low_axes.levels.insert(CreativeAxis::Technology, 0);
        let mut high_axes = CreativeAxes::neutral();
        high_axes.levels.insert(CreativeAxis::Technology, 100);
        let mut low_biased = 0;
        let mut high_biased = 0;
        for seed in 0..256 {
            let low_dna = dna(seed, "cyberpunk", low_axes.clone());
            let high_dna = dna(seed, "cyberpunk", high_axes.clone());
            let low = CharacterForge::generate(&low_dna).unwrap();
            let high = CharacterForge::generate(&high_dna).unwrap();
            assert_eq!(low, CharacterForge::generate(&low_dna).unwrap());
            assert_eq!(high, CharacterForge::generate(&high_dna).unwrap());
            low_biased += usize::from(low.character.archetype == "Memory-Bank Salvager");
            high_biased += usize::from(high.character.archetype == "Memory-Bank Salvager");
        }
        assert!(
            high_biased > low_biased,
            "controlled sample low={low_biased}, high={high_biased}"
        );
    }

    #[test]
    fn canonical_json_roundtrip_preserves_utf8_unicode() {
        let mut candidate =
            CharacterForge::generate(&dna(31415, "occult", CreativeAxes::neutral())).unwrap();
        let unicode = "séance refusing—and 雪の狐 🌙";
        candidate
            .manual_edit(
                CharacterField::BackstoryHook,
                CharacterFieldValue::Text(unicode.into()),
            )
            .unwrap();
        let bytes = serde_json::to_vec(&candidate).unwrap();
        let json = std::str::from_utf8(&bytes).unwrap();
        assert!(json.contains("séance"));
        assert!(json.contains("refusing—and"));
        assert!(json.contains("雪の狐 🌙"));
        let restored: CharacterCandidate = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(restored, candidate);
        assert_eq!(restored.character.backstory_hook, unicode);
    }

    #[test]
    fn rerolls_respect_locks_and_manual_edits_preserve_provenance() {
        let mut candidate =
            CharacterForge::generate(&dna(77, "dark-fantasy", CreativeAxes::neutral())).unwrap();
        let before = candidate.character.clone();
        CharacterForge::lock(&mut candidate, CharacterField::Appearance).unwrap();
        let locked_appearance = candidate.character.appearance.clone();
        let changed = CharacterForge::reroll_field(&mut candidate, CharacterField::Name).unwrap();
        assert!(changed.contains(&CharacterField::Name));
        assert_eq!(candidate.character.appearance, locked_appearance);
        assert_ne!(candidate.character.envelope.name, before.envelope.name);
        let provenance = candidate
            .character
            .envelope
            .generation_provenance
            .clone()
            .unwrap();
        candidate
            .manual_edit(
                CharacterField::Fear,
                CharacterFieldValue::Text("the bell below the lake".into()),
            )
            .unwrap();
        assert_eq!(candidate.character.fear, "the bell below the lake");
        assert_eq!(
            candidate.field_origins[&CharacterField::Fear.key().unwrap()],
            FieldValueOrigin::UserModified
        );
        let edited = candidate
            .character
            .envelope
            .generation_provenance
            .as_ref()
            .unwrap();
        assert_eq!(edited.parent_generation_id, Some(provenance.generation_id));
        assert_eq!(edited.action, GenerationAction::ManualEdit);
        candidate.validate().unwrap();
    }

    #[test]
    fn desire_reroll_respects_locked_dependent_field_firewall() {
        let mut candidate =
            CharacterForge::generate(&dna(7001, "nordic-myth", CreativeAxes::neutral())).unwrap();
        CharacterForge::lock(&mut candidate, CharacterField::RelationshipHook).unwrap();
        let locked_relationship = candidate.character.relationship_hook.clone();
        let result = CharacterForge::reroll_field(&mut candidate, CharacterField::Desire).unwrap();
        assert!(result.contains(&CharacterField::Desire));
        assert!(!result.contains(&CharacterField::RelationshipHook));
        assert!(result.contains(&CharacterField::BackstoryHook));
        assert!(result.contains(&CharacterField::StoryHooks));
        assert_eq!(candidate.character.relationship_hook, locked_relationship);
        candidate.validate().unwrap();
    }

    #[test]
    fn reroll_all_unlocked_preserves_locked_and_manually_edited_values() {
        let mut candidate =
            CharacterForge::generate(&dna(811, "steampunk", CreativeAxes::neutral())).unwrap();
        CharacterForge::lock(&mut candidate, CharacterField::Appearance).unwrap();
        let appearance = candidate.character.appearance.clone();
        candidate
            .manual_edit(
                CharacterField::Fear,
                CharacterFieldValue::Text("I fear the copper moon".into()),
            )
            .unwrap();
        let result = CharacterForge::reroll_all_unlocked(&mut candidate).unwrap();
        assert!(!result.contains(&CharacterField::Appearance));
        assert!(!result.contains(&CharacterField::Fear));
        assert_eq!(candidate.character.appearance, appearance);
        assert_eq!(candidate.character.fear, "I fear the copper moon");
        candidate.validate().unwrap();
    }

    #[test]
    fn unsupported_presets_and_invalid_constraints_return_structured_errors() {
        let error = CharacterForge::new_dna(
            RootSeed::new(3),
            "core",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::UnsupportedPreset);
        let mut constraints = BTreeMap::new();
        constraints.insert(ConstraintKey::new("unknown").unwrap(), "yes".into());
        let error = CharacterForge::new_dna(
            RootSeed::new(3),
            "dark-fantasy",
            CreativeAxes::neutral(),
            constraints,
        )
        .unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::InvalidConstraint);
        let mut constraints = BTreeMap::new();
        constraints.insert(ConstraintKey::new("inspiration").unwrap(), "雪の狐".into());
        assert!(
            CharacterForge::generate(
                &CharacterForge::new_dna(
                    RootSeed::new(u64::MAX),
                    "dark-fantasy",
                    CreativeAxes::neutral(),
                    constraints
                )
                .unwrap()
            )
            .is_ok()
        );
    }

    #[test]
    fn impossible_constraints_fail_without_partial_candidate_mutation() {
        let mut constraints = BTreeMap::new();
        constraints.insert(
            ConstraintKey::new("must_be_non_human").unwrap(),
            "true".into(),
        );
        constraints.insert(ConstraintKey::new("urban_only").unwrap(), "true".into());
        let error = CharacterForge::new_dna(
            RootSeed::new(6),
            "pirate",
            CreativeAxes::neutral(),
            constraints,
        )
        .and_then(|dna| CharacterForge::generate(&dna))
        .unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::InvalidConstraint);
    }

    #[test]
    fn supported_constraints_filter_content_choices_by_semantic_tags() {
        let mut constraints = BTreeMap::new();
        constraints.insert(ConstraintKey::new("no_magic").unwrap(), "true".into());
        constraints.insert(ConstraintKey::new("no_firearms").unwrap(), "true".into());
        constraints.insert(ConstraintKey::new("urban_only").unwrap(), "true".into());
        let catalog = ContentCatalog::load(CURRENT_CHARACTER_CONTENT_VERSION).unwrap();
        let mut observed = BTreeSet::new();
        for seed in 0..64 {
            let candidate = CharacterForge::generate(
                &CharacterForge::new_dna(
                    RootSeed::new(seed),
                    "dark-fantasy",
                    CreativeAxes::neutral(),
                    constraints.clone(),
                )
                .unwrap(),
            )
            .unwrap();
            assert!(
                candidate
                    .character
                    .envelope
                    .tags
                    .iter()
                    .any(|tag| tag == "urban")
            );
            let preset = catalog
                .preset(&candidate.dna.recipe.universe_preset)
                .unwrap();
            let magic: BTreeSet<_> = preset
                .abilities
                .iter()
                .filter(|choice| choice.tags.iter().any(|tag| tag == "magic"))
                .map(|choice| choice.value.as_str())
                .collect();
            let firearms: BTreeSet<_> = preset
                .gear
                .iter()
                .filter(|choice| choice.tags.iter().any(|tag| tag == "firearm"))
                .map(|choice| choice.value.as_str())
                .collect();
            assert!(
                candidate
                    .character
                    .abilities
                    .iter()
                    .all(|value| !magic.contains(value.as_str()))
            );
            assert!(!firearms.contains(candidate.character.signature_gear.as_str()));
            observed.insert(candidate.character.archetype);
        }

        let mut constraints = BTreeMap::new();
        constraints.insert(
            ConstraintKey::new("must_be_non_human").unwrap(),
            "true".into(),
        );
        let candidate = CharacterForge::generate(
            &CharacterForge::new_dna(
                RootSeed::new(321),
                "dark-fantasy",
                CreativeAxes::neutral(),
                constraints,
            )
            .unwrap(),
        )
        .unwrap();
        assert!(
            candidate
                .character
                .envelope
                .tags
                .iter()
                .any(|tag| tag == "non_human")
        );
        assert!(!observed.is_empty());
    }
}
