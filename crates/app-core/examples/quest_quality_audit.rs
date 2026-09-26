use forge_app_core::QuestForge;
use forge_contracts::{CreativeAxes, CreativeAxis, RootSeed};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

fn signature(q: &forge_contracts::Quest) -> String {
    serde_json::to_string(&(
        &q.envelope.name,
        &q.quest_type,
        &q.premise,
        &q.patron,
        &q.objective,
        &q.stakes,
        &q.opposition,
        &q.complications,
        &q.twist,
        &q.moral_pressure,
        &q.locations,
        &q.rewards,
        &q.failure_consequences,
        &q.follow_up_hooks,
    ))
    .unwrap()
}
fn features(q: &forge_contracts::Quest) -> Vec<String> {
    vec![
        q.envelope.name.clone(),
        q.quest_type.clone(),
        q.premise.clone(),
        q.patron.clone(),
        q.objective.clone(),
        q.stakes.clone(),
        q.opposition.clone(),
        q.complications.join("|"),
        q.twist.clone(),
        q.moral_pressure.clone(),
        q.locations.join("|"),
        q.rewards.join("|"),
        q.failure_consequences.join("|"),
        q.follow_up_hooks.join("|"),
    ]
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let presets = [
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
    let mut output = Vec::new();
    for preset in presets {
        let count: usize = if preset == "chaos-blend" { 1024 } else { 256 };
        let mut signatures = Vec::new();
        let mut candidates = Vec::new();
        let mut fields = BTreeMap::<&str, BTreeSet<String>>::new();
        for seed in 0..count {
            let dna = QuestForge::new_dna(
                RootSeed::new(seed as u64),
                preset,
                CreativeAxes::neutral(),
                BTreeMap::new(),
            )?;
            let q = QuestForge::generate(&dna)?.quest;
            signatures.push(signature(&q));
            candidates.push(features(&q));
            for (k, v) in [
                ("title", q.envelope.name),
                ("objective", q.objective),
                ("stakes", q.stakes),
                ("opposition", q.opposition),
                ("twist", q.twist),
                ("location", q.locations.join("|")),
            ] {
                fields.entry(k).or_default().insert(v);
            }
        }
        let unique = signatures.iter().collect::<BTreeSet<_>>().len();
        let (mut compared, mut max_equal, mut pairs_ge_8, mut pairs_ge_10) =
            (0usize, 0usize, 0usize, 0usize);
        let mut sum_equal = 0usize;
        for i in 0..count {
            for j in i + 1..count {
                let equal = candidates[i]
                    .iter()
                    .zip(&candidates[j])
                    .filter(|(a, b)| a == b)
                    .count();
                sum_equal += equal;
                max_equal = max_equal.max(equal);
                pairs_ge_8 += usize::from(equal >= 8);
                pairs_ge_10 += usize::from(equal >= 10);
                compared += 1;
            }
        }
        output.push(json!({"preset":preset,"seeds":count,"unique_candidates":unique,"full_duplicates":count-unique,"field_unique":fields,"candidate_pairs":compared,"mean_equal_fields":sum_equal as f64/compared as f64,"max_equal_fields":max_equal,"pairs_ge_8":pairs_ge_8,"pairs_ge_10":pairs_ge_10}));
    }
    let mut axis_changes = BTreeMap::new();
    for axis in CreativeAxis::ALL {
        let mut changed = 0;
        for seed in 0..256 {
            let mut low = CreativeAxes::neutral();
            let mut high = CreativeAxes::neutral();
            low.levels.insert(axis, 0);
            high.levels.insert(axis, 100);
            let a = QuestForge::generate(&QuestForge::new_dna(
                RootSeed::new(seed),
                "cyberpunk",
                low,
                BTreeMap::new(),
            )?)?
            .quest;
            let b = QuestForge::generate(&QuestForge::new_dna(
                RootSeed::new(seed),
                "cyberpunk",
                high,
                BTreeMap::new(),
            )?)?
            .quest;
            if signature(&a) != signature(&b) {
                changed += 1;
            }
        }
        axis_changes.insert(format!("{axis:?}"), changed);
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"presets":output,"cyberpunk_axis_changed_candidates_out_of_256":axis_changes})
        )?
    );
    Ok(())
}
