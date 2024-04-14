use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Base, Error, Metaschema, StringType, Validate};

use std::str::FromStr;

use crate::string_impl;

/// A string representing arbitrary binary data encoded using the Base 64 algorithm as defined by RFC4648
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Base64Datatype(String);
impl Validate for Base64Datatype {
    fn validate(_value: &str) -> Result<(), Error> {
        Ok(())
    }
}

string_impl!(
    Base64Datatype,
    description = "Binary data encoded using the Base 64 encoding algorithm as defined by RFC4648.",
    pattern = r#"^[0-9A-Za-z+\/]+={0,2}$"#,
    content_encoding = "base64"
);

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StringDatatype(String);
impl Validate for StringDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        match value.trim() == value {
            true => Ok(()),
            false => Err(Error::StringParse(
                "Trailing and leading whitespace is not allowed".to_string(),
            )),
        }
    }
}

string_impl!(
    StringDatatype,
    description = "A non-empty string with leading and trailing whitespace disallowed. Whitespace is: U+9, U+10, U+32 or [ \n\t]+",
    pattern =  "^\\S(.*\\S)?$",
    content_encoding = "string"
);
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EmailAddressDatatype(String);
impl Validate for EmailAddressDatatype {
    fn validate(_value: &str) -> Result<(), Error> {
        Ok(())
    }
}

string_impl!(
    EmailAddressDatatype,
    description = "An email address string formatted according to RFC 6531.",
    pattern = "^.+@.+$",
    content_encoding = "email"
);

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct HostnameDatatype(String);
impl Validate for HostnameDatatype {
    fn validate(_value: &str) -> Result<(), Error> {
        Ok(())
    }
}

string_impl!(
    HostnameDatatype,
    description = "An internationalized Internet host name string formatted according to section 2.3.2.3 of RFC5890.",
    format = "idn-hostname"
);

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct IPV4AddressDatatype(String);
impl Validate for IPV4AddressDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        match value.parse::<std::net::Ipv4Addr>() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::AddressParse(e)),
        }
    }
}

string_impl!(
    IPV4AddressDatatype,
    description = "An Internet Protocol version 4 address represented using dotted-quad syntax as defined in section 3.2 of RFC2673.",
    format = "ipv4",
    pattern = "^((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])\\.){{3}}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])$"
);

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct IPV6AddressDatatype(String);
impl Validate for IPV6AddressDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        match value.parse::<std::net::Ipv6Addr>() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::AddressParse(e)),
        }
    }
}

string_impl!(
    IPV6AddressDatatype,
    description = "An Internet Protocol version 6 address represented using the syntax defined in section 2.2 of RFC3513.",
    format = "ipv6",
    pattern = "^(([0-9a-fA-F]{{1,4}}:){{7,7}}[0-9a-fA-F]{{1,4}}|([0-9a-fA-F]{{1,4}}:){{1,7}}:|([0-9a-fA-F]{{1,4}}:){{1,6}}:[0-9a-fA-F]{{1,4}}|([0-9a-fA-F]{{1,4}}:){{1,5}}(:[0-9a-fA-F]{{1,4}}){{1,2}}|([0-9a-fA-F]{{1,4}}:){{1,4}}(:[0-9a-fA-F]{{1,4}}){{1,3}}|([0-9a-fA-F]{{1,4}}:){{1,3}}(:[0-9a-fA-F]{{1,4}}){{1,4}}|([0-9a-fA-F]{{1,4}}:){{1,2}}(:[0-9a-fA-F]{{1,4}}){{1,5}}|[0-9a-fA-F]{{1,4}}:((:[0-9a-fA-F]{{1,4}}){{1,6}})|:((:[0-9a-fA-F]{{1,4}}){{1,7}}|:)|[fF][eE]80:(:[0-9a-fA-F]{{0,4}}){{0,4}}%[0-9a-zA-Z]{{1,}}|::([fF]{{4}}(:0{{1,4}}){{0,1}}:){{0,1}}((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9]).){{3,3}}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])|([0-9a-fA-F]{{1,4}}:){{1,4}}:((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9]).){{3,3}}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9]))$"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_data_type_from_string() {
        let result = StringDatatype::try_from("abc").expect("fail");
        assert_eq!(result, StringDatatype("abc".to_string()));
    }

    #[test]
    fn test_deref() {
        let show = |s: &str| s.to_string();

        let sdt = StringDatatype::try_from("abc").expect("fail");
        // StringDatatype can be passed as &str
        assert_eq!(show(&sdt), "abc");

        // StringDatatype can be deref'd to &str
        assert_eq!(sdt.deref(), "abc");
    }
}
