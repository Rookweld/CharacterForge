//! Deterministic domain operations; independent of UI, Tauri, storage, and providers.

mod character;
mod character_content;
mod character_faction;
mod creature;
mod creature_content;
mod faction;
mod faction_content;
mod faction_context;
mod field_stream;
mod forge_further;
mod item;
mod item_content;
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
pub use item::{
    ITEM_CONTENT_VERSION, ITEM_FIELD_ORDER, ITEM_GENERATOR_VERSION, ItemCandidate, ItemField,
    ItemFieldValue, ItemForge, ItemOrigin,
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

pub use creature::{
    CREATURE_CONTENT_VERSION, CREATURE_FIELD_ORDER, CREATURE_GENERATOR_VERSION, CreatureCandidate,
    CreatureField, CreatureFieldValue, CreatureForge, CreatureOrigin,
};

pub use character_faction::{
    FORGE_FACTION_ACTION, FORGE_FACTION_RECIPE, FactionContextSnapshot, FactionFurtherRecipeRecord,
    FactionResultBundle, compile_faction_context, finalize_faction_review, forge_character_faction,
    validate_faction_bundle,
};
pub use faction_context::CharacterFactionFacts;

mod character_signature_item;
mod item_context;
pub use character_signature_item::{
    FORGE_SIGNATURE_ITEM_ACTION, FORGE_SIGNATURE_ITEM_RECIPE, SignatureItemContextSnapshot,
    SignatureItemRecipeRecord, SignatureItemResultBundle, compile_signature_item_context,
    finalize_signature_item_review, forge_character_signature_item, validate_signature_item_bundle,
};
pub use item_context::CharacterItemFacts;
