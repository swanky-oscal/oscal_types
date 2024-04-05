use serde::{Deserialize, Deserializer, Serialize};
use std::ops::Deref;
use std::str::FromStr;

use crate::{Base, Error};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub struct NCName(String);

impl Base for NCName {
    fn base_type() -> String {
        String::from("String")
    }
    fn ref_type() -> String {
        String::from("str")
    }
}

impl NCName {
    /// Returns `true` if `c` is a legal `NCNameStartChar` as defined
    /// by the ["Namespaces in XML 1.1"
    /// specification](https://www.w3.org/TR/xml-names11/#NT-NameStartChar).
    pub fn is_valid_start_char(c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '\u{C0}'..='\u{D6}' | '\u{D8}'..='\u{F6}' | '\u{F8}'..='\u{2FF}' | '\u{370}'..='\u{37D}' | '\u{37F}'..='\u{1FFF}' | '\u{200C}'..='\u{200D}' | '\u{2070}'..='\u{218F}' | '\u{2C00}'..='\u{2FEF}' | '\u{3001}'..='\u{D7FF}' | '\u{F900}'..='\u{FDCF}' | '\u{FDF0}'..='\u{FFFD}' | '\u{10000}'..='\u{EFFFF}')
    }

    /// Returns `true` if `c` is a legal `NCNameChar` as defined by
    /// the ["Namespaces in XML 1.1"
    /// specification](https://www.w3.org/TR/xml11/#NT-NameChar).
    pub fn is_valid_char(c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '-' | '.' | '\u{B7}' | '0'..='9' | '\u{C0}'..='\u{D6}' | '\u{D8}'..='\u{F6}' | '\u{F8}'..='\u{2FF}' | '\u{300}'..='\u{37D}' | '\u{37F}'..='\u{1FFF}' | '\u{200C}'..='\u{200D}' | '\u{203F}'..='\u{2040}' | '\u{2070}'..='\u{218F}' | '\u{2C00}'..='\u{2FEF}' | '\u{3001}'..='\u{D7FF}' | '\u{F900}'..='\u{FDCF}' | '\u{FDF0}'..='\u{FFFD}' | '\u{10000}'..='\u{EFFFF}')
    }

    pub fn new_if_valid(value: &str) -> Result<Self, Error> {
        let mut first = true;

        for c in value.chars() {
            if first {
                if !Self::is_valid_start_char(c) {
                    return Err(Error::NCNameIllegalFirstChar);
                }
                first = false;
            } else if !Self::is_valid_char(c) {
                return Err(Error::NCNameIllegalChar);
            }
        }
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for NCName {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_if_valid(value)
    }
}

impl FromStr for NCName {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new_if_valid(s)
    }
}

impl Deref for NCName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for NCName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NCName::try_from(s.as_ref()).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_first() {
        assert!("abc".parse::<NCName>().is_ok());
        assert!("_abc".parse::<NCName>().is_ok());
        assert!("Abc".parse::<NCName>().is_ok());
    }

    #[test]
    fn test_deserialize() {
        // Good names
        assert!(serde_json::from_str::<NCName>(r#""abc""#).is_ok());
        assert!(serde_json::from_str::<NCName>(r#""_abc""#).is_ok());
        assert!(serde_json::from_str::<NCName>(r#""abc-def""#).is_ok());

        // Bad names
        assert!(serde_json::from_str::<NCName>(r#"":abc""#).is_err());
        assert!(serde_json::from_str::<NCName>(r#""@abc""#).is_err());
    }

    #[test]
    fn test_serialize() {
        let name = "abc".parse::<NCName>().expect("fail");
        let json = serde_json::to_string(&name).expect("fail");
        assert_eq!(&json, r#""abc""#)
    }
}
