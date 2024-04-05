use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::str::FromStr;

use super::nc_name::NCName;
use crate::{Base, Error};

/// Wrapper for NCName
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TokenDatatype(NCName);

impl Base for TokenDatatype {
    fn base_type() -> String {
        String::from("String")
    }

    fn ref_type() -> String {
        String::from("str")
    }
}

impl TokenDatatype {
    fn new_if_valid(value: &str) -> Result<Self, Error> {
        Ok(Self(NCName::try_from(value)?))
    }
}

impl FromStr for TokenDatatype {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(NCName::try_from(s)?))
    }
}

impl TryFrom<&str> for TokenDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_if_valid(value)
    }
}

impl Deref for TokenDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        assert!(serde_json::from_str::<TokenDatatype>(r#""_abc""#).is_ok());
    }
}
