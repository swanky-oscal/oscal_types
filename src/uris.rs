use fluent_uri::Uri;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{Base, Error, Validate};

/// Repesents an absolute URI, with schema.  For relative paths,
/// use [URIReferenceDatatype].
///
/// URIDatatype uses [fluent_uri] for validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct URIDatatype(String);

impl Base for URIDatatype {
    fn base_type() -> String {
        String::from("String")
    }

    fn ref_type() -> String {
        String::from("str")
    }
}

impl TryFrom<&str> for URIDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value)?;
        Ok(Self(value.to_owned()))
    }
}

impl FromStr for URIDatatype {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl Validate for URIDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        let uri = value.parse::<Uri<String>>()?;
        match uri.is_absolute_uri() {
            true => Ok(()),
            false => Err(Error::UriAbsolute),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct URIReferenceDatatype(String);

impl Base for URIReferenceDatatype {
    fn base_type() -> String {
        String::from("String")
    }

    fn ref_type() -> String {
        String::from("str")
    }
}

impl TryFrom<&str> for URIReferenceDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value)?;
        Ok(Self(value.to_owned()))
    }
}

impl FromStr for URIReferenceDatatype {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl Validate for URIReferenceDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        let _ = value.parse::<Uri<String>>()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_uri() {
        let good_uri = "https://fedramp.gov/ns/oscal";
        assert!(good_uri.parse::<URIDatatype>().is_ok());

        let bad_uri = "#a78f7e4c-a27a-4b1e-901b-ebfecf2b0301";
        assert!(bad_uri.parse::<URIDatatype>().is_err());
    }

    #[test]
    fn test_valid_uri_reference() {
        let good_uri = "https://fedramp.gov/ns/oscal";
        assert!(good_uri.parse::<URIReferenceDatatype>().is_ok());

        let bad_uri = "#a78f7e4c-a27a-4b1e-901b-ebfecf2b0301";
        assert!(bad_uri.parse::<URIReferenceDatatype>().is_ok());
    }

    #[test]
    fn test_serde() {
        let json = format!("\"{}\"", uuid::Uuid::new_v4());
        let uri = serde_json::from_str::<URIDatatype>(&json).expect("fail");
        let result = serde_json::to_string(&uri).expect("fail");
        assert_eq!(&json, &result);
    }
}
