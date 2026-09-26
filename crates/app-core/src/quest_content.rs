//! Versioned declarative Quest content for Mission 03.
use forge_contracts::{ValidationCode, ValidationError, ValidationErrors, ValidationResult};
use serde::Deserialize;
use std::{collections::BTreeMap, sync::OnceLock};

const V3: &str = include_str!("../../../content/quests/v3/core.json");

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Corpus {
    pub content_version: u16,
    pub presets: Vec<Preset>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Preset {
    pub id: String,
    pub threads: Vec<Thread>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Thread {
    pub id: String,
    pub tags: Vec<String>,
    pub titles: Vec<String>,
    pub locations: Vec<String>,
    pub givers: Vec<String>,
    pub premises: Vec<String>,
    pub objectives: Vec<String>,
    pub stakes: Vec<String>,
    pub threats: Vec<String>,
    pub complications: Vec<String>,
    pub clues: Vec<String>,
    pub twists: Vec<String>,
    pub rewards: Vec<String>,
    pub consequences: Vec<String>,
    pub moral_pressures: Vec<String>,
    pub followups: Vec<String>,
}
#[derive(Clone, Debug)]
pub(crate) struct Catalog {
    pub presets: BTreeMap<String, Preset>,
}
impl Catalog {
    pub fn get() -> ValidationResult<&'static Self> {
        static CATALOG: OnceLock<ValidationResult<Catalog>> = OnceLock::new();
        CATALOG
            .get_or_init(|| Self::parse(V3))
            .as_ref()
            .map_err(Clone::clone)
    }
    fn parse(raw: &str) -> ValidationResult<Self> {
        let doc: Corpus =
            serde_json::from_str(raw).map_err(|e| error("content", &e.to_string()))?;
        if doc.content_version != 3 || doc.presets.len() != 11 {
            return Err(error(
                "content",
                "Quest corpus must contain version 3 and eleven presets",
            ));
        }
        let mut presets = BTreeMap::new();
        for preset in doc.presets {
            if preset.threads.len() < 2 {
                return Err(error(
                    &preset.id,
                    "each preset needs multiple compatible story threads",
                ));
            }
            let mut ids = std::collections::BTreeSet::new();
            for t in &preset.threads {
                if t.id.trim().is_empty() || !ids.insert(t.id.clone()) || t.tags.is_empty() {
                    return Err(error(
                        &preset.id,
                        "thread ids and semantic tags must be unique and nonempty",
                    ));
                }
                for pool in [
                    &t.titles,
                    &t.locations,
                    &t.givers,
                    &t.premises,
                    &t.objectives,
                    &t.stakes,
                    &t.threats,
                    &t.complications,
                    &t.clues,
                    &t.twists,
                    &t.rewards,
                    &t.consequences,
                    &t.moral_pressures,
                    &t.followups,
                ] {
                    if pool.len() < 2 || pool.iter().any(|x| x.trim().is_empty()) {
                        return Err(error(
                            &preset.id,
                            "every thread field needs at least two nonempty choices",
                        ));
                    }
                }
            }
            let id = preset.id.clone();
            if presets.insert(id.clone(), preset).is_some() {
                return Err(error(&id, "duplicate preset id"));
            }
        }
        Ok(Self { presets })
    }
}
fn error(path: &str, message: &str) -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(
        ValidationCode::InvalidEntity,
        path,
        message,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn v3_corpus_is_complete_and_deep() {
        let c = Catalog::get().unwrap();
        assert_eq!(c.presets.len(), 11);
        assert!(c.presets.values().all(|p| p.threads.len() >= 2));
    }
}
