use forge_app_core::{FACTION_GENERATOR_VERSION, FactionForge};
use forge_contracts::{CreativeAxes, RootSeed};

fn main() {
    println!("Faction generator version: {}", FACTION_GENERATOR_VERSION.0);
    for preset in [
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
    ] {
        for seed in 0..4 {
            let dna = FactionForge::new_dna(RootSeed::new(seed), preset, CreativeAxes::neutral())
                .expect("supported preset");
            print_candidate(preset, seed, &dna);
        }
    }
    for seed in 0..12 {
        let dna = FactionForge::new_dna(
            RootSeed::new(10_000 + seed),
            "chaos-blend",
            CreativeAxes::neutral(),
        )
        .expect("supported Chaos Blend preset");
        print_candidate("chaos-blend", 10_000 + seed, &dna);
    }
}

fn print_candidate(preset: &str, seed: u64, dna: &forge_contracts::ForgeDNA) {
    let candidate = FactionForge::generate(dna).expect("valid candidate");
    println!("\n## {preset} / seed {seed}");
    println!("- name: {}", candidate.faction.envelope.name);
    println!("- profile: {:?}", candidate.faction.faction_profile);
    println!("- identity: {}", candidate.faction.identity);
    println!("- ideology: {}", candidate.faction.ideology);
    println!("- primary_goal: {}", candidate.faction.primary_goal);
    println!("- methods: {}", candidate.faction.methods);
    println!("- culture: {}", candidate.faction.culture);
    println!("- resources: {}", candidate.faction.resources);
    println!("- public_face: {}", candidate.faction.public_face);
    println!("- hidden_truth: {}", candidate.faction.hidden_truth);
    println!("- symbols: {}", candidate.faction.symbols);
    println!("- reputation: {}", candidate.faction.reputation);
    println!(
        "- story_hooks: {}",
        candidate.faction.story_hooks.join(" | ")
    );
}
