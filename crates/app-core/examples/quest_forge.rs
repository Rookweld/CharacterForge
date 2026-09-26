use forge_app_core::QuestForge;
use forge_contracts::{CreativeAxes, CreativeAxis, RootSeed};
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let (mut seed, mut preset) = (1u64, "dark-fantasy".to_string());
    let mut axes = BTreeMap::new();
    while let Some(a) = args.next() {
        match a.as_str() {
            "--seed" => seed = args.next().ok_or("missing seed")?.parse()?,
            "--preset" => preset = args.next().ok_or("missing preset")?,
            "--axis" => {
                let v = args.next().ok_or("missing axis")?;
                let (k, n) = v.split_once('=').ok_or("axis must be name=0..100")?;
                let axis = match k {
                    "bleak" => CreativeAxis::Bleak,
                    "mythic" => CreativeAxis::Mythic,
                    "chaos" => CreativeAxis::Chaos,
                    "epic" => CreativeAxis::Epic,
                    "ambiguity" => CreativeAxis::Ambiguity,
                    "surreal" => CreativeAxis::Surreal,
                    "technology" => CreativeAxis::Technology,
                    "mysticism" => CreativeAxis::Mysticism,
                    _ => return Err(format!("unknown axis {k}").into()),
                };
                axes.insert(axis, n.parse::<u8>()?);
            }
            "--help" | "-h" => {
                println!("Usage: quest_forge --seed U64 --preset ID [--axis NAME=0..100]");
                return Ok(());
            }
            _ => return Err(format!("unknown argument {a}").into()),
        }
    }
    let mut creative = CreativeAxes::neutral();
    creative.levels.extend(axes);
    let dna = QuestForge::new_dna(RootSeed::new(seed), &preset, creative, BTreeMap::new())?;
    let candidate = QuestForge::generate(&dna)?;
    println!("{}", serde_json::to_string_pretty(&candidate)?);
    Ok(())
}
