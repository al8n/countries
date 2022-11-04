use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
struct Coordinate {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Geography {
    coordinates: Coordinate,
    #[serde(rename = "isLandlocked")]
    land_locked: bool,
    #[serde(rename = "capitalCity")]
    capital: Vec<String>,
    #[serde(rename = "landArea")]
    area: f64,
    region: String,
    subregion: String,
    #[serde(rename = "borderCountries")]
    border_countries: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Currency {
    name: String,
    short_name: Option<String>,
    #[serde(rename = "iso4217")]
    iso_4217: String,
    iso_numeric: Option<String>,
    symbol: String,
    subunit: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
    decimal_mark: Option<String>,
    decimal_places: u8,
    thousands_separator: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct OfficialLanguageName {
    common: String,
    native: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct OfficialLanguage {
    name: OfficialLanguageName,
    #[serde(rename = "iso639_3")]
    iso_639_3: String,
    #[serde(rename = "bcp47")]
    bcp_47: String,
    #[serde(rename = "iso15924")]
    iso_15924: String,
    iana: Vec<String>,
    #[serde(rename = "isExtinct")]
    extinct: bool,
    #[serde(rename = "isSpurious")]
    spurious: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Language {
    official: Vec<OfficialLanguage>,
    spoken: Option<Vec<String>>,
}

#[derive(Debug, Copy, Clone)]
enum TimezoneType {
    Link,
    Canonical,
}

impl serde::Serialize for TimezoneType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TimezoneType::Link => serializer.serialize_str("link"),
            TimezoneType::Canonical => serializer.serialize_str("canonical"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for TimezoneType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "link" => Ok(TimezoneType::Link),
            "canonical" => Ok(TimezoneType::Canonical),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown timezone type: {}",
                s
            ))),
        }
    }
}

impl ToTokenStream for TimezoneType {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        write!(
            out,
            r#"
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum TimezoneType {{
    Link,
    Canonical,
}}

impl core::fmt::Display for TimezoneType {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        match self {{
            TimezoneType::Link => write!(f, "link"),
            TimezoneType::Canonical => write!(f, "canonical"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl serde::Serialize for TimezoneType {{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        match self {{
            TimezoneType::Link => serializer.serialize_str("link"),
            TimezoneType::Canonical => serializer.serialize_str("canonical"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl<'de> serde::Deserialize<'de> for TimezoneType {{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {{
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {{
            "link" => Ok(TimezoneType::Link),
            "canonical" => Ok(TimezoneType::Canonical),
            _ => Err(serde::de::Error::custom(format!("Unknown timezone type: {{}}", s))),
        }}
    }}
}}
"#
        )?;
        writeln!(out)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Timezone {
    name: String,
    #[serde(rename = "type")]
    ty: TimezoneType,
    #[serde(rename = "linkedTo")]
    linked_to: Option<String>,
    #[serde(rename = "utcOffset")]
    utc_offset: String,
    #[serde(rename = "dstOffset")]
    dst_offset: String,
}

/// Driving side
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
enum DrivingSide {
    /// Left-hand side
    Left,
    /// Right-hand side
    Right,
}

impl core::fmt::Display for DrivingSide {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DrivingSide::Left => write!(f, "left"),
            DrivingSide::Right => write!(f, "right"),
        }
    }
}

impl serde::Serialize for DrivingSide {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DrivingSide::Left => serializer.serialize_str("left"),
            DrivingSide::Right => serializer.serialize_str("right"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for DrivingSide {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "left" => Ok(DrivingSide::Left),
            "right" => Ok(DrivingSide::Right),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown driving side: {}",
                s
            ))),
        }
    }
}

impl ToTokenStream for DrivingSide {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        write!(
            out,
            r#"
/// Driving side
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum DrivingSide {{
    /// Left-hand side
    Left,
    /// Right-hand side
    Right,
}}

impl core::fmt::Display for DrivingSide {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        match self {{
            DrivingSide::Left => write!(f, "left"),
            DrivingSide::Right => write!(f, "right"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl serde::Serialize for DrivingSide {{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        match self {{
            DrivingSide::Left => serializer.serialize_str("left"),
            DrivingSide::Right => serializer.serialize_str("right"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl<'de> serde::Deserialize<'de> for DrivingSide {{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {{
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {{
            "left" => Ok(DrivingSide::Left),
            "right" => Ok(DrivingSide::Right),
            _ => Err(serde::de::Error::custom(format!("Unknown driving side: {{}}", s))),
        }}
    }}
}}
"#
        )?;
        writeln!(out)?;
        Ok(())
    }
}

/// The unit of distance used (kilometer or mile)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
enum DistanceUint {
    /// Kilometer
    Kilometer,
    /// Mile
    Mile,
}

impl core::fmt::Display for DistanceUint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DistanceUint::Kilometer => write!(f, "kilometer"),
            DistanceUint::Mile => write!(f, "mile"),
        }
    }
}

impl serde::Serialize for DistanceUint {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DistanceUint::Kilometer => serializer.serialize_str("kilometer"),
            DistanceUint::Mile => serializer.serialize_str("mile"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for DistanceUint {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "kilometer" => Ok(DistanceUint::Kilometer),
            "mile" => Ok(DistanceUint::Mile),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown distance unit: {}",
                s
            ))),
        }
    }
}

impl ToTokenStream for DistanceUint {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        write!(
            out,
            r#"
/// The unit of distance used (kilometer or mile) 
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum DistanceUint {{
    /// Kilometer
    Kilometer,
    /// Mile
    Mile,
}}

impl core::fmt::Display for DistanceUint {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        match self {{
            DistanceUint::Kilometer => write!(f, "kilometer"),
            DistanceUint::Mile => write!(f, "mile"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl serde::Serialize for DistanceUint {{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        match self {{
            DistanceUint::Kilometer => serializer.serialize_str("kilometer"),
            DistanceUint::Mile => serializer.serialize_str("mile"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl<'de> serde::Deserialize<'de> for DistanceUint {{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {{
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {{
            "kilometer" => Ok(DistanceUint::Kilometer),
            "mile" => Ok(DistanceUint::Mile),
            _ => Err(serde::de::Error::custom(format!("Unknown distance unit: {{}}", s))),
        }}
    }}
}}
"#
        )?;
        Ok(())
    }
}

/// The unit of temperature (celsius or fahrenheit)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum TemperatureUint {
    /// Celsius
    Celsius,
    /// Fahrenheit
    Fahrenheit,
    /// Mixed
    Mixed,
}

impl core::fmt::Display for TemperatureUint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TemperatureUint::Celsius => write!(f, "celsius"),
            TemperatureUint::Fahrenheit => write!(f, "fahrenheit"),
            TemperatureUint::Mixed => write!(f, "mixed"),
        }
    }
}

impl serde::Serialize for TemperatureUint {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TemperatureUint::Celsius => serializer.serialize_str("celsius"),
            TemperatureUint::Fahrenheit => serializer.serialize_str("fahrenheit"),
            TemperatureUint::Mixed => serializer.serialize_str("mixed"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for TemperatureUint {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "celsius" => Ok(TemperatureUint::Celsius),
            "fahrenheit" => Ok(TemperatureUint::Fahrenheit),
            "mixed" => Ok(TemperatureUint::Mixed),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown temperature unit: {}",
                s
            ))),
        }
    }
}

impl ToTokenStream for TemperatureUint {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        write!(
            out,
            r#"
/// The unit of temperature (celsius or fahrenheit)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum TemperatureUint {{
    /// Celsius
    Celsius,
    /// Fahrenheit
    Fahrenheit,
    /// Mixed,
    Mixed,
}}

impl core::fmt::Display for TemperatureUint {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        match self {{
            TemperatureUint::Celsius => write!(f, "celsius"),
            TemperatureUint::Fahrenheit => write!(f, "fahrenheit"),
            TemperatureUint::Mixed => write!(f, "mixed"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl serde::Serialize for TemperatureUint {{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        match self {{
            TemperatureUint::Celsius => serializer.serialize_str("celsius"),
            TemperatureUint::Fahrenheit => serializer.serialize_str("fahrenheit"),
            TemperatureUint::Mixed => serializer.serialize_str("mixed"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl<'de> serde::Deserialize<'de> for TemperatureUint {{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {{
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {{
            "celsius" => Ok(TemperatureUint::Celsius),
            "fahrenheit" => Ok(TemperatureUint::Fahrenheit),
            "mixed" => Ok(TemperatureUint::Mixed),
            _ => Err(serde::de::Error::custom(format!("Unknown temperature unit: {{}}", s))),
        }}
    }}
}}        
"#
        )?;
        writeln!(out)?;
        Ok(())
    }
}

/// The system of measurement in use
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum MeasurementSystem {
    /// Metric system
    Metric,
    /// Imperial system
    Imperial,
}

impl core::fmt::Display for MeasurementSystem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MeasurementSystem::Metric => write!(f, "metric"),
            MeasurementSystem::Imperial => write!(f, "imperial"),
        }
    }
}

impl serde::Serialize for MeasurementSystem {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MeasurementSystem::Metric => serializer.serialize_str("metric"),
            MeasurementSystem::Imperial => serializer.serialize_str("imperial"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for MeasurementSystem {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "metric" => Ok(MeasurementSystem::Metric),
            "imperial" => Ok(MeasurementSystem::Imperial),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown measurement system: {}",
                s
            ))),
        }
    }
}

impl ToTokenStream for MeasurementSystem {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        write!(
            out,
            r#"
/// The system of measurement in use
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum MeasurementSystem {{
    /// Metric system
    Metric,
    /// Imperial system
    Imperial,
}}

impl core::fmt::Display for MeasurementSystem {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        match self {{
            MeasurementSystem::Metric => write!(f, "metric"),
            MeasurementSystem::Imperial => write!(f, "imperial"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl serde::Serialize for MeasurementSystem {{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        match self {{
            MeasurementSystem::Metric => serializer.serialize_str("metric"),
            MeasurementSystem::Imperial => serializer.serialize_str("imperial"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl<'de> serde::Deserialize<'de> for MeasurementSystem {{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {{
        let s = String::deserialize(deserializer)?;
        match s.as_str() {{
            "metric" => Ok(MeasurementSystem::Metric),
            "imperial" => Ok(MeasurementSystem::Imperial),
            _ => Err(serde::de::Error::custom(format!("Unknown measurement system: {{}}", s))),
        }}
    }}
}}
"#
        )?;
        Ok(())
    }
}

/// Type of clock used
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum HourClock {
    /// 12-hour clock
    Twelve,
    /// 24-hour clock
    TwentyFour,
    /// Mixed (12-hour clock or 24-hour clock)
    Mixed,
}

impl core::fmt::Display for HourClock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HourClock::Twelve => write!(f, "12hr"),
            HourClock::TwentyFour => write!(f, "24hr"),
            HourClock::Mixed => write!(f, "mixed"),
        }
    }
}

impl serde::Serialize for HourClock {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            HourClock::Twelve => serializer.serialize_str("12hr"),
            HourClock::TwentyFour => serializer.serialize_str("24hr"),
            HourClock::Mixed => serializer.serialize_str("mixed"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for HourClock {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "12hr" => Ok(HourClock::Twelve),
            "24hr" => Ok(HourClock::TwentyFour),
            "mixed" => Ok(HourClock::Mixed),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown hour clock: {}",
                s
            ))),
        }
    }
}

impl ToTokenStream for HourClock {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        write!(
            out,
            r#"
/// Type of clock used
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum HourClock {{
    /// 12-hour clock
    Twelve,
    /// 24-hour clock
    TwentyFour,
    /// Mixed (12-hour clock or 24-hour clock)
    Mixed,
}}

impl core::fmt::Display for HourClock {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        match self {{
            HourClock::Twelve => write!(f, "12hr"),
            HourClock::TwentyFour => write!(f, "24hr"),
            HourClock::Mixed => write!(f, "mixed"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl serde::Serialize for HourClock {{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        match self {{
            HourClock::Twelve => serializer.serialize_str("12hr"),
            HourClock::TwentyFour => serializer.serialize_str("24hr"),
            HourClock::Mixed => serializer.serialize_str("mixed"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl<'de> serde::Deserialize<'de> for HourClock {{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {{
        let s = String::deserialize(deserializer)?;
        match s.as_str() {{
            "12hr" => Ok(HourClock::Twelve),
            "24hr" => Ok(HourClock::TwentyFour),
            "mixed" => Ok(HourClock::Mixed),
            _ => Err(serde::de::Error::custom(format!("Unknown hour clock: {{}}", s))),
        }}
    }}
}}
"#
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Locale {
    ietf: Vec<String>,
    measurement_system: MeasurementSystem,
    driving_side: DrivingSide,
    hour_clock: HourClock,
    timezones: Vec<Timezone>,
    date_formats: HashMap<String, String>,
    week_starts_on: String,
    #[serde(rename = "distanceMeasurement")]
    distance_uint: DistanceUint,
    #[serde(rename = "temperatureMeasurement")]
    temperature_uint: TemperatureUint,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CountryName {
    common: String,
    official: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CountryMeta {
    common: String,
    official: String,
    native: Option<HashMap<String, CountryName>>,
    alternates: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum SubdivisionOfficialName {
    Number(usize),
    String(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SubdivisionName {
    official: SubdivisionOfficialName,
    common: Option<String>,
    native: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct IDD {
    prefix: Option<String>,
    suffixes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Subdivision {
    #[serde(rename = "isoCode")]
    iso_code: String,
    #[serde(rename = "type")]
    ty: Option<String>,
    name: HashMap<String, SubdivisionName>,
}

#[derive(Clone, Serialize, Deserialize)]
struct CountryData {
    name: CountryMeta,
    flag: String,
    cca2: String,
    cca3: String,
    ccn3: String,
    ioc: Option<String>,
    tld: Vec<String>,
    locale: Locale,
    geography: Geography,
    languages: Language,
    currencies: Vec<Currency>,
    idd: IDD,
    subdivisions: Vec<Subdivision>,
}

trait ToGetterTokenStream {
    fn to_getter_token_stream(
        &self,
        fn_name: &str,
        field_name: &str,
        output_type: &str,
        doc: &str,
    ) -> Result<String>;

    fn to_output(&self, name: &str) -> String;

    fn to_output_ty(&self) -> String;
}

impl<S: std::borrow::Borrow<str>> ToGetterTokenStream for S {
    fn to_getter_token_stream(
        &self,
        fn_name: &str,
        field_name: &str,
        output_type: &str,
        doc: &str,
    ) -> Result<String> {
        let mut buf = io::BufWriter::new(Vec::new());
        writeln!(buf, "{doc}")?;
        writeln!(buf, "\t#[inline]")?;
        writeln!(buf, "\tpub const fn {fn_name}(&self) -> {output_type} {{")?;
        writeln!(buf, "\t\t{}", self.to_output(field_name))?;
        writeln!(buf, "\t}}")?;
        Ok(String::from_utf8(buf.into_inner()?)?)
    }

    fn to_output(&self, name: &str) -> String {
        format!("self.{name}")
    }

    fn to_output_ty(&self) -> String {
        "&'static str".to_string()
    }
}

trait ToTokenStream {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()>;
}

impl ToTokenStream for CountryData {
    fn to_token_stream<W: Write>(buf: &mut W) -> Result<()> {
        static FIELDS: &[(&str, &str, &str)] = &[
            (
                "flag",
                "&'static str",
                r"
    /// Returns the country's flag",
            ),
            (
                "cca2",
                "&'static str",
                r"
    /// Returns [ISO 3166-1 alpha-2] code.
    /// 
    /// [ISO 3166-1 alpha-2]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2",
            ),
            (
                "cca3",
                "&'static str",
                r"
    /// Returns [ISO 3166-1 alpha-3] code.
    /// 
    /// [ISO 3166-1 alpha-3]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3",
            ),
            (
                "ccn3",
                "u16",
                r"
    /// Returns [ISO 3166-1 numeric] code.
    /// 
    /// [ISO 3166-1 numeric]: https://en.wikipedia.org/wiki/ISO_3166-1_numeric",
            ),
            (
                "ioc",
                "Option<&'static str>",
                r"
    /// Returns [International Olympic Committee] code.
    /// 
    /// [International Olympic Committee]: https://en.wikipedia.org/wiki/International_Olympic_Committee",
            ),
            (
                "tld",
                "&'static [&'static str]",
                r"
    /// Returns list of [Country Code Top Level Domain (ccTLD)] used
    /// 
    /// [Country Code Top Level Domain (ccTLD)]: https://en.wikipedia.org/wiki/Country_code_top-level_domain#Lists",
            ),
            (
                "locale",
                "&'static Locale",
                r"
    /// Returns the country's locale information",
            ),
            (
                "idd",
                "&'static IDD",
                r"
    /// Returns the country's [international dialing direct] information
    /// 
    /// [international dialing direct]: https://en.wikipedia.org/wiki/List_of_country_calling_codes",
            ),
        ];

        writeln!(buf, r"/// Country")?;
        writeln!(buf, "#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]")?;
        writeln!(buf, "pub struct Country {{")?;
        for field in FIELDS {
            writeln!(buf, "\t{}: {},", field.0, field.1)?;
        }
        writeln!(buf, "}}\n")?;

        writeln!(buf, "impl Country {{")?;
        for field in FIELDS {
            writeln!(
                buf,
                "{}",
                field
                    .0
                    .to_getter_token_stream(field.0, field.0, field.1, field.2)?
            )?;
        }
        writeln!(buf, "}}\n")?;
        Ok(())
    }
}

impl ToTokenStream for IDD {
    fn to_token_stream<W: Write>(buf: &mut W) -> Result<()> {
        static FIELDS: &[(&str, &str, &str)] = &[
            (
                "prefix",
                "&'static str",
                r"
    /// Returns the geographical code prefix (e.g. +1 for US)",
            ),
            (
                "prefix_u8",
                "u8",
                r"
    /// Returns the geographical code prefix (without '+') in `u8` (e.g. 1 for US)",
            ),
            (
                "suffixes",
                "&'static [&'static str]",
                r"
    /// Returns the list of suffixes assigned (e.g. 201 in US)",
            ),
            (
                "suffixes_u16",
                "&'static [u16]",
                r"
    /// Returns the list of suffixes assigned in u16 (e.g. 201 in US)",
            ),
        ];
        writeln!(
            buf,
            r"
/// [International dialing direct] info.
/// 
/// [International dialing direct]: https://en.wikipedia.org/wiki/List_of_country_calling_codes"
        )?;
        writeln!(
            buf,
            "#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]"
        )?;
        writeln!(buf, "#[allow(clippy::upper_case_acronyms)]")?;
        writeln!(buf, "pub struct IDD {{")?;
        for field in FIELDS {
            writeln!(buf, "\t{}: {},", field.0, field.1)?;
        }
        writeln!(buf, "}}\n")?;

        writeln!(buf, "impl IDD {{")?;
        for field in FIELDS {
            writeln!(
                buf,
                "{}",
                field
                    .0
                    .to_getter_token_stream(field.0, field.0, field.1, field.2)?
            )?;
        }
        writeln!(buf, "}}\n")?;
        Ok(())
    }
}

impl ToTokenStream for Locale {
    fn to_token_stream<W: Write>(buf: &mut W) -> Result<()> {
        static FIELDS: &[(&str, &str, &str)] = &[
            (
                "ietf",
                "&'static [&'static str]",
                r"
    /// Returns a list of [IETF] locale codes (e.g. `en-US`)
    /// 
    /// [IETF]: https://en.wikipedia.org/wiki/IETF_language_tag",
            ),
            (
                "date_formats",
                "&'static super::StaticMap<&'static str, &'static str>",
                r"
    /// Returns date formats for each IETF locale.
    /// 
    /// - Key is the IETF code
    /// - Value is the date format, where:
    ///   - `G` = era
    ///   - `y` = year
    ///   - `M` = month
    ///   - `d` = day ",
            ),
            (
                "measurement_system",
                "MeasurementSystem",
                r"
    /// Returns system of measurement in use. see [`MeasurementSystem`]",
            ),
            (
                "hour_clock",
                "HourClock",
                r"
    /// Returns the type of clock used. see [`HourClock`]",
            ),
            (
                "driving_side",
                "DrivingSide",
                r"
    /// Returns the side of the road traffic drives on. see [`DrivingSide`]",
            ),
            (
                "distance_uint",
                "DistanceUint",
                r"
    /// Returns the unit of distance used (kilometer or mile). see [`DistanceUint`]",
            ),
            (
                "temperature_uint",
                "TemperatureUint",
                r"
    /// Returns the unit of temperature (celsius or fahrenheit). see [`TemperatureUint`]",
            ),
        ];

        writeln!(
            buf,
            r"
/// [International dialing direct] info.
/// 
/// [International dialing direct]: https://en.wikipedia.org/wiki/List_of_country_calling_codes"
        )?;
        writeln!(
            buf,
            "#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]"
        )?;
        writeln!(buf, "pub struct Locale {{")?;
        for field in FIELDS {
            writeln!(buf, "\t{}: {},", field.0, field.1)?;
        }
        writeln!(buf, "}}\n")?;

        writeln!(buf, "impl Locale {{")?;
        for field in FIELDS {
            writeln!(
                buf,
                "{}",
                field
                    .0
                    .to_getter_token_stream(field.0, field.0, field.1, field.2)?
            )?;
        }
        writeln!(buf, "}}\n")?;
        Ok(())
    }
}

trait Generator: Write {
    fn gen_enum_impl(
        &mut self,
        name: &'static str,
        impls: &[Box<dyn ToGetterTokenStream>],
    ) -> Result<()> {
        writeln!(self, "impl {} {{", name)?;

        writeln!(self, "}}\n")?;
        Ok(())
    }

    fn gen_enum<D: core::fmt::Display>(
        &mut self,
        name: &'static str,
        variants: impl Iterator<Item = D>,
        doc: &'static str,
    ) -> Result<()> {
        writeln!(self, "/// {doc}")
            .and_then(|_| {
                writeln!(
                    self,
                    "#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]"
                )
            })
            .and_then(|_| writeln!(self, "#[repr(u8)]"))
            .and_then(|_| writeln!(self, "pub enum {name} {{"))?;

        for variant in variants {
            writeln!(self, "\t{},", variant)?;
        }

        writeln!(self, "}}\n")?;
        Ok(())
    }

    fn eof(&mut self) -> Result<()> {
        writeln!(self)?;
        Ok(())
    }
}

impl<W: Write> Generator for W {}

fn generate_enums(src: &[CountryData]) -> Result<()> {
    let out_path = PathBuf::from("src/enums.rs");
    let mut out = io::BufWriter::new(File::create(&out_path)?);
    writeln!(out, "// Auto generated file, please do not modify \n\n")?;
    out.gen_enum(
        "CCA2",
        src.iter().map(|c| &c.cca2),
        "ISO 3166-1 alpha-2 code",
    )?;
    out.gen_enum(
        "CCA3",
        src.iter().map(|c| &c.cca3),
        "ISO 3166-1 alpha-3 code",
    )?;
    out.eof()?;
    Ok(())
}

fn generate_types() -> Result<()> {
    let out_path = PathBuf::from("src/types.rs");
    let mut out = io::BufWriter::new(File::create(&out_path)?);
    writeln!(out, "// Auto generated file, please do not modify \n\n")?;
    <HourClock as ToTokenStream>::to_token_stream(&mut out)?;
    <DrivingSide as ToTokenStream>::to_token_stream(&mut out)?;
    <DistanceUint as ToTokenStream>::to_token_stream(&mut out)?;
    <TemperatureUint as ToTokenStream>::to_token_stream(&mut out)?;
    <MeasurementSystem as ToTokenStream>::to_token_stream(&mut out)?;
    <Locale as ToTokenStream>::to_token_stream(&mut out)?;
    <IDD as ToTokenStream>::to_token_stream(&mut out)?;
    <TimezoneType as ToTokenStream>::to_token_stream(&mut out)?;
    <CountryData as ToTokenStream>::to_token_stream(&mut out)?;
    Ok(())
}

fn main() -> Result<()> {
    let src_path = PathBuf::from("data.json");

    let src = File::open(&src_path)?;
    let countries: Vec<CountryData> = serde_json::from_reader(src)?;
    generate_enums(&countries)?;
    generate_types()?;

    // // impl
    // writeln!(out, "impl Country {{")?;

    // out.generate("name", "&'static str", &countries, true, |c| &c.name)?;
    // out.generate("alpha2", "&'static str", &countries, true, |c| &c.alpha2)?;
    // out.generate("alpha3", "&'static str", &countries, true, |c| &c.alpha3)?;

    // out.generate_from(
    //     "name",
    //     "&str",
    //     "UnknownCountry",
    //     &countries,
    //     false,
    //     "src.trim()",
    //     |c| {
    //         format!(
    //             "\"{}\" | \"{}\" | \"{}\"",
    //             c.name.trim().to_lowercase(),
    //             c.name.trim(),
    //             c.name.trim().to_uppercase()
    //         )
    //     },
    // )?;
    // out.generate_from(
    //     "alpha2",
    //     "&str",
    //     "UnknownAlpha2",
    //     &countries,
    //     false,
    //     "src.trim()",
    //     |c| {
    //         let chars = c.alpha2.chars();
    //         let mut upper = true;
    //         let mut s = String::with_capacity(2);
    //         for ch in chars {
    //             if upper {
    //                 upper = false;
    //                 s.push(ch);
    //             } else {
    //                 s.push(ch.to_ascii_lowercase());
    //             }
    //         }

    //         format!(
    //             "\"{}\" | \"{}\" | \"{}\"",
    //             c.alpha2.trim().to_lowercase(),
    //             s,
    //             c.alpha2.trim().to_uppercase()
    //         )
    //     },
    // )?;
    // out.generate_from(
    //     "alpha3",
    //     "&str",
    //     "UnknownAlpha3",
    //     &countries,
    //     false,
    //     "src.trim()",
    //     |c| {
    //         format!(
    //             "\"{}\" | \"{}\"",
    //             c.alpha3.trim().to_lowercase(),
    //             c.alpha3.trim()
    //         )
    //     },
    // )?;

    // out.generate("country_code", "u16", &countries, false, |c| {
    //     c.country_code.parse::<u16>().unwrap()
    // })?;
    // out.generate_from(
    //     "country_code",
    //     "u16",
    //     "UnknownCountryCode",
    //     &countries,
    //     true,
    //     "src",
    //     |c| c.country_code.parse::<u16>().unwrap(),
    // )?;
    // out.generate("country_code_str", "&'static str", &countries, true, |c| {
    //     &c.country_code
    // })?;
    // out.generate_from(
    //     "country_code_str",
    //     "&str",
    //     "UnknownCountryCode",
    //     &countries,
    //     false,
    //     "src.trim()",
    //     |c| format!("\"{}\"", c.country_code),
    // )?;

    // out.generate("phone_code", "Option<u16>", &countries, false, |c| {
    //     if c.phone_code.is_empty() {
    //         "None".to_owned()
    //     } else {
    //         format!("Some({})", c.phone_code.parse::<u16>().unwrap())
    //     }
    // })?;

    // out.generate(
    //     "phone_code_str",
    //     "Option<&'static str>",
    //     &countries,
    //     false,
    //     |c| {
    //         if c.phone_code.is_empty() {
    //             "None".to_owned()
    //         } else {
    //             format!("Some(\"{}\")", c.phone_code)
    //         }
    //     },
    // )?;
    // out.generate("capital", "Option<&'static str>", &countries, false, |c| {
    //     if c.capital.is_empty() {
    //         "None".to_owned()
    //     } else {
    //         format!("Some(\"{}\")", c.capital)
    //     }
    // })?;
    // out.generate("currency_code", "&'static str", &countries, true, |c| {
    //     &c.currency_code
    // })?;
    // out.generate("iso", "&'static str", &countries, true, |c| &c.iso)?;
    // out.generate_from(
    //     "iso",
    //     "&str",
    //     "Unkonwn3166_2",
    //     &countries,
    //     false,
    //     "src.trim()",
    //     |c| format!("\"{}\" | \"{}\"", c.iso.trim().to_lowercase(), c.iso.trim()),
    // )?;

    // out.generate("emoji", "&'static str", &countries, true, |c| &c.emoji)?;
    // out.generate("default_locale", "&'static str", &countries, true, |c| &c.default_locale)?;
    // out.generate("default_language", "&'static str", &countries, true, |c| &c.default_language)?;
    // out.generate("continent", "&'static str", &countries, true, |c| &c.continent)?;
    // out.generate("region", "&'static str", &countries, true, |c| &c.region)?;
    // out.generate("sub_region", "&'static str", &countries, true, |c| {
    //     &c.sub_region
    // })?;
    // out.generate(
    //     "intermediate_region",
    //     "Option<&'static str>",
    //     &countries,
    //     false,
    //     |c| {
    //         if c.intermediate_region.is_empty() {
    //             "None".to_owned()
    //         } else {
    //             format!(
    //                 "Some(\"{}\")",
    //                 c.intermediate_region
    //             )
    //         }
    //     },
    // )?;

    // out.generate("region_code_str", "&'static str", &countries, true, |c| {
    //     &c.region_code
    // })?;

    // out.generate(
    //     "sub_region_code_str",
    //     "&'static str",
    //     &countries,
    //     true,
    //     |c| &c.sub_region_code,
    // )?;
    // out.generate(
    //     "intermediate_region_code_str",
    //     "Option<&'static str>",
    //     &countries,
    //     false,
    //     |c| {
    //         if c.intermediate_region.is_empty() {
    //             "None".to_owned()
    //         } else {
    //             format!(
    //                 "Some(\"{}\")",
    //                 c.intermediate_region_code
    //             )
    //         }
    //     },
    // )?;

    // out.generate("region_code", "u16", &countries, false, |c| {
    //     c.region_code.parse::<u16>().unwrap()
    // })?;
    // out.generate("sub_region_code", "u16", &countries, false, |c| {
    //     c.sub_region_code.parse::<u16>().unwrap()
    // })?;
    // out.generate(
    //     "intermediate_region_code",
    //     "Option<u16>",
    //     &countries,
    //     false,
    //     |c| {
    //         if c.intermediate_region_code.is_empty() {
    //             "None".to_owned()
    //         } else {
    //             format!(
    //                 "Some({})",
    //                 c.intermediate_region_code.parse::<u16>().unwrap()
    //             )
    //         }
    //     },
    // )?;

    // out.close()?;

    Ok(())
}

// trait Generator: Write {
//     fn generate<'a, D, F>(
//         &mut self,
//         fn_name: &'static str,
//         return_ty: &'static str,
//         countries: &'a [Country],
//         quote: bool,
//         f: F,
//     ) -> Result<()>
//     where
//         D: core::fmt::Display + 'a,
//         F: FnMut(&'a Country) -> D,
//     {
//         self.write_fn(fn_name, return_ty)
//             .and_then(|_| self.match_variants(countries, quote, f))
//             .and_then(|_| self.close_fn())
//     }

//     #[allow(clippy::too_many_arguments)]
//     fn generate_from<'a, D, F>(
//         &mut self,
//         fn_name: &'static str,
//         from_ty: &'static str,
//         error: &'static str,
//         countries: &'a [Country],
//         cons: bool,
//         src: &'static str,
//         mut f: F,
//     ) -> Result<()>
//     where
//         D: core::fmt::Display + 'a,
//         F: FnMut(&'a Country) -> D,
//     {
//         writeln!(self, "\t#[inline]")?;
//         if cons {
//             writeln!(self, "\tpub const fn from_{}(src: {}) -> Result<Self, super::ParseCountryError> {{", fn_name, from_ty)?;
//         } else {
//             writeln!(self, "\tpub fn from_{}(src: {}) -> Result<Self, super::ParseCountryError> {{", fn_name, from_ty)?;
//         }

//         writeln!(self, "\t\tmatch {} {{", src)?;
//         for country in countries {
//             writeln!(
//                 self,
//                 "\t\t\t{} => Ok(Self::{}),",
//                 f(country),
//                 country.alpha2
//             )?;
//         }
//         writeln!(
//             self,
//             "\t\t\t_ => Err(super::ParseCountryError::{}),",
//             error
//         )?;
//         self.close_match()?;
//         self.close_fn()?;
//         Ok(())
//     }

//     fn match_variants<'a, D, F>(
//         &mut self,
//         countries: &'a [Country],
//         quote: bool,
//         mut f: F,
//     ) -> Result<()>
//     where
//         D: core::fmt::Display + 'a,
//         F: FnMut(&'a Country) -> D,
//     {
//         writeln!(self, "\t\tmatch self {{")?;
//         for country in countries {
//             if quote {
//                 writeln!(
//                     self,
//                     "\t\t\tCountry::{} => \"{}\",",
//                     country.alpha2,
//                     f(country)
//                 )?;
//             } else {
//                 writeln!(self, "\t\t\tCountry::{} => {},", country.alpha2, f(country))?;
//             }
//         }
//         self.close_match()?;
//         Ok(())
//     }

//     fn write_fn(&mut self, fn_name: &'static str, return_ty: &'static str) -> Result<()> {
//         writeln!(self, "\t#[inline]")?;
//         writeln!(
//             self,
//             "\tpub const fn {}(&self) -> {} {{",
//             fn_name, return_ty
//         )?;
//         Ok(())
//     }

//     fn close_match(&mut self) -> Result<()> {
//         writeln!(self, "\t\t}}")?;
//         Ok(())
//     }

//     fn close_fn(&mut self) -> Result<()> {
//         writeln!(self, "\t}}")?;
//         Ok(())
//     }

//     fn close(&mut self) -> Result<()> {
//         writeln!(self, "}}")?;
//         Ok(())
//     }
// }

// impl<W: Write> Generator for W {}
