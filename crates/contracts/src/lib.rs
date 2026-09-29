//! Versioned, UI- and persistence-independent domain contracts.

mod domain;
mod generation;
mod identity;
mod validation;
mod version;

pub use domain::{
    CanonState, Character, EntityEnvelope, EntityOrigin, EntityType, Location, LocationProfile,
    ProjectEnvelope, Quest, Relationship, RelationshipState, RelationshipType,
};
pub use generation::{
    ConstraintKey, Constraints, CreativeAxes, CreativeAxis, FieldControl, FieldDependencies,
    FieldKey, FieldNonce, ForgeDNA, GenerationAction, GenerationKind, GenerationMetadata,
    GenerationRecipe, PresetId, RootSeed,
};
pub use identity::{EntityId, ProjectId, RelationshipId, Uuid128};
pub use validation::{ValidationCode, ValidationError, ValidationErrors, ValidationResult};
pub use version::{ContentVersion, GeneratorVersion, RecipeVersion, SchemaVersion};

/// Initial compatible v1.0 FOUNDATION contract versions.
pub const INITIAL_SCHEMA_VERSION: SchemaVersion = SchemaVersion(1);
pub const INITIAL_GENERATOR_VERSION: GeneratorVersion = GeneratorVersion(1);
pub const INITIAL_CONTENT_VERSION: ContentVersion = ContentVersion(1);
pub const INITIAL_RECIPE_VERSION: RecipeVersion = RecipeVersion(1);
