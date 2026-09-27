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
const FIELDS_V3: [QuestField; 17] = [
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
    QuestField::Clue,
    QuestField::Briefing,
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
const V3: ContentVersion = ContentVersion(3);
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
    Clue,
    Briefing,
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
            Self::Clue => "16-clue",
            Self::Briefing => "17-briefing",
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
    #[serde(default)]
    pub clue: Option<String>,
    #[serde(default)]
    pub briefing: Option<String>,
}
impl QuestCandidate {
    /// Developer audit identifiers for selected v3 content; never serialized into Quest entities.
    #[doc(hidden)]
    pub fn audit_selection_ids(&self) -> ValidationResult<BTreeMap<String, String>> {
        if self.dna.content_version != V3 {
            return Ok(BTreeMap::new());
        }
        let preset = self.dna.recipe.universe_preset.as_str();
        let catalog = crate::quest_content::Catalog::get()?;
        let content = catalog.presets.get(preset).ok_or_else(|| {
            err(
                ValidationCode::UnsupportedPreset,
                "universe_preset",
                "missing Quest corpus preset",
            )
        })?;
        let thread_id = self
            .quest
            .envelope
            .tags
            .iter()
            .find_map(|tag| tag.strip_prefix("thread:"))
            .ok_or_else(|| {
                err(
                    ValidationCode::InvalidEntity,
                    "quest.thread",
                    "v3 audit requires a selected thread",
                )
            })?;
        let thread = content
            .threads
            .iter()
            .find(|thread| thread.id == thread_id)
            .ok_or_else(|| {
                err(
                    ValidationCode::InvalidEntity,
                    "quest.thread",
                    "selected thread is absent from the corpus",
                )
            })?;
        let mut selections = BTreeMap::new();
        selections.insert("theme/thread".into(), format!("thread/{}", thread.id));
        let mut add =
            |key: &str, selected: &str, primary_pool: &[String]| -> ValidationResult<()> {
                let (source_thread, pool) = content
                    .threads
                    .iter()
                    .find_map(|candidate| {
                        let pool = match key {
                            "title" => &candidate.titles,
                            "location" => &candidate.locations,
                            "quest_type" => &candidate.tags,
                            "premise" => &candidate.premises,
                            "quest_giver" => &candidate.givers,
                            "objective" => &candidate.objectives,
                            "stakes" => &candidate.stakes,
                            "threat" => &candidate.threats,
                            "complication" => &candidate.complications,
                            "twist" => &candidate.twists,
                            "clue" => &candidate.clues,
                            "reward" => &candidate.rewards,
                            "consequence" => &candidate.consequences,
                            "moral_pressure" => &candidate.moral_pressures,
                            "follow_up" => &candidate.followups,
                            _ => primary_pool,
                        };
                        pool.iter()
                            .any(|choice| choice == selected)
                            .then_some((candidate, pool))
                    })
                    .unwrap_or((thread, primary_pool));
                let index = pool
                    .iter()
                    .position(|choice| choice == selected)
                    .ok_or_else(|| {
                        err(
                            ValidationCode::InvalidEntity,
                            "quest.audit.selection",
                            "rendered selection is not present in its source pool",
                        )
                    })?;
                let fingerprint = blake3::hash(format!("{key}\0{selected}").as_bytes())
                    .to_hex()
                    .to_string();
                selections.insert(
                    key.into(),
                    format!("{key}/item-{}/option-{index}", &fingerprint[..12]),
                );
                if matches!(
                    key,
                    "title"
                        | "location"
                        | "complication"
                        | "twist"
                        | "reward"
                        | "moral_pressure"
                        | "follow_up"
                ) {
                    selections.insert(
                        format!("source/{key}"),
                        format!("thread/{}", source_thread.id),
                    );
                }
                Ok(())
            };
        add("title", &self.quest.envelope.name, &thread.titles)?;
        let type_tags = thread
            .tags
            .iter()
            .filter(|tag| tag.as_str() != "mix-compatible")
            .cloned()
            .collect::<Vec<_>>();
        add("quest_type", &self.quest.quest_type, &type_tags)?;
        add("premise", &self.quest.premise, &thread.premises)?;
        add(
            "location",
            self.quest
                .locations
                .first()
                .map(String::as_str)
                .unwrap_or_default(),
            &thread.locations,
        )?;
        add("quest_giver", &self.quest.patron, &thread.givers)?;
        add("objective", &self.quest.objective, &thread.objectives)?;
        add("stakes", &self.quest.stakes, &thread.stakes)?;
        add("threat", &self.quest.opposition, &thread.threats)?;
        add(
            "complication",
            self.quest
                .complications
                .first()
                .map(String::as_str)
                .unwrap_or_default(),
            &thread.complications,
        )?;
        add("twist", &self.quest.twist, &thread.twists)?;
        add(
            "clue",
            self.clue.as_deref().unwrap_or_default(),
            &thread.clues,
        )?;
        add(
            "reward",
            self.quest
                .rewards
                .first()
                .map(String::as_str)
                .unwrap_or_default(),
            &thread.rewards,
        )?;
        add(
            "consequence",
            self.quest
                .failure_consequences
                .first()
                .map(String::as_str)
                .unwrap_or_default(),
            &thread.consequences,
        )?;
        add(
            "moral_pressure",
            &self.quest.moral_pressure,
            &thread.moral_pressures,
        )?;
        add(
            "follow_up",
            self.quest
                .follow_up_hooks
                .first()
                .map(String::as_str)
                .unwrap_or_default(),
            &thread.followups,
        )?;
        let briefing_premise = self
            .briefing
            .as_deref()
            .unwrap_or_default()
            .split_once(": ")
            .map(|(_, premise)| premise)
            .unwrap_or_default();
        let giver_hash = blake3::hash(format!("briefing.giver\0{}", self.quest.patron).as_bytes())
            .to_hex()
            .to_string();
        let premise_hash = blake3::hash(format!("briefing.premise\0{briefing_premise}").as_bytes())
            .to_hex()
            .to_string();
        selections.insert(
            "briefing/composition".into(),
            format!(
                "briefing/giver-premise-v1/giver-{}/premise-{}",
                &giver_hash[..12],
                &premise_hash[..12]
            ),
        );
        Ok(selections)
    }
    pub fn manual_edit(
        &mut self,
        field: QuestField,
        value: QuestFieldValue,
    ) -> ValidationResult<()> {
        let mut updated = self.clone();
        apply_candidate_value(&mut updated, field, value)?;
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
        if self.dna.content_version == V3
            && (self.clue.as_ref().is_none_or(String::is_empty)
                || self.briefing.as_ref().is_none_or(String::is_empty))
        {
            return Err(err(
                ValidationCode::InvalidEntity,
                "quest.briefing",
                "v3 candidates require a clue and briefing",
            ));
        }
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
        validate_quest_constraints(&r.constraints)?;
        for f in FIELDS_V3 {
            r.add_field(f.key()?)?;
        }
        for f in FIELDS_V3.into_iter().skip(1) {
            if f == QuestField::Briefing {
                r.set_dependencies(
                    f.key()?,
                    [
                        QuestField::Frame.key()?,
                        QuestField::Patron.key()?,
                        QuestField::Premise.key()?,
                    ],
                )?;
            } else {
                r.set_dependencies(f.key()?, [QuestField::Frame.key()?])?;
            }
        }
        let mut d = ForgeDNA::new(r);
        d.content_version = V3;
        d.normalize()?;
        Ok(d)
    }
    pub fn generate(dna: &ForgeDNA) -> ValidationResult<QuestCandidate> {
        let mut d = dna.clone();
        d.normalize()?;
        validate_dna(&d)?;
        if d.content_version == V3 {
            return generate_v3(&d);
        }
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
            clue: None,
            briefing: None,
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
        let mut vals = to_values(&n)?;
        let dna_snapshot = n.dna.clone();
        let mut staged = vals.clone();
        let changed = reroll_field(&mut n.dna, &mut vals, &f.key()?, |key, stream| {
            let f = fields_for(&dna_snapshot)
                .iter()
                .copied()
                .into_iter()
                .find(|f| f.key().is_ok_and(|k| k == *key))
                .ok_or_else(|| err(ValidationCode::UnknownField, "field", "unknown Quest field"))?;
            let value = generate_versioned_value(f, &dna_snapshot, &staged, stream)?;
            staged.insert(key.clone(), value.clone());
            Ok(value)
        })?
        .changed_fields;
        for key in &changed {
            let f = fields_for(&n.dna)
                .iter()
                .copied()
                .into_iter()
                .find(|f| f.key().is_ok_and(|k| k == *key))
                .unwrap();
            apply_candidate_value(&mut n, f, vals[key].clone())?;
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
        let mut vals = to_values(&n)?;
        let dna_snapshot = n.dna.clone();
        let original_locks = n
            .dna
            .recipe
            .field_controls
            .iter()
            .map(|(key, control)| (key.clone(), control.locked))
            .collect::<BTreeMap<_, _>>();
        for (key, origin) in &n.field_origins {
            if *origin == QuestOrigin::UserModified
                && let Some(control) = n.dna.recipe.field_controls.get_mut(key)
            {
                control.locked = true;
            }
        }
        let mut staged = vals.clone();
        let changed = reroll_all_unlocked(&mut n.dna, &mut vals, |key, stream| {
            let f = fields_for(&dna_snapshot)
                .iter()
                .copied()
                .into_iter()
                .find(|f| f.key().is_ok_and(|k| k == *key))
                .unwrap();
            let value = generate_versioned_value(f, &dna_snapshot, &staged, stream)?;
            staged.insert(key.clone(), value.clone());
            Ok(value)
        })?
        .changed_fields;
        for (key, locked) in original_locks {
            n.dna.recipe.field_controls.get_mut(&key).unwrap().locked = locked;
        }
        for key in &changed {
            let f = fields_for(&n.dna)
                .iter()
                .copied()
                .into_iter()
                .find(|f| f.key().is_ok_and(|k| k == *key))
                .unwrap();
            apply_candidate_value(&mut n, f, vals[key].clone())?;
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
    if d.content_version != V2 && d.content_version != V3 {
        return Err(err(
            ValidationCode::UnsupportedVersion,
            "content_version",
            "unsupported Quest content version",
        ));
    }
    if d.content_version == V3 {
        validate_quest_constraints(&d.recipe.constraints)?;
    }
    let expected: FieldsSet = fields_for(d)
        .iter()
        .copied()
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
fn validate_quest_constraints(constraints: &forge_contracts::Constraints) -> ValidationResult<()> {
    for (key, value) in constraints {
        if key.as_str() != "inspiration" {
            return Err(err(
                ValidationCode::InvalidConstraint,
                &format!("constraints.{key}"),
                "unsupported Quest constraint; Quest Forge supports inspiration only; Character-only constraints do not apply to Quest generation",
            ));
        }
        if value.trim().is_empty() || value.chars().count() > 200 {
            return Err(err(
                ValidationCode::InvalidConstraint,
                "constraints.inspiration",
                "inspiration must contain 1 to 200 characters",
            ));
        }
    }
    Ok(())
}
fn fields_for(d: &ForgeDNA) -> &'static [QuestField] {
    if d.content_version == V3 {
        &FIELDS_V3
    } else {
        &FIELDS
    }
}
fn generate_versioned_value(
    f: QuestField,
    d: &ForgeDNA,
    values: &FieldValues<QuestFieldValue>,
    s: &mut FieldStream,
) -> ValidationResult<QuestFieldValue> {
    if d.content_version == V2 {
        return generate_value(f, d, values, s);
    }
    let catalog = crate::quest_content::Catalog::get()?;
    let preset = &catalog.presets[d.recipe.universe_preset.as_str()];
    let thread_id = if f == QuestField::Frame {
        select_thread(d, preset, s)?.id.clone()
    } else {
        values
            .get(&QuestField::Frame.key()?)
            .and_then(|v| text_value(v).ok())
            .unwrap_or_else(|| {
                select_thread(d, preset, s)
                    .map_or_else(|_| preset.threads[0].id.clone(), |t| t.id.clone())
            })
    };
    let primary_thread = preset
        .threads
        .iter()
        .find(|t| t.id == thread_id)
        .unwrap_or(&preset.threads[0]);
    if f == QuestField::Frame {
        return Ok(QuestFieldValue::Text(primary_thread.id.clone()));
    }
    let secondary_role = matches!(
        f,
        QuestField::Title
            | QuestField::Locations
            | QuestField::Complications
            | QuestField::Twist
            | QuestField::MoralPressure
            | QuestField::Rewards
            | QuestField::FollowUp
    );
    let mut source = primary_thread;
    if preset.id == "chaos-blend" && secondary_role {
        let chaos = *d
            .recipe
            .creative_axes
            .levels
            .get(&CreativeAxis::Chaos)
            .unwrap_or(&50) as u64;
        let compatible = preset
            .threads
            .iter()
            .filter(|t| {
                t.id != primary_thread.id && t.tags.iter().any(|tag| tag == "mix-compatible")
            })
            .collect::<Vec<_>>();
        if !compatible.is_empty() && s.next_bounded(100)? < chaos {
            source = compatible[s.next_bounded(compatible.len() as u64)? as usize];
        }
    }
    let thread = source;
    let type_tags = thread
        .tags
        .iter()
        .filter(|tag| tag.as_str() != "mix-compatible")
        .cloned()
        .collect::<Vec<_>>();
    let pool = match f {
        QuestField::Title => &thread.titles,
        QuestField::Type => &type_tags,
        QuestField::Premise => &thread.premises,
        QuestField::Patron => &thread.givers,
        QuestField::Objective => &thread.objectives,
        QuestField::Stakes => &thread.stakes,
        QuestField::Opposition => &thread.threats,
        QuestField::Complications => &thread.complications,
        QuestField::Twist => &thread.twists,
        QuestField::MoralPressure => &thread.moral_pressures,
        QuestField::Locations => &thread.locations,
        QuestField::Rewards => &thread.rewards,
        QuestField::Failure => &thread.consequences,
        QuestField::FollowUp => &thread.followups,
        QuestField::Clue => &thread.clues,
        QuestField::Briefing => &thread.premises,
        QuestField::Frame => unreachable!(),
    };
    let index = s.next_bounded(pool.len() as u64)? as usize;
    let value = if f == QuestField::Briefing {
        let giver = values
            .get(&QuestField::Patron.key()?)
            .and_then(|v| text_value(v).ok())
            .unwrap_or_else(|| thread.givers[0].clone());
        let premise = values
            .get(&QuestField::Premise.key()?)
            .and_then(|v| text_value(v).ok())
            .unwrap_or_else(|| pool[index].clone());
        format!("{giver}: {premise}")
    } else {
        pool[index].clone()
    };
    if f == QuestField::Briefing
        && let Some(inspiration) = d
            .recipe
            .constraints
            .get(&forge_contracts::ConstraintKey::new("inspiration")?)
    {
        return Ok(QuestFieldValue::Text(format!(
            "{value}. Inspiration: {inspiration}"
        )));
    }
    if matches!(
        f,
        QuestField::Complications
            | QuestField::Locations
            | QuestField::Rewards
            | QuestField::Failure
            | QuestField::FollowUp
    ) {
        Ok(QuestFieldValue::Texts(vec![value]))
    } else {
        Ok(QuestFieldValue::Text(value))
    }
}
fn select_thread<'a>(
    d: &ForgeDNA,
    preset: &'a crate::quest_content::Preset,
    stream: &mut FieldStream,
) -> ValidationResult<&'a crate::quest_content::Thread> {
    let axes = &d.recipe.creative_axes.levels;
    let level = |a| *axes.get(&a).unwrap_or(&50) as u64;
    let weights = preset
        .threads
        .iter()
        .map(|t| {
            let tags = t.tags.iter().map(String::as_str).collect::<BTreeSet<_>>();
            let mut w = 100
                + if tags
                    .iter()
                    .any(|x| ["scarcity", "curse", "survival", "debt", "loss"].contains(x))
                {
                    level(CreativeAxis::Bleak) * 2
                } else {
                    0
                };
            w += if tags
                .iter()
                .any(|x| ["mythic", "oath", "undead", "curse"].contains(x))
            {
                level(CreativeAxis::Mythic) * 2
            } else {
                0
            };
            w += if tags
                .iter()
                .any(|x| ["memory", "machine", "tide"].contains(x))
            {
                level(CreativeAxis::Chaos) * 2
            } else {
                0
            };
            w += if tags
                .iter()
                .any(|x| ["quest", "migration", "convoy"].contains(x))
            {
                level(CreativeAxis::Epic) * 2
            } else {
                0
            };
            w += if tags
                .iter()
                .any(|x| ["machine", "memory", "signal", "technology"].contains(x))
            {
                level(CreativeAxis::Technology) * 2
            } else {
                0
            };
            w += if tags
                .iter()
                .any(|x| ["mythic", "oath", "curse", "undead", "occult"].contains(x))
            {
                level(CreativeAxis::Mysticism) * 2
            } else {
                0
            };
            w += if tags
                .iter()
                .any(|x| ["memory", "myth", "curse", "surreal", "identity"].contains(x))
            {
                level(CreativeAxis::Surreal) * 2
            } else {
                0
            };
            w += if tags
                .iter()
                .any(|x| ["inheritance", "record", "identity", "debt", "myth"].contains(x))
            {
                level(CreativeAxis::Ambiguity) * 2
            } else {
                0
            };
            for (axis, tag) in [
                (CreativeAxis::Bleak, "bleak"),
                (CreativeAxis::Mythic, "mythic"),
                (CreativeAxis::Chaos, "chaos"),
                (CreativeAxis::Epic, "epic"),
                (CreativeAxis::Technology, "technology"),
                (CreativeAxis::Mysticism, "mysticism"),
                (CreativeAxis::Surreal, "surreal"),
                (CreativeAxis::Ambiguity, "ambiguity"),
            ] {
                if tags.contains(tag) {
                    w += level(axis) * if axis == CreativeAxis::Mythic { 40 } else { 8 };
                }
            }
            w
        })
        .collect::<Vec<_>>();
    let total = weights.iter().sum();
    let mut draw = stream.next_bounded(total)?;
    Ok(preset
        .threads
        .iter()
        .zip(weights)
        .find_map(|(t, w)| {
            if draw < w {
                Some(t)
            } else {
                draw -= w;
                None
            }
        })
        .unwrap_or(&preset.threads[0]))
}
fn generate_v3(d: &ForgeDNA) -> ValidationResult<QuestCandidate> {
    use crate::quest_content::Catalog;
    let catalog = Catalog::get()?;
    let preset = catalog
        .presets
        .get(d.recipe.universe_preset.as_str())
        .ok_or_else(|| {
            err(
                ValidationCode::UnsupportedPreset,
                "universe_preset",
                "missing Quest corpus preset",
            )
        })?;
    let frame_key = QuestField::Frame.key()?;
    let mut frame_stream = FieldStream::derive(d, &frame_key)?;
    let thread = select_thread(d, preset, &mut frame_stream)?;
    let mut vals = FieldValues::new();
    for f in FIELDS_V3 {
        let key = f.key()?;
        let mut stream = FieldStream::derive(d, &key)?;
        let value = if f == QuestField::Frame {
            QuestFieldValue::Text(thread.id.clone())
        } else {
            generate_versioned_value(f, d, &vals, &mut stream)?
        };
        vals.insert(key, value);
    }
    let id = EntityId::from_bytes(
        blake3::hash(&d.canonical_bytes()?).as_bytes()[..16]
            .try_into()
            .unwrap(),
    );
    let mut quest = Quest {
        envelope: EntityEnvelope {
            id,
            project_id: ProjectId::from_bytes([7; 16]),
            entity_type: EntityType::Quest,
            name: String::new(),
            schema_version: d.schema_version,
            canon_state: CanonState::Candidate,
            origin: EntityOrigin::Generated,
            favorite: false,
            created_at_unix_ms: 0,
            updated_at_unix_ms: 0,
            tags: vec![
                format!("preset:{}", d.recipe.universe_preset),
                format!("thread:{}", thread.id),
            ],
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
    let mut clue = String::new();
    let mut briefing = String::new();
    for f in FIELDS_V3 {
        let value = vals[&f.key()?].clone();
        match f {
            QuestField::Clue => clue = text_value(&value)?,
            QuestField::Briefing => briefing = text_value(&value)?,
            _ => {}
        }
        set(&mut quest, f, value)?;
    }
    quest.envelope.generation_provenance = Some(GenerationMetadata {
        generation_id: id,
        generated_at_unix_ms: 0,
        action: GenerationAction::Forge,
        parent_generation_id: None,
        dna: d.clone(),
    });
    let field_origins = FIELDS_V3
        .into_iter()
        .map(|f| Ok((f.key()?, QuestOrigin::Generated)))
        .collect::<ValidationResult<_>>()?;
    let candidate = QuestCandidate {
        quest,
        dna: d.clone(),
        field_origins,
        clue: Some(clue),
        briefing: Some(briefing),
    };
    candidate.validate()?;
    Ok(candidate)
}
fn text_value(value: &QuestFieldValue) -> ValidationResult<String> {
    match value {
        QuestFieldValue::Text(s) => Ok(s.clone()),
        _ => Err(err(
            ValidationCode::InvalidEntity,
            "quest.field",
            "expected text",
        )),
    }
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
        QuestField::FollowUp=>return Ok(QuestFieldValue::Texts(vec![format!("{}",["The sealed record names another district facing the same erasure","The witness sends a second copy to a rival authority","The patron's warrant bears a date from a case not yet opened"][n])])),
        QuestField::Clue | QuestField::Briefing => return Err(err(ValidationCode::UnsupportedVersion, "field", "v3-only Quest fields are unavailable in v2")),
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
        QuestField::Clue | QuestField::Briefing => {}
    }
    Ok(())
}
fn to_values(c: &QuestCandidate) -> ValidationResult<FieldValues<QuestFieldValue>> {
    let q = &c.quest;
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
    if c.dna.content_version == V3 {
        let thread = q
            .envelope
            .tags
            .iter()
            .find_map(|tag| tag.strip_prefix("thread:"))
            .unwrap_or("claim-of-the-dead")
            .to_string();
        m.insert(QuestField::Frame.key()?, QuestFieldValue::Text(thread));
        m.insert(
            QuestField::Clue.key()?,
            QuestFieldValue::Text(c.clue.clone().unwrap_or_default()),
        );
        m.insert(
            QuestField::Briefing.key()?,
            QuestFieldValue::Text(c.briefing.clone().unwrap_or_default()),
        );
    }
    Ok(m)
}
fn apply_candidate_value(
    c: &mut QuestCandidate,
    f: QuestField,
    value: QuestFieldValue,
) -> ValidationResult<()> {
    match f {
        QuestField::Clue => c.clue = Some(text_value(&value)?),
        QuestField::Briefing => c.briefing = Some(text_value(&value)?),
        QuestField::Frame => {
            let id = text_value(&value)?;
            c.quest
                .envelope
                .tags
                .retain(|tag| !tag.starts_with("thread:"));
            c.quest.envelope.tags.push(format!("thread:{id}"));
        }
        _ => set(&mut c.quest, f, value)?,
    }
    Ok(())
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
    fn version_three_recipe_generates_declarative_quest_content() {
        let mut dna = QuestForge::new_dna(
            RootSeed::new(44),
            "pirate",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        dna.content_version = ContentVersion(3);
        let candidate = QuestForge::generate(&dna).unwrap();
        assert!(candidate.clue.as_deref().is_some_and(|s| !s.is_empty()));
        assert!(candidate.briefing.as_deref().is_some_and(|s| !s.is_empty()));
        assert_eq!(
            candidate,
            serde_json::from_str(&serde_json::to_string(&candidate).unwrap()).unwrap()
        );
    }
    #[test]
    fn quest_rejects_character_only_and_unknown_constraints() {
        for key in ["no_magic", "must_be_non_human", "unknown_quest_rule"] {
            let mut constraints = BTreeMap::new();
            constraints.insert(
                forge_contracts::ConstraintKey::new(key).unwrap(),
                "true".into(),
            );
            assert_eq!(
                QuestForge::new_dna(
                    RootSeed::new(1),
                    "occult",
                    CreativeAxes::neutral(),
                    constraints
                )
                .unwrap_err()
                .errors[0]
                    .code,
                ValidationCode::InvalidConstraint
            );
        }
    }
    #[test]
    fn supported_constraint_matrix_is_consistent_across_every_preset() {
        for preset in PRESETS {
            let mut constraints = BTreeMap::new();
            constraints.insert(
                forge_contracts::ConstraintKey::new("inspiration").unwrap(),
                "A lantern under the harbor — 星".into(),
            );
            let dna = QuestForge::new_dna(
                RootSeed::new(1703),
                preset,
                CreativeAxes::neutral(),
                constraints,
            )
            .unwrap();
            let candidate = QuestForge::generate(&dna).unwrap();
            assert!(
                candidate
                    .briefing
                    .unwrap()
                    .contains("A lantern under the harbor — 星")
            );
            for invalid in [" ".to_owned(), "x".repeat(201)] {
                let mut invalid_constraints = BTreeMap::new();
                invalid_constraints.insert(
                    forge_contracts::ConstraintKey::new("inspiration").unwrap(),
                    invalid,
                );
                let error = QuestForge::new_dna(
                    RootSeed::new(1703),
                    preset,
                    CreativeAxes::neutral(),
                    invalid_constraints,
                )
                .unwrap_err();
                assert_eq!(error.errors[0].code, ValidationCode::InvalidConstraint);
            }
        }
    }
    #[test]
    fn chaos_axis_increases_secondary_role_source_mixing() {
        let mut low = CreativeAxes::neutral();
        let mut high = CreativeAxes::neutral();
        low.levels.insert(CreativeAxis::Chaos, 0);
        high.levels.insert(CreativeAxis::Chaos, 100);
        let mut low_cross = 0;
        let mut high_cross = 0;
        for seed in 0..256 {
            let a = QuestForge::generate(
                &QuestForge::new_dna(
                    RootSeed::new(seed),
                    "chaos-blend",
                    low.clone(),
                    BTreeMap::new(),
                )
                .unwrap(),
            )
            .unwrap();
            let b = QuestForge::generate(
                &QuestForge::new_dna(
                    RootSeed::new(seed),
                    "chaos-blend",
                    high.clone(),
                    BTreeMap::new(),
                )
                .unwrap(),
            )
            .unwrap();
            let ai = a.audit_selection_ids().unwrap();
            let bi = b.audit_selection_ids().unwrap();
            low_cross += usize::from(ai["source/location"] != ai["theme/thread"]);
            high_cross += usize::from(bi["source/location"] != bi["theme/thread"]);
        }
        assert!(
            high_cross >= 240,
            "Chaos high should mix source themes in almost every paired seed: {low_cross} -> {high_cross}"
        );
        assert!(
            high_cross > low_cross + 180,
            "Chaos must materially shift source mixing: {low_cross} -> {high_cross}"
        );
    }
    #[test]
    fn mythic_axis_shifts_the_selected_quest_story_family() {
        let mut low = CreativeAxes::neutral();
        let mut high = CreativeAxes::neutral();
        low.levels.insert(CreativeAxis::Mythic, 0);
        high.levels.insert(CreativeAxis::Mythic, 100);
        let count = |axes: CreativeAxes| {
            (0..256)
                .filter(|seed| {
                    let candidate = QuestForge::generate(
                        &QuestForge::new_dna(
                            RootSeed::new(*seed),
                            "chaos-blend",
                            axes.clone(),
                            BTreeMap::new(),
                        )
                        .unwrap(),
                    )
                    .unwrap();
                    candidate.audit_selection_ids().unwrap()["theme/thread"]
                        == "thread/mythic-salvage"
                })
                .count()
        };
        let low_count = count(low);
        let high_count = count(high);
        assert!(
            high_count >= low_count + 80,
            "Mythic should favor the tagged mythic family: {low_count} -> {high_count}"
        );
    }
    #[test]
    fn inspiration_constraint_is_visible_in_briefing_and_unicode_roundtrips() {
        let mut constraints = BTreeMap::new();
        constraints.insert(
            forge_contracts::ConstraintKey::new("inspiration").unwrap(),
            "雪の港 — séance 🗝️".into(),
        );
        let candidate = QuestForge::generate(
            &QuestForge::new_dna(
                RootSeed::new(5),
                "occult",
                CreativeAxes::neutral(),
                constraints,
            )
            .unwrap(),
        )
        .unwrap();
        let serialized = serde_json::to_string(&candidate).unwrap();
        assert!(serialized.contains("séance"));
        assert!(serialized.contains("—"));
        assert_eq!(
            serde_json::from_str::<QuestCandidate>(&serialized).unwrap(),
            candidate
        );
        let briefing = candidate.briefing.unwrap();
        assert!(briefing.contains(". Inspiration: 雪の港 — séance 🗝️"));
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
    fn frame_reroll_preserves_locked_descendants_and_clue_values() {
        let dna = QuestForge::new_dna(
            RootSeed::new(919),
            "pirate",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        let mut candidate = QuestForge::generate(&dna).unwrap();
        QuestForge::lock(&mut candidate, QuestField::Title).unwrap();
        let title = candidate.quest.envelope.name.clone();
        let changed = QuestForge::reroll_field(&mut candidate, QuestField::Frame).unwrap();
        assert!(!changed.contains(&QuestField::Title));
        assert_eq!(candidate.quest.envelope.name, title);
        assert!(candidate.clue.as_ref().is_some_and(|s| !s.is_empty()));
        candidate.validate().unwrap();
    }
    #[test]
    fn briefing_recomposes_after_patron_and_premise_rerolls() {
        let dna = QuestForge::new_dna(
            RootSeed::new(921),
            "chaos-blend",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        let mut candidate = QuestForge::generate(&dna).unwrap();
        QuestForge::reroll_field(&mut candidate, QuestField::Patron).unwrap();
        assert!(
            candidate
                .briefing
                .as_ref()
                .unwrap()
                .starts_with(&candidate.quest.patron)
        );
        assert!(
            candidate
                .briefing
                .as_ref()
                .unwrap()
                .contains(&candidate.quest.premise)
        );
        QuestForge::reroll_field(&mut candidate, QuestField::Premise).unwrap();
        assert!(
            candidate
                .briefing
                .as_ref()
                .unwrap()
                .starts_with(&candidate.quest.patron)
        );
        assert!(
            candidate
                .briefing
                .as_ref()
                .unwrap()
                .contains(&candidate.quest.premise)
        );
        candidate.validate().unwrap();
    }
    #[test]
    fn reroll_all_preserves_manual_edits_and_original_lock_flags() {
        let dna = QuestForge::new_dna(
            RootSeed::new(920),
            "pirate",
            CreativeAxes::neutral(),
            BTreeMap::new(),
        )
        .unwrap();
        let mut candidate = QuestForge::generate(&dna).unwrap();
        let edited = "My own title — 銀の港".to_owned();
        candidate
            .manual_edit(QuestField::Title, QuestFieldValue::Text(edited.clone()))
            .unwrap();
        let was_locked =
            candidate.dna.recipe.field_controls[&QuestField::Title.key().unwrap()].locked;
        QuestForge::reroll_all(&mut candidate).unwrap();
        assert_eq!(candidate.quest.envelope.name, edited);
        assert_eq!(
            candidate.dna.recipe.field_controls[&QuestField::Title.key().unwrap()].locked,
            was_locked
        );
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
