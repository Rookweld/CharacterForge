//! B10.R1/R2 deterministic projection: physical identity and role precede styling.
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
fn profession(c: &VisualContextSnapshot) -> String {
    // Use only an occupation word actually present in canonical role/archetype.
    let professions = [
        "physician",
        "doctor",
        "healer",
        "apothecary",
        "surgeon",
        "warden",
        "keeper",
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
    ];
    let archetype = words(&fact(c, F::Archetype));
    let role = words(&fact(c, F::Role));
    archetype
        .iter()
        .rev()
        .chain(role.iter().rev())
        .find(|s| professions.contains(&s.as_str()))
        .cloned()
        .unwrap_or_else(|| "person".into())
}
/// Suggested image details, not additional source facts. Canonical appearance and
/// equipment take precedence; this finite vocabulary never assigns species.
fn role_evidence(profession: &str) -> &'static str {
    match profession {
        "physician" | "doctor" | "healer" | "surgeon" | "apothecary" => {
            "medical instrument roll, folded bandages and stoppered medicine bottles; practical treatment apron or satchel, worn cuffs and medicinal stains; visible preparation for treating a patient"
        }
        "scholar" => {
            "annotated pages, writing implement and reference folio; ink-marked cuffs and well-used book edges"
        }
        "engineer" => {
            "measuring tools, repair kit and technical sketches; workwear with material dust and repaired seams"
        }
        "detective" => {
            "evidence notebook and inspection tools; travel-worn coat details and a visible clue under examination"
        }
        "courier" => {
            "message case and route notes; weather-worn carrying straps and scuffed travel gear"
        }
        "merchant" => {
            "sample goods, balance and transaction ledger; handled packaging and a practical carrying case"
        }
        "sailor" | "pilot" => {
            "navigation instruments and route chart; weathered working equipment and travel wear"
        }
        "hunter" | "ranger" => {
            "tracking kit and field supplies; repaired outdoor gear and traces of fieldwork"
        }
        "soldier" | "guard" | "knight" | "warrior" => {
            "source-described defensive equipment in functional use; maintained edges, repaired straps and service wear"
        }
        "butcher" => "cutting tools and work apron; marked work surfaces and wear from the trade",
        "priest" | "priestess" => {
            "ritual objects and a worn devotional text; service-worn ceremonial garment details"
        }
        "keeper" | "warden" => {
            "maintenance tools, keys and inspection notes; repaired workwear and signs of a tended post"
        }
        "mage" | "witch" => {
            "working implements and annotated ritual materials; material residue on practical equipment"
        }
        "thief" | "assassin" | "smuggler" => {
            "discreet tool case and concealed carrying compartments; worn fastenings and practical travel layers"
        }
        _ => {
            "functional use, material texture and wear of the source-described signature gear; occupation cues drawn from the canonical role"
        }
    }
}
fn has(tokens: &[String], word: &str) -> bool {
    tokens.iter().any(|s| s == word)
}
/// Interpret only public title/archetype vocabulary. These are optional staging
/// motifs, never an authoritative location, affiliation, uniform or possession.
fn label_cues(c: &VisualContextSnapshot) -> (Vec<String>, Vec<String>) {
    let tokens = words(&[fact(c, F::Name), fact(c, F::Title), fact(c, F::Archetype)].join(" "));
    let mut props = Vec::new();
    let mut setting = Vec::new();
    if has(&tokens, "chapel") {
        props.push(
            "chapel-associated garment accents: faded votive-stitch trim and candle-wax wear"
                .into(),
        );
        setting.push("weathered chapel stone arch or votive niche as a secondary backdrop".into());
    }
    if has(&tokens, "unlit") {
        props.push("extinguished lamp accessory and candle-soot marks on equipment".into());
        setting.push("unlit lamp silhouettes in the secondary backdrop".into());
    }
    if has(&tokens, "mile")
        && ["keeper", "road", "route", "way"]
            .iter()
            .any(|w| has(&tokens, w))
    {
        props.push("small mile-marker motif on a carrying strap or tool case".into());
        setting.push(
            "forgotten roadside mile-marker beside a lonely waystation or parish-route fragment"
                .into(),
        );
    }
    if has(&tokens, "bell") {
        props.push("worn bell-shaped accessory or brass clapper detail".into());
    }
    if has(&tokens, "ash") {
        props.push("ash-grey garment edging and soot-marked working materials".into());
    }
    (props, setting)
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
    let primary = match kind {
        K::Human => format!("human {} {}", genre(c), profession(c)),
        K::Humanoid => format!(
            "humanoid {} {} with the explicit canonical non-human appearance",
            genre(c),
            profession(c)
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
        E::Character => {
            add("Role / profession", &[F::Role]);
            add("Physical identity", &[F::Appearance]);
            add("Clothing / equipment", &[F::SignatureGear]);
            identity.push(format!("Role evidence (visual interpretation; canonical gear/appearance take precedence): {}", role_evidence(&profession(c))));
            let (props, settings) = label_cues(c);
            if !props.is_empty() {
                identity.push(format!(
                    "Title / archetype prop accents (visual interpretation): {}",
                    props.join("; ")
                ));
            }
            identity.push(if settings.is_empty() {
                "Setting: restrained working context suggested by the canonical role; subject and named equipment remain dominant".into()
            } else {
                format!("Setting (title / archetype visual interpretation): {}; subordinate to the working {} and source-described equipment", settings.join("; "), profession(c))
            });
            flavor.push(format!(
                "Identity labels / epithets (figurative): {}",
                labels.join("; ")
            ));
            if guard && !risks.is_empty() {
                flavor.push(format!(
                    "Symbolic motifs: {}",
                    risks
                        .iter()
                        .map(|s| format!(
                            "subtle {s} symbolism on garment trim or a small tool-case ornament"
                        ))
                        .collect::<Vec<_>>()
                        .join("; ")
                ));
            }
            let details = [F::Personality, F::Abilities, F::FactionHint]
                .iter()
                .map(|f| fact(c, *f))
                .collect::<Vec<_>>()
                .join("; ");
            flavor.push(format!(
                "Supporting design context (observable cues only): {details}"
            ));
        }
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
        },
        safeguards,
    )
}
