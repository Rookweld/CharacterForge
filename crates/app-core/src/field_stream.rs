use forge_contracts::{
    Constraints, FieldKey, FieldNonce, ForgeDNA, ValidationCode, ValidationError, ValidationErrors,
    ValidationResult,
};
use serde::Serialize;

const FIELD_STREAM_CONTEXT: &str = "character-forge/forge-dna/field-stream/v1";

#[derive(Serialize)]
struct DependencyNonce<'a> {
    field: &'a FieldKey,
    nonce: FieldNonce,
}

#[derive(Serialize)]
struct FieldSeedMaterial<'a> {
    schema_version: u16,
    generator_version: u16,
    content_version: u16,
    recipe_version: u16,
    root_seed: String,
    universe_preset: &'a str,
    creative_axes: &'a forge_contracts::CreativeAxes,
    constraints: &'a Constraints,
    field: &'a FieldKey,
    nonce: FieldNonce,
    dependencies: Vec<DependencyNonce<'a>>,
}

/// A deterministic XOF stream derived only from canonical DNA and one field's explicit inputs.
pub struct FieldStream {
    reader: blake3::OutputReader,
    fingerprint: [u8; 32],
}

impl FieldStream {
    pub fn derive(dna: &ForgeDNA, field: &FieldKey) -> ValidationResult<Self> {
        let mut canonical_dna = dna.clone();
        canonical_dna.normalize()?;
        let recipe = &canonical_dna.recipe;
        let control = recipe.field_controls.get(field).ok_or_else(|| {
            ValidationErrors::one(ValidationError::new(
                ValidationCode::UnknownField,
                format!("field_controls.{field}"),
                "field must be registered in the generation recipe",
            ))
        })?;
        let dependencies = recipe.dependencies.get(field);
        let dependency_nonces = dependencies
            .into_iter()
            .flatten()
            .map(|dependency| DependencyNonce {
                field: dependency,
                nonce: recipe.field_controls[dependency].nonce,
            })
            .collect();
        let material = FieldSeedMaterial {
            schema_version: canonical_dna.schema_version.0,
            generator_version: canonical_dna.generator_version.0,
            content_version: canonical_dna.content_version.0,
            recipe_version: recipe.recipe_version.0,
            root_seed: recipe.root_seed.to_string(),
            universe_preset: recipe.universe_preset.as_str(),
            creative_axes: &recipe.creative_axes,
            constraints: &recipe.constraints,
            field,
            nonce: control.nonce,
            dependencies: dependency_nonces,
        };
        let bytes = serde_json::to_vec(&material).map_err(|error| {
            ValidationErrors::one(ValidationError::new(
                ValidationCode::Serialization,
                "field_seed_material",
                format!("could not serialize deterministic field inputs: {error}"),
            ))
        })?;
        let mut hasher = blake3::Hasher::new_derive_key(FIELD_STREAM_CONTEXT);
        hasher.update(&bytes);
        let mut reader = hasher.clone().finalize_xof();
        let mut fingerprint = [0; 32];
        reader.fill(&mut fingerprint);
        let reader = hasher.finalize_xof();
        Ok(Self {
            reader,
            fingerprint,
        })
    }

    pub const fn fingerprint(&self) -> &[u8; 32] {
        &self.fingerprint
    }

    pub fn fill_bytes(&mut self, output: &mut [u8]) {
        self.reader.fill(output);
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut bytes = [0; 8];
        self.fill_bytes(&mut bytes);
        u64::from_le_bytes(bytes)
    }

    /// Uniformly samples 0..upper_exclusive using rejection sampling (no modulo bias).
    pub fn next_bounded(&mut self, upper_exclusive: u64) -> ValidationResult<u64> {
        if upper_exclusive == 0 {
            return Err(ValidationErrors::one(ValidationError::new(
                ValidationCode::InvalidRange,
                "upper_exclusive",
                "sampling range must be greater than zero",
            )));
        }
        let threshold = upper_exclusive.wrapping_neg() % upper_exclusive;
        loop {
            let value = self.next_u64();
            if value >= threshold {
                return Ok(value % upper_exclusive);
            }
        }
    }
}
