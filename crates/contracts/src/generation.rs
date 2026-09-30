use crate::{
    ContentVersion, GeneratorVersion, INITIAL_CONTENT_VERSION, INITIAL_GENERATOR_VERSION,
    INITIAL_RECIPE_VERSION, INITIAL_SCHEMA_VERSION, RecipeVersion, SchemaVersion, ValidationCode,
    ValidationError, ValidationErrors, ValidationResult,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RootSeed(u64);

impl RootSeed {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
    pub const fn value(self) -> u64 {
        self.0
    }

    pub fn parse_hex(value: &str) -> ValidationResult<Self> {
        if value.len() != 16 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::InvalidSeed,
                "root_seed",
                "root seed must be exactly 16 hexadecimal digits",
            )));
        }
        u64::from_str_radix(value, 16).map(Self).map_err(|_| {
            ValidationErrors::one(ValidationError::new(
                ValidationCode::InvalidSeed,
                "root_seed",
                "root seed is outside the supported 64-bit range",
            ))
        })
    }
}

impl std::fmt::Display for RootSeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

impl Serialize for RootSeed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for RootSeed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse_hex(&value).map_err(de::Error::custom)
    }
}

macro_rules! identifier_type {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> ValidationResult<Self> {
                let value = value.as_ref().trim().to_ascii_lowercase();
                let valid = !value.is_empty()
                    && value.len() <= 128
                    && value.as_bytes()[0].is_ascii_alphanumeric()
                    && value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'_' | b'-' | b'.')
                    });
                if !valid {
                    return Err(ValidationErrors::one(ValidationError::new(
                        ValidationCode::InvalidIdentifier,
                        $field,
                        "identifier must be 1–128 ASCII lowercase letters, digits, '.', '_' or '-'",
                    )));
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn validate(&self, path: &str) -> ValidationResult<()> {
                let canonical = Self::new(&self.0)?;
                if canonical.0 != self.0 {
                    return Err(ValidationErrors::one(ValidationError::new(
                        ValidationCode::InvalidIdentifier,
                        path,
                        "identifier is not in canonical normalized form",
                    )));
                }
                Ok(())
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

identifier_type!(FieldKey, "field_key");
identifier_type!(PresetId, "universe_preset");
identifier_type!(ConstraintKey, "constraints.key");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreativeAxis {
    Bleak,
    Mythic,
    Chaos,
    Epic,
    Ambiguity,
    Surreal,
    Technology,
    Mysticism,
}

impl CreativeAxis {
    pub const ALL: [Self; 8] = [
        Self::Bleak,
        Self::Mythic,
        Self::Chaos,
        Self::Epic,
        Self::Ambiguity,
        Self::Surreal,
        Self::Technology,
        Self::Mysticism,
    ];
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreativeAxes {
    pub levels: BTreeMap<CreativeAxis, u8>,
}

impl Default for CreativeAxes {
    fn default() -> Self {
        Self::neutral()
    }
}

impl CreativeAxes {
    pub fn neutral() -> Self {
        Self {
            levels: CreativeAxis::ALL
                .into_iter()
                .map(|axis| (axis, 50))
                .collect(),
        }
    }

    pub fn normalize(&mut self) -> ValidationResult<()> {
        let mut normalized = Self::neutral().levels;
        for (&axis, &level) in &self.levels {
            if level > 100 {
                return Err(ValidationErrors::one(ValidationError::new(
                    ValidationCode::AxisOutOfRange,
                    format!("creative_axes.{axis:?}"),
                    "creative-axis level must be in the inclusive range 0..=100",
                )));
            }
            normalized.insert(axis, level);
        }
        self.levels = normalized;
        Ok(())
    }

    pub fn validate(&self) -> ValidationResult<()> {
        if self.levels.len() != CreativeAxis::ALL.len()
            || CreativeAxis::ALL
                .iter()
                .any(|axis| !self.levels.contains_key(axis))
            || self.levels.values().any(|level| *level > 100)
        {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::AxisOutOfRange,
                "creative_axes",
                "canonical axes must include every axis with values in 0..=100",
            )));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldControl {
    pub nonce: FieldNonce,
    pub locked: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FieldNonce(pub u64);

pub type FieldDependencies = BTreeMap<FieldKey, BTreeSet<FieldKey>>;
pub type FieldControls = BTreeMap<FieldKey, FieldControl>;
pub type Constraints = BTreeMap<ConstraintKey, String>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationKind {
    Character,
    Quest,
    Location,
    Faction,
    Item,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationAction {
    Forge,
    RerollField,
    RerollUnlocked,
    ManualEdit,
    RestoreHistory,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationRecipe {
    pub kind: GenerationKind,
    pub recipe_version: RecipeVersion,
    pub root_seed: RootSeed,
    pub universe_preset: PresetId,
    pub creative_axes: CreativeAxes,
    pub constraints: Constraints,
    pub field_controls: FieldControls,
    /// Maps each field to the fields whose values it explicitly reads.
    pub dependencies: FieldDependencies,
}

impl GenerationRecipe {
    pub fn new(kind: GenerationKind, root_seed: RootSeed, universe_preset: PresetId) -> Self {
        Self {
            kind,
            recipe_version: INITIAL_RECIPE_VERSION,
            root_seed,
            universe_preset,
            creative_axes: CreativeAxes::neutral(),
            constraints: BTreeMap::new(),
            field_controls: BTreeMap::new(),
            dependencies: BTreeMap::new(),
        }
    }

    pub fn add_field(&mut self, field: FieldKey) -> ValidationResult<()> {
        field.validate("field_controls.key")?;
        if self.field_controls.contains_key(&field) {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::InvalidIdentifier,
                format!("field_controls.{field}"),
                "field is already registered",
            )));
        }
        self.field_controls.insert(field, FieldControl::default());
        Ok(())
    }

    pub fn set_dependencies(
        &mut self,
        field: FieldKey,
        dependencies: impl IntoIterator<Item = FieldKey>,
    ) -> ValidationResult<()> {
        let values: BTreeSet<_> = dependencies.into_iter().collect();
        let previous = self.dependencies.clone();
        self.dependencies.insert(field, values);
        if let Err(errors) = self.validate_dependencies() {
            self.dependencies = previous;
            return Err(errors);
        }
        Ok(())
    }

    pub fn normalize(&mut self) -> ValidationResult<()> {
        let mut normalized = self.clone();
        normalized.creative_axes.normalize()?;
        let mut normalized_constraints = BTreeMap::new();
        for (key, value) in &normalized.constraints {
            let key = ConstraintKey::new(key.as_str())?;
            let value = value.trim().to_owned();
            if value.is_empty() {
                return Err(ValidationErrors::one(ValidationError::new(
                    ValidationCode::EmptyValue,
                    format!("constraints.{key}"),
                    "constraint value must not be empty",
                )));
            }
            if normalized_constraints.insert(key.clone(), value).is_some() {
                return Err(ValidationErrors::one(ValidationError::new(
                    ValidationCode::InvalidIdentifier,
                    format!("constraints.{key}"),
                    "multiple constraint keys normalize to the same identifier",
                )));
            }
        }
        normalized.constraints = normalized_constraints;
        normalized.validate()?;
        *self = normalized;
        Ok(())
    }

    pub fn validate(&self) -> ValidationResult<()> {
        if self.recipe_version != INITIAL_RECIPE_VERSION {
            return Err(unsupported("recipe_version", self.recipe_version.0));
        }
        self.universe_preset.validate("universe_preset")?;
        self.creative_axes.validate()?;
        if self.field_controls.is_empty() {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::MissingField,
                "field_controls",
                "at least one generation field must be registered",
            )));
        }
        for key in self.field_controls.keys() {
            key.validate("field_controls.key")?;
        }
        for (key, value) in &self.constraints {
            key.validate("constraints.key")?;
            if value.trim().is_empty() || value != value.trim() {
                return Err(ValidationErrors::one(ValidationError::new(
                    ValidationCode::EmptyValue,
                    format!("constraints.{key}"),
                    "constraint value must be non-empty and trimmed",
                )));
            }
        }
        self.validate_dependencies()
    }

    fn validate_dependencies(&self) -> ValidationResult<()> {
        for (field, dependencies) in &self.dependencies {
            field.validate("dependencies.key")?;
            if !self.field_controls.contains_key(field) {
                return Err(ValidationErrors::one(ValidationError::new(
                    ValidationCode::InvalidDependency,
                    format!("dependencies.{field}"),
                    "dependent field is not registered",
                )));
            }
            for dependency in dependencies {
                dependency.validate("dependencies.value")?;
                if dependency == field || !self.field_controls.contains_key(dependency) {
                    return Err(ValidationErrors::one(ValidationError::new(
                        ValidationCode::InvalidDependency,
                        format!("dependencies.{field}.{dependency}"),
                        "dependency must be a different registered field",
                    )));
                }
            }
        }

        // Kahn's algorithm avoids recursion even for untrusted, very deep input graphs.
        let mut indegree: BTreeMap<FieldKey, usize> = self
            .field_controls
            .keys()
            .cloned()
            .map(|field| (field, 0))
            .collect();
        let mut dependents: BTreeMap<FieldKey, BTreeSet<FieldKey>> = BTreeMap::new();
        for (field, dependencies) in &self.dependencies {
            for dependency in dependencies {
                *indegree.entry(field.clone()).or_default() += 1;
                dependents
                    .entry(dependency.clone())
                    .or_default()
                    .insert(field.clone());
            }
        }
        let mut ready: BTreeSet<FieldKey> = indegree
            .iter()
            .filter_map(|(field, degree)| (*degree == 0).then_some(field.clone()))
            .collect();
        let mut visited = 0;
        while let Some(field) = ready.pop_first() {
            visited += 1;
            if let Some(children) = dependents.get(&field) {
                for child in children {
                    if let Some(degree) = indegree.get_mut(child) {
                        *degree -= 1;
                        if *degree == 0 {
                            ready.insert(child.clone());
                        }
                    }
                }
            }
        }
        if visited != indegree.len() {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::DependencyCycle,
                "dependencies",
                "field dependencies must form an acyclic graph",
            )));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForgeDNA {
    pub schema_version: SchemaVersion,
    pub generator_version: GeneratorVersion,
    pub content_version: ContentVersion,
    pub recipe: GenerationRecipe,
}

impl ForgeDNA {
    pub fn new(recipe: GenerationRecipe) -> Self {
        Self {
            schema_version: INITIAL_SCHEMA_VERSION,
            generator_version: INITIAL_GENERATOR_VERSION,
            content_version: INITIAL_CONTENT_VERSION,
            recipe,
        }
    }

    pub fn normalize(&mut self) -> ValidationResult<()> {
        self.recipe.normalize()?;
        self.validate()
    }

    pub fn validate(&self) -> ValidationResult<()> {
        if self.schema_version != INITIAL_SCHEMA_VERSION {
            return Err(unsupported("schema_version", self.schema_version.0));
        }
        if self.generator_version != INITIAL_GENERATOR_VERSION {
            return Err(unsupported("generator_version", self.generator_version.0));
        }
        if !matches!(self.content_version.0, 1..=3) {
            return Err(unsupported("content_version", self.content_version.0));
        }
        self.recipe.validate()
    }

    /// Stable compact JSON: struct declaration order plus BTreeMap key order.
    pub fn to_canonical_json(&self) -> ValidationResult<String> {
        let mut canonical = self.clone();
        canonical.normalize()?;
        serde_json::to_string(&canonical).map_err(|error| {
            ValidationErrors::one(ValidationError::new(
                ValidationCode::Serialization,
                "forge_dna",
                format!("could not serialize ForgeDNA: {error}"),
            ))
        })
    }

    pub fn from_canonical_json(value: &str) -> ValidationResult<Self> {
        let mut dna: Self = serde_json::from_str(value).map_err(|error| {
            ValidationErrors::one(ValidationError::new(
                ValidationCode::Serialization,
                "forge_dna",
                format!("invalid ForgeDNA JSON: {error}"),
            ))
        })?;
        dna.normalize()?;
        let canonical = dna.to_canonical_json()?;
        if canonical != value {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::Serialization,
                "forge_dna",
                "JSON is valid but is not in canonical ForgeDNA form",
            )));
        }
        Ok(dna)
    }

    pub fn canonical_bytes(&self) -> ValidationResult<Vec<u8>> {
        self.to_canonical_json().map(String::into_bytes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationMetadata {
    pub generation_id: crate::EntityId,
    pub generated_at_unix_ms: i64,
    pub action: GenerationAction,
    pub parent_generation_id: Option<crate::EntityId>,
    pub dna: ForgeDNA,
}

fn unsupported(path: &str, value: u16) -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(
        ValidationCode::UnsupportedVersion,
        path,
        format!("version {value} is not supported by this FOUNDATION build"),
    ))
}

impl FromStr for FieldKey {
    type Err = ValidationErrors;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(value: &str) -> FieldKey {
        FieldKey::new(value).unwrap()
    }

    #[test]
    fn recipe_defaults_axes_to_neutral_and_normalizes_constraint_order_and_text() {
        let mut recipe = GenerationRecipe::new(
            GenerationKind::Character,
            RootSeed::new(u64::MAX),
            PresetId::new(" Core-Fantasy ").unwrap(),
        );
        recipe.add_field(key("Name")).unwrap();
        recipe.constraints.insert(
            ConstraintKey::new(" Tone ").unwrap(),
            "  古い灯  ".to_owned(),
        );
        recipe.normalize().unwrap();

        assert_eq!(recipe.universe_preset.as_str(), "core-fantasy");
        assert_eq!(
            recipe
                .constraints
                .get(&ConstraintKey::new("tone").unwrap())
                .unwrap(),
            "古い灯"
        );
        assert!(
            CreativeAxis::ALL
                .iter()
                .all(|axis| recipe.creative_axes.levels[axis] == 50)
        );
        assert_eq!(recipe.root_seed.to_string(), "ffffffffffffffff");
    }

    #[test]
    fn validation_reports_empty_recipe_invalid_axis_and_dependency_cycle() {
        let mut recipe = GenerationRecipe::new(
            GenerationKind::Quest,
            RootSeed::new(0),
            PresetId::new("core").unwrap(),
        );
        let error = recipe.validate().unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::MissingField);

        recipe.add_field(key("a")).unwrap();
        recipe.add_field(key("b")).unwrap();
        recipe.creative_axes.levels.insert(CreativeAxis::Bleak, 101);
        assert_eq!(
            recipe.validate().unwrap_err().errors[0].code,
            ValidationCode::AxisOutOfRange
        );
        recipe.creative_axes.normalize().unwrap_err();
        recipe.creative_axes.levels.insert(CreativeAxis::Bleak, 50);

        recipe.set_dependencies(key("a"), [key("b")]).unwrap();
        let error = recipe.set_dependencies(key("b"), [key("a")]).unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::DependencyCycle);
    }

    #[test]
    fn three_field_dependency_cycle_is_rejected_without_partial_mutation() {
        let mut recipe = GenerationRecipe::new(
            GenerationKind::Character,
            RootSeed::new(3),
            PresetId::new("core").unwrap(),
        );
        for field in ["a", "b", "c"] {
            recipe.add_field(key(field)).unwrap();
        }
        recipe.set_dependencies(key("b"), [key("a")]).unwrap();
        recipe.set_dependencies(key("c"), [key("b")]).unwrap();
        let prior_dependencies = recipe.dependencies.clone();

        let error = recipe.set_dependencies(key("a"), [key("c")]).unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::DependencyCycle);
        assert_eq!(recipe.dependencies, prior_dependencies);
        assert!(recipe.validate().is_ok());

        // Public/deserialized input can still contain an invalid graph; validation rejects it
        // iteratively with a structured error instead of recursing through the cycle.
        recipe.dependencies.insert(key("a"), [key("c")].into());
        let error = recipe.validate().unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::DependencyCycle);
    }

    #[test]
    fn seed_parser_rejects_empty_short_and_non_hex_values_without_panicking() {
        for invalid in ["", "1", "gggggggggggggggg", "00000000000000000"] {
            let error = RootSeed::parse_hex(invalid).unwrap_err();
            assert_eq!(error.errors[0].code, ValidationCode::InvalidSeed);
        }
        assert_eq!(
            RootSeed::parse_hex("FFFFFFFFFFFFFFFF").unwrap().value(),
            u64::MAX
        );
    }

    #[test]
    fn dna_roundtrip_requires_and_preserves_canonical_semantics() {
        let mut recipe = GenerationRecipe::new(
            GenerationKind::Character,
            RootSeed::new(42),
            PresetId::new("mythic").unwrap(),
        );
        recipe.add_field(key("name")).unwrap();
        let dna = ForgeDNA::new(recipe);
        let serialized = dna.to_canonical_json().unwrap();
        let decoded = ForgeDNA::from_canonical_json(&serialized).unwrap();
        assert_eq!(decoded, dna);
        assert_eq!(decoded.to_canonical_json().unwrap(), serialized);

        let noncanonical = format!(" {serialized}");
        assert_eq!(
            ForgeDNA::from_canonical_json(&noncanonical)
                .unwrap_err()
                .errors[0]
                .code,
            ValidationCode::Serialization
        );

        let mut content_v2 = dna.clone();
        content_v2.content_version = ContentVersion(2);
        content_v2.normalize().unwrap();
        let serialized_v2 = content_v2.to_canonical_json().unwrap();
        assert_eq!(
            ForgeDNA::from_canonical_json(&serialized_v2).unwrap(),
            content_v2
        );

        let mut unsupported = dna;
        unsupported.content_version = ContentVersion(4);
        assert_eq!(
            unsupported.validate().unwrap_err().errors[0].code,
            ValidationCode::UnsupportedVersion
        );
    }

    #[test]
    fn location_generation_kind_has_a_stable_independent_key() {
        assert_eq!(
            serde_json::to_string(&GenerationKind::Location).unwrap(),
            "\"location\""
        );
        assert_eq!(
            serde_json::from_str::<GenerationKind>("\"location\"").unwrap(),
            GenerationKind::Location
        );
    }
}
