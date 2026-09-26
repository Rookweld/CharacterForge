use crate::FieldStream;
use forge_contracts::{
    FieldKey, FieldNonce, ForgeDNA, ValidationCode, ValidationError, ValidationErrors,
    ValidationResult,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub type FieldValues<T> = BTreeMap<FieldKey, T>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RerollResult {
    /// Sorted keys whose nonce and value were updated.
    pub changed_fields: Vec<FieldKey>,
}

pub fn lock_field(dna: &mut ForgeDNA, field: &FieldKey) -> ValidationResult<()> {
    set_locked(dna, field, true)
}

pub fn unlock_field(dna: &mut ForgeDNA, field: &FieldKey) -> ValidationResult<()> {
    set_locked(dna, field, false)
}

fn set_locked(dna: &mut ForgeDNA, field: &FieldKey, locked: bool) -> ValidationResult<()> {
    dna.validate()?;
    let control = dna.recipe.field_controls.get_mut(field).ok_or_else(|| {
        field_error(
            ValidationCode::UnknownField,
            field,
            "field is not registered",
        )
    })?;
    control.locked = locked;
    Ok(())
}

/// Rerolls a field and only its explicitly declared, unlocked dependents.
/// All counters and values are committed together after every generator succeeds.
pub fn reroll_field<T, F>(
    dna: &mut ForgeDNA,
    values: &mut FieldValues<T>,
    field: &FieldKey,
    mut generate: F,
) -> ValidationResult<RerollResult>
where
    F: FnMut(&FieldKey, &mut FieldStream) -> ValidationResult<T>,
{
    dna.validate()?;
    validate_values(dna, values)?;
    let control = dna.recipe.field_controls.get(field).ok_or_else(|| {
        field_error(
            ValidationCode::UnknownField,
            field,
            "field is not registered",
        )
    })?;
    if control.locked {
        return Err(field_error(
            ValidationCode::FieldLocked,
            field,
            "locked fields cannot be rerolled",
        ));
    }

    let changed = dependent_closure(dna, field);
    reroll_planned(dna, values, changed, &mut generate)
}

/// Rerolls every unlocked field once. All nonces are advanced before any value is generated,
/// so declared dependencies see one coherent recipe snapshot.
pub fn reroll_all_unlocked<T, F>(
    dna: &mut ForgeDNA,
    values: &mut FieldValues<T>,
    mut generate: F,
) -> ValidationResult<RerollResult>
where
    F: FnMut(&FieldKey, &mut FieldStream) -> ValidationResult<T>,
{
    dna.validate()?;
    validate_values(dna, values)?;
    let changed = dna
        .recipe
        .field_controls
        .iter()
        .filter_map(|(field, control)| (!control.locked).then_some(field.clone()))
        .collect();
    reroll_planned(dna, values, changed, &mut generate)
}

fn reroll_planned<T, F>(
    dna: &mut ForgeDNA,
    values: &mut FieldValues<T>,
    changed: BTreeSet<FieldKey>,
    generate: &mut F,
) -> ValidationResult<RerollResult>
where
    F: FnMut(&FieldKey, &mut FieldStream) -> ValidationResult<T>,
{
    if changed.is_empty() {
        return Ok(RerollResult::default());
    }

    let mut planned_dna = dna.clone();
    for field in &changed {
        let control = planned_dna
            .recipe
            .field_controls
            .get_mut(field)
            .ok_or_else(|| {
                field_error(
                    ValidationCode::UnknownField,
                    field,
                    "planned field is not registered",
                )
            })?;
        control.nonce = FieldNonce(control.nonce.0.checked_add(1).ok_or_else(|| {
            field_error(
                ValidationCode::NonceOverflow,
                field,
                "field nonce is already at u64::MAX",
            )
        })?);
    }

    let mut replacements = BTreeMap::new();
    for field in &changed {
        let mut stream = FieldStream::derive(&planned_dna, field)?;
        let value = generate(field, &mut stream)?;
        replacements.insert(field.clone(), value);
    }

    *dna = planned_dna;
    for (field, value) in replacements {
        values.insert(field, value);
    }
    Ok(RerollResult {
        changed_fields: changed.into_iter().collect(),
    })
}

fn dependent_closure(dna: &ForgeDNA, field: &FieldKey) -> BTreeSet<FieldKey> {
    let mut changed = BTreeSet::from([field.clone()]);
    let mut queue = VecDeque::from([field.clone()]);
    while let Some(source) = queue.pop_front() {
        for (dependent, dependencies) in &dna.recipe.dependencies {
            if !dependencies.contains(&source) || changed.contains(dependent) {
                continue;
            }
            let is_locked = dna.recipe.field_controls[dependent].locked;
            if !is_locked {
                changed.insert(dependent.clone());
                queue.push_back(dependent.clone());
            }
        }
    }
    changed
}

fn validate_values<T>(dna: &ForgeDNA, values: &FieldValues<T>) -> ValidationResult<()> {
    let controls: BTreeSet<_> = dna.recipe.field_controls.keys().collect();
    let actual: BTreeSet<_> = values.keys().collect();
    if controls != actual {
        return Err(ValidationErrors::one(ValidationError::new(
            ValidationCode::MissingField,
            "field_values",
            "field values and recipe controls must contain the same registered fields",
        )));
    }
    Ok(())
}

fn field_error(code: ValidationCode, field: &FieldKey, message: &str) -> ValidationErrors {
    ValidationErrors::one(ValidationError::new(
        code,
        format!("field_controls.{field}"),
        message,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_contracts::{GenerationKind, GenerationRecipe, PresetId, RootSeed};

    fn key(name: &str) -> FieldKey {
        FieldKey::new(name).unwrap()
    }

    fn dna(seed: u64) -> ForgeDNA {
        let mut recipe = GenerationRecipe::new(
            GenerationKind::Character,
            RootSeed::new(seed),
            PresetId::new("core").unwrap(),
        );
        for field in ["alpha", "beta", "gamma", "delta"] {
            recipe.add_field(key(field)).unwrap();
        }
        ForgeDNA::new(recipe)
    }

    fn values() -> FieldValues<u64> {
        ["alpha", "beta", "gamma", "delta"]
            .into_iter()
            .enumerate()
            .map(|(index, field)| (key(field), index as u64 + 10))
            .collect()
    }

    fn generated(dna: &ForgeDNA) -> FieldValues<u64> {
        dna.recipe
            .field_controls
            .keys()
            .map(|field| {
                let mut stream = FieldStream::derive(dna, field).unwrap();
                (field.clone(), stream.next_u64())
            })
            .collect()
    }

    #[test]
    fn same_forge_dna_reproduces_the_same_field_stream() {
        let left = dna(0x8f3c_2b17_9d4e_6a01);
        let right = left.clone();
        let mut a = FieldStream::derive(&left, &key("alpha")).unwrap();
        let mut b = FieldStream::derive(&right, &key("alpha")).unwrap();
        for _ in 0..1024 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn same_complete_recipe_reproduces_complete_deterministic_state() {
        let first_dna = dna(123);
        let second_dna = first_dna.clone();
        assert_eq!(generated(&first_dna), generated(&second_dna));
    }

    #[test]
    fn changing_root_seed_changes_field_stream() {
        let left = dna(1);
        let right = dna(2);
        assert_ne!(
            FieldStream::derive(&left, &key("alpha"))
                .unwrap()
                .fingerprint(),
            FieldStream::derive(&right, &key("alpha"))
                .unwrap()
                .fingerprint(),
        );
    }

    #[test]
    fn field_nonce_affects_only_its_field_when_no_dependency_is_declared() {
        let base = dna(55);
        let mut changed = base.clone();
        changed
            .recipe
            .field_controls
            .get_mut(&key("beta"))
            .unwrap()
            .nonce = FieldNonce(1);
        assert_ne!(
            FieldStream::derive(&base, &key("beta"))
                .unwrap()
                .fingerprint(),
            FieldStream::derive(&changed, &key("beta"))
                .unwrap()
                .fingerprint(),
        );
        for field in ["alpha", "gamma", "delta"] {
            assert_eq!(
                FieldStream::derive(&base, &key(field))
                    .unwrap()
                    .fingerprint(),
                FieldStream::derive(&changed, &key(field))
                    .unwrap()
                    .fingerprint(),
            );
        }
    }

    #[test]
    fn property_matrix_unrelated_nonce_changes_never_change_other_fingerprints() {
        for seed in 0..64 {
            let base = dna(seed);
            for changed_field in ["alpha", "beta", "gamma", "delta"] {
                let mut modified = base.clone();
                let control = modified
                    .recipe
                    .field_controls
                    .get_mut(&key(changed_field))
                    .unwrap();
                control.nonce = FieldNonce(seed + 1);
                for observed_field in ["alpha", "beta", "gamma", "delta"] {
                    if observed_field != changed_field {
                        assert_eq!(
                            FieldStream::derive(&base, &key(observed_field))
                                .unwrap()
                                .fingerprint(),
                            FieldStream::derive(&modified, &key(observed_field))
                                .unwrap()
                                .fingerprint(),
                            "root seed {seed}, changed {changed_field}, observed {observed_field}",
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn canonical_seed_is_independent_of_map_insertion_order() {
        let mut first = dna(77);
        let mut second = dna(77);
        let mut axes_a = BTreeMap::new();
        let mut axes_b = BTreeMap::new();
        for (axis, value) in forge_contracts::CreativeAxis::ALL.into_iter().zip(0..8) {
            axes_a.insert(axis, value * 10);
        }
        for (axis, value) in forge_contracts::CreativeAxis::ALL
            .into_iter()
            .zip(0..8)
            .rev()
        {
            axes_b.insert(axis, value * 10);
        }
        first.recipe.creative_axes.levels = axes_a;
        second.recipe.creative_axes.levels = axes_b;
        for (key_name, value) in [("tone", "Mýth"), ("era", "older")] {
            first.recipe.constraints.insert(
                forge_contracts::ConstraintKey::new(key_name).unwrap(),
                value.into(),
            );
        }
        for (key_name, value) in [("era", "older"), ("tone", "Mýth")] {
            second.recipe.constraints.insert(
                forge_contracts::ConstraintKey::new(key_name).unwrap(),
                value.into(),
            );
        }
        assert_eq!(
            first.canonical_bytes().unwrap(),
            second.canonical_bytes().unwrap()
        );
        assert_eq!(
            FieldStream::derive(&first, &key("alpha"))
                .unwrap()
                .fingerprint(),
            FieldStream::derive(&second, &key("alpha"))
                .unwrap()
                .fingerprint(),
        );
    }

    #[test]
    fn unicode_constraint_inputs_are_deterministic_and_distinct() {
        let mut first = dna(88);
        first.recipe.constraints.insert(
            forge_contracts::ConstraintKey::new("region").unwrap(),
            "星の港".into(),
        );
        let second = first.clone();
        assert_eq!(
            FieldStream::derive(&first, &key("alpha"))
                .unwrap()
                .fingerprint(),
            FieldStream::derive(&second, &key("alpha"))
                .unwrap()
                .fingerprint(),
        );
        let mut different = first.clone();
        different.recipe.constraints.insert(
            forge_contracts::ConstraintKey::new("region").unwrap(),
            "星の森".into(),
        );
        assert_ne!(
            FieldStream::derive(&first, &key("alpha"))
                .unwrap()
                .fingerprint(),
            FieldStream::derive(&different, &key("alpha"))
                .unwrap()
                .fingerprint(),
        );
    }

    #[test]
    fn single_field_reroll_changes_only_target_without_declared_dependencies() {
        let mut dna = dna(99);
        let mut values = values();
        let prior_values = values.clone();
        let result = reroll_field(&mut dna, &mut values, &key("beta"), |_, stream| {
            Ok(stream.next_u64())
        })
        .unwrap();
        assert_eq!(result.changed_fields, vec![key("beta")]);
        assert_eq!(values[&key("alpha")], prior_values[&key("alpha")]);
        assert_eq!(values[&key("gamma")], prior_values[&key("gamma")]);
        assert_eq!(values[&key("delta")], prior_values[&key("delta")]);
        assert_eq!(
            values[&key("beta")],
            FieldStream::derive(&dna, &key("beta")).unwrap().next_u64()
        );
        assert_eq!(dna.recipe.field_controls[&key("beta")].nonce, FieldNonce(1));
        for field in ["alpha", "gamma", "delta"] {
            assert_eq!(dna.recipe.field_controls[&key(field)].nonce, FieldNonce(0));
        }
    }

    #[test]
    fn locked_field_is_unchanged_across_single_field_reroll() {
        let mut dna = dna(4);
        let mut values = values();
        lock_field(&mut dna, &key("beta")).unwrap();
        let before_dna = dna.clone();
        let before_values = values.clone();
        let error = reroll_field(&mut dna, &mut values, &key("beta"), |_, stream| {
            Ok(stream.next_u64())
        })
        .unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::FieldLocked);
        assert_eq!(dna, before_dna);
        assert_eq!(values, before_values);
    }

    #[test]
    fn reroll_all_unlocked_preserves_locked_values_and_nonces() {
        let mut dna = dna(8);
        let mut values = values();
        lock_field(&mut dna, &key("beta")).unwrap();
        let locked_value = values[&key("beta")];
        let result =
            reroll_all_unlocked(&mut dna, &mut values, |_, stream| Ok(stream.next_u64())).unwrap();
        assert_eq!(
            result.changed_fields,
            vec![key("alpha"), key("delta"), key("gamma")]
        );
        assert_eq!(values[&key("beta")], locked_value);
        assert_eq!(dna.recipe.field_controls[&key("beta")].nonce, FieldNonce(0));
        for field in ["alpha", "gamma", "delta"] {
            assert_eq!(dna.recipe.field_controls[&key(field)].nonce, FieldNonce(1));
        }
    }

    #[test]
    fn unlocking_restores_reroll_eligibility_without_changing_nonce() {
        let mut dna = dna(11);
        let mut values = values();
        lock_field(&mut dna, &key("alpha")).unwrap();
        unlock_field(&mut dna, &key("alpha")).unwrap();
        assert_eq!(
            dna.recipe.field_controls[&key("alpha")].nonce,
            FieldNonce(0)
        );
        let result = reroll_field(&mut dna, &mut values, &key("alpha"), |_, stream| {
            Ok(stream.next_u64())
        })
        .unwrap();
        assert_eq!(result.changed_fields, vec![key("alpha")]);
        assert_eq!(
            dna.recipe.field_controls[&key("alpha")].nonce,
            FieldNonce(1)
        );
    }

    #[test]
    fn explicit_dependency_invalidation_is_transitive_and_excludes_unrelated_fields() {
        let mut dna = dna(12);
        dna.recipe
            .set_dependencies(key("gamma"), [key("alpha")])
            .unwrap();
        dna.recipe
            .set_dependencies(key("delta"), [key("gamma")])
            .unwrap();
        let mut values = values();
        let result = reroll_field(&mut dna, &mut values, &key("alpha"), |_, stream| {
            Ok(stream.next_u64())
        })
        .unwrap();
        assert_eq!(
            result.changed_fields,
            vec![key("alpha"), key("delta"), key("gamma")]
        );
        assert_eq!(dna.recipe.field_controls[&key("beta")].nonce, FieldNonce(0));
        for field in ["alpha", "gamma", "delta"] {
            assert_eq!(dna.recipe.field_controls[&key(field)].nonce, FieldNonce(1));
        }
    }

    #[test]
    fn locked_dependent_and_its_descendants_are_not_invalidated() {
        let mut dna = dna(15);
        dna.recipe
            .set_dependencies(key("gamma"), [key("alpha")])
            .unwrap();
        dna.recipe
            .set_dependencies(key("delta"), [key("gamma")])
            .unwrap();
        lock_field(&mut dna, &key("gamma")).unwrap();
        let mut values = values();
        let before_gamma = values[&key("gamma")];
        let before_delta = values[&key("delta")];
        let result = reroll_field(&mut dna, &mut values, &key("alpha"), |_, stream| {
            Ok(stream.next_u64())
        })
        .unwrap();
        assert_eq!(result.changed_fields, vec![key("alpha")]);
        assert_eq!(values[&key("gamma")], before_gamma);
        assert_eq!(values[&key("delta")], before_delta);
        assert_eq!(
            dna.recipe.field_controls[&key("delta")].nonce,
            FieldNonce(0)
        );
    }

    #[test]
    fn failed_generation_is_atomic_for_values_and_dna() {
        let mut dna = dna(16);
        dna.recipe
            .set_dependencies(key("gamma"), [key("alpha")])
            .unwrap();
        let mut values = values();
        let before_dna = dna.clone();
        let before_values = values.clone();
        let error = reroll_field(&mut dna, &mut values, &key("alpha"), |field, stream| {
            if field == &key("gamma") {
                return Err(ValidationErrors::one(ValidationError::new(
                    ValidationCode::InvalidEntity,
                    "gamma",
                    "intentional test failure",
                )));
            }
            Ok(stream.next_u64())
        })
        .unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::InvalidEntity);
        assert_eq!(dna, before_dna);
        assert_eq!(values, before_values);
    }

    #[test]
    fn nonce_overflow_is_structured_and_atomic() {
        let mut dna = dna(17);
        dna.recipe
            .field_controls
            .get_mut(&key("alpha"))
            .unwrap()
            .nonce = FieldNonce(u64::MAX);
        let mut values = values();
        let before_dna = dna.clone();
        let before_values = values.clone();
        let error = reroll_field(&mut dna, &mut values, &key("alpha"), |_, stream| {
            Ok(stream.next_u64())
        })
        .unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::NonceOverflow);
        assert_eq!(dna, before_dna);
        assert_eq!(values, before_values);
    }

    #[test]
    fn all_locked_reroll_is_safe_and_does_not_call_generator() {
        let mut dna = dna(18);
        let mut values = values();
        for field in ["alpha", "beta", "gamma", "delta"] {
            lock_field(&mut dna, &key(field)).unwrap();
        }
        let before_dna = dna.clone();
        let before_values = values.clone();
        let result = reroll_all_unlocked(&mut dna, &mut values, |_, _| {
            panic!("no fields are eligible")
        })
        .unwrap();
        assert!(result.changed_fields.is_empty());
        assert_eq!(dna, before_dna);
        assert_eq!(values, before_values);
    }

    #[test]
    fn invalid_field_sets_and_sampling_ranges_return_errors_not_panics() {
        let mut dna = dna(19);
        let mut missing_value = values();
        missing_value.remove(&key("delta"));
        let error = reroll_all_unlocked(&mut dna, &mut missing_value, |_, stream| {
            Ok(stream.next_u64())
        })
        .unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::MissingField);
        let mut stream = FieldStream::derive(&dna, &key("alpha")).unwrap();
        assert_eq!(
            stream.next_bounded(0).unwrap_err().errors[0].code,
            ValidationCode::InvalidRange
        );
        for _ in 0..256 {
            assert!(stream.next_bounded(u64::MAX).unwrap() < u64::MAX);
        }
    }

    #[test]
    fn version_and_seed_extremes_produce_valid_streams() {
        let dna = dna(u64::MAX);
        let mut stream = FieldStream::derive(&dna, &key("alpha")).unwrap();
        let first = stream.next_u64();
        let second = stream.next_u64();
        assert_ne!(first, second);
    }
}
