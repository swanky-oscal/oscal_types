use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Base, Error, Metaschema, Validate};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BooleanDatatype(bool);

impl Metaschema for BooleanDatatype {
    fn description() -> Option<&'static str> {
        Some("A binary value that is either: true or false.")
    }
    fn _type() -> Option<&'static str> {
        Some("boolean")
    }
}

impl Base for BooleanDatatype {
    fn base_type() -> String {
        String::from("bool")
    }
    fn ref_type() -> String {
        String::from("bool")
    }
}

impl Deref for BooleanDatatype {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// No validation
impl From<bool> for BooleanDatatype {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl TryFrom<&bool> for BooleanDatatype {
    type Error = Error;
    fn try_from(value: &bool) -> Result<Self, Self::Error> {
        Ok(Self(*value))
    }
}

impl Validate for BooleanDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        let _ = serde_json::from_str::<bool>(value).map_err(|_| Error::BooleanParse)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_se_boolean_datatype() {
        let test_value = "true";

        let result =
            serde_json::from_str::<BooleanDatatype>(test_value).expect("Failed to deserialize");
        assert_eq!(result.to_string(), test_value);

        let json = serde_json::to_string(&result).expect("failed to serialize");
        assert_eq!(json, test_value);
    }
    #[test]
    fn test_validate_json() {
        assert!(BooleanDatatype::validate("true").is_ok());
        assert!(BooleanDatatype::validate("false").is_ok());
        assert!(BooleanDatatype::validate("1").is_err());
    }
}
