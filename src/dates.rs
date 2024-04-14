//! Date data types.  Supported by [chrono].
//!
//! This lib does not support the OSCAL date-with-timezone data type. It is
//! not used in any FedRAMP OSCAL schema, and it is not supported by ISO 8601 or RFC3339.
//! Dates with timezone is also not supported by [chrono].
//!
//! If you are determined to use dates with timezones, then turn off validation by setting the
//! crate feature `no_date_validation`.
//!
use chrono::prelude::*;
use iso8601_duration::Duration;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};

use crate::{string_impl, Base, Error, Metaschema, StringType, Validate};

/// A Naive date with no timezone.
///
/// The OSCAL model states that DateDatatype can support an optional timezone.
/// However, ISO 8601 and RFC 3339 state tht timezones may only be attached to times.
/// Thus, a simple date with a timezone is not valid.
///
/// If you absoluely, positively must accept a date with a timezone, turn off validation
/// for dates.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "&str")]
pub struct DateDatatype(String);
impl Validate for DateDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        if cfg!(feature = "date_validation") {
            match value.parse::<NaiveDate>() {
                Err(e) => Err(Error::DateParse(e)),
                Ok(_) => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}

string_impl!(
    DateDatatype,
    description = "A string representing a 24-hour period with an optional timezone.",
    pattern = "^(((2000|2400|2800|(19|2[0-9](0[48]|[2468][048]|[13579][26])))-02-29)|(((19|2[0-9])[0-9]{2})-02-(0[1-9]|1[0-9]|2[0-8]))|(((19|2[0-9])[0-9]{2})-(0[13578]|10|12)-(0[1-9]|[12][0-9]|3[01]))|(((19|2[0-9])[0-9]{2})-(0[469]|11)-(0[1-9]|[12][0-9]|30)))(Z|(-((0[0-9]|1[0-2]):00|0[39]:30)|\\+((0[0-9]|1[0-4]):00|(0[34569]|10):30|(0[58]|12):45)))?$"
);

impl DateDatatype {
    /// Create a new date.
    /// The date is created from the current Local date.
    pub fn new() -> Self {
        let now = Local::now();
        Self(now.date_naive().to_string())
    }

    /// Convert to [chrono::NaiveDate]
    pub fn date_naive(&self) -> Result<NaiveDate, Error> {
        self.0.parse::<NaiveDate>().map_err(Error::DateParse)
    }
}

impl Default for DateDatatype {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "&str")]
pub struct DateTimeDatatype(String);

impl Validate for DateTimeDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        if cfg!(feature = "date_validation") {
            //Try to parse as a date time with timezone
            match value.parse::<DateTime<Utc>>() {
                Err(_) => {
                    // If no timezone was provided, try parsing as a naive datetime
                    match value.parse::<NaiveDateTime>() {
                        Err(e) => Err(Error::DateParse(e)),
                        Ok(_) => Ok(()),
                    }
                }
                Ok(_) => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}

string_impl!(
    DateTimeDatatype,
    description = "A string representing a point in time with an optional timezone.",
    pattern = r##"^(((2000|2400|2800|(19|2[0-9](0[48]|[2468][048]|[13579][26])))-02-29)|(((19|2[0-9])[0-9]{{2}})-02-(0[1-9]|1[0-9]|2[0-8]))|(((19|2[0-9])[0-9]{{2}})-(0[13578]|10|12)-(0[1-9]|[12][0-9]|3[01]))|(((19|2[0-9])[0-9]{{2}})-(0[469]|11)-(0[1-9]|[12][0-9]|30)))T(2[0-3]|[01][0-9]):([0-5][0-9]):([0-5][0-9])(\\.[0-9]+)?(Z|(-((0[0-9]|1[0-2]):00|0[39]:30)|\\+((0[0-9]|1[0-4]):00|(0[34569]|10):30|(0[58]|12):45)))?$"##
);

impl DateTimeDatatype {
    /// Create a new date-time.
    /// The date is created from the current Local date, and formatted in
    /// standard RFC33398 format (including timezone).  Ie, 2024-04-12T
    /// Requires the `date_validation` feature
    pub fn new() -> Self {
        Self(
            Local::now()
                .naive_local()
                .format("%Y-%m-%dT%H:%M:%S")
                .to_string(),
        )
    }

    /// Format the date into a pretty RFC 2822 string.
    /// Requires the `date_validation` feature
    pub fn to_rfc2822(&self) -> String {
        match DateTime::parse_from_rfc3339(&self.0) {
            Ok(dt) => dt.to_rfc2822(),
            Err(_) => "".to_string(),
        }
    }
}
impl Default for DateTimeDatatype {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "&str")]
pub struct DateTimeWithTimezoneDatatype(String);

string_impl!(
    DateTimeWithTimezoneDatatype,
    description = "A string representing a 24-hour period with a required timezone.",
    pattern = r#"^(((2000|2400|2800|(19|2[0-9](0[48]|[2468][048]|[13579][26])))-02-29)|(((19|2[0-9])[0-9]{{2}})-02-(0[1-9]|1[0-9]|2[0-8]))|(((19|2[0-9])[0-9]{{2}})-(0[13578]|10|12)-(0[1-9]|[12][0-9]|3[01]))|(((19|2[0-9])[0-9]{{2}})-(0[469]|11)-(0[1-9]|[12][0-9]|30)))(Z|(-((0[0-9]|1[0-2]):00|0[39]:30)|\\+((0[0-9]|1[0-4]):00|(0[34569]|10):30|(0[58]|12):45)))$"#
);

impl Validate for DateTimeWithTimezoneDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        match value.parse::<DateTime<Utc>>() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::DateParse(e)),
        }
    }
}

impl DateTimeWithTimezoneDatatype {
    pub fn new() -> Self {
        let utc: DateTime<Utc> = Utc::now();
        Self(utc.to_rfc3339())
    }
}

impl Default for DateTimeWithTimezoneDatatype {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "&str")]
pub struct DayTimeDurationDatatype(String);

string_impl!(
    DayTimeDurationDatatype,
    description = "An amount of time quantified in days, hours, minutes, and seconds.",
    format = "duration",
    pattern = "^-?P([0-9]+D(T(([0-9]+H([0-9]+M)?(([0-9]+|[0-9]+(\\.[0-9]+)?)S)?)|([0-9]+M(([0-9]+|[0-9]+(\\.[0-9]+)?)S)?)|([0-9]+|[0-9]+(\\.[0-9]+)?)S))?)|T(([0-9]+H([0-9]+M)?(([0-9]+|[0-9]+(\\.[0-9]+)?)S)?)|([0-9]+M(([0-9]+|[0-9]+(\\.[0-9]+)?)S)?)|([0-9]+|[0-9]+(\\.[0-9]+)?)S)$"
);

impl Validate for DayTimeDurationDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        let d = value
            .parse::<Duration>()
            .map_err(|_| Error::DurationParse)?;
        match d.num_days() {
            Some(_) => Ok(()),
            None => Err(Error::DurationParse),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "&str")]
pub struct YearMonthDurationDatatype(String);

string_impl!(
    YearMonthDurationDatatype,
    description = "An amount of time quantified in years and months based on ISO-8601 durations (see also RFC3339 appendix A).",
    format = "duration",
    pattern = "^-?P([0-9]+Y([0-9]+M)?)|[0-9]+M$"
);

impl Validate for YearMonthDurationDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        let d = value
            .parse::<Duration>()
            .map_err(|_| Error::DurationParse)?;
        match d.num_months() {
            Some(_) => Ok(()),
            None => Err(Error::DurationParse),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_json(value: &str) -> String {
        serde_json::to_string(value).expect("oops")
    }

    #[test]
    fn test_de_se_day_time_duration_datatype() {
        let test_value = "P4DT23H10S";
        let json_test_value = to_json(test_value);

        let date = serde_json::from_str::<DayTimeDurationDatatype>(&json_test_value)
            .expect("Failed to deserialize");
        assert_eq!(date.to_string(), test_value);

        let json = serde_json::to_string(&date).expect("failed to serialize");
        assert_eq!(json, json_test_value);
    }

    #[test]
    fn test_bad_day_time_duration_datatype() {
        let test_value = "P2Y3M4D";
        assert!(serde_json::from_str::<YearMonthDurationDatatype>(test_value).is_err());
    }


    #[test]
    fn test_de_se_year_month_duration_datatype() {
        let test_value = "P2Y3M";
        let json_test_value = to_json(test_value);

        let date = serde_json::from_str::<YearMonthDurationDatatype>(&json_test_value)
            .expect("Failed to deserialize");
        assert_eq!(date.to_string(), test_value);

        let json = serde_json::to_string(&date).expect("failed to serialize");
        assert_eq!(json, json_test_value);
    }

    #[test]
    fn test_bad_month_duration_datatype() {
        let test_value = "P2Y3M4D";
        assert!(serde_json::from_str::<YearMonthDurationDatatype>(test_value).is_err());
    }

    #[test]
    fn test_de_se_date_datatype() {
        let test_value = "2024-02-10";
        let json_test_value = to_json(test_value);

        let date =
            serde_json::from_str::<DateDatatype>(&json_test_value).expect("Failed to deserialize");
        assert_eq!(date.to_string(), test_value);

        let json = serde_json::to_string(&date).expect("failed to serialize");
        assert_eq!(json, json_test_value);
    }

    #[test]
    fn test_de_se_naive_datetime_datatype() {
        let test_value = "2024-04-13T09:57:13";
        let json_test_value = to_json(test_value);

        let date = serde_json::from_str::<DateTimeDatatype>(&json_test_value)
            .expect("Failed to deserialize");
        assert_eq!(date.to_string(), test_value);

        let json = serde_json::to_string(&date).expect("failed to serialize");
        assert_eq!(json, json_test_value);
    }

    #[test]
    fn test_de_se_with_tz_datetime_datatype() {
        let test_value = "2024-04-13T09:57:13Z";
        let json_test_value = to_json(test_value);

        let date = serde_json::from_str::<DateTimeDatatype>(&json_test_value)
            .expect("Failed to deserialize");
        assert_eq!(date.to_string(), test_value);

        let json = serde_json::to_string(&date).expect("failed to serialize");
        assert_eq!(json, json_test_value);
    }

    #[test]
    fn test_de_se_tz_offset_datetime_datatype() {
        let test_value = "2024-04-13T09:57:13+05:00";
        let json_test_value = to_json(test_value);

        let date = serde_json::from_str::<DateTimeDatatype>(&json_test_value)
            .expect("Failed to deserialize");
        assert_eq!(date.to_string(), test_value);

        let json = serde_json::to_string(&date).expect("failed to serialize");
        assert_eq!(json, json_test_value);
    }

    #[test]
    fn test_de_se_tz_offset_datetime_with_timezone_datatype() {
        let test_value = "2024-04-13T09:57:13+05:00";
        let json_test_value = to_json(test_value);

        let date = serde_json::from_str::<DateTimeWithTimezoneDatatype>(&json_test_value)
            .expect("Failed to deserialize");
        assert_eq!(date.to_string(), test_value);

        let json = serde_json::to_string(&date).expect("failed to serialize");
        assert_eq!(json, json_test_value);
    }
    #[test]
    fn test_de_se_no_offset_datetime_with_timezone_datatype() {
        let test_value = "2024-04-13T09:57:13";
        let json_test_value = to_json(test_value);

        assert!(serde_json::from_str::<DateTimeWithTimezoneDatatype>(&json_test_value).is_err());
    }
}
