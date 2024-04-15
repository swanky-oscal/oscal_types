pub use base::*;
pub use boolean::*;
pub use dates::*;
pub use error::Error;
pub(crate) use macros::*;
pub use numbers::*;
pub use strings::*;
pub use uris::*;
pub use uuid::*;
pub use validate::*;

pub mod base;
pub mod boolean;
pub mod dates;
pub mod error;
pub(crate) mod macros;
pub mod nc_name;
pub mod numbers;
pub mod strings;
pub mod uris;
pub mod uuid;
pub mod validate;

pub trait Metaschema {
    fn _type() -> Option<&'static str> {
        None
    }
    fn description() -> Option<&'static str> {
        None
    }
}

pub trait NumberType {
    fn minimum() -> Option<i64> {
        None
    }
    fn maximum() -> Option<i64> {
        None
    }
}

pub trait DecimalType {
    fn minimum() -> Option<f64> {
        None
    }
    fn maximum() -> Option<f64> {
        None
    }
}

pub trait StringType {
    fn format() -> Option<&'static str> {
        None
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn content_encoding() -> Option<&'static str> {
        None
    }
}

pub fn get_base_type(name: &str) -> Result<String, Error> {
    match name {
        "BooleanDatatype" => Ok(BooleanDatatype::base_type()),
        "DateDatatype" => Ok(DateDatatype::base_type()),
        "DateTimeDatatype" => Ok(DateTimeDatatype::base_type()),
        "DateTimeWithTimezoneDatatype" => Ok(DateTimeWithTimezoneDatatype::base_type()),
        "DayTimeDurationDatatype" => Ok(DayTimeDurationDatatype::base_type()),
        "DecimalDatatype" => Ok(DecimalDatatype::base_type()),
        "IntegerDatatype" => Ok(IntegerDatatype::base_type()),
        "NonNegativeIntegerDatatype" => Ok(NonNegativeIntegerDatatype::base_type()),
        "PositiveIntegerDatatype" => Ok(PositiveIntegerDatatype::base_type()),
        "StringDatatype" => Ok(StringDatatype::base_type()),
        "Base64Datatype" => Ok(Base64Datatype::base_type()),
        "EmailAddressDatatype" => Ok(EmailAddressDatatype::base_type()),
        "TokenDatatype" => Ok(TokenDatatype::base_type()),
        "URIDatatype" => Ok(URIDatatype::base_type()),
        "URIReferenceDatatype" => Ok(URIReferenceDatatype::base_type()),
        "UUIDDatatype" => Ok(UUIDDatatype::base_type()),
        _ => Err(Error::UnrecognizedTypeName(name.to_owned())),
    }
}

pub fn get_ref_type(name: &str) -> Result<String, Error> {
    match name {
        "BooleanDatatype" => Ok(BooleanDatatype::ref_type()),
        "DateDatatype" => Ok(DateDatatype::ref_type()),
        "DateTimeDatatype" => Ok(DateTimeDatatype::ref_type()),
        "DateTimeWithTimezoneDatatype" => Ok(DateTimeWithTimezoneDatatype::ref_type()),
        "DayTimeDurationDatatype" => Ok(DayTimeDurationDatatype::ref_type()),
        "DecimalDatatype" => Ok(DecimalDatatype::ref_type()),
        "IntegerDatatype" => Ok(IntegerDatatype::ref_type()),
        "NonNegativeIntegerDatatype" => Ok(NonNegativeIntegerDatatype::ref_type()),
        "PositiveIntegerDatatype" => Ok(PositiveIntegerDatatype::ref_type()),
        "StringDatatype" => Ok(StringDatatype::ref_type()),
        "Base64Datatype" => Ok(Base64Datatype::ref_type()),
        "EmailAddressDatatype" => Ok(EmailAddressDatatype::ref_type()),
        "TokenDatatype" => Ok(TokenDatatype::ref_type()),
        "URIDatatype" => Ok(URIDatatype::ref_type()),
        "URIReferenceDatatype" => Ok(URIReferenceDatatype::ref_type()),
        "UUIDDatatype" => Ok(UUIDDatatype::ref_type()),
        _ => Err(Error::UnrecognizedTypeName(name.to_owned())),
    }
}
