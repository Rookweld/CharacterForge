use forge_app_core::{CharacterForge, QuestForge};
use forge_contracts::{CreativeAxes, RootSeed};
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "4096".into())
        .parse::<u64>()?;
    for preset in PRESETS {
        for seed in 0..count {
            let character_dna = CharacterForge::new_dna(
                RootSeed::new(seed),
                preset,
                CreativeAxes::neutral(),
                BTreeMap::new(),
            )?;
            let character = CharacterForge::generate(&character_dna)?;
            character.validate()?;
            let character_replay = CharacterForge::generate(&character_dna)?;
            character_replay.validate()?;
            if serde_json::to_vec(&character)? != serde_json::to_vec(&character_replay)? {
                return Err(
                    format!("Character replay mismatch: preset={preset}, seed={seed}").into(),
                );
            }

            let quest_dna = QuestForge::new_dna(
                RootSeed::new(seed),
                preset,
                CreativeAxes::neutral(),
                BTreeMap::new(),
            )?;
            let quest = QuestForge::generate(&quest_dna)?;
            quest.validate()?;
            let quest_replay = QuestForge::generate(&quest_dna)?;
            quest_replay.validate()?;
            if serde_json::to_vec(&quest)? != serde_json::to_vec(&quest_replay)? {
                return Err(format!("Quest replay mismatch: preset={preset}, seed={seed}").into());
            }
        }
        println!(
            "PASS preset={preset} seeds=0..{} replay=exact entities=Character,Quest",
            count - 1
        );
    }
    println!(
        "PASS character_generations={} quest_generations={} exact_replays_each={}",
        count * PRESETS.len() as u64,
        count * PRESETS.len() as u64,
        count * PRESETS.len() as u64
    );
    Ok(())
}
