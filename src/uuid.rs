/// All OSCAL documents use a UUID [RFC4122](https://www.rfc-editor.org/rfc/rfc4122.html) to provide a stable and unique way to
/// refer to a given instance of an OSCAL document. UUIDs are generated when the
/// OSCAL document is created or revised.
///
/// While not strictly part of the metadata section of an OSCAL document, this
/// document identifier is part of the OSCAL document's core metadata.
///
/// OSCAL supports two types of UUIDs:
///
/// - [Version 4](https://www.rfc-editor.org/rfc/rfc4122.html#section-4.4): A randomly or pseudo-randomly generated UUID.
/// - [Version 5](https://www.rfc-editor.org/rfc/rfc4122.html#section-4.3): A name-based UUID based on SHA-1 hashing.
///
/// The OSCAL program recommends using a version 4 (random) UUID as the document
/// identifier, which is highly resistant to [collisions](https://en.wikipedia.org/wiki/Universally_unique_identifier#Collisions).
///
/// This implementation of OSCAL UUID uses the [uuid] crate
///
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use uuid::Uuid;

use crate::{Base, Error, Validate};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UUIDDatatype(String);

impl Base for UUIDDatatype {
    fn base_type() -> String {
        String::from("String")
    }

    fn ref_type() -> String {
        String::from("str")
    }
}

impl UUIDDatatype {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for UUIDDatatype {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for UUIDDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for UUIDDatatype {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl TryFrom<&str> for UUIDDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = Uuid::parse_str(value)?;
        Ok(Self(result.to_string()))
    }
}

impl Validate for UUIDDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        let _ = uuid::Uuid::parse_str(value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_uuid() {
        let input = "blah";
        assert!(UUIDDatatype::validate(input).is_err());

        let input = UUIDDatatype::new();
        assert!(UUIDDatatype::validate(&input.to_string()).is_ok());
    }
}
