//! App-owned, versioned relationship definitions and endpoint validation.

use forge_contracts::{
    EntityType, Relationship, RelationshipType, ValidationCode, ValidationError, ValidationErrors,
    ValidationResult,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RelationshipDefinition {
    pub key: &'static str,
    pub relationship_type: RelationshipType,
    pub source: EntityType,
    pub target: EntityType,
    pub directed: bool,
}

pub const RELATIONSHIP_DEFINITIONS: [RelationshipDefinition; 4] = [
    RelationshipDefinition {
        key: "born_in",
        relationship_type: RelationshipType::BornIn,
        source: EntityType::Character,
        target: EntityType::Location,
        directed: true,
    },
    RelationshipDefinition {
        key: "member_of",
        relationship_type: RelationshipType::MemberOf,
        source: EntityType::Character,
        target: EntityType::Faction,
        directed: true,
    },
    RelationshipDefinition {
        key: "owns",
        relationship_type: RelationshipType::Owns,
        source: EntityType::Character,
        target: EntityType::Item,
        directed: true,
    },
    RelationshipDefinition {
        key: "opposed_by",
        relationship_type: RelationshipType::OpposedBy,
        source: EntityType::Quest,
        target: EntityType::Creature,
        directed: true,
    },
];

pub fn relationship_definition(kind: RelationshipType) -> &'static RelationshipDefinition {
    RELATIONSHIP_DEFINITIONS
        .iter()
        .find(|definition| definition.relationship_type == kind)
        .expect("all relationship types have an app-owned definition")
}

pub fn validate_relationship_endpoints(
    relationship: &Relationship,
    source: EntityType,
    target: EntityType,
) -> ValidationResult<()> {
    relationship.validate()?;
    let definition = relationship_definition(relationship.relationship_type);
    if source != definition.source || target != definition.target {
        return Err(ValidationErrors::one(ValidationError::new(
            ValidationCode::InvalidEntity,
            "relationship.endpoints",
            format!(
                "{} requires {:?} → {:?}",
                definition.key, definition.source, definition.target
            ),
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_initial_definitions_are_stable_and_directed() {
        let actual: Vec<_> = RELATIONSHIP_DEFINITIONS
            .iter()
            .map(|d| (d.key, d.source, d.target))
            .collect();
        assert_eq!(
            actual,
            [
                ("born_in", EntityType::Character, EntityType::Location),
                ("member_of", EntityType::Character, EntityType::Faction),
                ("owns", EntityType::Character, EntityType::Item),
                ("opposed_by", EntityType::Quest, EntityType::Creature),
            ]
        );
        assert!(RELATIONSHIP_DEFINITIONS.iter().all(|d| d.directed));
    }

    #[test]
    fn endpoint_types_are_enforced() {
        let relationship = Relationship {
            id: forge_contracts::RelationshipId::from_bytes([1; 16]),
            project_id: forge_contracts::ProjectId::from_bytes([2; 16]),
            relationship_type: RelationshipType::BornIn,
            source_id: forge_contracts::EntityId::from_bytes([3; 16]),
            target_id: forge_contracts::EntityId::from_bytes([4; 16]),
            state: forge_contracts::RelationshipState::Candidate,
            origin: forge_contracts::EntityOrigin::Manual,
            created_at_unix_ms: 1,
            updated_at_unix_ms: 1,
        };
        assert!(
            validate_relationship_endpoints(
                &relationship,
                EntityType::Character,
                EntityType::Location
            )
            .is_ok()
        );
        assert!(
            validate_relationship_endpoints(&relationship, EntityType::Quest, EntityType::Location)
                .is_err()
        );
        for (index, definition) in RELATIONSHIP_DEFINITIONS.iter().enumerate() {
            let mut assertion = relationship.clone();
            assertion.id = forge_contracts::RelationshipId::from_bytes([index as u8 + 5; 16]);
            assertion.relationship_type = definition.relationship_type;
            assert!(
                validate_relationship_endpoints(&assertion, definition.source, definition.target)
                    .is_ok(),
                "{}",
                definition.key
            );
        }
    }
}
