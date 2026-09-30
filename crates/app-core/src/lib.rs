//! Deterministic domain operations; independent of UI, Tauri, storage, and providers.

mod character;
mod character_content;
mod faction;
mod faction_content;
mod field_stream;
mod forge_further;
mod location;
mod quest;
mod quest_content;
mod relationship;
mod reroll;

pub use character::{
    CharacterCandidate, CharacterField, CharacterFieldValue, CharacterForge, FieldValueOrigin,
};
pub use faction::{
    FACTION_CONTENT_VERSION, FACTION_FIELD_ORDER, FACTION_GENERATOR_VERSION, FactionCandidate,
    FactionField, FactionFieldValue, FactionForge, FactionOrigin,
};
pub use field_stream::FieldStream;
pub use forge_further::{
    FORGE_HOMETOWN_ACTION, FORGE_HOMETOWN_RECIPE, ForgeActionDefinition, ForgeFurtherRecipeRecord,
    ForgeResultBundle, ForgeResultMapping, HometownContextSnapshot, HometownSource,
    compile_hometown_context, finalize_hometown_review, forge_hometown,
};
pub use location::{
    LOCATION_FIELD_ORDER, LocationCandidate, LocationField, LocationFieldValue, LocationForge,
    LocationOrigin,
};
pub use quest::{QuestCandidate, QuestField, QuestFieldValue, QuestForge};
pub use relationship::{
    RELATIONSHIP_DEFINITIONS, RelationshipDefinition, relationship_definition,
    validate_relationship_endpoints,
};
pub use reroll::{
    FieldValues, RerollResult, lock_field, reroll_all_unlocked, reroll_field, unlock_field,
};
