use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Base, Error};

use super::{DecimalType, NumberType};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DecimalDatatype(f64);

impl Base for DecimalDatatype {
    fn base_type() -> String {
        String::from("f64")
    }
    fn ref_type() -> String {
        String::from("&f64")
    }
}

impl Deref for DecimalDatatype {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for DecimalDatatype {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl TryFrom<&f64> for DecimalDatatype {
    type Error = Error;
    fn try_from(value: &f64) -> Result<Self, Self::Error> {
        Ok(Self(*value))
    }
}

impl DecimalType for DecimalDatatype {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct IntegerDatatype(i64);

impl Base for IntegerDatatype {
    fn base_type() -> String {
        String::from("i64")
    }
    fn ref_type() -> String {
        String::from("&i64")
    }
}

impl Deref for IntegerDatatype {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i64> for IntegerDatatype {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl TryFrom<&i64> for IntegerDatatype {
    type Error = Error;
    fn try_from(value: &i64) -> Result<Self, Self::Error> {
        Ok(Self(*value))
    }
}

impl NumberType for IntegerDatatype {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NonNegativeIntegerDatatype(u64);

impl Base for NonNegativeIntegerDatatype {
    fn base_type() -> String {
        String::from("u64")
    }

    fn ref_type() -> String {
        String::from("&u64")
    }
}

impl Deref for NonNegativeIntegerDatatype {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for NonNegativeIntegerDatatype {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl TryFrom<&u64> for NonNegativeIntegerDatatype {
    type Error = Error;
    fn try_from(value: &u64) -> Result<Self, Self::Error> {
        Ok(Self(*value))
    }
}

impl NumberType for NonNegativeIntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(0)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PositiveIntegerDatatype(u64);

impl Base for PositiveIntegerDatatype {
    fn base_type() -> String {
        String::from("u64")
    }

    fn ref_type() -> String {
        String::from("&u64")
    }
}

impl Deref for PositiveIntegerDatatype {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for PositiveIntegerDatatype {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl TryFrom<&u64> for PositiveIntegerDatatype {
    type Error = Error;
    fn try_from(value: &u64) -> Result<Self, Self::Error> {
        Ok(Self(*value))
    }
}

impl NumberType for PositiveIntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(1)
    }
}
