use forge_app_core::{CharacterCandidate, CharacterForge};
use forge_contracts::{CreativeAxes, CreativeAxis, RootSeed};
use serde::Serialize;
use std::collections::BTreeMap;

const PRESETS: [&str; 11] = [
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

#[derive(Serialize)]
struct AuditReport {
    method: String,
    presets: Vec<PresetAudit>,
    chaos_extended: PresetAudit,
    controlled_axis_comparisons: Vec<AxisComparison>,
}

#[derive(Serialize)]
struct PresetAudit {
    preset: String,
    seeds: usize,
    fields: Vec<FieldAudit>,
    list_items: Vec<FieldAudit>,
    similarity: SimilarityAudit,
    archetype_clusters: Vec<ArchetypeCluster>,
}

#[derive(Serialize)]
struct FieldAudit {
    field: String,
    observations: usize,
    distinct_values: usize,
    top_values: Vec<ValueCount>,
}

#[derive(Serialize)]
struct ValueCount {
    value: String,
    count: usize,
}

#[derive(Serialize)]
struct SimilarityAudit {
    compared_field_count: usize,
    pair_count: usize,
    mean_equal_fields: f64,
    max_equal_fields: usize,
    pairs_equal_8_or_more: usize,
    pairs_equal_10_or_more: usize,
    identical_character_pairs: usize,
}

#[derive(Serialize)]
struct ArchetypeCluster {
    archetype: String,
    character_count: usize,
    distinct_values: BTreeMap<String, usize>,
}

#[derive(Serialize)]
struct AxisComparison {
    preset: String,
    axis: String,
    seeds: usize,
    low: u8,
    high: u8,
    changed_fields: BTreeMap<String, usize>,
    archetype_counts_low: BTreeMap<String, usize>,
    archetype_counts_high: BTreeMap<String, usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args()
        .nth(1)
        .ok_or("pass an output path; metrics are written as UTF-8 JSON without shell decoding")?;
    let presets = PRESETS
        .iter()
        .map(|preset| audit_preset(preset, 256))
        .collect::<Result<Vec<_>, _>>()?;
    let chaos_extended = audit_preset("chaos-blend", 1024)?;
    let controlled_axis_comparisons = vec![
        compare_axis("cyberpunk", CreativeAxis::Technology, 256)?,
        compare_axis("chaos-blend", CreativeAxis::Chaos, 256)?,
    ];
    let report = AuditReport {
        method: "Sequential root seeds starting at 0; neutral axes except the listed controlled comparison; exact field-value equality for multi-field similarity; top three values per field, ordered by frequency then lexical value.".into(),
        presets,
        chaos_extended,
        controlled_axis_comparisons,
    };
    std::fs::write(output, serde_json::to_vec_pretty(&report)?)?;
    Ok(())
}

fn generate(
    preset: &str,
    seed: u64,
    axis: Option<(CreativeAxis, u8)>,
) -> Result<CharacterCandidate, Box<dyn std::error::Error>> {
    let mut axes = CreativeAxes::neutral();
    if let Some((axis, level)) = axis {
        axes.levels.insert(axis, level);
    }
    let dna = CharacterForge::new_dna(RootSeed::new(seed), preset, axes, BTreeMap::new())?;
    Ok(CharacterForge::generate(&dna)?)
}

fn audit_preset(preset: &str, seeds: usize) -> Result<PresetAudit, Box<dyn std::error::Error>> {
    let candidates = (0..seeds)
        .map(|seed| generate(preset, seed as u64, None))
        .collect::<Result<Vec<_>, _>>()?;
    let mut fields: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut list_items: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut archetypes: BTreeMap<String, Vec<&CharacterCandidate>> = BTreeMap::new();
    let mut identities = Vec::with_capacity(candidates.len());

    for candidate in &candidates {
        let character = &candidate.character;
        let scalar_and_bundle_values = [
            ("name", character.envelope.name.clone()),
            ("archetype", character.archetype.clone()),
            ("role", character.role.clone()),
            ("title", character.title.clone()),
            ("appearance", character.appearance.clone()),
            ("faction_hint", character.faction_hint.clone()),
            ("signature_gear", character.signature_gear.clone()),
            ("desire", character.desire.clone()),
            ("fear", character.fear.clone()),
            ("flaw", character.flaw.clone()),
            ("secret", character.secret.clone()),
            ("relationship_hook", character.relationship_hook.clone()),
            ("backstory_hook", character.backstory_hook.clone()),
            (
                "personality_bundle",
                serde_json::to_string(&character.personality)?,
            ),
            ("skills_bundle", serde_json::to_string(&character.skills)?),
            (
                "abilities_bundle",
                serde_json::to_string(&character.abilities)?,
            ),
            (
                "story_hooks_bundle",
                serde_json::to_string(&character.story_hooks)?,
            ),
        ];
        let mut identity = Vec::with_capacity(scalar_and_bundle_values.len());
        for (field, value) in scalar_and_bundle_values {
            fields.entry(field.into()).or_default().push(value.clone());
            identity.push(value);
        }
        identities.push(identity);
        for (field, values) in [
            ("personality_traits", &character.personality),
            ("skills", &character.skills),
            ("abilities", &character.abilities),
            ("story_hooks", &character.story_hooks),
        ] {
            list_items
                .entry(field.into())
                .or_default()
                .extend(values.iter().cloned());
        }
        archetypes
            .entry(character.archetype.clone())
            .or_default()
            .push(candidate);
    }

    let fields = fields
        .into_iter()
        .map(|(field, values)| field_audit(field, values))
        .collect();
    let list_items = list_items
        .into_iter()
        .map(|(field, values)| field_audit(field, values))
        .collect();
    let archetype_clusters = archetypes
        .into_iter()
        .map(|(archetype, group)| {
            let mut distinct_values = BTreeMap::new();
            for (field, values) in [
                (
                    "personality_bundle",
                    group
                        .iter()
                        .map(|c| serde_json::to_string(&c.character.personality))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                (
                    "skills_bundle",
                    group
                        .iter()
                        .map(|c| serde_json::to_string(&c.character.skills))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                (
                    "ability",
                    group
                        .iter()
                        .map(|c| serde_json::to_string(&c.character.abilities))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                (
                    "fear",
                    group
                        .iter()
                        .map(|c| Ok(c.character.fear.clone()))
                        .collect::<Result<Vec<_>, serde_json::Error>>()?,
                ),
                (
                    "flaw",
                    group
                        .iter()
                        .map(|c| Ok(c.character.flaw.clone()))
                        .collect::<Result<Vec<_>, serde_json::Error>>()?,
                ),
                (
                    "secret",
                    group
                        .iter()
                        .map(|c| Ok(c.character.secret.clone()))
                        .collect::<Result<Vec<_>, serde_json::Error>>()?,
                ),
                (
                    "relationship_hook",
                    group
                        .iter()
                        .map(|c| Ok(c.character.relationship_hook.clone()))
                        .collect::<Result<Vec<_>, serde_json::Error>>()?,
                ),
                (
                    "backstory_hook",
                    group
                        .iter()
                        .map(|c| Ok(c.character.backstory_hook.clone()))
                        .collect::<Result<Vec<_>, serde_json::Error>>()?,
                ),
                (
                    "story_hooks_bundle",
                    group
                        .iter()
                        .map(|c| serde_json::to_string(&c.character.story_hooks))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
            ] {
                distinct_values.insert(field.into(), unique_count(&values));
            }
            Ok::<_, Box<dyn std::error::Error>>(ArchetypeCluster {
                archetype,
                character_count: group.len(),
                distinct_values,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PresetAudit {
        preset: preset.into(),
        seeds,
        fields,
        list_items,
        similarity: similarity_audit(&identities),
        archetype_clusters,
    })
}

fn field_audit(field: String, values: Vec<String>) -> FieldAudit {
    let mut counts = BTreeMap::<String, usize>::new();
    for value in &values {
        *counts.entry(value.clone()).or_default() += 1;
    }
    let mut top_values: Vec<_> = counts
        .iter()
        .map(|(value, count)| ValueCount {
            value: value.clone(),
            count: *count,
        })
        .collect();
    top_values.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| left.value.cmp(&right.value))
    });
    top_values.truncate(3);
    FieldAudit {
        field,
        observations: values.len(),
        distinct_values: counts.len(),
        top_values,
    }
}

fn unique_count(values: &[String]) -> usize {
    values
        .iter()
        .collect::<std::collections::BTreeSet<_>>()
        .len()
}

fn identity_fields(candidate: &CharacterCandidate) -> Vec<String> {
    let character = &candidate.character;
    vec![
        character.envelope.name.clone(),
        character.archetype.clone(),
        character.role.clone(),
        character.title.clone(),
        character.appearance.clone(),
        character.faction_hint.clone(),
        character.signature_gear.clone(),
        character.desire.clone(),
        character.fear.clone(),
        character.flaw.clone(),
        character.secret.clone(),
        character.relationship_hook.clone(),
        character.backstory_hook.clone(),
        serde_json::to_string(&character.personality)
            .expect("String list serialization is infallible"),
        serde_json::to_string(&character.skills).expect("String list serialization is infallible"),
        serde_json::to_string(&character.abilities)
            .expect("String list serialization is infallible"),
        serde_json::to_string(&character.story_hooks)
            .expect("String list serialization is infallible"),
    ]
}

fn similarity_audit(identities: &[Vec<String>]) -> SimilarityAudit {
    let pair_count = identities.len() * identities.len().saturating_sub(1) / 2;
    let mut equal_sum = 0usize;
    let mut max_equal_fields = 0usize;
    let mut pairs_equal_8_or_more = 0usize;
    let mut pairs_equal_10_or_more = 0usize;
    let mut identical_character_pairs = 0usize;
    for first in 0..identities.len() {
        for second in (first + 1)..identities.len() {
            let matches = identities[first]
                .iter()
                .zip(&identities[second])
                .filter(|(left, right)| left == right)
                .count();
            equal_sum += matches;
            max_equal_fields = max_equal_fields.max(matches);
            pairs_equal_8_or_more += usize::from(matches >= 8);
            pairs_equal_10_or_more += usize::from(matches >= 10);
            identical_character_pairs += usize::from(matches == identities[first].len());
        }
    }
    SimilarityAudit {
        compared_field_count: identities.first().map_or(0, Vec::len),
        pair_count,
        mean_equal_fields: if pair_count == 0 {
            0.0
        } else {
            equal_sum as f64 / pair_count as f64
        },
        max_equal_fields,
        pairs_equal_8_or_more,
        pairs_equal_10_or_more,
        identical_character_pairs,
    }
}

fn compare_axis(
    preset: &str,
    axis: CreativeAxis,
    seeds: usize,
) -> Result<AxisComparison, Box<dyn std::error::Error>> {
    let mut changed_fields = BTreeMap::<String, usize>::new();
    let mut archetype_counts_low = BTreeMap::<String, usize>::new();
    let mut archetype_counts_high = BTreeMap::<String, usize>::new();
    for seed in 0..seeds {
        let low = generate(preset, seed as u64, Some((axis, 0)))?;
        let high = generate(preset, seed as u64, Some((axis, 100)))?;
        let low_fields = identity_fields(&low);
        let high_fields = identity_fields(&high);
        for (name, (low_value, high_value)) in identity_field_names()
            .into_iter()
            .zip(low_fields.iter().zip(&high_fields))
        {
            if low_value != high_value {
                *changed_fields.entry(name.into()).or_default() += 1;
            }
        }
        *archetype_counts_low
            .entry(low.character.archetype.clone())
            .or_default() += 1;
        *archetype_counts_high
            .entry(high.character.archetype.clone())
            .or_default() += 1;
        if generate(preset, seed as u64, Some((axis, 0)))? != low
            || generate(preset, seed as u64, Some((axis, 100)))? != high
        {
            return Err("axis comparison did not reproduce deterministically".into());
        }
    }
    Ok(AxisComparison {
        preset: preset.into(),
        axis: format!("{axis:?}").to_ascii_lowercase(),
        seeds,
        low: 0,
        high: 100,
        changed_fields,
        archetype_counts_low,
        archetype_counts_high,
    })
}

fn identity_field_names() -> [&'static str; 17] {
    [
        "name",
        "archetype",
        "role",
        "title",
        "appearance",
        "faction_hint",
        "signature_gear",
        "desire",
        "fear",
        "flaw",
        "secret",
        "relationship_hook",
        "backstory_hook",
        "personality_bundle",
        "skills_bundle",
        "abilities_bundle",
        "story_hooks_bundle",
    ]
}
