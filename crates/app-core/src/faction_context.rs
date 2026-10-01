//! Versioned, optional Character intent consumed by the canonical FactionForge.
//! No context constraint means the original standalone field stream path.
use crate::{FactionField, FactionFieldValue, FieldStream};
use forge_contracts::{
    FactionProfile, ForgeDNA, ValidationCode, ValidationError, ValidationErrors, ValidationResult,
};
use serde::{Deserialize, Serialize};

pub(crate) const CONTEXT_KEY: &str = "faction.character-context-v1";
const MAX_CONTEXT_BYTES: usize = 32_768;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharacterFactionFacts {
    pub name: String,
    pub title: String,
    pub archetype: String,
    pub role: String,
    pub personality: Vec<String>,
    pub desire: String,
    pub fear: String,
    pub flaw: String,
    pub faction_hint: String,
    pub relationship_hook: String,
    pub backstory_hook: String,
    pub secret: String,
}
impl CharacterFactionFacts {
    pub fn validate(&self) -> ValidationResult<()> {
        let texts = [
            &self.name,
            &self.title,
            &self.archetype,
            &self.role,
            &self.desire,
            &self.fear,
            &self.flaw,
            &self.faction_hint,
            &self.relationship_hook,
            &self.backstory_hook,
            &self.secret,
        ];
        if texts.iter().any(|v| v.trim().is_empty())
            || self.personality.is_empty()
            || self.personality.len() > 16
            || self.personality.iter().any(|v| v.trim().is_empty())
            || serde_json::to_vec(self).map_err(|_| invalid())?.len() > MAX_CONTEXT_BYTES
        {
            return Err(invalid());
        }
        Ok(())
    }
}

pub(crate) fn from_dna(dna: &ForgeDNA) -> ValidationResult<Option<CharacterFactionFacts>> {
    if dna.recipe.constraints.is_empty() {
        return Ok(None);
    }
    if dna.recipe.constraints.len() != 1 {
        return Err(invalid());
    }
    let (key, text) = dna.recipe.constraints.iter().next().ok_or_else(invalid)?;
    if key.as_str() != CONTEXT_KEY || text.len() > MAX_CONTEXT_BYTES {
        return Err(invalid());
    }
    let facts: CharacterFactionFacts = serde_json::from_str(text).map_err(|_| invalid())?;
    facts.validate()?;
    // Stable canonical payload; equivalent but noncanonical JSON is not silently reinterpreted.
    if serde_json::to_string(&facts).map_err(|_| invalid())? != *text {
        return Err(invalid());
    }
    Ok(Some(facts))
}

fn invalid() -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(
        ValidationCode::InvalidConstraint,
        "faction.character-context-v1",
        "Faction context must be canonical typed Character facts (at most 32 KiB and 16 personality traits); unsupported constraints are rejected",
    ))
}

fn hinted_profiles(text: &str) -> Vec<FactionProfile> {
    let text = text.to_lowercase();
    // Direct faction intent outranks occupational flavor. These are creative profile hints,
    // not inferred persistent entities or vocabulary/cardinality rules.
    let rules: [(FactionProfile, &[&str]); 13] = [
        (
            FactionProfile::Guild,
            &["guild", "trade", "craft", "union", "artisan"],
        ),
        (FactionProfile::Order, &["order", "knight", "warden"]),
        (FactionProfile::Cult, &["cult", "initiate"]),
        (
            FactionProfile::Government,
            &[
                "government",
                "council",
                "bureau",
                "registry",
                "office",
                "board",
                "department",
                "moot",
            ],
        ),
        (
            FactionProfile::Kingdom,
            &["kingdom", "royal", "crown", "court"],
        ),
        (
            FactionProfile::Military,
            &["military", "army", "regiment", "soldier", "fleet"],
        ),
        (
            FactionProfile::Corporation,
            &["corporation", "company", "consortium", "corporate"],
        ),
        (
            FactionProfile::Gang,
            &["gang", "crew", "smuggler", "syndicate"],
        ),
        (
            FactionProfile::Rebellion,
            &["rebellion", "rebel", "resistance", "liberation"],
        ),
        (
            FactionProfile::ReligiousOrder,
            &["temple", "priest", "monk", "religious", "abbey"],
        ),
        (FactionProfile::Tribe, &["tribe", "clan", "kinship"]),
        (
            FactionProfile::SecretSociety,
            &[
                "secret society",
                "conspiracy",
                "hidden circle",
                "refuses to list its own members",
                "never publish an address",
            ],
        ),
        (
            FactionProfile::Other,
            &[
                "mutual aid",
                "collective",
                "network",
                "cooperative",
                "compact",
                "refuge",
            ],
        ),
    ];
    rules
        .into_iter()
        .filter_map(|(profile, words)| {
            words
                .iter()
                .any(|word| text.contains(word))
                .then_some(profile)
        })
        .collect()
}

pub(crate) fn adapt(
    facts: &CharacterFactionFacts,
    field: FactionField,
    base: FactionFieldValue,
    profile: Option<FactionProfile>,
    stream: &mut FieldStream,
) -> ValidationResult<FactionFieldValue> {
    if field == FactionField::Profile {
        let direct = hinted_profiles(&facts.faction_hint);
        // An ambiguous authored hint stays Other instead of being overridden by
        // a random institution or by the Character's occupation.
        return if direct.is_empty() {
            Ok(FactionFieldValue::Profile(FactionProfile::Other))
        } else {
            Ok(FactionFieldValue::Profile(
                direct[stream.next_bounded(direct.len() as u64)? as usize],
            ))
        };
    }
    // Names, resources, symbols and methods retain the accepted preset/Chaos corpus.
    if matches!(
        field,
        FactionField::Name
            | FactionField::Resources
            | FactionField::Symbols
            | FactionField::Methods
    ) {
        return Ok(base);
    }
    let index = stream.next_bounded(4)? as usize;
    let name = &facts.name;
    let hint = &facts.faction_hint;
    let personality = facts.personality.join(" / ");
    let text = match field {
        FactionField::Identity => format!("{} Members gather around “{hint}”. {name}'s work as “{}” gives this organization a practical focus.", crate::faction::profile_structure(profile.unwrap_or(FactionProfile::Other)), facts.role),
        FactionField::Ideology => [
            format!("A shared duty matters more than a spotless reputation. Members treat “{}” as a warning against promises they cannot keep.", facts.flaw),
            format!("No member faces their fear alone. The charter names “{}” as a danger to confront together.", facts.fear),
            format!("Membership is earned through useful work rather than inherited rank; “{}” shapes the group's idea of service.", facts.role),
            format!("A private ambition should create a public benefit. Members debate how “{}” can serve more than one person.", facts.desire),
        ][index].clone(),
        FactionField::PrimaryGoal => [
            format!("Build a shared campaign around {name}'s aim: “{}”. Members of “{hint}” must agree which first step they can sustain.", facts.desire),
            format!("Secure the people and material needed for “{}”, without leaving {name} to carry the risk alone.", facts.desire),
            format!("Turn “{}” into a plan that members of “{hint}” can act on together.", facts.desire),
            format!("Win enough trust to advance “{}”; {name}'s experience as “{}” guides the first undertaking.", facts.desire, facts.archetype),
        ][index].clone(),
        FactionField::Culture => format!("{}. New members compare their habits with {name}'s temperament: “{personality}”; differences are aired before joint work begins.", text_of(base)),
        FactionField::PublicFace => [
            format!("It introduces itself through {name}'s work as “{}”, offering a public face for “{hint}”.", facts.role),
            format!("The circle sponsors work associated with “{}”; {name} is one visible contact, rather than its unquestioned leader.", facts.title),
            format!("Its public gatherings explain what “{hint}” means in practice, using {name}'s experience as “{}”.", facts.archetype),
            format!("Members present a shared service rooted in “{hint}”, while {name} handles questions about “{}”.", facts.role),
        ][index].clone(),
        FactionField::HiddenTruth => [
            format!("A sealed account concerns {name}: “{}”. The inner circle has not decided who else should know.", facts.secret),
            format!("Its most delicate obligation comes from {name}'s secret: “{}”. Members disagree about the price of protecting it.", facts.secret),
            format!("The charter's unwritten exception protects this fact about {name}: “{}”. A trusted witness wants the exception reviewed.", facts.secret),
            format!("One closed meeting keeps returning to “{}”, a truth about {name} that could divide the membership.", facts.secret),
        ][index].clone(),
        FactionField::Reputation => format!("{}. Outsiders judge its promises against {name}'s aim, “{}”, and the conduct of people associated with “{hint}”.", text_of(base), facts.desire),
        FactionField::StoryHooks => { let mut hooks = vec![
            format!("A membership dispute puts this connection under pressure: “{}”. {name} must choose who can speak for “{hint}”.", facts.relationship_hook),
            format!("A claimant returns with evidence concerning “{}”. The organization must decide what it owes {name}.", facts.backstory_hook),
            format!("A proposed undertaking could advance “{}”, but would require discussing “{}” in an open meeting.", facts.desire, facts.secret),
        ]; hooks.rotate_left(index % 3); return Ok(FactionFieldValue::Texts(hooks)); },
        _ => return Ok(base),
    };
    Ok(FactionFieldValue::Text(text))
}

fn text_of(value: FactionFieldValue) -> String {
    match value {
        FactionFieldValue::Text(text) => text,
        _ => String::new(),
    }
}
