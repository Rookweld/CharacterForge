//! Deterministic domain operations; independent of UI, Tauri, storage, and providers.

mod character;
mod character_content;
mod field_stream;
mod reroll;

pub use character::{
    CharacterCandidate, CharacterField, CharacterFieldValue, CharacterForge, FieldValueOrigin,
};
pub use field_stream::FieldStream;
pub use reroll::{
    FieldValues, RerollResult, lock_field, reroll_all_unlocked, reroll_field, unlock_field,
};
