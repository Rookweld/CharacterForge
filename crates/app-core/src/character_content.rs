//! Declarative, versioned Character content loaded from `content/characters/v1`.

use forge_contracts::{
    ContentVersion, CreativeAxes, CreativeAxis, PresetId, ValidationCode, ValidationError,
    ValidationErrors, ValidationResult,
};
use serde::Deserialize;
use std::{collections::BTreeMap, sync::OnceLock};

const CHARACTER_CONTENT_V1: &str = include_str!("../../../content/characters/v1/core.json");

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AxisBias {
    pub axis: CreativeAxis,
    pub preferred_level: u8,
}

#[derive(Clone, Debug)]
pub(crate) struct Choice {
    pub value: String,
    pub weight: u32,
    pub bias: Option<AxisBias>,
    pub tags: Vec<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ChoiceDocument {
    Text(String),
    Weighted(WeightedChoiceDocument),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WeightedChoiceDocument {
    value: String,
    #[serde(default = "default_weight")]
    weight: u32,
    #[serde(default)]
    bias: Option<AxisBias>,
    #[serde(default)]
    tags: Vec<String>,
}

impl ChoiceDocument {
    fn into_choice(self, path: &str) -> ValidationResult<Choice> {
        let choice = match self {
            Self::Text(value) => Choice {
                value,
                weight: default_weight(),
                bias: None,
                tags: Vec::new(),
            },
            Self::Weighted(choice) => Choice {
                value: choice.value,
                weight: choice.weight,
                bias: choice.bias,
                tags: choice.tags,
            },
        };
        if choice.value.trim().is_empty() {
            return Err(content_error(
                ValidationCode::EmptyValue,
                path,
                "content value must contain non-whitespace text",
            ));
        }
        if choice.weight == 0 {
            return Err(content_error(
                ValidationCode::InvalidConstraint,
                path,
                "content weight must be greater than zero",
            ));
        }
        if choice.bias.is_some_and(|bias| bias.preferred_level > 100) {
            return Err(content_error(
                ValidationCode::AxisOutOfRange,
                path,
                "axis preference must be within 0..=100",
            ));
        }
        Ok(choice)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ContentDocument {
    content_version: u16,
    presets: Vec<PresetDocument>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PresetDocument {
    id: String,
    given_names: Vec<ChoiceDocument>,
    family_names: Vec<ChoiceDocument>,
    archetypes: Vec<ChoiceDocument>,
    roles: Vec<ChoiceDocument>,
    titles: Vec<ChoiceDocument>,
    appearances: Vec<ChoiceDocument>,
    personalities: Vec<ChoiceDocument>,
    desires: Vec<ChoiceDocument>,
    fears: Vec<ChoiceDocument>,
    flaws: Vec<ChoiceDocument>,
    secrets: Vec<ChoiceDocument>,
    skills: Vec<ChoiceDocument>,
    abilities: Vec<ChoiceDocument>,
    factions: Vec<ChoiceDocument>,
    gear: Vec<ChoiceDocument>,
    relationships: Vec<ChoiceDocument>,
    backstories: Vec<ChoiceDocument>,
    hooks: Vec<ChoiceDocument>,
}

#[derive(Clone, Debug)]
pub(crate) struct PresetContent {
    pub id: PresetId,
    pub given_names: Vec<Choice>,
    pub family_names: Vec<Choice>,
    pub archetypes: Vec<Choice>,
    pub roles: Vec<Choice>,
    pub titles: Vec<Choice>,
    pub appearances: Vec<Choice>,
    pub personalities: Vec<Choice>,
    pub desires: Vec<Choice>,
    pub fears: Vec<Choice>,
    pub flaws: Vec<Choice>,
    pub secrets: Vec<Choice>,
    pub skills: Vec<Choice>,
    pub abilities: Vec<Choice>,
    pub factions: Vec<Choice>,
    pub gear: Vec<Choice>,
    pub relationships: Vec<Choice>,
    pub backstories: Vec<Choice>,
    pub hooks: Vec<Choice>,
}

impl PresetDocument {
    fn into_content(self) -> ValidationResult<PresetContent> {
        let id = PresetId::new(&self.id)?;
        Ok(PresetContent {
            id,
            given_names: convert_choices("given_names", self.given_names)?,
            family_names: convert_choices("family_names", self.family_names)?,
            archetypes: convert_choices("archetypes", self.archetypes)?,
            roles: convert_choices("roles", self.roles)?,
            titles: convert_choices("titles", self.titles)?,
            appearances: convert_choices("appearances", self.appearances)?,
            personalities: convert_choices("personalities", self.personalities)?,
            desires: convert_choices("desires", self.desires)?,
            fears: convert_choices("fears", self.fears)?,
            flaws: convert_choices("flaws", self.flaws)?,
            secrets: convert_choices("secrets", self.secrets)?,
            skills: convert_choices("skills", self.skills)?,
            abilities: convert_choices("abilities", self.abilities)?,
            factions: convert_choices("factions", self.factions)?,
            gear: convert_choices("gear", self.gear)?,
            relationships: convert_choices("relationships", self.relationships)?,
            backstories: convert_choices("backstories", self.backstories)?,
            hooks: convert_choices("hooks", self.hooks)?,
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ContentCatalog {
    pub version: ContentVersion,
    presets: BTreeMap<String, PresetContent>,
}

impl ContentCatalog {
    pub fn load() -> ValidationResult<&'static Self> {
        static CONTENT: OnceLock<ValidationResult<ContentCatalog>> = OnceLock::new();
        match CONTENT.get_or_init(parse_content) {
            Ok(catalog) => Ok(catalog),
            Err(errors) => Err(errors.clone()),
        }
    }

    pub fn preset(&self, id: &PresetId) -> ValidationResult<&PresetContent> {
        self.presets.get(id.as_str()).ok_or_else(|| {
            ValidationErrors::one(ValidationError::new(
                ValidationCode::UnsupportedPreset,
                "universe_preset",
                format!("unsupported Character preset '{id}'"),
            ))
        })
    }

    #[cfg(test)]
    pub fn presets(&self) -> impl Iterator<Item = &PresetContent> {
        self.presets.values()
    }
}

fn parse_content() -> ValidationResult<ContentCatalog> {
    let document: ContentDocument =
        serde_json::from_str(CHARACTER_CONTENT_V1).map_err(|error| {
            ValidationErrors::one(ValidationError::new(
                ValidationCode::Serialization,
                "character_content",
                format!("invalid embedded Character content: {error}"),
            ))
        })?;
    if document.content_version != 1 {
        return Err(content_error(
            ValidationCode::UnsupportedVersion,
            "character_content.content_version",
            "only Character content version 1 is supported",
        ));
    }
    let version = ContentVersion(document.content_version);
    let mut presets = BTreeMap::new();
    for preset in document.presets {
        let preset = preset.into_content()?;
        if presets
            .insert(preset.id.as_str().to_owned(), preset)
            .is_some()
        {
            return Err(content_error(
                ValidationCode::InvalidIdentifier,
                "character_content.presets",
                "preset identifiers must be unique",
            ));
        }
    }
    if presets.len() != CORE_PRESETS.len()
        || CORE_PRESETS.iter().any(|id| !presets.contains_key(*id))
    {
        return Err(content_error(
            ValidationCode::UnsupportedPreset,
            "character_content.presets",
            "content must define every canonical v1 Character preset exactly once",
        ));
    }
    Ok(ContentCatalog { version, presets })
}

fn convert_choices(path: &str, choices: Vec<ChoiceDocument>) -> ValidationResult<Vec<Choice>> {
    if choices.len() < 2 {
        return Err(content_error(
            ValidationCode::MissingField,
            format!("character_content.{path}"),
            "each semantic content family must provide at least two choices",
        ));
    }
    choices
        .into_iter()
        .enumerate()
        .map(|(index, choice)| choice.into_choice(&format!("character_content.{path}[{index}]")))
        .collect()
}

fn default_weight() -> u32 {
    1
}

fn content_error(
    code: ValidationCode,
    path: impl Into<String>,
    message: impl Into<String>,
) -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(code, path, message))
}

pub(crate) const CORE_PRESETS: [&str; 11] = [
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

pub(crate) fn validate_choice_axes(axes: &CreativeAxes, choice: &Choice) -> i32 {
    let Some(bias) = choice.bias else {
        return choice.weight.min(i32::MAX as u32) as i32;
    };
    let level = axes.levels.get(&bias.axis).copied().unwrap_or(50) as i32;
    let preference = bias.preferred_level as i32;
    let multiplier = (100 + ((level - 50) * (preference - 50) / 25)).clamp(1, 200);
    let weighted = (choice.weight as u64).saturating_mul(multiplier as u64) / 100;
    weighted.clamp(1, i32::MAX as u64) as i32
}
