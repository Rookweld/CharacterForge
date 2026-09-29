use crate::{ValidationCode, ValidationError, ValidationErrors, ValidationResult};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::{fmt, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Uuid128([u8; 16]);

impl Uuid128 {
    pub const NIL: Self = Self([0; 16]);

    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    pub const fn is_nil(&self) -> bool {
        let bytes = self.0;
        let mut index = 0;
        while index < bytes.len() {
            if bytes[index] != 0 {
                return false;
            }
            index += 1;
        }
        true
    }
}

impl fmt::Display for Uuid128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, byte) in self.0.iter().enumerate() {
            if matches!(index, 4 | 6 | 8 | 10) {
                f.write_str("-")?;
            }
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl FromStr for Uuid128 {
    type Err = ValidationErrors;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let invalid = || {
            ValidationErrors::one(ValidationError::new(
                ValidationCode::InvalidIdentifier,
                "uuid",
                "expected a canonical 36-character UUID",
            ))
        };
        if value.len() != 36 {
            return Err(invalid());
        }

        let mut bytes = [0_u8; 16];
        let mut byte_index = 0;
        let mut high_nibble = None;
        for (index, byte) in value.bytes().enumerate() {
            if matches!(index, 8 | 13 | 18 | 23) {
                if byte != b'-' || high_nibble.is_some() {
                    return Err(invalid());
                }
                continue;
            }
            let nibble = match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                b'A'..=b'F' => byte - b'A' + 10,
                _ => return Err(invalid()),
            };
            if let Some(high) = high_nibble.take() {
                if byte_index >= bytes.len() {
                    return Err(invalid());
                }
                bytes[byte_index] = (high << 4) | nibble;
                byte_index += 1;
            } else {
                high_nibble = Some(nibble);
            }
        }
        if byte_index != bytes.len() || high_nibble.is_some() {
            return Err(invalid());
        }
        Ok(Self(bytes))
    }
}

impl Serialize for Uuid128 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Uuid128 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::from_str(&value).map_err(de::Error::custom)
    }
}

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(pub Uuid128);

        impl $name {
            pub const fn from_bytes(bytes: [u8; 16]) -> Self {
                Self(Uuid128::from_bytes(bytes))
            }

            pub const fn uuid(&self) -> Uuid128 {
                self.0
            }

            pub fn validate(&self, path: &str) -> ValidationResult<()> {
                if self.0.is_nil() {
                    return Err(ValidationErrors::one(ValidationError::new(
                        ValidationCode::InvalidIdentifier,
                        path,
                        "nil UUID is not a valid identity",
                    )));
                }
                Ok(())
            }
        }
    };
}

id_type!(ProjectId);
id_type!(EntityId);
id_type!(RelationshipId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_serializes_as_canonical_lowercase_text() {
        let id = Uuid128::from_str("00112233-4455-6677-8899-AABBCCDDEEFF").unwrap();
        assert_eq!(id.to_string(), "00112233-4455-6677-8899-aabbccddeeff");
        let encoded = serde_json::to_string(&id).unwrap();
        assert_eq!(encoded, "\"00112233-4455-6677-8899-aabbccddeeff\"");
        assert_eq!(serde_json::from_str::<Uuid128>(&encoded).unwrap(), id);
    }

    #[test]
    fn invalid_uuid_returns_a_structured_error() {
        let error = Uuid128::from_str("bad").unwrap_err();
        assert_eq!(error.errors[0].code, ValidationCode::InvalidIdentifier);
    }
}
