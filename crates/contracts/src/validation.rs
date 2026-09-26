use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationCode {
    EmptyValue,
    InvalidIdentifier,
    InvalidSeed,
    InvalidRange,
    UnsupportedVersion,
    AxisOutOfRange,
    MissingField,
    UnknownField,
    FieldLocked,
    NonceOverflow,
    InvalidDependency,
    DependencyCycle,
    Serialization,
    InvalidEntity,
    InvalidTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationError {
    pub code: ValidationCode,
    pub path: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(code: ValidationCode, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            path: path.into(),
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>,
}

impl ValidationErrors {
    pub fn one(error: ValidationError) -> Self {
        Self {
            errors: vec![error],
        }
    }

    pub fn push(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ValidationError> {
        self.errors.iter()
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, error) in self.errors.iter().enumerate() {
            if index > 0 {
                f.write_str("; ")?;
            }
            write!(f, "{}: {}", error.path, error.message)?;
        }
        Ok(())
    }
}

impl Error for ValidationErrors {}

pub type ValidationResult<T> = Result<T, ValidationErrors>;
