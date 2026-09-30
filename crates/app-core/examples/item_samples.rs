use forge_app_core::{ITEM_GENERATOR_VERSION, ItemForge};
use forge_contracts::{CreativeAxes, RootSeed};

fn main() {
    println!("Item generator version: {}", ITEM_GENERATOR_VERSION.0);
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
            let dna = ItemForge::new_dna(RootSeed::new(seed), preset, CreativeAxes::neutral())
                .expect("supported preset");
            print_candidate(preset, seed, &dna);
        }
    }
    for seed in 0..12 {
        let dna = ItemForge::new_dna(
            RootSeed::new(10_000 + seed),
            "chaos-blend",
            CreativeAxes::neutral(),
        )
        .expect("supported Chaos Blend preset");
        print_candidate("chaos-blend", 10_000 + seed, &dna);
    }
}

fn print_candidate(preset: &str, seed: u64, dna: &forge_contracts::ForgeDNA) {
    let candidate = ItemForge::generate(dna).expect("valid candidate");
    println!("\n## {preset} / seed {seed}");
    println!("- name: {}", candidate.item.envelope.name);
    println!("- profile: {:?}", candidate.item.item_profile);
    println!("- appearance: {}", candidate.item.appearance);
    println!("- purpose: {}", candidate.item.purpose);
    println!("- properties: {}", candidate.item.properties);
    println!("- origin: {}", candidate.item.origin);
    println!("- history: {}", candidate.item.history);
    println!("- power_or_function: {}", candidate.item.power_or_function);
    println!("- cost_or_drawback: {}", candidate.item.cost_or_drawback);
    println!("- rarity: {}", candidate.item.rarity);
    println!("- condition: {}", candidate.item.condition);
    println!("- secret: {}", candidate.item.secret);
    println!("- story_hooks: {}", candidate.item.story_hooks.join(" | "));
}
