use forge_app_core::{CharacterForge, HometownSource, forge_character_signature_item};
use forge_contracts::{CreativeAxes, RootSeed};
use std::collections::BTreeMap;
fn main() {
    let mut samples = Vec::new();
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
        "chaos-blend",
    ] {
        for seed in 0..4 {
            let dna = CharacterForge::new_dna(
                RootSeed::new(seed),
                preset,
                CreativeAxes::neutral(),
                BTreeMap::new(),
            )
            .unwrap();
            let candidate = CharacterForge::generate(&dna).unwrap();
            let project = candidate.character.envelope.project_id;
            let source = HometownSource {
                character: candidate.character,
                field_origins: candidate.field_origins,
                write_version: 1,
            };
            let bundle = forge_character_signature_item(&source, 100 + seed, project).unwrap();
            samples.push(serde_json::json!({"preset":preset,"source_seed":seed,"source":source,"bundle":bundle}));
        }
    }
    println!("{}", serde_json::to_string_pretty(&samples).unwrap());
}
