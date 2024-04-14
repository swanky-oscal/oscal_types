#[macro_export]
macro_rules! string_impl {
    (  $t:ty, description = $d:expr, pattern =  $pattern:expr, content_encoding = $encoding:expr ) => {
        impl Metaschema for $t {
            fn _type() -> Option<&'static str> {
                Some("string")
            }
            fn description() -> Option<&'static str> {
                Some($d)
            }
        }

        impl StringType for $t {
            fn pattern() -> Option<&'static str> {
                Some($pattern)
            }
            fn content_encoding() -> Option<&'static str> {
                Some($encoding)
            }
        }

        impl Base for $t {
            fn base_type() -> String {
                String::from("String")
            }

            fn ref_type() -> String {
                String::from("str")
            }
        }

        impl Deref for $t {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl FromStr for $t {
            type Err = Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl TryFrom<&str> for $t {
            type Error = Error;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match Self::validate(value) {
                    Ok(()) => Ok(Self(value.to_string())),
                    Err(e) => Err(e),
                }
            }
        }
    };

    (  $t:ty, description = $d:expr, format = $format:expr , pattern =  $pattern:expr, content_encoding = $encoding:expr ) => {
        impl Metaschema for $t {
            fn _type() -> Option<&'static str> {
                Some("string")
            }
            fn description() -> Option<&'static str> {
                Some($d)
            }
        }

        impl StringType for $t {
            fn format() -> Option<&'static str> {
                Some($format)
            }
            fn pattern() -> Option<&'static str> {
                Some($pattern)
            }
            fn content_encoding() -> Option<&'static str> {
                Some($encoding)
            }
        }

        impl Base for $t {
            fn base_type() -> String {
                String::from("String")
            }

            fn ref_type() -> String {
                String::from("str")
            }
        }

        impl Deref for $t {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl FromStr for $t {
            type Err = Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl TryFrom<&str> for $t {
            type Error = Error;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match Self::validate(value) {
                    Ok(()) => Ok(Self(value.to_string())),
                    Err(e) => Err(e),
                }
            }
        }
    };

    (  $t:ty, description = $d:expr, format = $format:expr, pattern =  $pattern:expr ) => {
        impl Metaschema for $t {
            fn _type() -> Option<&'static str> {
                Some("string")
            }
            fn description() -> Option<&'static str> {
                Some($d)
            }
        }

        impl StringType for $t {
            fn format() -> Option<&'static str> {
                Some($format)
            }
            fn pattern() -> Option<&'static str> {
                Some($pattern)
            }
            fn content_encoding() -> Option<&'static str> {
                None
            }
        }

        impl Base for $t {
            fn base_type() -> String {
                String::from("String")
            }

            fn ref_type() -> String {
                String::from("str")
            }
        }

        impl Deref for $t {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl FromStr for $t {
            type Err = Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl TryFrom<&str> for $t {
            type Error = Error;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match Self::validate(value) {
                    Ok(()) => Ok(Self(value.to_string())),
                    Err(e) => Err(e),
                }
            }
        }
    };
    (  $t:ty, description = $d:expr, pattern =  $pattern:expr ) => {
        impl Metaschema for $t {
            fn _type() -> Option<&'static str> {
                Some("string")
            }
            fn description() -> Option<&'static str> {
                Some($d)
            }
        }

        impl StringType for $t {
            fn format() -> Option<&'static str> {
                None
            }
            fn pattern() -> Option<&'static str> {
                Some($pattern)
            }
            fn content_encoding() -> Option<&'static str> {
                None
            }
        }

        impl Base for $t {
            fn base_type() -> String {
                String::from("String")
            }

            fn ref_type() -> String {
                String::from("str")
            }
        }

        impl Deref for $t {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl FromStr for $t {
            type Err = Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl TryFrom<&str> for $t {
            type Error = Error;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match Self::validate(value) {
                    Ok(()) => Ok(Self(value.to_string())),
                    Err(e) => Err(e),
                }
            }
        }
    };
    (  $t:ty, description = $d:expr, format =  $format:expr ) => {
        impl Metaschema for $t {
            fn _type() -> Option<&'static str> {
                Some("string")
            }
            fn description() -> Option<&'static str> {
                Some($d)
            }
        }

        impl StringType for $t {
            fn format() -> Option<&'static str> {
                Some($format)
            }
            fn pattern() -> Option<&'static str> {
                None
            }
            fn content_encoding() -> Option<&'static str> {
                None
            }
        }

        impl Base for $t {
            fn base_type() -> String {
                String::from("String")
            }

            fn ref_type() -> String {
                String::from("str")
            }
        }

        impl Deref for $t {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl FromStr for $t {
            type Err = Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl TryFrom<&str> for $t {
            type Error = Error;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match Self::validate(value) {
                    Ok(()) => Ok(Self(value.to_string())),
                    Err(e) => Err(e),
                }
            }
        }
    };
}
