use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::str::FromStr;

use crate::{Base, Error, Metaschema, StringType, Validate, string_impl};
use super::nc_name::NCName;

/// A string representing arbitrary binary data encoded using the Base 64 algorithm as defined by RFC4648
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "&str")]
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
#[serde(try_from = "&str")]
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
#[serde(try_from = "&str")]
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
#[serde(try_from = "&str")]
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
#[serde(try_from = "&str")]
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
#[serde(try_from = "&str")]
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "&str")]
pub struct MarkupLineDatatype(String);
impl Validate for MarkupLineDatatype {
    fn validate(_value: &str) -> Result<(), Error> {
        Ok(())
    }
}

string_impl!(
    MarkupLineDatatype,
    description = "A single line of Markdown content conformant to the Commonmark specification.",
    pattern = "^[^\n]+$"
);

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "&str")]
pub struct MarkupMultilineDatatype(String);
impl Validate for MarkupMultilineDatatype {
    fn validate(_value: &str) -> Result<(), Error> {
        Ok(())
    }
}

string_impl!(
    MarkupMultilineDatatype,
    description = "A multiple lines of Markdown content conformant to the Commonmark specification.",
    pattern = "^[^\n]+$"
);

/// Wrapper for NCName
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "&str")]
pub struct TokenDatatype(String);

string_impl!(
    TokenDatatype,
    description = "A non-colonized name as defined by XML Schema Part 2: Datatypes Second Edition. https://www.w3.org/TR/xmlschema11-2/#NCName.",
    pattern = "^(\\p{{L}}|_)(\\p{{L}}|\\p{{N}}|[.\\-_])*$"
);

impl Validate for TokenDatatype {
    fn validate(value: &str) -> Result<(), Error> {
       NCName::try_from(value).map(|_| ())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Dummy {
        pub sdt: StringDatatype
    }

    #[test]
    fn test_de_se_string_data_type_in_struct() {
        let value = "abc";
        let dummy = Dummy{sdt: StringDatatype::try_from(value).unwrap()};
        let result = serde_json::to_string(&dummy);
        assert!(result.is_ok());
        let json = result.unwrap();
        assert_eq!(json, format!(r#"{{"sdt":"{}"}}"#, value));

        let result = serde_json::from_str::<Dummy>(&json);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), dummy);
    }

    #[test]
    fn test_string_data_type_from_string() {
        let test = "abc";
        let json_test = format!("\"{}\"", test);

        let sdt = StringDatatype::try_from(test).expect("StringDatatype::try_from");

        let result = serde_json::to_string(&sdt);
        assert!(result.is_ok());
        let json = result.unwrap();
        assert_eq!(json, json_test);

        let result = serde_json::from_str::<StringDatatype>(&json);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), sdt);
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

    #[test]
    fn test_de_se_token_datatype() {
        assert!(serde_json::from_str::<TokenDatatype>(r#""_abc""#).is_ok());
    }
    
}


