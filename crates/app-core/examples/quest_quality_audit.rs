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
    let mut value_frequency: BTreeMap<&str, BTreeMap<String, usize>> = BTreeMap::new();
    let mut id_frequency: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
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
                *value_frequency
                    .entry(k)
                    .or_default()
                    .entry(v.clone())
                    .or_default() += 1;
                v.clone()
            })
            .collect::<Vec<_>>();
        for (key, id) in c.audit_selection_ids().unwrap() {
            *id_frequency.entry(key).or_default().entry(id).or_default() += 1;
        }
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
    let (mut sum, mut max, mut ge8, mut ge10, mut ge12) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let mut pair_counts = Vec::new();
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
            pair_counts.push(n);
            ge8 += usize::from(n >= 8);
            ge10 += usize::from(n >= 10);
            ge12 += usize::from(n >= 12);
        }
    }
    pair_counts.sort_unstable();
    let median = if pair_counts.len().is_multiple_of(2) {
        (pair_counts[pair_counts.len() / 2 - 1] + pair_counts[pair_counts.len() / 2]) as f64 / 2.0
    } else {
        pair_counts[pair_counts.len() / 2] as f64
    };
    let concentration=value_frequency.iter().map(|(field,freq)|{let mut ordered=freq.iter().collect::<Vec<_>>();ordered.sort_by(|a,b|b.1.cmp(a.1));let top3=ordered.iter().take(3).map(|(_,count)|**count).sum::<usize>();(*field,json!({"unique":freq.len(),"most_common_count":ordered.first().map_or(0,|(_,n)|**n),"most_common_frequency":ordered.first().map_or(0.0,|(_,n)|**n as f64/candidates.len() as f64),"top3_combined_count":top3,"top3_combined_frequency":top3 as f64/candidates.len() as f64,"top_values":ordered.iter().take(3).map(|(value,count)|json!({"value":value,"count":count})).collect::<Vec<_>>()}))}).collect::<BTreeMap<_,_>>();
    let id_concentration=id_frequency.iter().map(|(field,freq)|{let mut ordered=freq.iter().collect::<Vec<_>>();ordered.sort_by(|a,b|b.1.cmp(a.1));(field.clone(),json!({"unique_ids":freq.len(),"top3_ids":ordered.iter().take(3).map(|(id,count)|json!({"id":id,"count":count})).collect::<Vec<_>>()}))}).collect::<BTreeMap<_,_>>();
    json!({"seeds":candidates.len(),"unique_expanded":sigs.len(),"exact_expanded_duplicates":candidates.len()-sigs.len(),"public_14_pairs":pairs,"mean_equal_public_14":sum as f64/pairs.max(1) as f64,"median_equal_public_14":median,"max_equal_public_14":max,"pairs_ge_8_of_14":ge8,"pairs_ge_10_of_14":ge10,"pairs_ge_12_of_14":ge12,"field_distinct_values":pool.iter().map(|(k,v)|(*k,v.len())).collect::<BTreeMap<_,_>>(),"rendered_field_concentration":concentration,"internal_id_concentration":id_concentration,"thread_clusters":clusters})
}
fn cluster_analysis(candidates: &[forge_app_core::QuestCandidate]) -> Value {
    let rows = candidates
        .iter()
        .map(|c| {
            let q = &c.quest;
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
        })
        .collect::<Vec<_>>();
    let ids = candidates
        .iter()
        .map(|c| c.audit_selection_ids().unwrap())
        .collect::<Vec<_>>();
    let mut by_thread = BTreeMap::<String, Vec<usize>>::new();
    for (i, c) in candidates.iter().enumerate() {
        by_thread.entry(thread(&c.quest)).or_default().push(i);
    }
    let mut thread_pairs = BTreeMap::<String, usize>::new();
    let mut incidents = BTreeMap::<usize, usize>::new();
    let mut identical_fields = BTreeMap::<String, usize>::new();
    let mut shared_ids = BTreeMap::<String, usize>::new();
    let mut high = 0usize;
    let mut parent = (0..rows.len()).collect::<Vec<_>>();
    let (mut same_thread, mut same_thread_equal) = (0usize, 0usize);
    for i in 0..rows.len() {
        for j in i + 1..rows.len() {
            let equal = rows[i].iter().zip(&rows[j]).filter(|(a, b)| a == b).count();
            let a = thread(&candidates[i].quest);
            let b = thread(&candidates[j].quest);
            if a == b {
                same_thread += 1;
                same_thread_equal += equal;
            }
            if equal >= 10 {
                high += 1;
                let mut root_a = i;
                while parent[root_a] != root_a {
                    root_a = parent[root_a];
                }
                let mut root_b = j;
                while parent[root_b] != root_b {
                    root_b = parent[root_b];
                }
                if root_a != root_b {
                    parent[root_b] = root_a;
                }
                *incidents.entry(i).or_default() += 1;
                *incidents.entry(j).or_default() += 1;
                let cluster = if a <= b {
                    format!("{a}|{b}")
                } else {
                    format!("{b}|{a}")
                };
                *thread_pairs.entry(cluster).or_default() += 1;
                for (idx, name) in [
                    "title",
                    "quest_type",
                    "premise",
                    "quest_giver",
                    "objective",
                    "stakes",
                    "threat",
                    "complication",
                    "twist",
                    "moral_pressure",
                    "location",
                    "reward",
                    "consequence",
                    "follow_up",
                ]
                .iter()
                .enumerate()
                {
                    if rows[i][idx] == rows[j][idx] {
                        *identical_fields.entry((*name).to_owned()).or_default() += 1;
                    }
                }
                for (key, value) in &ids[i] {
                    if ids[j].get(key) == Some(value) {
                        *shared_ids.entry(format!("{key}={value}")).or_default() += 1;
                    }
                }
            }
        }
    }
    let mut components = BTreeMap::<usize, Vec<usize>>::new();
    for seed in 0..parent.len() {
        let mut root = seed;
        while parent[root] != root {
            root = parent[root];
        }
        let mut cursor = seed;
        while parent[cursor] != cursor {
            let next = parent[cursor];
            parent[cursor] = root;
            cursor = next;
        }
        components.entry(root).or_default().push(seed);
    }
    let mut components = components
        .into_values()
        .filter(|seeds| seeds.len() > 1)
        .collect::<Vec<_>>();
    components.sort_by_key(|seeds| std::cmp::Reverse(seeds.len()));
    let largest_clusters = components
        .iter()
        .take(10)
        .map(|seeds| json!({"size":seeds.len(),"seeds":seeds}))
        .collect::<Vec<_>>();
    let mut top_shared = shared_ids.into_iter().collect::<Vec<_>>();
    top_shared.sort_by(|a, b| b.1.cmp(&a.1));
    top_shared.truncate(30);
    let mut top_seeds = incidents.into_iter().collect::<Vec<_>>();
    top_seeds.sort_by(|a, b| b.1.cmp(&a.1));
    top_seeds.truncate(50);
    let mut thread_clusters=by_thread.into_iter().map(|(thread,seeds)|json!({"thread":thread,"seed_count":seeds.len(),"seed_ids_preview":seeds.iter().take(64).copied().collect::<Vec<_>>()})).collect::<Vec<_>>();
    thread_clusters.sort_by(|a, b| b["seed_count"].as_u64().cmp(&a["seed_count"].as_u64()));
    let records=ids.iter().enumerate().map(|(seed,selection_ids)|json!({"seed":seed,"thread":thread(&candidates[seed].quest),"selection_ids":selection_ids})).collect::<Vec<_>>();
    json!({"pairs_ge_10":high,"high_pairs_by_thread_pair":thread_pairs,"high_pair_matching_fields":identical_fields,"most_shared_internal_selection_ids_in_high_pairs":top_shared,"seeds_with_most_high_pair_neighbors":top_seeds,"largest_ge_10_connected_clusters":largest_clusters,"thread_clusters":thread_clusters,"mean_equal_fields_given_same_thread":same_thread_equal as f64/same_thread.max(1) as f64,"selection_records":records})
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
        let cluster_report = if preset == "chaos-blend" {
            Some(cluster_analysis(&v3))
        } else {
            None
        };
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
        report.push(json!({"preset":preset,"metrics":summary,"internal_selection_cluster_analysis":cluster_report,"representative_samples":examples}));
    }
    let mut axis_report = BTreeMap::new();
    for axis in CreativeAxis::ALL {
        let mut dist = BTreeMap::<String, BTreeMap<String, usize>>::new();
        let mut location_source_mix = BTreeMap::<String, BTreeMap<String, usize>>::new();
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
                if axis == CreativeAxis::Chaos {
                    let ids = c.audit_selection_ids()?;
                    let mixed = ids.get("source/location") != ids.get("theme/thread");
                    *location_source_mix
                        .entry(label.into())
                        .or_default()
                        .entry(
                            if mixed {
                                "secondary source"
                            } else {
                                "spine source"
                            }
                            .into(),
                        )
                        .or_default() += 1;
                }
            }
        }
        axis_report.insert(
            format!("{axis:?}"),
            json!({"thread_distribution":dist,"location_source_distribution":location_source_mix}),
        );
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
