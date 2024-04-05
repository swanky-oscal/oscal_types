//! Date data types.  Supported by [chrono].
//!
//! This lib does not support the OSCAL date-with-timezone data type. It is
//! not used in any FedRAMP OSCAL schema, and it is not supported by [chrono].
//!
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};

use crate::{Base, Error};

/// A Naive date with no timezone.
/// All dates are UTC based.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DateDatatype(String);

impl Base for DateDatatype {
    fn base_type() -> String {
        String::from("String")
    }
    fn ref_type() -> String {
        String::from("str")
    }
}

impl DateDatatype {
    /// Create a new date.
    /// The date is created from the current UTC date.
    pub fn new() -> Self {
        Self(Utc::now().date_naive().to_string())
    }
}

impl Default for DateDatatype {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for DateDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for DateDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")?;
        Ok(Self(result.to_string()))
    }
}

impl ToString for DateDatatype {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DateTimeDatatype(String);
impl Base for DateTimeDatatype {
    fn base_type() -> String {
        String::from("str")
    }
    fn ref_type() -> String {
        String::from("str")
    }
}

impl TryFrom<&str> for DateTimeDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = value.parse::<NaiveDateTime>()?;
        Ok(Self(result.to_string()))
    }
}

impl FromStr for DateTimeDatatype {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.parse::<NaiveDateTime>()?;
        Ok(Self(result.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DateTimeWithTimezoneDatatype(String);
impl Base for DateTimeWithTimezoneDatatype {
    fn base_type() -> String {
        String::from("str")
    }
    fn ref_type() -> String {
        String::from("str")
    }
}

impl DateTimeWithTimezoneDatatype {
    pub fn new() -> Self {
        let utc: DateTime<Utc> = Utc::now();
        Self(utc.to_string())
    }
}

impl Default for DateTimeWithTimezoneDatatype {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for DateTimeWithTimezoneDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for DateTimeWithTimezoneDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = value.parse::<DateTime<Utc>>()?;
        Ok(Self(result.to_string()))
    }
}

impl FromStr for DateTimeWithTimezoneDatatype {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.parse::<DateTime<Utc>>()?;
        Ok(Self(result.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DateTimeDurationDatatype(String);
impl Base for DateTimeDurationDatatype {
    fn base_type() -> String {
        String::from("String")
    }
    fn ref_type() -> String {
        String::from("str")
    }
}

impl DateTimeDurationDatatype {
    pub fn new() -> Self {
        let utc: DateTime<Utc> = Utc::now();
        Self(utc.to_string())
    }
}
impl Default for DateTimeDurationDatatype {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for DateTimeDurationDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for DateTimeDurationDatatype {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_to_string() {
        let value = "2024-02-10";
        let json = format!("\"{}\"", value);
        let date = serde_json::from_str::<DateDatatype>(&json).expect("Failed to deserialize");
        assert_eq!(&date.to_string(), value);
        let result = serde_json::to_string(&date).expect("Failed to serialize");
        assert_eq!(&result, &json);
    }

    #[test]
    fn test_validate_date_time() {
        let input = "2015-02-18";
        let result = chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d");
        assert!(result.is_ok());

        let input = "2015-02-18-05:00";
        let result = chrono::DateTime::parse_from_rfc3339(input);
        println!("{:?}", &result);
    }
}
