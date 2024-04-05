use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Base, Error, Validate};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BooleanDatatype(bool);

impl Base for BooleanDatatype {
    fn base_type() -> String {
        String::from("String")
    }
    fn ref_type() -> String {
        String::from("str")
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
    fn test_validate_json() {
        assert!(BooleanDatatype::validate("true").is_ok());
        assert!(BooleanDatatype::validate("false").is_ok());
        assert!(BooleanDatatype::validate("1").is_err());
    }

    #[test]
    fn test_validate_xml() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct Toy {
            val: bool,
        }

        let toy = Toy { val: true };
        let xml = serde_xml_rs::to_string(&toy).expect("fail");
        println!("{}", &xml);

        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><Toy><val>1</val></Toy>"#;
        let de_toy = serde_xml_rs::from_str::<Toy>(xml).expect("fail");
        assert_eq!(&toy, &de_toy);
    }
}
