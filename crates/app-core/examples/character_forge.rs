use forge_app_core::CharacterForge;
use forge_contracts::{CreativeAxes, CreativeAxis, RootSeed};
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let mut seed = 1_u64;
    let mut preset = String::from("dark-fantasy");
    let mut axes = BTreeMap::new();
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--seed" => seed = parse_seed(&required_value(&mut args, "--seed")?)?,
            "--preset" => preset = required_value(&mut args, "--preset")?,
            "--axis" => {
                let value = required_value(&mut args, "--axis")?;
                let (name, level) = value.split_once('=').ok_or("axis must be name=0..100")?;
                let axis = match name {
                    "bleak" => CreativeAxis::Bleak,
                    "mythic" => CreativeAxis::Mythic,
                    "chaos" => CreativeAxis::Chaos,
                    "epic" => CreativeAxis::Epic,
                    "ambiguity" => CreativeAxis::Ambiguity,
                    "surreal" => CreativeAxis::Surreal,
                    "technology" => CreativeAxis::Technology,
                    "mysticism" => CreativeAxis::Mysticism,
                    _ => return Err(format!("unsupported creative axis: {name}").into()),
                };
                let level: u8 = level.parse()?;
                if level > 100 {
                    return Err("axis level must be within 0..=100".into());
                }
                axes.insert(axis, level);
            }
            "--help" | "-h" => {
                println!(
                    "Usage: cargo run -p forge-app-core --example character_forge -- [--seed U64] [--preset ID] [--axis NAME=0..100]"
                );
                return Ok(());
            }
            other => return Err(format!("unknown argument: {other}").into()),
        }
    }

    let mut creative_axes = CreativeAxes::neutral();
    creative_axes.levels.extend(axes);
    let dna =
        CharacterForge::new_dna(RootSeed::new(seed), &preset, creative_axes, BTreeMap::new())?;
    let candidate = CharacterForge::generate(&dna)?;
    println!("{}", serde_json::to_string_pretty(&candidate)?);
    Ok(())
}

fn required_value(args: &mut impl Iterator<Item = String>, option: &str) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("{option} requires a value"))
}

fn parse_seed(value: &str) -> Result<u64, Box<dyn std::error::Error>> {
    if let Some(hex) = value.strip_prefix("0x") {
        Ok(u64::from_str_radix(hex, 16)?)
    } else {
        Ok(value.parse()?)
    }
}
