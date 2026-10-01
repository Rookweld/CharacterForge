//! Optional, versioned Character intent for canonical ItemForge. Empty DNA keeps standalone replay.
use crate::{FieldStream, ItemField, ItemFieldValue};
use forge_contracts::{
    CreativeAxis, ForgeDNA, ItemProfile, ValidationCode, ValidationError, ValidationErrors,
    ValidationResult,
};
use serde::{Deserialize, Serialize};
pub(crate) const CONTEXT_KEY: &str = "item.character-signature-context-v1";
const MAX_BYTES: usize = 32_768;
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharacterItemFacts {
    pub name: String,
    pub title: String,
    pub archetype: String,
    pub role: String,
    pub appearance: String,
    pub personality: Vec<String>,
    pub desire: String,
    pub fear: String,
    pub flaw: String,
    pub signature_gear: String,
    pub skills: Vec<String>,
    pub abilities: Vec<String>,
    pub relationship_hook: String,
    pub backstory_hook: String,
    pub secret: String,
}
impl CharacterItemFacts {
    pub fn validate(&self) -> ValidationResult<()> {
        let texts = [
            &self.name,
            &self.title,
            &self.archetype,
            &self.role,
            &self.appearance,
            &self.desire,
            &self.fear,
            &self.flaw,
            &self.signature_gear,
            &self.relationship_hook,
            &self.backstory_hook,
            &self.secret,
        ];
        if texts.iter().any(|s| s.trim().is_empty())
            || [&self.personality, &self.skills, &self.abilities]
                .iter()
                .any(|v| v.is_empty() || v.len() > 16 || v.iter().any(|s| s.trim().is_empty()))
            || serde_json::to_vec(self).map_err(|_| invalid())?.len() > MAX_BYTES
        {
            return Err(invalid());
        }
        Ok(())
    }
}
fn invalid() -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(
        ValidationCode::InvalidConstraint,
        CONTEXT_KEY,
        "Signature Item context requires canonical typed Character facts, at most 32 KiB and 16 entries per list",
    ))
}
pub(crate) fn from_dna(dna: &ForgeDNA) -> ValidationResult<Option<CharacterItemFacts>> {
    if dna.recipe.constraints.is_empty() {
        return Ok(None);
    }
    if dna.recipe.constraints.len() != 1 {
        return Err(invalid());
    }
    let (key, text) = dna.recipe.constraints.iter().next().ok_or_else(invalid)?;
    if key.as_str() != CONTEXT_KEY || text.len() > MAX_BYTES {
        return Err(invalid());
    }
    let facts: CharacterItemFacts = serde_json::from_str(text).map_err(|_| invalid())?;
    facts.validate()?;
    if serde_json::to_string(&facts).map_err(|_| invalid())? != *text {
        return Err(invalid());
    }
    Ok(Some(facts))
}
fn hinted_profile(hint: &str) -> ItemProfile {
    let lowercase = hint.to_lowercase();
    // Decorations and containers do not determine the principal object's profile.
    let end = [
        " with ",
        " that ",
        " stamped ",
        " etched ",
        " wrapped ",
        " lined ",
        " sealed ",
        " carved ",
        " inked ",
        " cut ",
        " embroidered ",
        " fitted ",
    ]
    .iter()
    .filter_map(|marker| lowercase.find(marker))
    .min()
    .unwrap_or(lowercase.len());
    let head = &lowercase[..end];
    let words: Vec<_> = head.split(|c: char| !c.is_alphabetic()).collect();
    let has = |terms: &[&str]| terms.iter().any(|term| words.contains(term));
    if has(&[
        "clasp",
        "saw",
        "hook",
        "gauge",
        "needle",
        "whetstone",
        "pot",
        "receiver",
        "sniffer",
        "cage",
        "kit",
        "shim",
        "remote",
        "thermos",
        "vane",
        "compass",
        "lantern",
        "lamp",
        "mirror",
        "watch",
        "instrument",
        "radio",
        "lens",
        "recorder",
        "flute",
        "tool",
    ]) || (has(&["axe"]) && lowercase.contains("rigging"))
    {
        return ItemProfile::Tool;
    }
    if has(&[
        "book",
        "ledger",
        "journal",
        "notebook",
        "manual",
        "map",
        "chart",
        "timetable",
    ]) || head.contains("tide table")
    {
        return ItemProfile::Book;
    }
    if has(&[
        "key", "seal", "permit", "badge", "token", "passport", "license",
    ]) && !head.contains("seal-bone")
    {
        return ItemProfile::KeyItem;
    }
    if has(&[
        "sword",
        "dagger",
        "knife",
        "pistol",
        "rifle",
        "spear",
        "blade",
        "bow",
        "weapon",
        "harpoon",
        "flintlock",
        "pepperbox",
        "carbine",
        "axe",
    ]) {
        return ItemProfile::Weapon;
    }
    if has(&["armor", "armour", "shield", "helmet", "mail", "breastplate"]) {
        return ItemProfile::Armor;
    }
    if has(&["potion", "vial", "medicine", "ration", "elixir"]) {
        return ItemProfile::Consumable;
    }
    if has(&["ore", "seed", "material", "ingot"]) {
        return ItemProfile::Resource;
    }
    if has(&["coin", "gem", "jewel", "treasure"]) {
        return ItemProfile::Treasure;
    }
    if has(&["relic", "artifact", "amulet", "charm", "ring", "bell"]) {
        return ItemProfile::Artifact;
    }
    ItemProfile::Other
}
fn gear_function(gear: &str) -> &'static str {
    let lower = gear.to_lowercase();
    if lower.contains("receiver") || lower.contains("sniffer") || lower.contains("radio") {
        "It lets its keeper receive or inspect signals through the described apparatus."
    } else if lower.contains("cage") {
        "It provides a portable enclosure; its latch and lining still require inspection before use."
    } else if lower.contains("needle") {
        "It helps its keeper carry out fine stitching and repairs with practiced hands."
    } else if lower.contains("saw") {
        "It supports controlled cutting with the keeper's own trained hands."
    } else if lower.contains("gauge") {
        "It presents a reading that its keeper must interpret and check against other evidence."
    } else if lower.contains("whetstone") {
        "It gives its keeper a surface for tending a working edge."
    } else if lower.contains("hook") || lower.contains("rigging") {
        "It helps its keeper catch, hold, or tend lines using practiced handling."
    } else if lower.contains("clasp") {
        "It secures the keeper's carried material and supports the additional use described in the gear account."
    } else if lower.contains("lantern") || lower.contains("lamp") {
        "It provides a carried light whose supply and condition must be maintained."
    } else if lower.contains("pot") || lower.contains("thermos") {
        "It holds the keeper's provisions; its condition limits what it can safely retain."
    } else if lower.contains("remote") {
        "It offers controls whose response must be tested on the intended device."
    } else if lower.contains("shim") || lower.contains("seal kit") {
        "It supports precise manipulation or marking; compatible hardware and the keeper's skill remain necessary."
    } else if lower.contains("vane") {
        "It offers a directional indication whose meaning the keeper must assess."
    } else {
        match hinted_profile(gear) {
            ItemProfile::Weapon => {
                "It serves as the described personal weapon; handling, aim, and consequences depend on the keeper."
            }
            ItemProfile::Book => {
                "It keeps written material available for reading, comparison, and annotation; it cannot establish the truth of an entry."
            }
            ItemProfile::KeyItem => {
                "It presents the described key or credential; access still depends on compatible locks or a recipient's recognition."
            }
            ItemProfile::Armor => {
                "It supplies the protection described by its construction; fit and condition limit its use."
            }
            ItemProfile::Consumable => {
                "It holds the described consumable for deliberate use; replenishment and suitability require attention."
            }
            ItemProfile::Resource => {
                "It keeps the described material available for the keeper's work; quality and supply remain limited."
            }
            ItemProfile::Treasure => {
                "It carries portable value; appraisal and willingness to accept it remain separate questions."
            }
            ItemProfile::Artifact => {
                "It preserves the described object and its associations; extraordinary claims still require evidence."
            }
            _ => {
                "It makes the described personal object available for its keeper's work; the account of the gear sets its practical limits."
            }
        }
    }
}

pub(crate) fn adapt(
    f: &CharacterItemFacts,
    dna: &ForgeDNA,
    source: &str,
    field: ItemField,
    base: ItemFieldValue,
    stream: &mut FieldStream,
) -> ValidationResult<ItemFieldValue> {
    if field == ItemField::Profile {
        return Ok(ItemFieldValue::Profile(hinted_profile(&f.signature_gear)));
    }
    if matches!(
        field,
        ItemField::Name | ItemField::Rarity | ItemField::Condition
    ) {
        return Ok(base);
    }
    let i = stream.next_bounded(4)? as usize;
    let name = &f.name;
    let gear = &f.signature_gear;
    let skills = f.skills.join(" / ");
    let abilities = f.abilities.join(" / ");
    let text=match field {
        ItemField::Appearance => {
            let theme=crate::item_content::theme(source).ok_or_else(invalid)?;
            let finish=(i + usize::from(*dna.recipe.creative_axes.levels.get(&CreativeAxis::Technology).unwrap_or(&50))/20)%6;
            format!("A personal interpretation of “{gear}”. {} Its carrying arrangement is recognizable beside {name}, described as “{}”.",theme.surfaces[finish],f.appearance)
        },
        ItemField::Purpose => [
            format!("Help {name} carry out “{}”, while pursuing “{}”. The commission centers on “{gear}”.",f.role,f.desire),
            format!("Keep “{gear}” ready for {name}'s work as “{}”; its keeper hopes it can help with “{}”.",f.role,f.desire),
            format!("Give {name} a dependable companion for “{}”, shaped around the familiar “{gear}”.",f.desire),
            format!("Support the practical work behind “{}”: {name}'s “{gear}” must serve a person, not merely display their title.",f.archetype),
        ][i].clone(),
        ItemField::Properties => format!("Handling marks distinguish this interpretation of “{gear}” from another keeper's example. Its fittings and usable surfaces follow the described object. Handling notes refer to “{skills}”; ownership does not grant those talents."),
        ItemField::Origin => format!("{} A surviving commission identifies its intended keeper as {name}, “{}”, and describes the requested object as “{gear}”.",text_of(base),f.title),
        ItemField::History => format!("{} A later custody note connects it with this account of {name}: “{}”. The note needs corroboration.",text_of(base),f.backstory_hook),
        ItemField::PowerOrFunction => format!("{} {name} uses “{gear}” alongside “{skills}”. Keepers distinguish its function from the Character's own abilities: “{abilities}”.",gear_function(gear)),
        ItemField::CostOrDrawback => [format!("Maintaining “{gear}” takes time its keeper might need elsewhere."), format!("A damaged or unfamiliar part of “{gear}” can make its use unreliable until inspected."), format!("Carrying “{gear}” makes the keeper recognizable to people who know the object."), format!("Using “{gear}” well requires the keeper's judgment; it cannot decide whom to trust.")][i].clone() + &format!(" For {name}, the difficult choice is whether using it could bring about “{}”; “{}” makes that judgment harder.",f.fear,f.flaw),
        ItemField::Secret => [
            format!("A concealed custody note links “{gear}” to this secret about {name}: “{}”. It does not reveal who wrote the note.",f.secret),
            format!("One prior keeper recorded “{}” beside {name}'s name. The record was hidden in the wrapping of “{gear}”.",f.secret),
            format!("Someone recognizes “{gear}” because of “{}”, a fact {name} has kept out of the public account.",f.secret),
            format!("The most guarded account of “{gear}” concerns {name}: “{}”. A second witness could confirm or dispute it.",f.secret),
        ][i].clone(),
        ItemField::StoryHooks => {
            let mut hooks=vec![format!("A dispute over “{gear}” draws in this connection: “{}”. Who can speak for {name}'s claim?",f.relationship_hook),format!("Evidence concerning “{}” calls the item's custody into question; {name} must decide what to disclose.",f.backstory_hook),format!("Using “{gear}” might advance “{}”, but a witness asks about “{}” first.",f.desire,f.secret)];
            hooks.rotate_left(i%3);return Ok(ItemFieldValue::Texts(hooks));
        },
        _ => return Ok(base),
    };
    Ok(ItemFieldValue::Text(text))
}
fn text_of(v: ItemFieldValue) -> String {
    if let ItemFieldValue::Text(s) = v {
        s
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CharacterForge, HometownSource, ItemForge, forge_character_signature_item};
    use forge_contracts::{ConstraintKey, CreativeAxes, RootSeed};
    use std::collections::BTreeMap;
    fn bundle() -> crate::SignatureItemResultBundle {
        let dna = CharacterForge::new_dna(
            RootSeed::new(0),
            "dark-fantasy",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        let c = CharacterForge::generate(&dna).unwrap();
        forge_character_signature_item(
            &HometownSource {
                character: c.character.clone(),
                field_origins: c.field_origins,
                write_version: 1,
            },
            100,
            c.character.envelope.project_id,
        )
        .unwrap()
    }
    #[test]
    fn signature_item_context_rejects_unknown_noncanonical_oversized_and_unbounded_facts() {
        let original = bundle();
        for case in 0..6 {
            let mut dna = original.primary.dna.clone();
            let key = ConstraintKey::new(CONTEXT_KEY).unwrap();
            let text = dna.recipe.constraints.get(&key).unwrap().clone();
            match case {
                0 => {
                    dna.recipe
                        .constraints
                        .insert(ConstraintKey::new("unknown").unwrap(), text);
                }
                1 => {
                    dna.recipe.constraints.insert(
                        key,
                        serde_json::to_string_pretty(&original.recipe.context_snapshot.facts)
                            .unwrap(),
                    );
                }
                2 => {
                    dna.recipe
                        .constraints
                        .insert(key, "x".repeat(MAX_BYTES + 1));
                }
                3 => {
                    let mut facts = original.recipe.context_snapshot.facts.clone();
                    facts.skills = vec!["skill".into(); 17];
                    dna.recipe
                        .constraints
                        .insert(key, serde_json::to_string(&facts).unwrap());
                }
                4 => {
                    let mut facts = original.recipe.context_snapshot.facts.clone();
                    facts.signature_gear = " ".into();
                    dna.recipe
                        .constraints
                        .insert(key, serde_json::to_string(&facts).unwrap());
                }
                _ => {
                    let mut value = serde_json::from_str::<serde_json::Value>(&text).unwrap();
                    value["hidden_truth"] = "invented".into();
                    dna.recipe
                        .constraints
                        .insert(key, serde_json::to_string(&value).unwrap());
                }
            }
            assert!(ItemForge::generate(&dna).is_err(), "case {case}");
        }
        let standalone =
            ItemForge::new_dna(RootSeed::new(1), "dark-fantasy", CreativeAxes::neutral()).unwrap();
        assert_eq!(from_dna(&standalone).unwrap(), None);
    }
    #[test]
    fn signature_item_principal_object_outranks_decoration_and_function_remains_grounded() {
        for (gear, profile) in [
            ("a seal-bone needle for mending sails", ItemProfile::Tool),
            (
                "a bone-handled field saw stamped with a monastery seal",
                ItemProfile::Tool,
            ),
            (
                "a folding cage lined with old campaign maps",
                ItemProfile::Tool,
            ),
            (
                "a hand-cranked receiver wrapped in a school map",
                ItemProfile::Tool,
            ),
            (
                "a tide table inked inside a waterproof coat",
                ItemProfile::Book,
            ),
            (
                "a compact pepperbox stamped with a factory serial",
                ItemProfile::Weapon,
            ),
            (
                "a blackthorn stick with a map hidden in its ferrule",
                ItemProfile::Other,
            ),
            (
                "a velvet key roll embroidered with vanished street names",
                ItemProfile::KeyItem,
            ),
        ] {
            assert_eq!(hinted_profile(gear), profile, "{gear}");
        }
        assert!(gear_function("a folding cage lined with maps").contains("enclosure"));
        assert!(gear_function("a brass lantern").contains("light"));
        assert!(gear_function("a pocket receiver wrapped in a map").contains("signals"));
    }
}
