use std::net::AddrParseError;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Address parsing error")]
    AddressParse(#[from] AddrParseError),
    #[error("Boolean parsing error")]
    BooleanParse,
    #[error("UUID parsing error")]
    UuidParse(#[from] uuid::Error),
    #[error("Date parsing error")]
    DateParse(#[from] chrono::ParseError),
    #[error("Duration parsing error")]
    DurationParse,
    #[error("String parsing error {0}")]
    StringParse(String),
    #[error("URI parsing error")]
    UriParse(#[from] fluent_uri::ParseError),
    #[error("URI must be absolute")]
    UriAbsolute,
    #[error("NCName illegal first char")]
    NCNameIllegalFirstChar,
    #[error("NCName illegal  char")]
    NCNameIllegalChar,
    #[error("Not a recognized type: {0}")]
    UnrecognizedTypeName(String),
}
