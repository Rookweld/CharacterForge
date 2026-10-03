//! B10.R3: subject anchor, semantic occupation and category-bounded interpretation.
use super::{
    LiteralizationRisk as L, PromptSemantics, SpeciesMode as M, SubjectClassification,
    SubjectKind as K, VisualContextSnapshot, VisualField as F,
};
const ANIMALS: [&str; 8] = [
    "rat", "raven", "crow", "wolf", "serpent", "spider", "beast", "carrion",
];
const RISKS: [&str; 13] = [
    "rat", "raven", "crow", "wolf", "serpent", "spider", "beast", "ghost", "bone", "plague",
    "carrion", "ash", "butcher",
];
fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(str::to_lowercase)
        .collect()
}
fn fact(c: &VisualContextSnapshot, field: F) -> String {
    c.facts
        .iter()
        .filter(|f| f.field == field)
        .flat_map(|f| f.values.iter())
        .cloned()
        .collect::<Vec<_>>()
        .join(" / ")
}
fn genre(c: &VisualContextSnapshot) -> &'static str {
    match c.source.generation.as_ref().map(|g| g.preset.as_str()) {
        Some("dark-fantasy") => "dark fantasy",
        Some("nordic-myth") => "Norse mythic",
        Some("cyberpunk") => "cyberpunk",
        Some("pirate") => "maritime adventure",
        Some("post-apocalyptic") => "post-apocalyptic",
        Some("steampunk") => "steampunk",
        Some("occult") => "occult",
        Some("monster-hunter") => "monster hunting",
        Some("rogue-assassin") => "rogue and assassin",
        Some("contemporary-weird") => "contemporary weird",
        Some("chaos-blend") => "mixed-genre",
        _ => "",
    }
}
pub(super) fn style_line(c: &VisualContextSnapshot) -> String {
    let d = &c.art_direction;
    let medium = if d.medium_style.trim().is_empty() {
        "clear concept illustration"
    } else {
        d.medium_style.trim()
    };
    let mut parts = vec![format!("Style: {} {}", genre(c), medium).trim().to_owned()];
    if !d.palette.trim().is_empty() {
        parts.push(format!("palette: {}", d.palette));
    }
    if !d.materials.trim().is_empty() {
        parts.push(format!("materials: {}", d.materials));
    }
    if !d.camera_composition.trim().is_empty() {
        parts.push(format!("camera: {}", d.camera_composition));
    }
    parts.join("; ")
}
/// Only physical appearance can override a Character's human default.
/// Finite positive species/anatomy cues; titles/archetypes and art direction never do.
fn physical_kind(appearance: &str) -> (K, M) {
    let tokens = words(appearance);
    let cosmetic = [
        "mask",
        "costume",
        "tattoo",
        "symbol",
        "symbolism",
        "motif",
        "emblem",
        "pendant",
        "necklace",
        "brooch",
        "helmet",
    ];
    let species = [
        "elf",
        "elven",
        "orc",
        "orcish",
        "dwarf",
        "dwarven",
        "goblin",
        "troll",
        "tiefling",
        "android",
        "robot",
        "vampire",
        "undead",
        "zombie",
        "skeleton",
        "demon",
        "faerie",
        "werewolf",
        "lycanthrope",
    ];
    let animals = [
        "rat", "rodent", "raven", "crow", "wolf", "serpent", "snake", "spider", "dragon", "beast",
    ];
    for (i, word) in tokens.iter().enumerate() {
        if !species.contains(&word.as_str()) && !animals.contains(&word.as_str()) {
            continue;
        }
        let lo = i.saturating_sub(2);
        let hi = (i + 4).min(tokens.len());
        if tokens[lo..hi].iter().any(|s| {
            cosmetic.contains(&s.as_str()) || ["like", "not", "no", "without"].contains(&s.as_str())
        }) {
            continue;
        }
        let next = tokens.get(i + 1).map(String::as_str).unwrap_or("");
        let previous = tokens
            .get(i.wrapping_sub(1))
            .map(String::as_str)
            .unwrap_or("");
        let prefix = i == 0 || (i == 1 && ["a", "an"].contains(&previous));
        let declared = previous == "species" || previous == "anthropomorphic";
        let anatomy = [
            "head",
            "headed",
            "body",
            "bodied",
            "skin",
            "ears",
            "features",
            "physiology",
        ]
        .contains(&next);
        let person = [
            "woman",
            "man",
            "person",
            "humanoid",
            "physician",
            "doctor",
            "warrior",
            "child",
            "adult",
        ]
        .contains(&next);
        if declared
            || anatomy
            || person
            || ((prefix || species.contains(&word.as_str()))
                && (next.is_empty() || ["with", "whose", "wearing", "in"].contains(&next)))
        {
            return (
                if species.contains(&word.as_str())
                    || person
                    || (anatomy
                        && tokens.get(i + 2).is_some_and(|s| {
                            ["physician", "doctor", "woman", "man", "person", "warrior"]
                                .contains(&s.as_str())
                        }))
                    || tokens
                        .iter()
                        .any(|s| ["anthropomorphic", "humanoid"].contains(&s.as_str()))
                {
                    K::Humanoid
                } else {
                    K::Beast
                },
                M::Explicit,
            );
        }
    }
    if appearance.to_lowercase().contains("species:") {
        return (
            if tokens.iter().any(|s| s == "human") {
                K::Human
            } else {
                K::Unknown
            },
            M::Explicit,
        );
    }
    if tokens.windows(2).any(|w| w == ["non", "human"]) || tokens.iter().any(|s| s == "humanoid") {
        return (K::Humanoid, M::Explicit);
    }
    if tokens.iter().enumerate().any(|(i, s)| {
        s == "human"
            && !i
                .checked_sub(1)
                .is_some_and(|n| ["not", "no"].contains(&tokens[n].as_str()))
    }) {
        return (K::Human, M::Explicit);
    }
    (K::Human, M::InferredDefault)
}
// Category limits are selection limits, not character-string truncation.
pub(super) const OCCUPATION_LIMIT: usize = 2;
pub(super) const ACCENT_LIMIT: usize = 2;
pub(super) const SETTING_LIMIT: usize = 1;
pub(super) const MOTIF_LIMIT: usize = 1;
pub(super) const DOMINANCE_NEGATIVES: [&str; 6] = [
    "object-only composition",
    "architectural detail as sole subject",
    "emblem-only image",
    "still life",
    "empty environment",
    "shrine, relic, door or wall ornament as sole subject",
];
fn has(tokens: &[String], word: &str) -> bool {
    tokens.iter().any(|s| s == word)
}
fn occupation(c: &VisualContextSnapshot) -> Option<String> {
    // Bare keeper/warden are narrative responsibilities, not specific professions.
    const OCCUPATIONS: [&str; 29] = [
        "physician",
        "doctor",
        "healer",
        "apothecary",
        "surgeon",
        "knight",
        "priest",
        "priestess",
        "scholar",
        "hunter",
        "guard",
        "sailor",
        "smuggler",
        "merchant",
        "soldier",
        "assassin",
        "ranger",
        "engineer",
        "pilot",
        "butcher",
        "warrior",
        "mage",
        "witch",
        "detective",
        "courier",
        "thief",
        "blacksmith",
        "farmer",
        "carpenter",
    ];
    for field in [F::Archetype, F::Role, F::Appearance] {
        let tokens = words(&fact(c, field));
        for (i, word) in tokens.iter().enumerate().rev() {
            if OCCUPATIONS.contains(&word.as_str())
                && !tokens[i.saturating_sub(3)..i]
                    .iter()
                    .any(|s| ["not", "no", "never", "without"].contains(&s.as_str()))
            {
                return Some(word.clone());
            }
        }
    }
    None
}
/// Whole source-derived clauses, ranked by physical/functional relevance.
/// Overlong clauses stay in the exact context rather than being cut into fragments.
fn clause(text: &str, preferred: &[&str], max_words: usize) -> Option<String> {
    text.split(['.', ';', '\n', '!', '?'])
        .map(str::trim)
        .filter(|s| !s.is_empty() && s.len() <= 512 && s.split_whitespace().count() <= max_words)
        .min_by_key(|s| {
            if words(s).iter().any(|w| preferred.contains(&w.as_str())) {
                0
            } else {
                1
            }
        })
        .map(super::scrub_identifiers)
}
fn occupation_cues(occupation: Option<&str>) -> Vec<String> {
    let candidates: &[&str] = match occupation {
        Some("physician" | "doctor" | "healer" | "surgeon" | "apothecary") => {
            &["medical satchel", "folded bandages"]
        }
        Some("scholar") => &["annotated pages", "ink-marked cuffs"],
        Some("engineer" | "blacksmith" | "carpenter") => &["measuring tools", "work-worn cuffs"],
        Some("detective") => &["evidence notebook", "inspection tool"],
        Some("courier") => &["message case", "weather-worn carrying strap"],
        Some("merchant") => &["transaction ledger", "sample goods"],
        Some("sailor" | "pilot") => &["navigation instrument", "route chart"],
        Some("hunter" | "ranger") => &["tracking kit", "field-worn clothing"],
        Some("soldier" | "guard" | "knight" | "warrior") => {
            &["repaired equipment strap", "service wear"]
        }
        Some("butcher") => &["cutting tool", "work apron"],
        Some("priest" | "priestess") => &["worn devotional text"],
        Some("mage" | "witch") => &["annotated ritual materials"],
        Some("thief" | "assassin" | "smuggler") => &["discreet tool case"],
        Some("farmer") => &["field tool", "earth-marked workwear"],
        _ => &[],
    };
    candidates
        .iter()
        .take(OCCUPATION_LIMIT)
        .map(|s| (*s).into())
        .collect()
}
fn framing(role: super::VisualRole) -> &'static str {
    match role {
        super::VisualRole::Portrait => "portrait with face and carried gear readable",
        super::VisualRole::FullBody => "full-body Character view",
        super::VisualRole::CharacterSheet => "consistent Character reference views",
        _ => "Character-centered view",
    }
}
fn selection(
    c: &VisualContextSnapshot,
    risks: &[&str],
    guard: bool,
) -> super::CharacterPromptSelection {
    let occupation = occupation(c);
    let appearance_text = fact(c, F::Appearance);
    let appearance = if guard {
        clause(
            &appearance_text,
            &["eye", "eyes", "scar", "ears", "skin"],
            32,
        )
    } else {
        let pieces = appearance_text
            .split(['.', ';', '\n', '!', '?'])
            .filter_map(|s| clause(s, &[], 32))
            .take(2)
            .collect::<Vec<_>>();
        if pieces.is_empty() {
            None
        } else {
            Some(pieces.join("; "))
        }
    };
    let signature_gear = clause(
        &fact(c, F::SignatureGear),
        &["pistol", "weapon", "satchel", "cane", "tool"],
        24,
    );
    let canonical = words(
        &[
            appearance.as_deref().unwrap_or(""),
            signature_gear.as_deref().unwrap_or(""),
        ]
        .join(" "),
    );
    let label_words =
        words(&[fact(c, F::Name), fact(c, F::Title), fact(c, F::Archetype)].join(" "));
    // Each cue family can be represented only once, including canonical anchor facts.
    let mut props = Vec::new();
    if has(&label_words, "unlit") && !has(&canonical, "lamp") {
        props.push("one extinguished lamp carried as a small accessory".into());
    }
    if has(&label_words, "chapel") && !has(&canonical, "chapel") {
        props.push("small chapel-associated garment stitch".into());
    }
    if has(&label_words, "bell") && !has(&canonical, "bell") {
        props.push("small bell-shaped accessory".into());
    }
    if has(&label_words, "ash") && !has(&canonical, "ash") {
        props.push("ash-grey garment edging".into());
    }
    props.truncate(ACCENT_LIMIT);
    let mut setting = Vec::new();
    if has(&label_words, "mile")
        && ["keeper", "road", "route", "way"]
            .iter()
            .any(|s| has(&label_words, s))
    {
        setting.push("one faint roadside mile-marker in the distant background".into());
    }
    setting.truncate(SETTING_LIMIT);
    let motifs = if guard {
        risks
            .iter()
            .filter(|risk| !has(&canonical, risk))
            .take(MOTIF_LIMIT)
            .map(|s| format!("subtle {s} symbolism as one small garment motif"))
            .collect()
    } else {
        Vec::new()
    };
    let occupation_evidence = occupation_cues(occupation.as_deref())
        .into_iter()
        .filter(|s| !words(s).iter().any(|w| canonical.contains(w)))
        .take(OCCUPATION_LIMIT)
        .collect();
    super::CharacterPromptSelection {
        occupation,
        narrative_role: fact(c, F::Role),
        appearance,
        signature_gear,
        occupation_evidence,
        prop_accents: props,
        setting,
        motifs,
    }
}
fn character_identity(
    c: &VisualContextSnapshot,
    kind: K,
    plan: &super::CharacterPromptSelection,
) -> Vec<String> {
    let species = match kind {
        K::Human => "human",
        K::Humanoid => "explicitly non-human humanoid",
        K::Beast => "explicitly non-human animal-form",
        _ => "explicit canonical-species",
    };
    let profession = plan.occupation.as_deref().unwrap_or("");
    let mut anchor = format!(
        "A single {species} {} {profession} Character as the dominant focal subject",
        genre(c)
    );
    anchor = anchor.split_whitespace().collect::<Vec<_>>().join(" ");
    if let Some(appearance) = &plan.appearance {
        anchor.push_str(&format!("; {appearance}"));
    }
    if let Some(gear) = &plan.signature_gear {
        anchor.push_str(&format!("; carrying {gear}"));
    }
    anchor.push_str(&format!("; {}.", framing(c.role)));
    let mut lines = vec![anchor];
    if !plan.occupation_evidence.is_empty() {
        lines.push(format!(
            "Occupation evidence (visual interpretation, secondary): {}",
            plan.occupation_evidence.join("; ")
        ));
    }
    if let Some(role) = clause(&plan.narrative_role, &[], 32)
        && plan.occupation.as_deref() != Some(role.to_lowercase().as_str())
    {
        lines.push(format!("Narrative role: {role}"));
    }
    if !plan.prop_accents.is_empty() {
        lines.push(format!(
            "Prop accents (visual interpretation, secondary): {}",
            plan.prop_accents.join("; ")
        ));
    }
    if !plan.setting.is_empty() {
        lines.push(format!(
            "Setting (visual interpretation, secondary): {}",
            plan.setting.join("; ")
        ));
    }
    if !plan.motifs.is_empty() {
        lines.push(format!(
            "Symbolic motif (visual interpretation, secondary): {}",
            plan.motifs.join("; ")
        ));
    }
    lines
        .into_iter()
        .map(|s| super::scrub_identifiers(&s))
        .collect()
}
pub(super) fn render_character(b: &super::VisualGenerationBrief) -> String {
    let mut lines = b.prompt_semantics.identity.clone();
    let d = &b.context.art_direction;
    let medium =
        clause(&d.medium_style, &[], 12).unwrap_or_else(|| "clear concept illustration".into());
    let mut style = vec![
        format!("Style: {} {medium}", genre(&b.context))
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
    ];
    for (label, text) in [
        ("palette", &d.palette),
        ("materials", &d.materials),
        ("atmosphere", &d.atmosphere),
    ] {
        if let Some(cue) = clause(text, &[], 8) {
            style.push(format!("{label}: {cue}"));
        }
    }
    lines.push(style.join("; "));
    let mut composition = format!(
        "Composition: {}; the Character is primary; props, symbols, architecture and environment are secondary supporting elements.",
        framing(b.context.role)
    );
    if let Some(camera) = clause(&d.camera_composition, &[], 12) {
        composition.push_str(&format!(" Camera: {camera}"));
    }
    lines.push(composition);
    lines.join("\n")
}
pub(super) fn compile_semantics(
    c: &VisualContextSnapshot,
    _composition: &str,
) -> (SubjectClassification, PromptSemantics, Vec<String>) {
    use forge_contracts::EntityType as E;
    let labels = [fact(c, F::Name), fact(c, F::Title), fact(c, F::Archetype)];
    let label_words = words(&labels.join(" "));
    let risks = RISKS
        .iter()
        .filter(|word| label_words.iter().any(|s| s == **word))
        .copied()
        .collect::<Vec<_>>();
    let (kind, mode) = match c.source.entity_type {
        E::Character => physical_kind(&fact(c, F::Appearance)),
        E::Creature => (K::Creature, M::Explicit),
        E::Item => (K::Object, M::Explicit),
        E::Location => (K::Location, M::Explicit),
        E::Faction => (K::FactionSymbolic, M::Explicit),
        _ => (K::Unknown, M::InferredDefault),
    };
    let guard = c.source.entity_type == E::Character && kind == K::Human;
    let classification = SubjectClassification {
        subject_kind: kind,
        species_mode: mode,
        literalization_risk: if risks.iter().any(|r| ANIMALS.contains(r)) {
            L::High
        } else if risks.is_empty() {
            L::Low
        } else {
            L::Medium
        },
        humanoid_guardrail: guard,
    };
    let mut negative = vec![
        "undisclosed secrets".into(),
        "invented authoritative insignia".into(),
        "invented anatomy".into(),
        "invented game statistics".into(),
    ];
    let mut safeguards =
        vec!["Canonical physical identity precedes names and figurative labels.".into()];
    if c.source.entity_type == E::Character {
        safeguards.push("Role evidence and label-derived props/settings are visual interpretations, subordinate to canonical appearance/equipment; no Canon facts or insignia are established.".into());
        // Target anonymous aesthetic drift, not attractiveness or genuine clergy.
        let requested_style = words(&c.art_direction.medium_style);
        if !["glamour", "fashion", "beauty", "beautiful"]
            .iter()
            .any(|s| has(&requested_style, s))
        {
            negative.extend(
                [
                    "anonymous glamour headshot",
                    "fashion editorial without occupational cues",
                ]
                .map(str::to_owned),
            );
        }
        let clerical =
            words(&[labels.join(" "), fact(c, F::Role), fact(c, F::Appearance)].join(" "));
        if has(&label_words, "chapel")
            && !["nun", "monk", "priest", "priestess", "abbot", "bishop"]
                .iter()
                .any(|s| has(&clerical, s))
        {
            negative.extend(
                [
                    "nun-only portrait without occupational tools",
                    "monk-only portrait without occupational tools",
                ]
                .map(str::to_owned),
            );
        }
    }
    if guard {
        negative.extend(["animal head", "animal body", "mascot", "parody"].map(str::to_owned));
        for risk in &risks {
            if ANIMALS.contains(risk) {
                negative.push(format!("literal {risk}"));
            }
        }
        if risks.contains(&"rat") {
            negative.push("anthropomorphic rodent".into());
        }
        if risks.contains(&"wolf") {
            negative.push("anthropomorphic wolf".into());
        }
        if risks.contains(&"ghost") {
            negative.push("literal ghost replacing the person".into());
        }
        if risks.contains(&"bone") {
            negative.push("skeleton replacing the person".into());
        }
        safeguards
            .push("Human anatomy retained; animal/epithet terms are symbolic by default.".into());
    } else if c.source.entity_type == E::Character {
        safeguards.push("Explicit canonical non-human appearance retained; no human-only anatomy negatives applied.".into());
    }
    if !c.art_direction.avoid.trim().is_empty() {
        negative.push(c.art_direction.avoid.trim().into());
    }
    negative = negative
        .into_iter()
        .map(|s| super::scrub_identifiers(&s))
        .collect();
    if c.source.entity_type == E::Character {
        negative.extend(DOMINANCE_NEGATIVES.map(str::to_owned));
        safeguards.push("Character-first anchor; interpretation budgets: occupation 2, prop accents 2, setting 1, motif 1; canonical context remains exact.".into());
        let plan = selection(c, &risks, guard);
        let identity = character_identity(c, kind, &plan);
        return (
            classification,
            PromptSemantics {
                identity,
                flavor_symbolism: Vec::new(),
                negative_guardrails: negative,
                character_selection: Some(plan),
            },
            safeguards,
        );
    }
    let primary = match kind {
        K::Human => format!("human {} {}", genre(c), occupation(c).unwrap_or_default()),
        K::Humanoid => format!(
            "humanoid {} {} with the explicit canonical non-human appearance",
            genre(c),
            occupation(c).unwrap_or_default()
        ),
        K::Beast => format!(
            "explicit non-human {} character with the canonical animal form",
            genre(c)
        ),
        K::Creature => format!("literal non-human {} creature", genre(c)),
        K::Object => format!("{} object / item concept", genre(c)),
        K::Location => format!("{} environment / location concept", genre(c)),
        K::FactionSymbolic => format!(
            "{} faction symbolic design / representative visual language",
            genre(c)
        ),
        K::Unknown => {
            if mode == M::Explicit {
                "character with the explicitly declared canonical species".into()
            } else {
                "unclassified subject".into()
            }
        }
    };
    let mut identity = vec![primary.split_whitespace().collect::<Vec<_>>().join(" ")];
    let mut flavor = vec![];
    let mut add = |label: &str, fields: &[F]| {
        let text = fields
            .iter()
            .map(|f| fact(c, *f))
            .filter(|s| !s.trim().is_empty())
            .collect::<Vec<_>>()
            .join("; ");
        if !text.is_empty() {
            identity.push(format!("{label}: {text}"));
        }
    };
    match c.source.entity_type {
        E::Character => unreachable!("Character is handled before generic family assembly"),
        E::Item => {
            add(
                "Object physical form / material",
                &[F::Appearance, F::Condition],
            );
            add(
                "Object function",
                &[F::Purpose, F::PowerOrFunction, F::Properties],
            );
            add(
                "Object design context",
                &[F::ItemProfile, F::Origin, F::Rarity],
            );
            flavor.push(format!("Item identity label: {}", fact(c, F::Name)));
        }
        E::Location => {
            add(
                "Environment / architecture",
                &[F::VisualIdentity, F::NotableFeatures],
            );
            add(
                "Place identity / purpose",
                &[F::Summary, F::Purpose, F::LocationProfile],
            );
            add("Atmosphere", &[F::Atmosphere, F::Danger]);
            flavor.push(format!("Location identity label: {}", fact(c, F::Name)));
        }
        E::Creature => {
            add("Literal creature anatomy", &[F::Appearance]);
            add("Creature habitat / ecology", &[F::Habitat, F::Ecology]);
            add(
                "Creature behavior / visible traits",
                &[
                    F::Behavior,
                    F::Threat,
                    F::AbilitiesSummary,
                    F::Weakness,
                    F::CreatureProfile,
                ],
            );
            flavor.push(format!("Creature identity label: {}", fact(c, F::Name)));
            safeguards.push(
                "Creature literal anatomy is preserved; Character metaphor guards are not applied."
                    .into(),
            );
        }
        E::Faction => {
            add(
                "Symbols / emblem / representative design",
                &[F::Symbols, F::PublicFace],
            );
            add(
                "Faction design context",
                &[F::Identity, F::Culture, F::Ideology, F::FactionProfile],
            );
            flavor.push(format!("Faction identity label: {}", fact(c, F::Name)));
        }
        _ => {}
    }
    if !c.art_direction.atmosphere.trim().is_empty() {
        flavor.push(format!("Art atmosphere: {}", c.art_direction.atmosphere));
    }
    identity = identity
        .into_iter()
        .map(|s| super::scrub_identifiers(&s))
        .collect();
    flavor = flavor
        .into_iter()
        .map(|s| super::scrub_identifiers(&s))
        .collect();
    (
        classification,
        PromptSemantics {
            identity,
            flavor_symbolism: flavor,
            negative_guardrails: negative,
            character_selection: None,
        },
        safeguards,
    )
}
