use forge_app_core::{QuestField, QuestForge};
use forge_contracts::{ContentVersion, CreativeAxes, CreativeAxis, RootSeed};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

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
fn thread(q: &forge_contracts::Quest) -> String {
    q.envelope
        .tags
        .iter()
        .find_map(|x| x.strip_prefix("thread:"))
        .unwrap_or("v2")
        .to_owned()
}
fn sample(c: &forge_app_core::QuestCandidate) -> Value {
    json!({"title":c.quest.envelope.name,"quest_type":c.quest.quest_type,"briefing":c.briefing,"quest_giver":c.quest.patron,"premise":c.quest.premise,"objective":c.quest.objective,"stakes":c.quest.stakes,"threat":c.quest.opposition,"complications":c.quest.complications,"clue":c.clue,"twist":c.quest.twist,"moral_pressure":c.quest.moral_pressure,"locations":c.quest.locations,"rewards":c.quest.rewards,"consequences":c.quest.failure_consequences,"followups":c.quest.follow_up_hooks,"thread":thread(&c.quest)})
}
fn metrics(candidates: &[forge_app_core::QuestCandidate]) -> Value {
    let mut pool: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    let mut clusters = BTreeMap::<String, usize>::new();
    let mut sigs = BTreeSet::new();
    let mut rows = Vec::new();
    let mut public_rows = Vec::new();
    for c in candidates {
        let q = &c.quest;
        let values = [
            ("title", q.envelope.name.clone()),
            ("location", q.locations.join("|")),
            ("quest_giver", q.patron.clone()),
            ("objective", q.objective.clone()),
            ("stakes", q.stakes.clone()),
            ("complication", q.complications.join("|")),
            ("twist", q.twist.clone()),
            ("threat", q.opposition.clone()),
            ("clue", c.clue.clone().unwrap_or_default()),
            ("reward", q.rewards.join("|")),
            ("consequence", q.failure_consequences.join("|")),
            ("moral_pressure", q.moral_pressure.clone()),
            ("premise", q.premise.clone()),
            ("briefing", c.briefing.clone().unwrap_or_default()),
            ("quest_type", q.quest_type.clone()),
            ("follow_up", q.follow_up_hooks.join("|")),
        ];
        let row = values
            .iter()
            .map(|(k, v)| {
                pool.entry(k).or_default().insert(v.clone());
                v.clone()
            })
            .collect::<Vec<_>>();
        public_rows.push(vec![
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
        ]);
        sigs.insert(serde_json::to_string(&row).unwrap());
        rows.push(row);
        *clusters.entry(thread(q)).or_default() += 1;
    }
    let mut pairs = 0usize;
    let (mut sum, mut max, mut ge8, mut ge10) = (0usize, 0usize, 0usize, 0usize);
    for i in 0..public_rows.len() {
        for j in i + 1..public_rows.len() {
            let n = public_rows[i]
                .iter()
                .zip(&public_rows[j])
                .filter(|(a, b)| a == b)
                .count();
            pairs += 1;
            sum += n;
            max = max.max(n);
            ge8 += usize::from(n >= 8);
            ge10 += usize::from(n >= 10);
        }
    }
    json!({"seeds":candidates.len(),"unique_expanded":sigs.len(),"exact_expanded_duplicates":candidates.len()-sigs.len(),"public_14_pairs":pairs,"mean_equal_public_14":sum as f64/pairs.max(1) as f64,"max_equal_public_14":max,"pairs_ge_8_of_14":ge8,"pairs_ge_10_of_14":ge10,"field_distinct_values":pool.iter().map(|(k,v)|(*k,v.len())).collect::<BTreeMap<_,_>>(),"thread_clusters":clusters})
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut report = Vec::new();
    let mut chaos_before = Vec::new();
    let mut chaos_after = Vec::new();
    for preset in PRESETS {
        let count = if preset == "chaos-blend" { 1024 } else { 256 };
        let mut v3 = Vec::new();
        let mut examples = Vec::new();
        for seed in 0..count {
            let dna = QuestForge::new_dna(
                RootSeed::new(seed as u64),
                preset,
                CreativeAxes::neutral(),
                BTreeMap::new(),
            )?;
            let c = QuestForge::generate(&dna)?;
            if seed < 2 {
                examples.push(sample(&c));
            }
            v3.push(c);
        }
        let summary = metrics(&v3);
        if preset == "chaos-blend" {
            for seed in 0..count {
                let mut d = QuestForge::new_dna(
                    RootSeed::new(seed as u64),
                    preset,
                    CreativeAxes::neutral(),
                    BTreeMap::new(),
                )?;
                d.content_version = ContentVersion(2);
                for f in [QuestField::Clue, QuestField::Briefing] {
                    let key = f.key()?;
                    d.recipe.field_controls.remove(&key);
                    d.recipe.dependencies.remove(&key);
                }
                let c = QuestForge::generate(&d)?;
                chaos_before.push(c);
            }
            chaos_after = v3.clone();
        }
        report.push(json!({"preset":preset,"metrics":summary,"representative_samples":examples}));
    }
    let mut axis_report = BTreeMap::new();
    for axis in CreativeAxis::ALL {
        let mut dist = BTreeMap::<String, BTreeMap<String, usize>>::new();
        for (label, level) in [("low", 0u8), ("high", 100u8)] {
            for seed in 0..256u64 {
                let mut a = CreativeAxes::neutral();
                a.levels.insert(axis, level);
                let c = QuestForge::generate(&QuestForge::new_dna(
                    RootSeed::new(seed),
                    "chaos-blend",
                    a,
                    BTreeMap::new(),
                )?)?;
                *dist
                    .entry(label.into())
                    .or_default()
                    .entry(thread(&c.quest))
                    .or_default() += 1;
            }
        }
        axis_report.insert(format!("{axis:?}"), json!(dist));
    }
    let mut constraints = forge_contracts::Constraints::new();
    constraints.insert(
        forge_contracts::ConstraintKey::new("inspiration")?,
        "a map written in starlight — 星 🗝️".into(),
    );
    let constrained = QuestForge::generate(&QuestForge::new_dna(
        RootSeed::new(31415),
        "occult",
        CreativeAxes::neutral(),
        constraints,
    )?)?;
    let mut extremes = Vec::new();
    for axis in CreativeAxis::ALL {
        for level in [0, 100] {
            let mut a = CreativeAxes::neutral();
            a.levels.insert(axis, level);
            let c = QuestForge::generate(&QuestForge::new_dna(
                RootSeed::new(42),
                "chaos-blend",
                a,
                BTreeMap::new(),
            )?)?;
            extremes.push(json!({"axis":format!("{axis:?}"),"level":level,"sample":sample(&c)}));
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"presets":report,"chaos_blend_before_v2":metrics(&chaos_before),"chaos_blend_after_v3":metrics(&chaos_after),"controlled_axis_thread_distributions_256_each":axis_report,"axis_extreme_samples":extremes,"constrained_sample":sample(&constrained)})
        )?
    );
    Ok(())
}
