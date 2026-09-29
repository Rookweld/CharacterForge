//! Deterministic domain operations; independent of UI, Tauri, storage, and providers.

mod character;
mod character_content;
mod field_stream;
mod quest;
mod quest_content;
mod relationship;
mod reroll;

pub use character::{
    CharacterCandidate, CharacterField, CharacterFieldValue, CharacterForge, FieldValueOrigin,
};
pub use field_stream::FieldStream;
pub use quest::{QuestCandidate, QuestField, QuestFieldValue, QuestForge};
pub use relationship::{
    RELATIONSHIP_DEFINITIONS, RelationshipDefinition, relationship_definition,
    validate_relationship_endpoints,
};
pub use reroll::{
    FieldValues, RerollResult, lock_field, reroll_all_unlocked, reroll_field, unlock_field,
};
