//! Deterministic domain operations; independent of UI, Tauri, storage, and providers.

mod field_stream;
mod reroll;

pub use field_stream::FieldStream;
pub use reroll::{
    FieldValues, RerollResult, lock_field, reroll_all_unlocked, reroll_field, unlock_field,
};
