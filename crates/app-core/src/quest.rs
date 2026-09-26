//! Deterministic Quest Forge pipeline. Quest content is separate from Character pools.
use crate::{
    FieldStream, FieldValues,
    reroll::{lock_field, reroll_all_unlocked, reroll_field, unlock_field},
};
use forge_contracts::{
    CanonState, ContentVersion, CreativeAxes, CreativeAxis, EntityEnvelope, EntityId, EntityOrigin,
    EntityType, FieldKey, ForgeDNA, GenerationAction, GenerationKind, GenerationMetadata,
    GenerationRecipe, PresetId, ProjectId, Quest, RootSeed, ValidationCode, ValidationError,
    ValidationErrors, ValidationResult,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

const FIELDS: [QuestField; 15] = [
    QuestField::Frame,
    QuestField::Title,
    QuestField::Type,
    QuestField::Premise,
    QuestField::Patron,
    QuestField::Objective,
    QuestField::Stakes,
    QuestField::Opposition,
    QuestField::Complications,
    QuestField::Twist,
    QuestField::MoralPressure,
    QuestField::Locations,
    QuestField::Rewards,
    QuestField::Failure,
    QuestField::FollowUp,
];
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
const V2: ContentVersion = ContentVersion(2);
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestField {
    Frame,
    Title,
    Type,
    Premise,
    Patron,
    Objective,
    Stakes,
    Opposition,
    Complications,
    Twist,
    MoralPressure,
    Locations,
    Rewards,
    Failure,
    FollowUp,
}
impl QuestField {
    pub fn key(self) -> ValidationResult<FieldKey> {
        FieldKey::new(match self {
            Self::Frame => "01-frame",
            Self::Title => "02-title",
            Self::Type => "03-type",
            Self::Premise => "04-premise",
            Self::Patron => "05-patron",
            Self::Objective => "06-objective",
            Self::Stakes => "07-stakes",
            Self::Opposition => "08-opposition",
            Self::Complications => "09-complications",
            Self::Twist => "10-twist",
            Self::MoralPressure => "11-moral-pressure",
            Self::Locations => "12-locations",
            Self::Rewards => "13-rewards",
            Self::Failure => "14-failure",
            Self::FollowUp => "15-follow-up",
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum QuestFieldValue {
    Text(String),
    Texts(Vec<String>),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestOrigin {
    Generated,
    UserModified,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestCandidate {
    pub quest: Quest,
    pub dna: ForgeDNA,
    pub field_origins: BTreeMap<FieldKey, QuestOrigin>,
}
impl QuestCandidate {
    pub fn manual_edit(
        &mut self,
        field: QuestField,
        value: QuestFieldValue,
    ) -> ValidationResult<()> {
        let mut updated = self.clone();
        set(&mut updated.quest, field, value)?;
        updated
            .field_origins
            .insert(field.key()?, QuestOrigin::UserModified);
        if let Some(provenance) = &mut updated.quest.envelope.generation_provenance {
            provenance.action = GenerationAction::ManualEdit;
        }
        updated.validate()?;
        *self = updated;
        Ok(())
    }
    pub fn validate(&self) -> ValidationResult<()> {
        self.quest.validate()?;
        validate_dna(&self.dna)?;
        if self
            .quest
            .envelope
            .generation_provenance
            .as_ref()
            .is_none_or(|p| p.dna != self.dna)
        {
            return Err(err(
                ValidationCode::InvalidEntity,
                "quest.provenance",
                "candidate provenance must match DNA",
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct QuestForge;
impl QuestForge {
    pub fn new_dna(
        seed: RootSeed,
        preset: &str,
        axes: CreativeAxes,
        constraints: forge_contracts::Constraints,
    ) -> ValidationResult<ForgeDNA> {
        let p = PresetId::new(preset)?;
        if !PRESETS.contains(&p.as_str()) {
            return Err(err(
                ValidationCode::UnsupportedPreset,
                "universe_preset",
                "unsupported core v1 Quest preset",
            ));
        }
        let mut r = GenerationRecipe::new(GenerationKind::Quest, seed, p);
        r.creative_axes = axes;
        r.constraints = constraints;
        for f in FIELDS {
            r.add_field(f.key()?)?;
        }
        for f in FIELDS.into_iter().skip(1) {
            r.set_dependencies(f.key()?, [QuestField::Frame.key()?])?;
        }
        let mut d = ForgeDNA::new(r);
        d.content_version = V2;
        d.normalize()?;
        Ok(d)
    }
    pub fn generate(dna: &ForgeDNA) -> ValidationResult<QuestCandidate> {
        let mut d = dna.clone();
        d.normalize()?;
        validate_dna(&d)?;
        let mut vals = FieldValues::new();
        for f in FIELDS {
            let k = f.key()?;
            let mut s = FieldStream::derive(&d, &k)?;
            let v = generate_value(f, &d, &vals, &mut s)?;
            vals.insert(k, v);
        }
        let project = ProjectId::from_bytes([7; 16]);
        let id = EntityId::from_bytes(
            blake3::hash(&d.canonical_bytes()?).as_bytes()[..16]
                .try_into()
                .unwrap(),
        );
        let mut q = Quest {
            envelope: EntityEnvelope {
                id,
                project_id: project,
                entity_type: EntityType::Quest,
                name: String::new(),
                schema_version: d.schema_version,
                canon_state: CanonState::Candidate,
                origin: EntityOrigin::Generated,
                favorite: false,
                created_at_unix_ms: 0,
                updated_at_unix_ms: 0,
                tags: vec![format!("preset:{}", d.recipe.universe_preset)],
                notes: None,
                generation_provenance: None,
            },
            quest_type: String::new(),
            premise: String::new(),
            patron: String::new(),
            objective: String::new(),
            stakes: String::new(),
            opposition: String::new(),
            complications: vec![],
            twist: String::new(),
            moral_pressure: String::new(),
            locations: vec![],
            rewards: vec![],
            failure_consequences: vec![],
            follow_up_hooks: vec![],
        };
        for f in FIELDS {
            set(&mut q, f, vals[&f.key()?].clone())?;
        }
        q.envelope.generation_provenance = Some(GenerationMetadata {
            generation_id: id,
            generated_at_unix_ms: 0,
            action: GenerationAction::Forge,
            parent_generation_id: None,
            dna: d.clone(),
        });
        let field_origins = FIELDS
            .into_iter()
            .map(|f| Ok((f.key()?, QuestOrigin::Generated)))
            .collect::<ValidationResult<_>>()?;
        let c = QuestCandidate {
            quest: q,
            dna: d,
            field_origins,
        };
        c.validate()?;
        Ok(c)
    }
    pub fn lock(c: &mut QuestCandidate, f: QuestField) -> ValidationResult<()> {
        lock_field(&mut c.dna, &f.key()?)?;
        sync_provenance(c)
    }
    pub fn unlock(c: &mut QuestCandidate, f: QuestField) -> ValidationResult<()> {
        unlock_field(&mut c.dna, &f.key()?)?;
        sync_provenance(c)
    }
    pub fn reroll_field(
        c: &mut QuestCandidate,
        f: QuestField,
    ) -> ValidationResult<Vec<QuestField>> {
        let mut n = c.clone();
        n.validate()?;
        let mut vals = to_values(&n.quest)?;
        let dna_snapshot = n.dna.clone();
        let changed = reroll_field(&mut n.dna, &mut vals, &f.key()?, |key, stream| {
            let f = FIELDS
                .into_iter()
                .find(|f| f.key().is_ok_and(|k| k == *key))
                .ok_or_else(|| err(ValidationCode::UnknownField, "field", "unknown Quest field"))?;
            generate_value(f, &dna_snapshot, &FieldValues::new(), stream)
        })?
        .changed_fields;
        for key in &changed {
            let f = FIELDS
                .into_iter()
                .find(|f| f.key().is_ok_and(|k| k == *key))
                .unwrap();
            set(&mut n.quest, f, vals[key].clone())?;
        }
        sync_provenance(&mut n)?;
        n.validate()?;
        *c = n;
        Ok(changed
            .into_iter()
            .filter_map(|k| {
                FIELDS
                    .into_iter()
                    .find(|f| f.key().ok().as_ref() == Some(&k))
            })
            .collect())
    }
    pub fn reroll_all(c: &mut QuestCandidate) -> ValidationResult<Vec<QuestField>> {
        let mut n = c.clone();
        n.validate()?;
        let mut vals = to_values(&n.quest)?;
        let dna_snapshot = n.dna.clone();
        let changed = reroll_all_unlocked(&mut n.dna, &mut vals, |key, stream| {
            let f = FIELDS
                .into_iter()
                .find(|f| f.key().is_ok_and(|k| k == *key))
                .unwrap();
            generate_value(f, &dna_snapshot, &FieldValues::new(), stream)
        })?
        .changed_fields;
        for key in &changed {
            let f = FIELDS
                .into_iter()
                .find(|f| f.key().is_ok_and(|k| k == *key))
                .unwrap();
            set(&mut n.quest, f, vals[key].clone())?;
        }
        sync_provenance(&mut n)?;
        n.validate()?;
        *c = n;
        Ok(changed
            .into_iter()
            .filter_map(|k| {
                FIELDS
                    .into_iter()
                    .find(|f| f.key().ok().as_ref() == Some(&k))
            })
            .collect())
    }
}
fn validate_dna(d: &ForgeDNA) -> ValidationResult<()> {
    d.validate()?;
    if d.recipe.kind != GenerationKind::Quest
        || !PRESETS.contains(&d.recipe.universe_preset.as_str())
    {
        return Err(err(
            ValidationCode::UnsupportedPreset,
            "universe_preset",
            "Quest Forge requires a supported Quest preset",
        ));
    }
    if d.content_version != V2 {
        return Err(err(
            ValidationCode::UnsupportedVersion,
            "content_version",
            "unsupported Quest content version",
        ));
    }
    let expected: FieldsSet = FIELDS
        .into_iter()
        .map(QuestField::key)
        .collect::<ValidationResult<_>>()?;
    if d.recipe
        .field_controls
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        != expected
    {
        return Err(err(
            ValidationCode::InvalidDependency,
            "field_controls",
            "Quest DNA field set differs from canonical v1",
        ));
    }
    Ok(())
}
type FieldsSet = BTreeSet<FieldKey>;
fn generate_value(
    f: QuestField,
    d: &ForgeDNA,
    _v: &FieldValues<QuestFieldValue>,
    s: &mut FieldStream,
) -> ValidationResult<QuestFieldValue> {
    let preset = d.recipe.universe_preset.as_str();
    let axis = |a| *d.recipe.creative_axes.levels.get(&a).unwrap_or(&50);
    let chaos = axis(CreativeAxis::Chaos);
    let bleak = axis(CreativeAxis::Bleak);
    let epic = axis(CreativeAxis::Epic);
    let (place, authority, crisis, threat, object, reward) = theme(preset);
    let frame = if f == QuestField::Frame {
        pick(s, &["civic record", "buried debt", "contested inheritance"])?
    } else {
        "civic record"
    };
    let n = (s.next_bounded(3)? as usize) + (chaos as usize / 34);
    let n = n % 3;
    let line=match f {
        QuestField::Frame=>frame.to_owned(),
        QuestField::Title=>["The Last Tally", "A Debt Paid in Salt", "Before the Bell Is Struck"][n].to_string(),
        QuestField::Type=>["recovery under deadline","public reckoning","covert rescue"][n].to_string(),
        QuestField::Premise=>format!("The {authority} hires the party when {crisis}; the proof lies at {place}. {}",["A ledger entry makes the dispute a matter of life and law.","The only witness is the person the record condemns.","The official account was changed after the first victims disappeared."][n]),
        QuestField::Patron=>format!("{} {authority}, who risks {} if the crisis becomes public",["A junior clerk in the","A dismissed inspector from the","The last elected officer of the"][n],["their office","their freedom","their family's claim to the district"][n]),
        QuestField::Objective=>[format!("Reach {place}, then recover the {object} before the next ruling by {authority}."),format!("Find a lawful route into {place}, then copy and authenticate the {object} before the next ruling by {authority}."),format!("Enter {place}, then carry the {object} to the public hearing before the next ruling by {authority}.")][n].clone(),
        QuestField::Stakes=>format!("If the party fails, {crisis}; {} lose {}.",["residents of the affected district","the witness and their dependents","families who still hold a legal claim"][n],["their last legal standing","access to the only evidence","protection before the next count"][n]),
        QuestField::Opposition=>format!("{threat} {} and can expose the patron's part in the crisis.",["controls the only safe route through {place}".replace("{place}",place),"has already moved the evidence out of {place}".replace("{place}",place),"will testify that the {object} is a forgery".replace("{object}",object)] [n]),
        QuestField::Complications=>return Ok(QuestFieldValue::Texts(vec![format!("{} at {place}.",["The route closes when the deadline begins","A second entrance is flooded and can only be opened from inside","The evidence is divided between the public archive and a private locker"][n]),format!("{}.",["The proof also implicates someone the patron promised to protect","The only witness demands immunity before speaking","The patron's signature appears on the order that caused the crisis"][n])])),
        QuestField::Twist=>format!("The {object} is genuine, but {}.",["it proves the patron helped cause the crisis; revealing it may save the many and condemn the few","the opposition is using it to keep the crisis contained, not to profit from it","it names the patron as the only person legally able to stop the coming order"][n]),
        QuestField::MoralPressure=>if bleak>50 { ["Publishing the cleanest proof ruins the witness whose testimony could prevent a wider disaster.","The patron offers a safe settlement that protects the living but leaves the dead officially erased.","Saving the district requires making one family surrender the claim they inherited."][n] } else { ["The witness will speak only if the patron is allowed to face the hearing beside them.","The patron offers a settlement that protects both the living and the witness, if the record is corrected in public.","The district can be saved without a conviction, but only if the party persuades its rivals to share the claim."][n] }.into(),
        QuestField::Locations=>return Ok(QuestFieldValue::Texts(vec![place.to_string(),format!("{}",if epic>50 { ["the mountain signal tower","the records balcony","the locked approach below the hearing room"][n] } else { ["the service passage","the neighboring block","the clerk's side office"][n] })])),
        QuestField::Rewards=>return Ok(QuestFieldValue::Texts(vec![format!("{reward}, held in trust until the displaced can claim it"),format!("{}",["A sealed record naming the next person at risk","A signed warrant that protects the witness","A map of the hidden route to the archive"][n])])),
        QuestField::Failure=>return Ok(QuestFieldValue::Texts(vec![format!("{}.",["The crisis becomes permanent at the next census","The witness is charged before the evidence can be heard","The district loses its legal status at the next ruling"][n]),format!("The patron takes the blame, while {threat} gains lawful control of {place}.")])) ,
        QuestField::FollowUp=>return Ok(QuestFieldValue::Texts(vec![format!("{}",["The sealed record names another district facing the same erasure","The witness sends a second copy to a rival authority","The patron's warrant bears a date from a case not yet opened"][n])]))
    };
    Ok(QuestFieldValue::Text(line))
}
fn theme(
    p: &str,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    match p {
        "pirate" => (
            "the tide-locked customs vault",
            "harbor court",
            "the port will erase a drowned crew from its rolls by tomorrow's tide",
            "a mutinous pilot holding the only safe chart",
            "brass tide-bell",
            "a share of the recovered salvage",
        ),
        "cyberpunk" => (
            "a sealed municipal memory clinic",
            "Civic Algorithm Office",
            "a corporation will overwrite 400 residents' identities at midnight",
            "the security daemon that owns the clinic's access keys",
            "original consent ledger",
            "an untraceable identity shard",
        ),
        "steampunk" => (
            "the pressure station beneath Foundry Row",
            "boiler tribunal",
            "the district loses heat when the regulator is condemned at dawn",
            "the inspector who forged the pressure readings",
            "cracked regulator core",
            "a patent on a safe pressure valve",
        ),
        "nordic-myth" => (
            "the boundary cairn above the winter road",
            "oath-speaker",
            "three villages lose their protection when the cairn is moved",
            "a sworn guardian who cannot break their vow",
            "first carved boundary stone",
            "safe passage through the winter oath-road",
        ),
        "post-apocalyptic" => (
            "the dry reservoir control house",
            "ration council",
            "the lower camp is cut off from water at the next ration count",
            "a convoy boss who controls the last working pump",
            "hand-copied aquifer map",
            "a season's clean water allocation",
        ),
        "occult" => (
            "the records room beneath Saint Orra's hospital",
            "night archivist",
            "patients vanish from official records before the dawn bell",
            "a dead clerk whose name was erased from every register",
            "carbon copy of the missing ledger",
            "the true names needed to call the patients home",
        ),
        "monster-hunter" => (
            "the abandoned molt-house at the north pass",
            "wardens' lodge",
            "the next migration will carry the brood into a market town",
            "a trophy hunter hiding the creature's trail",
            "shed scale still warm with its maker's mark",
            "a cure for the venom already in the wells",
        ),
        "rogue-assassin" => (
            "the counting room beneath the opera",
            "old network's handler",
            "a framed courier is executed when the ledger is opened at dawn",
            "the handler's own second-in-command",
            "duplicate contract bearing a false seal",
            "safe passage and the target's true name",
        ),
        "contemporary-weird" => (
            "the apartment building's basement mailroom",
            "tenants' association",
            "the city evicts an entire floor after its apartments disappear from the tax roll",
            "a municipal clerk whose software lists one extra unit",
            "stamped occupancy register",
            "a lawful address for every missing tenant",
        ),
        "chaos-blend" => (
            "the tide-powered signal archive",
            "port's machine-oracle",
            "the city will classify a living district as an abandoned myth at sunrise",
            "a union of chart smugglers and civic automata",
            "brass memory chart",
            "a claim ticket honored by both harbor law and machine code",
        ),
        _ => (
            "the ash court below the ruined keep",
            "remnant council",
            "the valley is condemned at the next bell",
            "a bailiff sworn to the wrong ruler",
            "the iron witness seal",
            "a charter restoring the valley's name",
        ),
    }
}
fn pick<'a>(s: &mut FieldStream, a: &'a [&'a str]) -> ValidationResult<&'a str> {
    Ok(a[s.next_bounded(a.len() as u64)? as usize])
}
fn set(q: &mut Quest, f: QuestField, x: QuestFieldValue) -> ValidationResult<()> {
    let t = |v| match v {
        QuestFieldValue::Text(x) => Ok(x),
        _ => Err(err(
            ValidationCode::InvalidEntity,
            "quest.field",
            "expected text",
        )),
    };
    let ts = |v| match v {
        QuestFieldValue::Texts(x) => Ok(x),
        _ => Err(err(
            ValidationCode::InvalidEntity,
            "quest.field",
            "expected text list",
        )),
    };
    match f {
        QuestField::Frame => {}
        QuestField::Title => q.envelope.name = t(x)?,
        QuestField::Type => q.quest_type = t(x)?,
        QuestField::Premise => q.premise = t(x)?,
        QuestField::Patron => q.patron = t(x)?,
        QuestField::Objective => q.objective = t(x)?,
        QuestField::Stakes => q.stakes = t(x)?,
        QuestField::Opposition => q.opposition = t(x)?,
        QuestField::Complications => q.complications = ts(x)?,
        QuestField::Twist => q.twist = t(x)?,
        QuestField::MoralPressure => q.moral_pressure = t(x)?,
        QuestField::Locations => q.locations = ts(x)?,
        QuestField::Rewards => q.rewards = ts(x)?,
        QuestField::Failure => q.failure_consequences = ts(x)?,
        QuestField::FollowUp => q.follow_up_hooks = ts(x)?,
    }
    Ok(())
}
fn to_values(q: &Quest) -> ValidationResult<FieldValues<QuestFieldValue>> {
    let mut m = FieldValues::new();
    for (f, v) in [
        (
            QuestField::Frame,
            QuestFieldValue::Text("civic record".into()),
        ),
        (
            QuestField::Title,
            QuestFieldValue::Text(q.envelope.name.clone()),
        ),
        (
            QuestField::Type,
            QuestFieldValue::Text(q.quest_type.clone()),
        ),
        (
            QuestField::Premise,
            QuestFieldValue::Text(q.premise.clone()),
        ),
        (QuestField::Patron, QuestFieldValue::Text(q.patron.clone())),
        (
            QuestField::Objective,
            QuestFieldValue::Text(q.objective.clone()),
        ),
        (QuestField::Stakes, QuestFieldValue::Text(q.stakes.clone())),
        (
            QuestField::Opposition,
            QuestFieldValue::Text(q.opposition.clone()),
        ),
        (
            QuestField::Complications,
            QuestFieldValue::Texts(q.complications.clone()),
        ),
        (QuestField::Twist, QuestFieldValue::Text(q.twist.clone())),
        (
            QuestField::MoralPressure,
            QuestFieldValue::Text(q.moral_pressure.clone()),
        ),
        (
            QuestField::Locations,
            QuestFieldValue::Texts(q.locations.clone()),
        ),
        (
            QuestField::Rewards,
            QuestFieldValue::Texts(q.rewards.clone()),
        ),
        (
            QuestField::Failure,
            QuestFieldValue::Texts(q.failure_consequences.clone()),
        ),
        (
            QuestField::FollowUp,
            QuestFieldValue::Texts(q.follow_up_hooks.clone()),
        ),
    ] {
        m.insert(f.key()?, v);
    }
    Ok(m)
}
fn sync_provenance(c: &mut QuestCandidate) -> ValidationResult<()> {
    if let Some(p) = &mut c.quest.envelope.generation_provenance {
        p.dna = c.dna.clone();
        p.action = GenerationAction::RerollField;
    }
    Ok(())
}
fn err(c: ValidationCode, p: &str, m: &str) -> forge_contracts::ValidationErrors {
    ValidationErrors::one(ValidationError::new(c, p, m))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic_generation_and_preset_support() {
        for p in PRESETS {
            let d = QuestForge::new_dna(
                RootSeed::new(44),
                p,
                CreativeAxes::neutral(),
                BTreeMap::new(),
            )
            .unwrap();
            let q = QuestForge::generate(&d).unwrap();
            assert_eq!(q, QuestForge::generate(&d).unwrap());
            q.validate().unwrap();
        }
    }
    #[test]
    fn quest_reroll_obeys_locks() {
        let d = QuestForge::new_dna(
            RootSeed::new(9),
            "pirate",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        let mut q = QuestForge::generate(&d).unwrap();
        QuestForge::lock(&mut q, QuestField::Title).unwrap();
        let name = q.quest.envelope.name.clone();
        QuestForge::reroll_all(&mut q).unwrap();
        assert_eq!(q.quest.envelope.name, name);
        q.validate().unwrap();
    }
    #[test]
    fn manual_edit_survives_unrelated_reroll_and_seed_changes_candidate() {
        let make = |seed| {
            QuestForge::new_dna(
                RootSeed::new(seed),
                "contemporary-weird",
                CreativeAxes::neutral(),
                BTreeMap::new(),
            )
            .unwrap()
        };
        let mut candidate = QuestForge::generate(&make(12)).unwrap();
        let edited = "The tenants' association, meeting in the laundry room".to_owned();
        candidate
            .manual_edit(QuestField::Patron, QuestFieldValue::Text(edited.clone()))
            .unwrap();
        QuestForge::reroll_field(&mut candidate, QuestField::Title).unwrap();
        assert_eq!(candidate.quest.patron, edited);
        assert_eq!(
            candidate.field_origins[&QuestField::Patron.key().unwrap()],
            QuestOrigin::UserModified
        );
        assert_ne!(
            signature(&candidate.quest),
            signature(&QuestForge::generate(&make(13)).unwrap().quest)
        );
    }
    #[test]
    fn unicode_constraints_and_extreme_axes_are_deterministic() {
        let mut axes = CreativeAxes::neutral();
        for axis in CreativeAxis::ALL {
            axes.levels.insert(axis, 100);
        }
        let mut constraints = forge_contracts::Constraints::new();
        constraints.insert(
            forge_contracts::ConstraintKey::new("inspiration").unwrap(),
            "星の港 — séance 🗝️".into(),
        );
        let dna =
            QuestForge::new_dna(RootSeed::new(u64::MAX), "occult", axes, constraints).unwrap();
        let before = dna.clone();
        assert_eq!(
            QuestForge::generate(&dna).unwrap(),
            QuestForge::generate(&dna).unwrap()
        );
        assert_eq!(dna, before);
    }
    fn signature(q: &Quest) -> String {
        serde_json::to_string(&(
            q.envelope.name.clone(),
            q.premise.clone(),
            q.objective.clone(),
            q.stakes.clone(),
            q.opposition.clone(),
            q.twist.clone(),
        ))
        .unwrap()
    }
}
