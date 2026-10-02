//! B9/B10.R1/R2 pure provider-neutral planning. No provider, persistence or I/O.
#[path = "visual_prompt.rs"]
mod prompt;
use forge_contracts::{
    CanonState, Character, CreativeAxes, Creature, EntityEnvelope, EntityId, EntityType, Faction,
    Item, Location, ProjectId, ValidationCode, ValidationError, ValidationErrors, ValidationResult,
};
use prompt::{compile_semantics, style_line};
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualRole {
    Portrait,
    FullBody,
    CharacterSheet,
    EstablishingShot,
    EnvironmentConcept,
    InteriorConcept,
    Emblem,
    Banner,
    UniformConcept,
    ItemConcept,
    InventoryIcon,
    PropSheet,
    CreatureConcept,
    AnatomyDetail,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AspectIntent {
    Portrait,
    Landscape,
    Square,
}
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtDirection {
    pub medium_style: String,
    pub palette: String,
    pub materials: String,
    pub camera_composition: String,
    pub atmosphere: String,
    pub avoid: String,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualField {
    Name,
    Title,
    Archetype,
    Role,
    Appearance,
    Personality,
    SignatureGear,
    Abilities,
    FactionHint,
    LocationProfile,
    Summary,
    VisualIdentity,
    Atmosphere,
    Purpose,
    NotableFeatures,
    Danger,
    FactionProfile,
    Identity,
    Culture,
    PublicFace,
    Symbols,
    Ideology,
    ItemProfile,
    Properties,
    Origin,
    PowerOrFunction,
    Condition,
    Rarity,
    CreatureProfile,
    Habitat,
    Behavior,
    Ecology,
    Threat,
    AbilitiesSummary,
    Weakness,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VisualFact {
    pub field: VisualField,
    pub values: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum VisualEntity {
    Character(Box<Character>),
    Location(Box<Location>),
    Faction(Box<Faction>),
    Item(Box<Item>),
    Creature(Box<Creature>),
}
impl VisualEntity {
    pub fn envelope(&self) -> &EntityEnvelope {
        match self {
            Self::Character(x) => &x.envelope,
            Self::Location(x) => &x.envelope,
            Self::Faction(x) => &x.envelope,
            Self::Item(x) => &x.envelope,
            Self::Creature(x) => &x.envelope,
        }
    }
    fn validate(&self) -> ValidationResult<()> {
        match self {
            Self::Character(x) => x.validate(),
            Self::Location(x) => x.validate(),
            Self::Faction(x) => x.validate(),
            Self::Item(x) => x.validate(),
            Self::Creature(x) => x.validate(),
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VisualRecipeDefinition {
    pub recipe_id: String,
    pub version: u16,
    pub source_type: EntityType,
    pub role: VisualRole,
    pub required_fields: Vec<VisualField>,
    pub optional_fields: Vec<VisualField>,
    pub composition_intent: String,
    pub aspect_intent: AspectIntent,
    pub transparent_background_meaningful: bool,
    pub environment_present: bool,
    pub forbidden_categories: Vec<String>,
}
use VisualField as F;
use VisualRole as R;
pub fn visual_recipes(kind: EntityType) -> Vec<VisualRecipeDefinition> {
    let (family, roles, required, optional): (&str, Vec<R>, Vec<F>, Vec<F>) = match kind {
        EntityType::Character => (
            "character",
            vec![R::Portrait, R::FullBody, R::CharacterSheet],
            vec![
                F::Name,
                F::Title,
                F::Archetype,
                F::Role,
                F::Appearance,
                F::SignatureGear,
            ],
            vec![F::Personality, F::Abilities, F::FactionHint],
        ),
        EntityType::Location => (
            "location",
            vec![
                R::EstablishingShot,
                R::EnvironmentConcept,
                R::InteriorConcept,
            ],
            vec![
                F::Name,
                F::LocationProfile,
                F::Summary,
                F::VisualIdentity,
                F::Atmosphere,
                F::Purpose,
                F::NotableFeatures,
            ],
            vec![F::Danger],
        ),
        EntityType::Faction => (
            "faction",
            vec![R::Emblem, R::Banner, R::UniformConcept],
            vec![
                F::Name,
                F::FactionProfile,
                F::Identity,
                F::Culture,
                F::PublicFace,
                F::Symbols,
            ],
            vec![F::Ideology],
        ),
        EntityType::Item => (
            "item",
            vec![R::ItemConcept, R::InventoryIcon, R::PropSheet],
            vec![
                F::Name,
                F::ItemProfile,
                F::Appearance,
                F::Purpose,
                F::Properties,
                F::Origin,
                F::PowerOrFunction,
                F::Condition,
                F::Rarity,
            ],
            vec![],
        ),
        EntityType::Creature => (
            "creature",
            vec![R::CreatureConcept, R::FullBody, R::AnatomyDetail],
            vec![
                F::Name,
                F::CreatureProfile,
                F::Appearance,
                F::Habitat,
                F::Behavior,
                F::Ecology,
                F::Threat,
                F::AbilitiesSummary,
            ],
            vec![F::Weakness],
        ),
        _ => return vec![],
    };
    roles.into_iter().map(|role| {
        let (composition,aspect,transparent,environment)=match role {
            R::Portrait=>("Portrait emphasizing visible face and expression; do not invent anatomy.",AspectIntent::Portrait,false,false),
            R::FullBody=>("Show the complete subject with readable silhouette and consistent proportions.",AspectIntent::Portrait,false,false),
            R::CharacterSheet=>("A character reference sheet with consistent appearance across front, side and back views.",AspectIntent::Landscape,false,false),
            R::EstablishingShot=>("A wide establishing view showing the described place and notable features.",AspectIntent::Landscape,false,true),
            R::EnvironmentConcept=>("An environment concept with readable spatial relationships and described atmosphere.",AspectIntent::Landscape,false,true),
            R::InteriorConcept=>("Explore a plausible interior of the described place; do not replace its exterior truth.",AspectIntent::Landscape,false,true),
            R::Emblem=>("A legible emblem based on the described symbols; preserve specific motifs.",AspectIntent::Square,true,false),
            R::Banner=>("A readable banner carrying the described motifs; no invented authoritative insignia.",AspectIntent::Portrait,false,false),
            R::UniformConcept=>("A uniform concept exploring visible culture and symbols without inventing membership facts.",AspectIntent::Portrait,false,false),
            R::ItemConcept=>("An isolated item concept showing described construction and usable surfaces.",AspectIntent::Landscape,false,false),
            R::InventoryIcon=>("A single readable item silhouette suitable for a small inventory icon.",AspectIntent::Square,true,false),
            R::PropSheet=>("A prop reference sheet with consistent front, side and detail views.",AspectIntent::Landscape,false,false),
            R::CreatureConcept=>("An isolated creature concept with readable silhouette and described anatomy.",AspectIntent::Landscape,false,false),
            R::AnatomyDetail=>("A visible anatomy detail preserving the described creature's structure.",AspectIntent::Square,false,false),
        };
        VisualRecipeDefinition { recipe_id:format!("visual-{family}-{}-v1",role_key(role)),version:1,source_type:kind,role,required_fields:required.clone(),optional_fields:optional.clone(),composition_intent:composition.into(),aspect_intent:aspect,transparent_background_meaningful:transparent,environment_present:environment,forbidden_categories:vec!["secret".into(),"hidden_truth".into(),"story_hooks".into(),"notes".into(),"generation_constraints".into()] }
    }).collect()
}
fn role_key(role: R) -> String {
    serde_json::to_value(role)
        .expect("enum serialization")
        .as_str()
        .expect("enum key")
        .into()
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VisualProvenance {
    pub generation_kind: String,
    pub generator_version: u16,
    pub content_version: u16,
    pub preset: String,
    pub creative_axes: CreativeAxes,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VisualSourceReference {
    pub project_id: ProjectId,
    pub entity_id: EntityId,
    pub entity_type: EntityType,
    pub write_version: i64,
    pub canon_state: CanonState,
    pub canon_revision: Option<i64>,
    pub source_body_hash: String,
    pub generation: Option<VisualProvenance>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VisualContextSnapshot {
    pub format_version: u16,
    pub visual_recipe_id: String,
    pub visual_recipe_version: u16,
    pub source: VisualSourceReference,
    pub role: R,
    pub facts: Vec<VisualFact>,
    pub art_direction: ArtDirection,
    pub context_hash: String,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectKind {
    Human,
    Humanoid,
    Creature,
    Beast,
    Object,
    Location,
    FactionSymbolic,
    Unknown,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpeciesMode {
    Explicit,
    InferredDefault,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiteralizationRisk {
    Low,
    Medium,
    High,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SubjectClassification {
    pub subject_kind: SubjectKind,
    pub species_mode: SpeciesMode,
    pub literalization_risk: LiteralizationRisk,
    pub humanoid_guardrail: bool,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptSemantics {
    pub identity: Vec<String>,
    pub flavor_symbolism: Vec<String>,
    pub negative_guardrails: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VisualGenerationBrief {
    pub format_version: u16,
    pub subject_classification: SubjectClassification,
    pub prompt_semantics: PromptSemantics,
    pub safeguards_applied: Vec<String>,
    pub negative_prompt_text: String,
    pub context: VisualContextSnapshot,
    pub subject_description: String,
    pub must_include: Vec<VisualFact>,
    pub supporting_details: Vec<VisualFact>,
    pub composition_intent: String,
    pub aspect_intent: AspectIntent,
    pub background_intent: String,
    pub transparent_background_meaningful: bool,
    pub avoid: Vec<String>,
    pub consistency_notes: Vec<String>,
    pub prompt_text: String,
    pub brief_hash: String,
}
fn invalid() -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(
        ValidationCode::InvalidEntity,
        "visual_brief",
        "Visual brief requires a supported saved source, compatible role and bounded visible facts/direction",
    ))
}
/// Streaming canonical JSON hash with a hard 1 MiB source/brief limit.
pub fn visual_digest<T: Serialize>(value: &T) -> ValidationResult<String> {
    struct Sink {
        bytes: usize,
        hash: blake3::Hasher,
    }
    impl std::io::Write for Sink {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            if self.bytes.saturating_add(buf.len()) > 1_048_576 {
                return Err(std::io::Error::other("Visual source exceeds size limit"));
            }
            self.bytes += buf.len();
            self.hash.update(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut sink = Sink {
        bytes: 0,
        hash: blake3::Hasher::new(),
    };
    serde_json::to_writer(&mut sink, value).map_err(|_| invalid())?;
    Ok(sink.hash.finalize().to_hex().to_string())
}
fn direction_values(d: &ArtDirection) -> [(&'static str, &str); 6] {
    [
        ("Medium/style", &d.medium_style),
        ("Palette", &d.palette),
        ("Materials", &d.materials),
        ("Camera/composition", &d.camera_composition),
        ("Atmosphere", &d.atmosphere),
        ("Avoid", &d.avoid),
    ]
}
pub fn compile_visual_brief(
    entity: &VisualEntity,
    write_version: i64,
    canon_revision: Option<i64>,
    direction: ArtDirection,
    role: R,
) -> ValidationResult<VisualGenerationBrief> {
    entity.validate()?;
    let e = entity.envelope();
    if write_version <= 0
        || canon_revision.is_some_and(|n| n <= 0 || e.canon_state != CanonState::Canon)
        || !matches!(e.canon_state, CanonState::Candidate | CanonState::Canon)
        || direction_values(&direction).iter().any(|(_, s)| {
            s.len() > 512 || s.chars().any(|c| c.is_control() && c != '\n' && c != '\t')
        })
    {
        return Err(invalid());
    }
    let recipe = visual_recipes(e.entity_type)
        .into_iter()
        .find(|r| r.role == role)
        .ok_or_else(invalid)?;
    let mut facts = vec![VisualFact {
        field: F::Name,
        values: vec![e.name.clone()],
    }];
    let mut add = |field, values: Vec<String>| {
        facts.push(VisualFact { field, values });
    };
    macro_rules! text {
        ($field:ident,$value:expr) => {
            add(F::$field, vec![$value.clone()])
        };
    }
    match entity {
        VisualEntity::Character(c) => {
            text!(Title, c.title);
            text!(Archetype, c.archetype);
            text!(Role, c.role);
            text!(Appearance, c.appearance);
            text!(SignatureGear, c.signature_gear);
            add(F::Personality, c.personality.clone());
            add(F::Abilities, c.abilities.clone());
            text!(FactionHint, c.faction_hint);
        }
        VisualEntity::Location(c) => {
            add(
                F::LocationProfile,
                vec![
                    serde_json::to_value(c.location_profile)
                        .map_err(|_| invalid())?
                        .as_str()
                        .ok_or_else(invalid)?
                        .into(),
                ],
            );
            text!(Summary, c.summary);
            text!(VisualIdentity, c.visual_identity);
            text!(Atmosphere, c.atmosphere);
            text!(Purpose, c.purpose);
            add(F::NotableFeatures, c.notable_features.clone());
            text!(Danger, c.danger);
        }
        VisualEntity::Faction(c) => {
            add(
                F::FactionProfile,
                vec![
                    serde_json::to_value(c.faction_profile)
                        .map_err(|_| invalid())?
                        .as_str()
                        .ok_or_else(invalid)?
                        .into(),
                ],
            );
            text!(Identity, c.identity);
            text!(Culture, c.culture);
            text!(PublicFace, c.public_face);
            text!(Symbols, c.symbols);
            if role == R::UniformConcept {
                text!(Ideology, c.ideology);
            }
        }
        VisualEntity::Item(c) => {
            add(
                F::ItemProfile,
                vec![
                    serde_json::to_value(c.item_profile)
                        .map_err(|_| invalid())?
                        .as_str()
                        .ok_or_else(invalid)?
                        .into(),
                ],
            );
            text!(Appearance, c.appearance);
            text!(Purpose, c.purpose);
            text!(Properties, c.properties);
            text!(Origin, c.origin);
            text!(PowerOrFunction, c.power_or_function);
            text!(Condition, c.condition);
            text!(Rarity, c.rarity);
        }
        VisualEntity::Creature(c) => {
            add(
                F::CreatureProfile,
                vec![
                    serde_json::to_value(c.creature_profile)
                        .map_err(|_| invalid())?
                        .as_str()
                        .ok_or_else(invalid)?
                        .into(),
                ],
            );
            text!(Appearance, c.appearance);
            text!(Habitat, c.habitat);
            text!(Behavior, c.behavior);
            text!(Ecology, c.ecology);
            text!(Threat, c.threat);
            text!(AbilitiesSummary, c.abilities_summary);
            if role == R::AnatomyDetail
                && [
                    "eye", "limb", "bone", "scale", "skin", "wing", "fang", "shell", "joint",
                ]
                .iter()
                .any(|word| c.weakness.to_lowercase().contains(word))
            {
                text!(Weakness, c.weakness);
            }
        }
    }
    if facts.iter().any(|f| {
        f.values.is_empty()
            || f.values.len() > 16
            || f.values
                .iter()
                .any(|s| s.trim().is_empty() || s.len() > 4096)
    }) || serde_json::to_vec(&facts).map_err(|_| invalid())?.len() > 32768
    {
        return Err(invalid());
    }
    let generation = e.generation_provenance.as_ref().map(|p| VisualProvenance {
        generation_kind: serde_json::to_value(p.dna.recipe.kind)
            .expect("enum")
            .as_str()
            .expect("kind")
            .into(),
        generator_version: p.dna.generator_version.0,
        content_version: p.dna.content_version.0,
        preset: p.dna.recipe.universe_preset.as_str().into(),
        creative_axes: p.dna.recipe.creative_axes.clone(),
    });
    let context_hash = visual_digest(&(
        &recipe.recipe_id,
        recipe.version,
        role,
        &facts,
        &direction,
        &generation,
    ))?;
    let source = VisualSourceReference {
        project_id: e.project_id,
        entity_id: e.id,
        entity_type: e.entity_type,
        write_version,
        canon_state: e.canon_state,
        canon_revision,
        source_body_hash: visual_digest(entity)?,
        generation,
    };
    let must_include = facts
        .iter()
        .filter(|f| recipe.required_fields.contains(&f.field))
        .cloned()
        .collect();
    let supporting_details = facts
        .iter()
        .filter(|f| recipe.optional_fields.contains(&f.field))
        .cloned()
        .collect();
    let context = VisualContextSnapshot {
        format_version: 1,
        visual_recipe_id: recipe.recipe_id,
        visual_recipe_version: 1,
        source,
        role,
        facts,
        art_direction: direction,
        context_hash,
    };
    let (subject_classification, prompt_semantics, safeguards_applied) =
        compile_semantics(&context, &recipe.composition_intent);
    let negative_prompt_text = prompt_semantics.negative_guardrails.join(", ");
    let subject_description = prompt_semantics.identity[0].clone();
    let avoid = prompt_semantics.negative_guardrails.clone();
    let mut brief = VisualGenerationBrief {
        format_version: 3,
        subject_classification,
        prompt_semantics,
        safeguards_applied,
        negative_prompt_text,
        subject_description,
        context,
        must_include,
        supporting_details,
        composition_intent: recipe.composition_intent,
        aspect_intent: recipe.aspect_intent,
        background_intent: if recipe.environment_present {
            "Include the described environment."
        } else {
            "Isolate the subject against a neutral background; habitat is context, not a required scene."
        }.into(),
        transparent_background_meaningful: recipe.transparent_background_meaningful,
        avoid,
        consistency_notes: vec![
            "Source facts take precedence over optional art direction when they conflict.".into(),
            "Faction hints are design language, not authoritative membership, clothing or emblems.".into(),
            "Abilities describe the source; depict only observable cues without inventing anatomy.".into(),
        ],
        prompt_text: String::new(),
        brief_hash: String::new(),
    };
    brief.prompt_text = render_prompt(&brief);
    brief.brief_hash = visual_digest(&brief)?;
    Ok(brief)
}
fn scrub_identifiers(s: &str) -> String {
    let mut out = String::new();
    let mut index = 0;
    while index < s.len() {
        let tail = &s[index..];
        let uuid = tail.get(..36).is_some_and(|t| {
            [8, 13, 18, 23].iter().all(|&n| t.as_bytes()[n] == b'-')
                && t.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
        });
        let hash = tail
            .get(..64)
            .is_some_and(|t| t.chars().all(|c| c.is_ascii_hexdigit()));
        if uuid || hash {
            out.push_str("[identifier omitted]");
            index += if uuid { 36 } else { 64 };
        } else {
            let c = tail.chars().next().expect("remaining character");
            out.push(c);
            index += c.len_utf8();
        }
    }
    out
}
fn render_prompt(b: &VisualGenerationBrief) -> String {
    let mut lines = b.prompt_semantics.identity.clone();
    lines.extend(b.prompt_semantics.flavor_symbolism.clone());
    // Style/composition are last. Negative terms never enter the positive channel.
    lines.push(style_line(&b.context));
    lines.push(format!("Composition: {}", b.composition_intent));
    if b.context.source.entity_type == EntityType::Character {
        lines.push("Keep occupational tools, working garment details and identifying motifs readable within the chosen framing; role and distinctive appearance are the focal points.".into());
    }
    lines
        .into_iter()
        .map(|s| scrub_identifiers(&s))
        .collect::<Vec<_>>()
        .join("\n")
}
/// Metadata only: no paths, image bytes, database rows or Canon selection.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualAssetStatus {
    Candidate,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VisualAssetCandidate {
    pub status: VisualAssetStatus,
    pub asset_id: String,
    pub source: VisualSourceReference,
    pub role: R,
    pub recipe_id: String,
    pub recipe_version: u16,
    pub context_hash: String,
    pub brief_hash: String,
    pub provider: Option<String>,
    pub provider_model: Option<String>,
    pub seed: Option<u64>,
    pub content_hash: Option<String>,
    pub created_at_unix_ms: Option<i64>,
}
#[derive(Clone, Debug)]
pub struct ImageProviderCapabilities {
    pub aspects: Vec<AspectIntent>,
    pub transparent_background: bool,
    pub reference_images: bool,
    pub seed: bool,
    pub variation: bool,
}
/// Interface only. Provider-specific request/settings types live in future adapters.
pub trait ImageProviderAdapter {
    type Settings;
    type Request;
    type Output;
    type Error;
    fn capabilities(&self) -> ImageProviderCapabilities;
    fn compile_request(
        &self,
        brief: &VisualGenerationBrief,
        settings: &Self::Settings,
    ) -> Result<Self::Request, Self::Error>;
    fn generate(&self, request: Self::Request) -> Result<Self::Output, Self::Error>;
}

pub fn validate_visual_brief(brief: &VisualGenerationBrief) -> ValidationResult<()> {
    let mut unsigned = brief.clone();
    unsigned.brief_hash.clear();
    let recipe = visual_recipes(brief.context.source.entity_type)
        .into_iter()
        .find(|r| r.role == brief.context.role)
        .ok_or_else(invalid)?;
    let facts = &brief.context.facts;
    if facts.len() > 16
        || facts.iter().enumerate().any(|(index, f)| {
            facts[..index].iter().any(|prior| prior.field == f.field)
                || (!recipe.required_fields.contains(&f.field)
                    && !recipe.optional_fields.contains(&f.field))
                || f.values.is_empty()
                || f.values.len() > 16
                || f.values
                    .iter()
                    .any(|v| v.trim().is_empty() || v.len() > 4096)
        })
        || !recipe
            .required_fields
            .iter()
            .all(|key| facts.iter().any(|f| f.field == *key))
    {
        return Err(invalid());
    }
    let must_include: Vec<_> = facts
        .iter()
        .filter(|f| recipe.required_fields.contains(&f.field))
        .cloned()
        .collect();
    let supporting_details: Vec<_> = facts
        .iter()
        .filter(|f| recipe.optional_fields.contains(&f.field))
        .cloned()
        .collect();
    if brief.must_include != must_include
        || brief.supporting_details != supporting_details
        || brief.composition_intent != recipe.composition_intent
        || brief.aspect_intent != recipe.aspect_intent
        || brief.transparent_background_meaningful != recipe.transparent_background_meaningful
    {
        return Err(invalid());
    }
    let expected_context_hash = visual_digest(&(
        &recipe.recipe_id,
        recipe.version,
        brief.context.role,
        &brief.context.facts,
        &brief.context.art_direction,
        &brief.context.source.generation,
    ))?;
    if brief.context.visual_recipe_id != recipe.recipe_id
        || brief.context.visual_recipe_version != recipe.version
        || brief.context.context_hash != expected_context_hash
    {
        return Err(invalid());
    }
    let (classification, semantics, safeguards) =
        compile_semantics(&brief.context, &brief.composition_intent);
    if brief.subject_classification != classification
        || brief.prompt_semantics != semantics
        || brief.safeguards_applied != safeguards
        || brief.avoid != semantics.negative_guardrails
        || brief.negative_prompt_text != semantics.negative_guardrails.join(", ")
        || brief.subject_description != semantics.identity[0]
    {
        return Err(invalid());
    }
    if brief.format_version != 3
        || brief.context.format_version != 1
        || brief.brief_hash != visual_digest(&unsigned)?
        || brief.prompt_text != render_prompt(brief)
    {
        return Err(invalid());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CharacterForge, CreatureForge, FactionForge, ItemForge, LocationForge};
    use forge_contracts::{CreativeAxes, RootSeed};
    use std::collections::BTreeMap;
    fn entities() -> Vec<VisualEntity> {
        let seed = RootSeed::new(42);
        let axes = CreativeAxes::neutral();
        vec![
            VisualEntity::Character(Box::new(
                CharacterForge::generate(
                    &CharacterForge::new_dna(seed, "nordic-myth", axes.clone(), BTreeMap::new())
                        .unwrap(),
                )
                .unwrap()
                .character,
            )),
            VisualEntity::Location(Box::new(
                LocationForge::generate(
                    &LocationForge::new_dna(seed, "nordic-myth", axes.clone()).unwrap(),
                )
                .unwrap()
                .location,
            )),
            VisualEntity::Faction(Box::new(
                FactionForge::generate(
                    &FactionForge::new_dna(seed, "nordic-myth", axes.clone()).unwrap(),
                )
                .unwrap()
                .faction,
            )),
            VisualEntity::Item(Box::new(
                ItemForge::generate(
                    &ItemForge::new_dna(seed, "nordic-myth", axes.clone()).unwrap(),
                )
                .unwrap()
                .item,
            )),
            VisualEntity::Creature(Box::new(
                CreatureForge::generate(
                    &CreatureForge::new_dna(seed, "nordic-myth", axes).unwrap(),
                )
                .unwrap()
                .creature,
            )),
        ]
    }
    #[test]
    fn visual_roles_and_fifteen_recipes_are_stable_and_family_specific() {
        let mut total = 0;
        for entity in entities() {
            let recipes = visual_recipes(entity.envelope().entity_type);
            assert_eq!(recipes.len(), 3);
            for r in recipes {
                total += 1;
                assert_eq!(r.version, 1);
                assert!(r.recipe_id.ends_with("-v1"));
                let key = role_key(r.role);
                assert_eq!(
                    serde_json::from_str::<VisualRole>(&format!("\"{key}\"")).unwrap(),
                    r.role
                );
                assert!(r.required_fields.contains(&F::Name));
                assert!(r.forbidden_categories.contains(&"secret".into()));
            }
        }
        assert_eq!(total, 15);
        assert!(visual_recipes(EntityType::Quest).is_empty());
        assert!(serde_json::from_str::<VisualRole>("\"arbitrary\"").is_err());
        assert!(
            compile_visual_brief(
                &entities()[0],
                1,
                None,
                ArtDirection::default(),
                R::InventoryIcon
            )
            .is_err()
        );
    }
    #[test]
    fn visual_all_families_all_roles_replay_read_only_with_exact_facts_and_safe_prompt() {
        for entity in entities() {
            let before = entity.clone();
            for recipe in visual_recipes(entity.envelope().entity_type) {
                let brief =
                    compile_visual_brief(&entity, 7, None, ArtDirection::default(), recipe.role)
                        .unwrap();
                assert_eq!(
                    brief,
                    compile_visual_brief(&entity, 7, None, ArtDirection::default(), recipe.role)
                        .unwrap()
                );
                assert_eq!(brief.context.source.entity_id, entity.envelope().id);
                assert_eq!(brief.context.source.write_version, 7);
                assert_eq!(brief.context.source.canon_revision, None);
                assert_eq!(
                    brief.context.facts[0].values,
                    [entity.envelope().name.clone()]
                );
                assert_eq!(brief.context.visual_recipe_id, recipe.recipe_id);
                let value = serde_json::to_value(&entity).unwrap();
                for fact in &brief.context.facts {
                    let key = serde_json::to_value(fact.field).unwrap();
                    let key = key.as_str().unwrap();
                    if key == "name" {
                        continue;
                    }
                    let expected = &value["value"][key];
                    let values = if let Some(list) = expected.as_array() {
                        list.iter()
                            .map(|x| x.as_str().unwrap().to_string())
                            .collect::<Vec<_>>()
                    } else {
                        vec![expected.as_str().unwrap().to_string()]
                    };
                    assert_eq!(fact.values, values, "{key}");
                }
                assert!(
                    !brief
                        .prompt_text
                        .contains(&entity.envelope().id.uuid().to_string())
                );
                assert!(
                    !brief
                        .prompt_text
                        .contains(&brief.context.source.source_body_hash)
                );
                assert!(!brief.prompt_text.contains(&brief.context.context_hash));
                assert!(
                    !brief.prompt_text.contains(
                        value["value"]["secret"]
                            .as_str()
                            .unwrap_or("SECRET_SENTINEL")
                    )
                );
                assert!(
                    !brief.prompt_text.contains(
                        value["value"]["hidden_truth"]
                            .as_str()
                            .unwrap_or("HIDDEN_SENTINEL")
                    )
                );
                validate_visual_brief(&brief).unwrap();
                if entity.envelope().entity_type == EntityType::Creature {
                    assert!(
                        !brief
                            .context
                            .facts
                            .iter()
                            .any(|f| serde_json::to_value(f.field).unwrap() == "signs_of_presence")
                    );
                }
            }
            assert_eq!(entity, before);
        }
    }
    #[test]
    fn visual_secret_changes_preserve_visual_semantics_but_exact_source_hash_changes() {
        for mut entity in entities() {
            let role = visual_recipes(entity.envelope().entity_type)[0].role;
            let original =
                compile_visual_brief(&entity, 1, None, ArtDirection::default(), role).unwrap();
            match &mut entity {
                VisualEntity::Character(x) => x.secret = "SECRET_SENTINEL".into(),
                VisualEntity::Location(x) => x.secret = "SECRET_SENTINEL".into(),
                VisualEntity::Faction(x) => x.hidden_truth = "SECRET_SENTINEL".into(),
                VisualEntity::Item(x) => x.secret = "SECRET_SENTINEL".into(),
                VisualEntity::Creature(x) => x.secret = "SECRET_SENTINEL".into(),
            }
            let changed =
                compile_visual_brief(&entity, 2, None, ArtDirection::default(), role).unwrap();
            assert_eq!(original.context.facts, changed.context.facts);
            assert_eq!(original.context.context_hash, changed.context.context_hash);
            assert_eq!(original.prompt_text, changed.prompt_text);
            assert_ne!(
                original.context.source.source_body_hash,
                changed.context.source.source_body_hash
            );
            assert_ne!(original.brief_hash, changed.brief_hash);
            assert!(
                !serde_json::to_string(&changed)
                    .unwrap()
                    .contains("SECRET_SENTINEL")
            );
        }
    }
    #[test]
    fn visual_visible_fact_and_direction_changes_hash_and_prompt_stays_bounded() {
        let mut entity = entities().remove(0);
        let original =
            compile_visual_brief(&entity, 1, None, ArtDirection::default(), R::Portrait).unwrap();
        if let VisualEntity::Character(c) = &mut entity {
            c.appearance = "Visible silver hair".into();
        }
        assert_ne!(
            original.context.context_hash,
            compile_visual_brief(&entity, 1, None, ArtDirection::default(), R::Portrait)
                .unwrap()
                .context
                .context_hash
        );
        let direction = ArtDirection {
            palette: "Iron and amber".into(),
            ..ArtDirection::default()
        };
        let directed = compile_visual_brief(&entity, 1, None, direction, R::Portrait).unwrap();
        assert!(directed.prompt_text.contains("Iron and amber"));
        assert_ne!(directed.context.context_hash, original.context.context_hash);
        assert!(
            compile_visual_brief(&entity, 0, None, ArtDirection::default(), R::Portrait).is_err()
        );
        assert!(
            compile_visual_brief(&entity, 1, Some(1), ArtDirection::default(), R::Portrait)
                .is_err()
        );
        assert!(
            compile_visual_brief(
                &entity,
                1,
                None,
                ArtDirection {
                    palette: "x".repeat(513),
                    ..ArtDirection::default()
                },
                R::Portrait
            )
            .is_err()
        );
        if let VisualEntity::Character(c) = &mut entity {
            c.personality = vec!["trait".into(); 17];
        }
        assert!(
            compile_visual_brief(&entity, 1, None, ArtDirection::default(), R::Portrait).is_err()
        );
    }
    fn physician(appearance: &str) -> VisualEntity {
        let mut c = CharacterForge::generate(
            &CharacterForge::new_dna(
                RootSeed::new(42),
                "dark-fantasy",
                CreativeAxes::neutral(),
                BTreeMap::new(),
            )
            .unwrap(),
        )
        .unwrap()
        .character;
        c.envelope.name = "Keeper of the Unlit Mile".into();
        c.title = "Keeper of the Unlit Mile".into();
        c.archetype = "Chapel Rat Physician".into();
        c.role = "physician".into();
        c.appearance = appearance.into();
        VisualEntity::Character(Box::new(c))
    }
    #[test]
    fn physician_metaphor_is_human_with_separate_negative_guards_and_ordered_prompt() {
        let source =
            physician("A scarred human physician with silver hair and a patched dark coat");
        let before = source.clone();
        let brief =
            compile_visual_brief(&source, 1, None, ArtDirection::default(), R::Portrait).unwrap();
        assert_eq!(
            brief.subject_classification.subject_kind,
            SubjectKind::Human
        );
        assert!(brief.subject_classification.humanoid_guardrail);
        assert_eq!(
            brief.subject_classification.literalization_risk,
            LiteralizationRisk::High
        );
        assert!(
            brief
                .prompt_text
                .starts_with("human dark fantasy physician\nRole / profession:")
        );
        assert!(brief.prompt_text.contains("subtle rat symbolism"));
        for cue in [
            "medical instrument roll",
            "folded bandages",
            "stoppered medicine bottles",
            "practical treatment apron",
            "worn cuffs",
            "medicinal stains",
            "chapel-associated garment accents",
            "extinguished lamp",
            "candle-soot",
            "roadside mile-marker",
            "waystation",
            "tool-case ornament",
        ] {
            assert!(
                brief.prompt_text.contains(cue),
                "Missing specificity cue: {cue}"
            );
        }
        assert!(
            brief
                .negative_prompt_text
                .contains("anonymous glamour headshot")
        );
        assert!(
            brief
                .negative_prompt_text
                .contains("nun-only portrait without occupational tools")
        );
        assert!(!brief.prompt_text.contains("beautiful cinematic portrait"));
        for guard in [
            "literal rat",
            "anthropomorphic rodent",
            "animal head",
            "mascot",
            "parody",
        ] {
            assert!(brief.negative_prompt_text.contains(guard));
            assert!(!brief.prompt_text.contains(guard));
        }
        let mut last = 0;
        for section in [
            "Role / profession:",
            "Physical identity:",
            "Clothing / equipment:",
            "Setting (",
            "Symbolic motifs:",
            "Style:",
            "Composition:",
        ] {
            let at = brief.prompt_text.find(section).unwrap();
            assert!(at > last);
            last = at;
        }
        assert_eq!(source, before);
        assert_eq!(
            brief,
            compile_visual_brief(&source, 1, None, ArtDirection::default(), R::Portrait).unwrap()
        );
        assert_eq!(brief.format_version, 3);
        validate_visual_brief(&brief).unwrap();
    }
    #[test]
    fn public_title_and_archetype_change_props_and_scene_not_only_labels() {
        let original = physician("A human with silver hair and a patched dark coat");
        let mut changed = original.clone();
        if let VisualEntity::Character(c) = &mut changed {
            c.envelope.name = "Bell Physician".into();
            c.title = "Bell Keeper".into();
            c.archetype = "Street Physician".into();
        }
        let a =
            compile_visual_brief(&original, 1, None, ArtDirection::default(), R::Portrait).unwrap();
        let b =
            compile_visual_brief(&changed, 1, None, ArtDirection::default(), R::Portrait).unwrap();
        assert_eq!(
            a.subject_classification.subject_kind,
            b.subject_classification.subject_kind
        );
        assert_ne!(a.prompt_semantics.identity, b.prompt_semantics.identity);
        assert!(a.prompt_text.contains("chapel stone arch"));
        assert!(a.prompt_text.contains("extinguished lamp"));
        assert!(a.prompt_text.contains("parish-route fragment"));
        assert!(!b.prompt_text.contains("chapel stone arch"));
        assert!(!b.prompt_text.contains("extinguished lamp"));
        assert!(!b.prompt_text.contains("mile-marker"));
        assert!(b.prompt_text.contains("brass clapper detail"));
        assert!(a.prompt_text.contains("medical instrument roll"));
        assert!(b.prompt_text.contains("medical instrument roll"));
        assert_ne!(a.brief_hash, b.brief_hash);
        assert_ne!(a.context.context_hash, b.context.context_hash);
        validate_visual_brief(&a).unwrap();
        validate_visual_brief(&b).unwrap();
    }
    #[test]
    fn occupation_cues_are_specific_read_only_and_derived_hash_validated() {
        for (occupation, cue) in [
            ("apothecary", "medicine bottles"),
            ("engineer", "measuring tools"),
            ("scholar", "annotated pages"),
            ("detective", "evidence notebook"),
            ("courier", "message case"),
            ("unlisted occupation", "source-described signature gear"),
        ] {
            let mut source = physician("A human with a scar and silver hair");
            if let VisualEntity::Character(c) = &mut source {
                c.envelope.name = "Test Worker".into();
                c.title = "Test Worker".into();
                c.archetype = occupation.into();
                c.role = occupation.into();
                c.signature_gear = "Named iron cane with a red wrap".into();
            }
            let before = source.clone();
            let brief =
                compile_visual_brief(&source, 1, None, ArtDirection::default(), R::FullBody)
                    .unwrap();
            assert!(brief.prompt_text.contains(cue), "{occupation}");
            assert!(
                brief
                    .prompt_text
                    .contains("Named iron cane with a red wrap")
            );
            assert!(brief.prompt_text.contains("visual interpretation"));
            assert!(brief.prompt_text.contains("Keep occupational tools"));
            assert_eq!(source, before);
            let gear = brief
                .context
                .facts
                .iter()
                .find(|f| f.field == F::SignatureGear)
                .unwrap();
            assert_eq!(gear.values, ["Named iron cane with a red wrap"]);
            assert_eq!(
                brief,
                compile_visual_brief(&source, 1, None, ArtDirection::default(), R::FullBody)
                    .unwrap()
            );
            validate_visual_brief(&brief).unwrap();
            let mut forged = brief;
            forged
                .prompt_semantics
                .identity
                .push("arbitrary canonical uniform".into());
            forged.prompt_text = render_prompt(&forged);
            forged.brief_hash.clear();
            forged.brief_hash = visual_digest(&forged).unwrap();
            assert!(validate_visual_brief(&forged).is_err());
        }
    }
    #[test]
    fn specificity_negatives_preserve_actual_clergy_and_explicit_style() {
        let ordinary = physician("A human with silver hair and a patched coat");
        let brief =
            compile_visual_brief(&ordinary, 1, None, ArtDirection::default(), R::Portrait).unwrap();
        assert!(brief.negative_prompt_text.contains("nun-only"));
        assert!(brief.negative_prompt_text.contains("monk-only"));
        for clerical_field in [F::Title, F::Archetype, F::Role, F::Appearance] {
            let mut source = ordinary.clone();
            if let VisualEntity::Character(c) = &mut source {
                match clerical_field {
                    F::Title => c.title = "Chapel Nun".into(),
                    F::Archetype => c.archetype = "Chapel Priest".into(),
                    F::Role => c.role = "physician and monk".into(),
                    F::Appearance => c.appearance = "A human nun wearing a habit".into(),
                    _ => unreachable!(),
                }
            }
            let b = compile_visual_brief(&source, 1, None, ArtDirection::default(), R::Portrait)
                .unwrap();
            assert!(!b.negative_prompt_text.contains("nun-only"));
            assert!(!b.negative_prompt_text.contains("monk-only"));
        }
        let styled = compile_visual_brief(
            &ordinary,
            1,
            None,
            ArtDirection {
                medium_style: "beautiful fashion portrait".into(),
                ..Default::default()
            },
            R::Portrait,
        )
        .unwrap();
        assert!(!styled.negative_prompt_text.contains("anonymous glamour"));
        assert!(!styled.negative_prompt_text.contains("fashion editorial"));
        assert!(styled.negative_prompt_text.contains("literal rat"));
        assert!(
            styled.prompt_text.find("medical instrument roll").unwrap()
                < styled
                    .prompt_text
                    .find("beautiful fashion portrait")
                    .unwrap()
        );
    }
    #[test]
    fn risky_character_epithets_and_animal_costumes_do_not_define_species() {
        for word in [
            "rat", "raven", "crow", "wolf", "serpent", "spider", "beast", "ghost", "bone",
            "plague", "carrion", "ash", "butcher",
        ] {
            let mut source = physician(
                "Silver hair, a raven-haired look and a rat-headed mask with a wolf emblem",
            );
            if let VisualEntity::Character(c) = &mut source {
                c.archetype = format!("Chapel {word} Physician");
            }
            let brief = compile_visual_brief(
                &source,
                1,
                None,
                ArtDirection {
                    medium_style: "anthropomorphic rat portrait".into(),
                    ..Default::default()
                },
                R::Portrait,
            )
            .unwrap();
            assert_eq!(
                brief.subject_classification.subject_kind,
                SubjectKind::Human
            );
            assert_eq!(
                brief.subject_classification.species_mode,
                SpeciesMode::InferredDefault
            );
            assert!(brief.subject_classification.humanoid_guardrail);
            assert!(
                brief
                    .prompt_text
                    .contains(&format!("subtle {word} symbolism"))
            );
        }
        let mut source = physician("Silver hair and a patched coat");
        if let VisualEntity::Character(c) = &mut source {
            c.archetype = "Crown Physician".into();
        }
        let brief =
            compile_visual_brief(&source, 1, None, ArtDirection::default(), R::Portrait).unwrap();
        assert_eq!(
            brief.subject_classification.literalization_risk,
            LiteralizationRisk::Low
        );
        assert!(!brief.negative_prompt_text.contains("literal crow"));
    }
    #[test]
    fn explicit_nonhuman_character_appearance_preserves_literal_form() {
        for (appearance, kind) in [
            (
                "An elf with pointed ears and silver hair",
                SubjectKind::Humanoid,
            ),
            ("Elven physician with long ears", SubjectKind::Humanoid),
            (
                "An anthropomorphic rat with fur and a rodent head",
                SubjectKind::Humanoid,
            ),
            (
                "A rat-headed physician with whiskers and fur",
                SubjectKind::Humanoid,
            ),
            ("A rat with fur and whiskers", SubjectKind::Beast),
            (
                "Species: mothkin; compound eyes and four arms",
                SubjectKind::Unknown,
            ),
            (
                "A werewolf with fur and a lupine muzzle",
                SubjectKind::Humanoid,
            ),
        ] {
            let brief = compile_visual_brief(
                &physician(appearance),
                1,
                None,
                ArtDirection::default(),
                R::Portrait,
            )
            .unwrap();
            assert_eq!(
                brief.subject_classification.subject_kind, kind,
                "{appearance}"
            );
            assert_eq!(
                brief.subject_classification.species_mode,
                SpeciesMode::Explicit
            );
            assert!(!brief.subject_classification.humanoid_guardrail);
            assert!(brief.prompt_text.contains(appearance));
            assert!(!brief.negative_prompt_text.contains("literal rat"));
            assert!(!brief.negative_prompt_text.contains("animal head"));
            validate_visual_brief(&brief).unwrap();
        }
    }
    #[test]
    fn literal_creatures_and_all_family_prompt_structures_remain_distinct() {
        let mut sources = entities();
        if let VisualEntity::Creature(c) = &mut sources[4] {
            c.appearance = "A literal giant rat with fur, whiskers and a long tail".into();
        }
        for (source, role, kind, section) in [
            (
                &sources[0],
                R::Portrait,
                SubjectKind::Human,
                "Physical identity:",
            ),
            (
                &sources[1],
                R::EstablishingShot,
                SubjectKind::Location,
                "Environment / architecture:",
            ),
            (
                &sources[2],
                R::Emblem,
                SubjectKind::FactionSymbolic,
                "Symbols / emblem / representative design:",
            ),
            (
                &sources[3],
                R::ItemConcept,
                SubjectKind::Object,
                "Object physical form / material:",
            ),
            (
                &sources[4],
                R::CreatureConcept,
                SubjectKind::Creature,
                "Literal creature anatomy:",
            ),
        ] {
            let brief =
                compile_visual_brief(source, 1, None, ArtDirection::default(), role).unwrap();
            assert_eq!(brief.subject_classification.subject_kind, kind);
            assert!(brief.prompt_text.contains(section));
            assert!(brief.prompt_text.contains("clear concept illustration"));
            if kind != SubjectKind::Human {
                assert!(!brief.subject_classification.humanoid_guardrail);
                assert!(!brief.negative_prompt_text.contains("animal head"));
                assert!(
                    !brief
                        .prompt_text
                        .contains("Role evidence (visual interpretation")
                );
                assert!(!brief.prompt_text.contains("Keep occupational tools"));
                assert!(!brief.negative_prompt_text.contains("anonymous glamour"));
            }
            validate_visual_brief(&brief).unwrap();
        }
    }
    #[test]
    fn negative_direction_is_separate_redacted_and_hash_verified() {
        let source = physician("Silver hair and patched coat");
        let original =
            compile_visual_brief(&source, 1, None, ArtDirection::default(), R::Portrait).unwrap();
        let direction = ArtDirection {
            avoid: format!(
                "oversaturation {} {}",
                "a".repeat(64),
                source.envelope().id.uuid()
            ),
            ..Default::default()
        };
        let brief = compile_visual_brief(&source, 1, None, direction, R::Portrait).unwrap();
        assert!(brief.negative_prompt_text.contains("oversaturation"));
        assert!(!brief.prompt_text.contains("oversaturation"));
        assert!(!brief.negative_prompt_text.contains(&"a".repeat(64)));
        assert!(
            !brief
                .negative_prompt_text
                .contains(&source.envelope().id.uuid().to_string())
        );
        assert_ne!(brief.brief_hash, original.brief_hash);
        let mut forged = brief;
        forged.subject_classification.subject_kind = SubjectKind::Beast;
        forged.brief_hash.clear();
        forged.brief_hash = visual_digest(&forged).unwrap();
        assert!(validate_visual_brief(&forged).is_err());
    }

    #[test]
    fn visual_prompt_redacts_identifiers_in_visible_prose_and_operator_direction() {
        let entity = entities().remove(0);
        let uuid = entity.envelope().id.uuid().to_string();
        let hash = "a".repeat(64);
        let brief = compile_visual_brief(
            &entity,
            1,
            None,
            ArtDirection {
                materials: format!("f{uuid} {hash} Ελληνικά"),
                ..ArtDirection::default()
            },
            R::Portrait,
        )
        .unwrap();
        assert!(!brief.prompt_text.contains(&uuid));
        assert!(!brief.prompt_text.contains(&hash));
        assert!(brief.prompt_text.contains("Ελληνικά"));
    }
    #[test]
    fn visual_size_limits_and_self_hashed_invalid_contexts_are_rejected() {
        assert!(visual_digest(&"x".repeat(1_048_577)).is_err());
        let entity = entities().remove(0);
        let original =
            compile_visual_brief(&entity, 1, None, ArtDirection::default(), R::Portrait).unwrap();
        let mut forged = original.clone();
        forged.context.context_hash = "wrong".into();
        forged.brief_hash.clear();
        forged.brief_hash = visual_digest(&forged).unwrap();
        assert!(validate_visual_brief(&forged).is_err());
        let mut forged = original;
        forged.context.facts.push(VisualFact {
            field: F::Threat,
            values: vec!["Unrelated invented detail".into()],
        });
        forged.brief_hash.clear();
        forged.brief_hash = visual_digest(&forged).unwrap();
        assert!(validate_visual_brief(&forged).is_err());
    }
    #[test]
    fn visual_contextual_faction_and_item_never_dump_private_forge_dna_constraints() {
        let mut entity = entities().remove(0);
        let VisualEntity::Character(c) = &mut entity else {
            panic!("Character fixture")
        };
        c.secret = "PRIVATE_CONTEXT_SENTINEL".into();
        let source = crate::HometownSource {
            character: (**c).clone(),
            field_origins: BTreeMap::new(),
            write_version: 1,
        };
        let faction = crate::forge_character_faction(&source, 42, c.envelope.project_id).unwrap();
        let item =
            crate::forge_character_signature_item(&source, 42, c.envelope.project_id).unwrap();
        for (entity, role) in [
            (
                VisualEntity::Faction(Box::new(faction.primary.faction)),
                R::Emblem,
            ),
            (
                VisualEntity::Item(Box::new(item.primary.item)),
                R::ItemConcept,
            ),
        ] {
            let brief =
                compile_visual_brief(&entity, 1, None, ArtDirection::default(), role).unwrap();
            assert!(
                !serde_json::to_string(&brief)
                    .unwrap()
                    .contains("PRIVATE_CONTEXT_SENTINEL")
            );
            assert_eq!(
                visual_digest(&entity).unwrap(),
                blake3::hash(&serde_json::to_vec(&entity).unwrap())
                    .to_hex()
                    .to_string()
            );
        }
    }
}
