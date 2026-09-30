use forge_app_core::{CREATURE_GENERATOR_VERSION, CreatureForge};
use forge_contracts::{CreativeAxes, RootSeed};

fn main() {
    println!(
        "Creature generator version: {}",
        CREATURE_GENERATOR_VERSION.0
    );
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
            let dna = CreatureForge::new_dna(RootSeed::new(seed), preset, CreativeAxes::neutral())
                .expect("supported preset");
            print_candidate(preset, seed, &dna);
        }
    }
    for seed in 0..12 {
        let dna = CreatureForge::new_dna(
            RootSeed::new(10_000 + seed),
            "chaos-blend",
            CreativeAxes::neutral(),
        )
        .expect("supported Chaos Blend preset");
        print_candidate("chaos-blend", 10_000 + seed, &dna);
    }
}

fn print_candidate(preset: &str, seed: u64, dna: &forge_contracts::ForgeDNA) {
    let candidate = CreatureForge::generate(dna).expect("valid candidate");
    println!("\n## {preset} / seed {seed}");
    println!("- name: {}", candidate.creature.envelope.name);
    println!("- profile: {:?}", candidate.creature.creature_profile);
    println!("- appearance: {}", candidate.creature.appearance);
    println!("- habitat: {}", candidate.creature.habitat);
    println!("- behavior: {}", candidate.creature.behavior);
    println!("- ecology: {}", candidate.creature.ecology);
    println!("- threat: {}", candidate.creature.threat);
    println!(
        "- abilities_summary: {}",
        candidate.creature.abilities_summary
    );
    println!("- weakness: {}", candidate.creature.weakness);
    println!("- origin: {}", candidate.creature.origin);
    println!(
        "- signs_of_presence: {}",
        candidate.creature.signs_of_presence
    );
    println!(
        "- loot_or_resource: {}",
        candidate.creature.loot_or_resource
    );
    println!("- secret: {}", candidate.creature.secret);
    println!(
        "- story_hooks: {}",
        candidate.creature.story_hooks.join(" | ")
    );
}
