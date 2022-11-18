use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use tera::{Context, Tera};

lazy_static::lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("templates/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Variant {
    name: String,
    doc: String,
}

struct EnumDeref {
    name: &'static str,
    version: usize,
    variants: Vec<Variant>,
}

impl EnumDeref {
    const TEMPLATE: &str = "enum.tpl";

    fn new(name: &'static str, version: usize, variants: Vec<Variant>) -> Self {
        Self {
            name,
            variants,
            version,
        }
    }

    fn to_context(&self) -> Context {
        let mut context = Context::new();
        context.insert("name", &self.name);
        context.insert("version", &self.version);
        context.insert("variants", &self.variants);
        context
    }

    fn render<W: Write>(&self, buf: &mut W) -> Result<()> {
        match TEMPLATES.render(Self::TEMPLATE, &self.to_context()) {
            Ok(s) => writeln!(buf, "{}", s)?,
            Err(e) => {
                eprintln!("Error: {}", e);
                let mut cause = e.source();
                while let Some(e) = cause {
                    eprintln!("Reason: {}", e);
                    cause = e.source();
                }
            }
        };
        Ok(())
    }
}

struct Struct {
    name: &'static str,
    doc: &'static str,
    fields: &'static [StructField],
    derives: &'static str,
}

impl Struct {
    const TEMPLATE: &str = "struct.tpl";

    fn new(name: &'static str, doc: &'static str, fields: &'static [StructField]) -> Self {
        Self {
            name,
            doc,
            fields,
            derives: "#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]",
        }
    }

    fn with_derives(
        name: &'static str,
        doc: &'static str,
        fields: &'static [StructField],
        derives: &'static str,
    ) -> Self {
        Self {
            name,
            doc,
            fields,
            derives,
        }
    }

    fn to_context(&self) -> Context {
        let mut context = Context::new();
        context.insert("mod", &self.name.to_snake_case());
        context.insert("name", &self.name);
        context.insert("doc", &self.doc);
        context.insert("fields", &self.fields);
        context.insert("derives", &self.derives);
        context
    }

    fn render<W: Write>(&self, buf: &mut W) -> Result<()> {
        match TEMPLATES.render(Self::TEMPLATE, &self.to_context()) {
            Ok(s) => writeln!(buf, "{}", s)?,
            Err(e) => {
                eprintln!("Error: {}", e);
                let mut cause = e.source();
                while let Some(e) = cause {
                    eprintln!("Reason: {}", e);
                    cause = e.source();
                }
            }
        };
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
struct StructField {
    name: &'static str,
    ty: &'static str,
    doc: &'static str,
    getter: &'static str,
}

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

impl ToValue for Geography {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        let mut border_countries = String::from("&[");
        for c in self.border_countries.iter() {
            border_countries.push_str(&format!("crate::CCA3::{},", c.to_uppercase()));
        }
        border_countries.push(']');
        writeln!(buf,"&Geography {{ latitude: {}f64, longitude: {}f64, land_locked: {}, capital: {} area: {}f64, region: \"{}\", subregion: \"{}\", border_countries: {} }},", self.coordinates.latitude, self.coordinates.longitude, self.land_locked, self.capital.to_value_string()?, self.area, self.region, self.subregion, border_countries)?;
        Ok(())
    }
}

impl ToTokenStream for Geography {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        static FIELDS: &[StructField] = &[
            StructField {
                name: "latitude",
                ty: "f64",
                doc: "/// Returns the latitude",
                getter: "latitude",
            },
            StructField {
                name: "longitude",
                ty: "f64",
                doc: "/// Returns the longitude",
                getter: "longitude",
            },
            StructField {
                name: "land_locked",
                ty: "bool",
                doc:
                    "/// Returns whether or not the country is landlocked (not bordering the ocean)",
                getter: "is_landlocked",
            },
            StructField {
                name: "capital",
                ty: "&'static [&'static str]",
                doc: "/// Returns the name of the capital cities",
                getter: "capitals",
            },
            StructField {
                name: "area",
                ty: "f64",
                doc: "/// Returns the land area of the country",
                getter: "area",
            },
            StructField {
                name: "region",
                ty: "&'static str",
                doc: "/// Returns the region of the country",
                getter: "region",
            },
            StructField {
                name: "subregion",
                ty: "&'static str",
                doc: "/// Returns the subregion of the country",
                getter: "subregion",
            },
            StructField {
                name: "border_countries",
                ty: "&'static [crate::CCA3]",
                doc: r"/// Returns list of countries by their [ISO 3166-1 alpha-3] codes that border the country
    /// 
    /// [ISO 3166-1 alpha-3]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3",
                getter: "border_countries",
            },
        ];
        Struct::with_derives(
            "Geography",
            "",
            FIELDS,
            "#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]",
        )
        .render(out)
    }
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

impl ToValue for Option<String> {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        match self {
            Some(s) => write!(buf, "Some(\"{}\")", s)?,
            None => write!(buf, "None")?,
        }
        Ok(())
    }
}

impl ToValue for Vec<Currency> {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(buf, "&[")?;
        for currency in self {
            write!(buf, "{}, ", currency.to_value_string()?)?;
        }
        writeln!(buf, "],")?;
        Ok(())
    }
}

impl ToValue for Currency {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(
            buf,
            "&Currency {{ name: \"{}\", short_name: {}, iso_4217: \"{}\", iso_numeric: {}, symbol: \"{}\", subunit: {}, prefix: {}, suffix: {}, decimal_mark: {}, decimal_places: {}, thousands_separator: {} }}",
            self.name,
            self.short_name.to_value_string()?,
            self.iso_4217,
            self.iso_numeric.to_value_string()?,
            self.symbol,
            self.subunit.to_value_string()?,
            self.prefix.to_value_string()?,
            self.suffix.to_value_string()?,
            self.decimal_mark.as_ref().and_then(|s| if s.is_empty() { None } else { Some(format!("Some('{}')", s)) }).unwrap_or_else(|| "None".to_string()),
            self.decimal_places,
            self.thousands_separator.as_ref().and_then(|s| if s.is_empty() { None } else { Some(format!("Some('{}')", s)) }).unwrap_or_else(|| "None".to_string()),
        )?;
        Ok(())
    }
}

impl ToTokenStream for Currency {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        static FIELDS: &[StructField] = &[
            StructField {
                name: "name",
                ty: "&'static str",
                doc: "/// Returns the name of the currency",
                getter: "name",
            },
            StructField {
                name: "short_name",
                ty: "Option<&'static str>",
                doc: "/// Returns the short name of the currency",
                getter: "short_name",
            },
            StructField {
                name: "iso_4217",
                ty: "&'static str",
                doc: r"/// Returns the [ISO 4217] currency code
    ///
    /// [ISO 4217]: https://en.wikipedia.org/wiki/ISO_4217",
                getter: "iso4217",
            },
            StructField {
                name: "iso_numeric",
                ty: "Option<&'static str>",
                doc: r"/// Returns the [ISO 4217 numeric] currency code
    ///
    /// [ISO 4217 numeric]: https://en.wikipedia.org/wiki/ISO_4217#cite_note-ISO4217-1",
                getter: "iso_numeric",
            },
            StructField {
                name: "symbol",
                ty: "&'static str",
                doc: "/// Returns the currency symbol",
                getter: "symbol",
            },
            StructField {
                name: "subunit",
                ty: "Option<&'static str>",
                doc: "/// Returns the name of the subunit of the currency",
                getter: "subunit",
            },
            StructField {
                name: "prefix",
                ty: "Option<&'static str>",
                doc: "/// Returns the prefix of the currency symbol",
                getter: "prefix",
            },
            StructField {
                name: "suffix",
                ty: "Option<&'static str>",
                doc: "/// Returns the suffix of the currency symbol",
                getter: "suffix",
            },
            StructField {
                name: "decimal_mark",
                ty: "Option<char>",
                doc: "/// Returns the decimal mark of the currency",
                getter: "decimal_mark",
            },
            StructField {
                name: "decimal_places",
                ty: "u8",
                doc: "/// Returns the number of decimal places of the currency",
                getter: "decimal_places",
            },
            StructField {
                name: "thousands_separator",
                ty: "Option<char>",
                doc: "/// Returns the thousands separator of the currency",
                getter: "thousands_separator",
            },
        ];

        Struct::new("Currency", "", FIELDS).render(out)
    }
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

impl ToValue for OfficialLanguage {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(buf,
            "&Language {{ name: \"{}\", native_name: {}, iana: {} iso_639_3: \"{}\", bcp_47: \"{}\", iso_15924: \"{}\", extinct: {}, spurious: {}, }},",
            self.name.common,
            self.name.native.as_ref().map(|name| format!("Some(\"{}\")", name)).unwrap_or_else(|| String::from("None")),
            self.iana.to_value_string()?,
            self.iso_639_3,
            self.bcp_47,
            self.iso_15924,
            self.extinct,
            self.spurious
        )?;

        Ok(())
    }
}

impl ToTokenStream for OfficialLanguage {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        static FIELDS: &[StructField] = &[
            StructField {
                name: "name",
                ty: "&'static str",
                doc: "/// Returns the name of the language",
                getter: "name",
            },
            StructField {
                name: "native_name",
                ty: "Option<&'static str>",
                doc: "/// Returns the native name of the language",
                getter: "native_name",
            },
            StructField {
                name: "iso_639_3",
                ty: "&'static str",
                doc: r"/// Returns the [ISO 639-3] language code.
    ///
    /// [ISO 639-3]: https://en.wikipedia.org/wiki/ISO_639-3",
                getter: "iso639_3",
            },
            StructField {
                name: "bcp_47",
                ty: "&'static str",
                doc: r"/// Returns the [BCP 47] tag.
    ///
    /// [BCP 47]: https://en.wikipedia.org/wiki/IETF_language_tag",
                getter: "bcp47",
            },
            StructField {
                name: "iso_15924",
                ty: "&'static str",
                doc: r"/// Returns the [ISO 15924] script code.
    ///
    /// [ISO 15924]: https://en.wikipedia.org/wiki/ISO_15924",
                getter: "iso15924",
            },
            StructField {
                name: "iana",
                ty: "&'static [&'static str]",
                doc: r"/// Returns array of assigned [IANA] tags.
    ///
    /// [IANA]: https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
    // TODO: add IANA struct which contains the information, and replace str with that struct",
                getter: "iana",
            },
            StructField {
                name: "extinct",
                ty: "bool",
                doc: "/// Returns whether the language is extinct",
                getter: "is_extinct",
            },
            StructField {
                name: "spurious",
                ty: "bool",
                doc: "/// Returns whether the language is spurious",
                getter: "is_spurious",
            },
        ];

        Struct::new("Language", "", FIELDS).render(out)
    }
}

impl ToValue for Vec<OfficialLanguage> {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(buf, "&[")?;
        for l in self.iter() {
            write!(buf, " {}", l.to_value_string()?)?;
        }
        writeln!(buf, "],")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Language {
    official: Vec<OfficialLanguage>,
    spoken: Option<Vec<String>>,
}

impl ToTokenStream for Language {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        <OfficialLanguage as ToTokenStream>::to_token_stream(out)?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum TimezoneType {
    Link,
    Canonical,
}

impl ToValue for TimezoneType {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        match self {
            TimezoneType::Link => write!(buf, "TimezoneType::Link")?,
            TimezoneType::Canonical => write!(buf, "TimezoneType::Canonical")?,
        }
        Ok(())
    }
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
#[cfg_attr(all(feature = "async-graphql", feature = "alloc"), derive(::async_graphql::Enum))]
#[repr(u8)]
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
        <&'de str as serde::Deserialize<'de>>::deserialize(deserializer).and_then(|s| match s {{
            "link" | "Link" => Ok(TimezoneType::Link),
            "canonical" | "Canonical" => Ok(TimezoneType::Canonical),
            _ => Err(serde::de::Error::custom(format!("Unknown timezone type: {{}}", s))),
        }})
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

impl ToValue for Timezone {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(
            buf,
            "&Timezone {{ name: \"{}\", ty: {}, linked_to: {}, utc_offset: \"{}\", dst_offset: \"{}\" }}",
            self.name,
            self.ty.to_value_string()?,
            self.linked_to.to_value_string()?,
            self.utc_offset,
            self.dst_offset,
        )?;
        Ok(())
    }
}

impl ToValue for Vec<Timezone> {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(buf, "&[")?;
        for t in self {
            write!(buf, "{},", t.to_value_string()?)?;
        }
        write!(buf, "]")?;
        Ok(())
    }
}

impl ToTokenStream for Timezone {
    fn to_token_stream<W: Write>(buf: &mut W) -> Result<()> {
        <TimezoneType as ToTokenStream>::to_token_stream(buf)?;

        static FIELDS: &[StructField] = &[
            StructField {
                name: "name",
                ty: "&'static str",
                doc: r"/// Returns the name of the timezone",
                getter: "name",
            },
            StructField {
                name: "ty",
                ty: "TimezoneType",
                doc: r"/// Returns the type of timezone (primary or alias)",
                getter: "timezone_type",
            },
            StructField {
                name: "linked_to",
                ty: "Option<&'static str>",
                doc: r"/// Returns the name of the timezone this timezone is linked to",
                getter: "linked_to",
            },
            StructField {
                name: "utc_offset",
                ty: "&'static str",
                doc: "/// Returns the UTC offset of the timezone",
                getter: "utc_offset",
            },
            StructField {
                name: "dst_offset",
                ty: "&'static str",
                doc: "/// Returns the DST offset of the timezone",
                getter: "dst_offset",
            },
        ];

        Struct {
            name: "Timezone",
            doc: r"
/// Timezone info, reference: [tz database timezones].
///
/// [tz database timezones]: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones",
            fields: FIELDS,
            derives: "#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]",
        }
        .render(buf)
    }
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

impl ToValue for DrivingSide {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        match self {
            DrivingSide::Left => write!(buf, "DrivingSide::Left")?,
            DrivingSide::Right => write!(buf, "DrivingSide::Right")?,
        }
        Ok(())
    }
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
#[cfg_attr(all(feature = "async-graphql", feature = "alloc"), derive(::async_graphql::Enum))]
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
        <&'de str as serde::Deserialize<'de>>::deserialize(deserializer).and_then(|s| match s {{
            "left" | "Left" | "l" | "L" => Ok(DrivingSide::Left),
            "right" | "Right" | "r" | "R" => Ok(DrivingSide::Right),
            _ => Err(serde::de::Error::custom(format!("Unknown driving side: {{}}", s))),
        }})
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

impl ToValue for DistanceUint {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        match self {
            DistanceUint::Kilometer => write!(buf, "DistanceUint::Kilometer")?,
            DistanceUint::Mile => write!(buf, "DistanceUint::Mile")?,
        }
        Ok(())
    }
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
#[cfg_attr(all(feature = "async-graphql", feature = "alloc"), derive(::async_graphql::Enum))]
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
        <&'de str as serde::Deserialize<'de>>::deserialize(deserializer).and_then(|s| match s {{
            "kilometer" | "km" | "Kilometer" | "Km" | "KM" => Ok(DistanceUint::Kilometer),
            "mile" | "mi" | "Mile" | "Mi" | "MI" => Ok(DistanceUint::Mile),
            _ => Err(serde::de::Error::custom(format!("Unknown distance unit: {{}}", s))),
        }})
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

impl ToValue for TemperatureUint {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        match self {
            TemperatureUint::Celsius => write!(buf, "TemperatureUint::Celsius")?,
            TemperatureUint::Fahrenheit => write!(buf, "TemperatureUint::Fahrenheit")?,
            TemperatureUint::Mixed => write!(buf, "TemperatureUint::Mixed")?,
        }
        Ok(())
    }
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
        String::deserialize(deserializer).and_then(|s| match s.as_str() {
            "celsius" | "Celsius" => Ok(TemperatureUint::Celsius),
            "fahrenheit" | "Fahrenheit" => Ok(TemperatureUint::Fahrenheit),
            "mixed"
            | "Mixed"
            | "celsius or fahrenheit"
            | "Celsius or Fahrenheit"
            | "celsius/fahrenheit"
            | "Celsius/Fahrenheit"
            | " fahrenheit or celsius"
            | "Fahrenheit or Celsius"
            | "fahrenheit/celsius"
            | "Fahrenheit/Celsius" => Ok(TemperatureUint::Mixed),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown temperature unit: {}",
                s
            ))),
        })
    }
}

impl ToTokenStream for TemperatureUint {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        write!(
            out,
            r#"
/// The unit of temperature (celsius or fahrenheit)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(all(feature = "async-graphql", feature = "alloc"), derive(::async_graphql::Enum))]
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
        <&'de str as serde::Deserialize<'de>>::deserialize(deserializer).and_then(|s| match s {{
            "celsius" | "Celsius" => Ok(TemperatureUint::Celsius),
            "fahrenheit" | "Fahrenheit" => Ok(TemperatureUint::Fahrenheit),
            "mixed" | "Mixed" | "celsius or fahrenheit" | "Celsius or Fahrenheit" | "celsius/fahrenheit" | "Celsius/Fahrenheit"| " fahrenheit or celsius" | "Fahrenheit or Celsius" | "fahrenheit/celsius" | "Fahrenheit/Celsius" => Ok(TemperatureUint::Mixed),
            _ => Err(serde::de::Error::custom(format!("Unknown temperature unit: {{}}", s))),
        }})
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

impl ToValue for MeasurementSystem {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        match self {
            MeasurementSystem::Metric => write!(buf, "MeasurementSystem::Metric")?,
            MeasurementSystem::Imperial => write!(buf, "MeasurementSystem::Imperial")?,
        }
        Ok(())
    }
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
            "metric" | "Metric" => Ok(MeasurementSystem::Metric),
            "imperial" | "Imperial" => Ok(MeasurementSystem::Imperial),
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
#[cfg_attr(all(feature = "async-graphql", feature = "alloc"), derive(::async_graphql::Enum))]
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
        let s = <&'de str as serde::Deserialize<'de>>::deserialize(deserializer)?;
        match s {{
            "metric" | "Metric" => Ok(MeasurementSystem::Metric),
            "imperial" | "Imperial" => Ok(MeasurementSystem::Imperial),
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

impl ToValue for HourClock {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        match self {
            HourClock::Twelve => write!(buf, "HourClock::Twelve")?,
            HourClock::TwentyFour => write!(buf, "HourClock::TwentyFour")?,
            HourClock::Mixed => write!(buf, "HourClock::Mixed")?,
        }
        Ok(())
    }
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
        String::deserialize(deserializer).and_then(|s| match s.as_str() {
            "12hr" | "12" => Ok(HourClock::Twelve),
            "24hr" | "24" => Ok(HourClock::TwentyFour),
            "Mixed" | "mixed" | "12hr/24hr" | "24hr/12hr" | "12/24" | "24/12" | "12 or 24"
            | "12hr or 24hr" | "24hr or 12hr" | "24 or 12" => Ok(HourClock::Mixed),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown hour clock: {}",
                s
            ))),
        })
    }
}

impl ToTokenStream for HourClock {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        write!(
            out,
            r#"
/// Type of clock used
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(all(feature = "async-graphql", feature = "alloc"), derive(::async_graphql::Enum))]
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
        <&'de str as serde::Deserialize<'de>>::deserialize(deserializer).and_then(|s| {{
            match s {{
                "12hr" | "12" => Ok(HourClock::Twelve),
                "24hr" | "24" => Ok(HourClock::TwentyFour),
                "Mixed" | "mixed" | "12hr/24hr" | "24hr/12hr" | "12/24" | "24/12" | "12 or 24" | "12hr or 24hr" | "24hr or 12hr" | "24 or 12" => Ok(HourClock::Mixed),
                _ => Err(serde::de::Error::custom(format!("Unknown hour clock: {{}}", s))),
            }}
        }})
    }}
}}
"#
        )?;
        Ok(())
    }
}

/// Day
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Day {
    /// Sunday
    Sunday,
    /// Monday
    Monday,
    /// Tuesday
    Tuesday,
    /// Wednesday
    Wednesday,
    /// Thursday
    Thursday,
    /// Friday
    Friday,
    /// Saturday
    Saturday,
}

impl ToValue for Day {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        match self {
            Day::Sunday => write!(buf, "Day::Sunday")?,
            Day::Monday => write!(buf, "Day::Monday")?,
            Day::Tuesday => write!(buf, "Day::Tuesday")?,
            Day::Wednesday => write!(buf, "Day::Wednesday")?,
            Day::Thursday => write!(buf, "Day::Thursday")?,
            Day::Friday => write!(buf, "Day::Friday")?,
            Day::Saturday => write!(buf, "Day::Saturday")?,
        }
        Ok(())
    }
}

impl core::fmt::Display for Day {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Day::Sunday => write!(f, "Sunday"),
            Day::Monday => write!(f, "Monday"),
            Day::Tuesday => write!(f, "Tuesday"),
            Day::Wednesday => write!(f, "Wednesday"),
            Day::Thursday => write!(f, "Thursday"),
            Day::Friday => write!(f, "Friday"),
            Day::Saturday => write!(f, "Saturday"),
        }
    }
}

impl serde::Serialize for Day {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Day::Sunday => serializer.serialize_str("Sunday"),
            Day::Monday => serializer.serialize_str("Monday"),
            Day::Tuesday => serializer.serialize_str("Tuesday"),
            Day::Wednesday => serializer.serialize_str("Wednesday"),
            Day::Thursday => serializer.serialize_str("Thursday"),
            Day::Friday => serializer.serialize_str("Friday"),
            Day::Saturday => serializer.serialize_str("Saturday"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Day {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "sunday" | "Sunday" | "Sun" | "sun" => Ok(Self::Sunday),
            "monday" | "Monday" | "Mon" | "mon" => Ok(Self::Monday),
            "tuesday" | "Tuesday" | "Tue" | "tue" => Ok(Self::Tuesday),
            "wednesday" | "Wednesday" | "Wed" | "wed" => Ok(Self::Wednesday),
            "thursday" | "Thursday" | "Thu" | "thu" => Ok(Self::Thursday),
            "friday" | "Friday" | "Fri" | "fri" => Ok(Self::Friday),
            "saturday" | "Saturday" | "Sat" | "sat" => Ok(Self::Saturday),
            _ => Err(serde::de::Error::custom(format!("Unknown day: {}", s))),
        }
    }
}

impl Day {
    /// Returns short name of the day
    #[inline]
    pub const fn short(&self) -> &'static str {
        match self {
            Day::Sunday => "Sun",
            Day::Monday => "Mon",
            Day::Tuesday => "Tue",
            Day::Wednesday => "Wed",
            Day::Thursday => "Thu",
            Day::Friday => "Fri",
            Day::Saturday => "Sat",
        }
    }

    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Day::Sunday => "Sunday",
            Day::Monday => "Monday",
            Day::Tuesday => "Tuesday",
            Day::Wednesday => "Wednesday",
            Day::Thursday => "Thursday",
            Day::Friday => "Friday",
            Day::Saturday => "Saturday",
        }
    }
}

impl ToTokenStream for Day {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        write!(
            out,
            r#"
/// Day
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(all(feature = "async-graphql", feature = "alloc"), derive(::async_graphql::Enum))]
#[repr(u8)]
pub enum Day {{
    /// Sunday
    Sunday,
    /// Monday
    Monday,
    /// Tuesday
    Tuesday,
    /// Wednesday
    Wednesday,
    /// Thursday
    Thursday,
    /// Friday
    Friday,
    /// Saturday
    Saturday,
}}

impl core::fmt::Display for Day {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        match self {{
            Day::Sunday => write!(f, "Sunday"),
            Day::Monday => write!(f, "Monday"),
            Day::Tuesday => write!(f, "Tuesday"),
            Day::Wednesday => write!(f, "Wednesday"),
            Day::Thursday => write!(f, "Thursday"),
            Day::Friday => write!(f, "Friday"),
            Day::Saturday => write!(f, "Saturday"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl serde::Serialize for Day {{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        match self {{
            Day::Sunday => serializer.serialize_str("Sunday"),
            Day::Monday => serializer.serialize_str("Monday"),
            Day::Tuesday => serializer.serialize_str("Tuesday"),
            Day::Wednesday => serializer.serialize_str("Wednesday"),
            Day::Thursday => serializer.serialize_str("Thursday"),
            Day::Friday => serializer.serialize_str("Friday"),
            Day::Saturday => serializer.serialize_str("Saturday"),
        }}
    }}
}}

#[cfg(feature="serde")]
impl<'de> serde::Deserialize<'de> for Day {{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {{
        let s = <&'de str as serde::Deserialize<'de>>::deserialize(deserializer)?;
        match s {{
            "sunday" | "Sunday" | "Sun" | "sun" => Ok(Self::Sunday),
            "monday" | "Monday" | "Mon" | "mon" => Ok(Self::Monday),
            "tuesday" | "Tuesday" | "Tue" | "tue" => Ok(Self::Tuesday),
            "wednesday" | "Wednesday" | "Wed" | "wed" => Ok(Self::Wednesday),
            "thursday" | "Thursday" | "Thu" | "thu" => Ok(Self::Thursday),
            "friday" | "Friday" | "Fri" | "fri" => Ok(Self::Friday),
            "saturday" | "Saturday" | "Sat" | "sat" => Ok(Self::Saturday),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown day: {{}}",
                s
            ))),
        }}
    }}
}}


impl Day {{
    /// Returns short name of the day
    #[inline]
    pub const fn short(&self) -> &'static str {{
        match self {{
            Day::Sunday => "Sun",
            Day::Monday => "Mon",
            Day::Tuesday => "Tue",
            Day::Wednesday => "Wed",
            Day::Thursday => "Thu",
            Day::Friday => "Fri",
            Day::Saturday => "Sat",
        }}
    }}

    #[inline]
    pub const fn as_str(&self) -> &'static str {{
        match self {{
            Day::Sunday => "Sunday",
            Day::Monday => "Monday",
            Day::Tuesday => "Tuesday",
            Day::Wednesday => "Wednesday",
            Day::Thursday => "Thursday",
            Day::Friday => "Friday",
            Day::Saturday => "Saturday",
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
    week_starts_on: Day,
    #[serde(rename = "distanceMeasurement")]
    distance_uint: DistanceUint,
    #[serde(rename = "temperatureMeasurement")]
    temperature_uint: TemperatureUint,
}

impl ToValue for HashMap<String, String> {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(buf, "&crate::StaticMap::new(&[")?;
        for (key, value) in self {
            write!(buf, "(\"{}\", \"{}\"),", key, value)?;
        }
        write!(buf, "]),")?;
        Ok(())
    }
}

impl ToValue for Locale {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        writeln!(
            buf,
            "&Locale {{ ietf: {} date_formats: {} timezones: {}, measurement_system: {}, hour_clock: {}, driving_side: {}, distance_unit: {}, temperature_unit: {}, week_start_on: {}}},",
            self.ietf.to_value_string()?,
            self.date_formats.to_value_string()?,
            self.timezones.to_value_string()?,
            self.measurement_system.to_value_string()?,
            self.hour_clock.to_value_string()?,
            self.driving_side.to_value_string()?,
            self.distance_uint.to_value_string()?,
            self.temperature_uint.to_value_string()?,
            self.week_starts_on.to_value_string()?,
        )?;
        Ok(())
    }
}

impl ToTokenStream for Locale {
    fn to_token_stream<W: Write>(buf: &mut W) -> Result<()> {
        <Day as ToTokenStream>::to_token_stream(buf)?;
        <HourClock as ToTokenStream>::to_token_stream(buf)?;
        <DrivingSide as ToTokenStream>::to_token_stream(buf)?;
        <DistanceUint as ToTokenStream>::to_token_stream(buf)?;
        <TemperatureUint as ToTokenStream>::to_token_stream(buf)?;
        <MeasurementSystem as ToTokenStream>::to_token_stream(buf)?;
        <Timezone as ToTokenStream>::to_token_stream(buf)?;

        static FIELDS: &[StructField] = &[
            StructField {
                name: "ietf",
                ty: "&'static [&'static str]",
                doc: "/// Returns the IETF locale code (e.g. `en-US`)",
                getter: "ietf",
            },
            StructField {
                name: "timezones",
                ty: "&'static [&'static Timezone]",
                doc: r"/// Returns the list of [tz database timezones]
    /// 
    /// [tz database timezones]: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones",
                getter: "timezones",
            },
            StructField {
                name: "date_formats",
                ty: "&'static crate::StaticMap<&'static str, &'static str>",
                doc: r"
    /// Returns date formats for each IETF locale.
    /// 
    /// - Key is the IETF code
    /// - Value is the date format, where:
    ///   - `G` = era
    ///   - `y` = year
    ///   - `M` = month
    ///   - `d` = day ",
                getter: "date_formats",
            },
            StructField {
                name: "measurement_system",
                ty: "MeasurementSystem",
                doc: "/// Returns system of measurement in use. see [`MeasurementSystem`]",
                getter: "measurement_system",
            },
            StructField {
                name: "hour_clock",
                ty: "HourClock",
                doc: "/// Returns the type of clock used. see [`HourClock`]",
                getter: "hour_clock",
            },
            StructField {
                name: "driving_side",
                ty: "DrivingSide",
                doc: "/// Returns the side of the road traffic drives on. see [`DrivingSide`]",
                getter: "driving_side",
            },
            StructField {
                name: "distance_unit",
                ty: "DistanceUint",
                doc: "/// Returns the unit of distance used (kilometer or mile). see [`DistanceUint`]",
                getter: "distance_unit",
            },
            StructField {
                name: "temperature_unit",
                ty: "TemperatureUint",
                doc: "/// Returns the unit of temperature (celsius or fahrenheit). see [`TemperatureUint`]",
                getter: "temperature_unit",
            },
            StructField {
                name: "week_start_on",
                ty: "Day",
                doc: "/// Returns which day is the first day of the week on the calendar. see [`Day`]",
                getter: "week_start_on",
            },
        ];

        Struct::new(
            "Locale",
            r"
/// Locale",
            FIELDS,
        )
        .render(buf)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CountryName {
    common: String,
    official: String,
}

impl ToTokenStream for CountryName {
    fn to_token_stream<W: Write>(buf: &mut W) -> Result<()> {
        static FIELDS: &[StructField] = &[
            StructField {
                name: "common",
                ty: "&'static str",
                doc: "/// Returns the common name of the country",
                getter: "common",
            },
            StructField {
                name: "official",
                ty: "&'static str",
                doc: "/// Returns the official name of the country",
                getter: "official",
            },
        ];

        Struct::new("CountryName", r"", FIELDS).render(buf)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CountryMeta {
    common: String,
    official: String,
    native: Option<HashMap<String, CountryName>>,
    alternates: Option<Vec<String>>,
}

impl ToValue for CountryMeta {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        writeln!(buf, "&CountryMeta {{")?;
        writeln!(buf, "\t\tcommon: \"{}\",", self.common)?;
        writeln!(buf, "\t\tofficial: \"{}\",", self.official)?;
        if let Some(natives) = &self.native {
            let mut val = String::from("&[");
            for (k, v) in natives {
                val.push_str(&format!(
                    "(\"{}\", &CountryName {{ common: \"{}\", official: \"{}\" }}), ",
                    k, v.common, v.official
                ));
            }
            val.push(']');
            writeln!(buf, "\t\tnative: &crate::StaticMap::new({}),", val)?;
        } else {
            writeln!(buf, "\t\tnative: &crate::StaticMap::new(&[]),")?;
        }

        if let Some(alternates) = &self.alternates {
            let mut val = String::from("&[");
            for alternate in alternates {
                val.push_str(&format!("\"{}\",", alternate));
            }
            val.push(']');
            writeln!(buf, "\t\talternates: {},", val)?;
        } else {
            writeln!(buf, "\t\talternates: &[],")?;
        }
        writeln!(buf, "\t}},")?;
        Ok(())
    }
}

impl ToTokenStream for CountryMeta {
    fn to_token_stream<W: Write>(buf: &mut W) -> Result<()> {
        <CountryName as ToTokenStream>::to_token_stream(buf)?;
        static FIELDS: &[StructField] = &[
            StructField {
                name: "common",
                ty: "&'static str",
                doc: "/// Returns the common name of the country",
                getter: "common",
            },
            StructField {
                name: "official",
                ty: "&'static str",
                doc: "/// Returns the official name of the country",
                getter: "official",
            },
            StructField {
                name: "native",
                ty: "&'static crate::StaticMap<&'static str, &'static CountryName>",
                doc: "/// Returns the name of the country in native languages",
                getter: "native",
            },
            StructField {
                name: "alternates",
                ty: "&'static [&'static str]",
                doc: "/// Returns the alternate names of the country",
                getter: "alternates",
            },
        ];

        Struct::new("CountryMeta", r"", FIELDS).render(buf)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum SubdivisionOfficialName {
    Number(usize),
    String(String),
}

impl ToValue for SubdivisionOfficialName {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        match self {
            SubdivisionOfficialName::Number(n) => write!(buf, "{n}")?,
            SubdivisionOfficialName::String(s) => write!(buf, "{s}")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SubdivisionName {
    official: SubdivisionOfficialName,
    common: Option<String>,
    native: Option<String>,
}

impl ToValue for SubdivisionName {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(
            buf,
            "&SubdivisionMeta {{ official: \"{}\", common: {}, native: {} }},",
            self.official.to_value_string()?,
            self.common.to_value_string()?,
            self.native.to_value_string()?,
        )?;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct IDD {
    prefix: Option<String>,
    suffixes: Option<Vec<String>>,
}

impl ToValue for IDD {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        writeln!(
            buf,
            "&IDD {{ prefix: \"{}\", suffixes: {} }},",
            self.prefix.as_ref().unwrap_or(&"".to_string()),
            self.suffixes
                .as_ref()
                .map(|v| { v.to_value_string().unwrap() })
                .unwrap_or_else(|| "&[]".to_string())
        )?;

        Ok(())
    }
}

impl ToTokenStream for IDD {
    fn to_token_stream<W: Write>(buf: &mut W) -> Result<()> {
        static FIELDS: &[StructField] = &[
            StructField {
                name: "prefix",
                ty: "&'static str",
                doc: "/// Returns the geographical code prefix (e.g. +1 for US)",
                getter: "prefix",
            },
            StructField {
                name: "suffixes",
                ty: "&'static [&'static str]",
                doc: "/// Returns the list of suffixes assigned (e.g. 201 in US)",
                getter: "suffixes",
            },
        ];

        Struct::new(
            "IDD",
            r"
/// [International dialing direct] info.
/// 
/// [International dialing direct]: https://en.wikipedia.org/wiki/List_of_country_calling_codes",
            FIELDS,
        )
        .render(buf)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Subdivision {
    #[serde(rename = "isoCode")]
    iso_code: String,
    #[serde(rename = "type")]
    ty: Option<String>,
    name: HashMap<String, SubdivisionName>,
}

impl ToValue for HashMap<String, SubdivisionName> {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(buf, "&crate::StaticMap::new(&[")?;
        for (k, v) in self {
            write!(buf, "(\"{}\", {}),", k, v.to_value_string()?)?;
        }
        write!(buf, "]),")?;
        Ok(())
    }
}

impl ToValue for Vec<Subdivision> {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(buf, "&[")?;
        for v in self {
            write!(buf, "{}", v.to_value_string()?)?;
        }
        writeln!(buf, "],")?;
        Ok(())
    }
}

impl ToValue for Subdivision {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(
            buf,
            "&Subdivision {{ iso: \"{}\", ty: {}, meta: {} }},",
            self.iso_code,
            self.ty.to_value_string()?,
            self.name.to_value_string()?,
        )?;
        Ok(())
    }
}

impl ToTokenStream for Subdivision {
    fn to_token_stream<W: Write>(out: &mut W) -> Result<()> {
        static META_FIELDS: &[StructField] = &[
            StructField {
                name: "official",
                doc: "/// Returns the official name of the subdivision",
                ty: "&'static str",
                getter: "official",
            },
            StructField {
                name: "common",
                doc: "/// Returns the common name of the subdivision",
                ty: "Option<&'static str>",
                getter: "common",
            },
            StructField {
                name: "native",
                doc: "/// Returns the native name of the subdivision",
                ty: "Option<&'static str>",
                getter: "native",
            },
        ];

        Struct::new("SubdivisionMeta", "", META_FIELDS).render(out)?;

        static FIELDS: &[StructField] = &[
            StructField {
                name: "iso",
                doc: r"/// Returns the [ISO 3166-2 code] of the subdivision
    ///
    /// [ISO 3166-2]: https://en.wikipedia.org/wiki/ISO_3166-2",
                ty: "&'static str",
                getter: "iso_code",
            },
            StructField {
                name: "ty",
                doc: "/// Returns the type of the subdivision",
                ty: "Option<&'static str>",
                getter: "subdivision_type",
            },
            StructField {
                name: "meta",
                doc: "/// Returns the meta of the subdivision",
                ty: "&'static crate::StaticMap<&'static str, &'static SubdivisionMeta>",
                getter: "meta",
            },
        ];

        Struct::new("Subdivision", "", FIELDS).render(out)
    }
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

trait ToValue {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()>;

    fn to_value_string(&self) -> Result<String> {
        let mut buf = io::BufWriter::new(Vec::new());
        self.to_value(&mut buf)?;
        Ok(String::from_utf8(buf.into_inner()?)?)
    }
}

impl ToValue for Vec<String> {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        write!(buf, "&[")?;
        for item in self.iter() {
            write!(buf, "\"{}\",", item)?;
        }
        write!(buf, "],")?;
        Ok(())
    }
}

impl ToTokenStream for CountryData {
    fn to_token_stream<W: Write>(buf: &mut W) -> Result<()> {
        <CountryMeta as ToTokenStream>::to_token_stream(buf)?;
        static FIELDS: &[StructField] = &[
            StructField {
                name: "name",
                doc: "/// Returns the name metadata of the country",
                ty: "&'static CountryMeta",
                getter: "name",
            },
            StructField {
                name: "flag",
                ty: "&'static str",
                doc: r"
    /// Returns the country's flag",
                getter: "flag",
            },
            StructField {
                name: "cca2",
                ty: "&'static str",
                doc: r"
    /// Returns [ISO 3166-1 alpha-2] code.
    /// 
    /// [ISO 3166-1 alpha-2]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2",
                getter: "cca2",
            },
            StructField {
                name: "cca3",
                ty: "&'static str",
                doc: r"
    /// Returns [ISO 3166-1 alpha-3] code.
    /// 
    /// [ISO 3166-1 alpha-3]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3",
                getter: "cca3",
            },
            StructField {
                name: "ccn3",
                ty: "&'static str",
                doc: r"
    /// Returns [ISO 3166-1 numeric] code.
    /// 
    /// [ISO 3166-1 numeric]: https://en.wikipedia.org/wiki/ISO_3166-1_numeric",
                getter: "ccn3",
            },
            StructField {
                name: "ioc",
                ty: "Option<&'static str>",
                doc: r"
    /// Returns [International Olympic Committee] code.
    /// 
    /// [International Olympic Committee]: https://en.wikipedia.org/wiki/International_Olympic_Committee",
                getter: "ioc",
            },
            StructField {
                name: "tld",
                ty: "&'static [&'static str]",
                doc: r"
    /// Returns list of [Country Code Top Level Domain (ccTLD)] used
    /// 
    /// [Country Code Top Level Domain (ccTLD)]: https://en.wikipedia.org/wiki/Country_code_top-level_domain#Lists",
                getter: "tld",
            },
            StructField {
                name: "locale",
                ty: "&'static Locale",
                doc: r"
    /// Returns the country's locale information",
                getter: "locale",
            },
            StructField {
                name: "idd",
                ty: "&'static IDD",
                doc: r"
    /// Returns the country's [international dialing direct] information
    /// 
    /// [international dialing direct]: https://en.wikipedia.org/wiki/List_of_country_calling_codes",
                getter: "idd",
            },
            StructField {
                name: "geography",
                ty: "&'static Geography",
                doc: r"
    /// Returns the country's geographical information",
                getter: "geography",
            },
            StructField {
                name: "official_languages",
                ty: "&'static [&'static Language]",
                doc: r"
    /// Returns the country's official languages information",
                getter: "official_languages",
            },
            StructField {
                name: "spoken_languages",
                ty: "&'static [&'static str]",
                doc: r"
    /// Returns the country's spoken language codes",
                getter: "spoken_languages",
            },
            StructField {
                name: "currencies",
                ty: "&'static [&'static Currency]",
                doc: r"
    /// Returns the list of currencies used in the country",
                getter: "currencies",
            },
            StructField {
                name: "subdivisions",
                ty: "&'static [&'static Subdivision]",
                doc: r"
    /// Returns the subdivisions (states, provinces, etc.) map whose key is [ISO 639-3] in the country
    ///
    /// [ISO 639-3]: https://en.wikipedia.org/wiki/ISO_639-3",
                getter: "subdivisions",
            },
        ];

        Struct::with_derives(
            "Country",
            "",
            FIELDS,
            "#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]",
        )
        .render(buf)
    }
}

impl ToValue for CountryData {
    fn to_value<W: Write>(&self, buf: &mut W) -> Result<()> {
        writeln!(buf, "&Country {{")?;
        write!(buf, "\tname: ")?;
        self.name.to_value(buf)?;
        writeln!(buf, "\tflag: \"{}\",", self.flag)?;
        writeln!(buf, "\tcca2: \"{}\",", self.cca2)?;
        writeln!(buf, "\tcca3: \"{}\",", self.cca3)?;
        writeln!(buf, "\tccn3: \"{}\",", self.ccn3)?;
        if let Some(ioc) = &self.ioc {
            writeln!(buf, "\tioc: Some(\"{}\"),", ioc)?;
        } else {
            writeln!(buf, "\tioc: None,")?;
        }
        write!(buf, "\ttld: ")?;
        self.tld.to_value(buf)?;
        write!(buf, "\n\tidd: ")?;
        self.idd.to_value(buf)?;
        write!(buf, "\tgeography: ")?;
        self.geography.to_value(buf)?;
        write!(buf, "\tofficial_languages: ")?;
        self.languages.official.to_value(buf)?;
        writeln!(
            buf,
            "\tspoken_languages: {}",
            self.languages
                .spoken
                .as_ref()
                .map(|s| s.to_value_string().unwrap())
                .unwrap_or_else(|| "&[],".to_string())
        )?;
        write!(buf, "\tlocale: ")?;
        self.locale.to_value(buf)?;
        write!(buf, "\tcurrencies: ")?;
        self.currencies.to_value(buf)?;
        write!(buf, "\tsubdivisions: ")?;
        self.subdivisions.to_value(buf)?;
        writeln!(buf, "}};\n")?;
        Ok(())
    }
}

trait Generator: Write {
    fn eof(&mut self) -> Result<()> {
        writeln!(self)?;
        Ok(())
    }
}

impl<W: Write> Generator for W {}

fn generate_enums(src: &[CountryData]) -> Result<()> {
    let out_path = PathBuf::from("src/enums.rs");
    let mut out = io::BufWriter::new(File::create(out_path)?);
    writeln!(out, "// Auto generated file, please do not modify \n\n")?;

    EnumDeref::new(
        "CCA2",
        2,
        src.iter()
            .map(|c| Variant {
                name: c.cca2.to_uppercase(),
                doc: c.name.official.clone(),
            })
            .collect(),
    )
    .render(&mut out)?;
    EnumDeref::new(
        "CCA3",
        3,
        src.iter()
            .map(|c| Variant {
                name: c.cca3.to_uppercase(),
                doc: c.name.official.clone(),
            })
            .collect(),
    )
    .render(&mut out)?;

    out.eof()?;
    Ok(())
}

fn generate_types() -> Result<()> {
    let out_path = PathBuf::from("src/types.rs");
    let mut out = io::BufWriter::new(File::create(out_path)?);
    writeln!(out, "// Auto generated file, please do not modify \n\n")?;

    <Locale as ToTokenStream>::to_token_stream(&mut out)?;
    <IDD as ToTokenStream>::to_token_stream(&mut out)?;
    <Geography as ToTokenStream>::to_token_stream(&mut out)?;
    <Currency as ToTokenStream>::to_token_stream(&mut out)?;
    <Subdivision as ToTokenStream>::to_token_stream(&mut out)?;
    <Language as ToTokenStream>::to_token_stream(&mut out)?;
    <CountryData as ToTokenStream>::to_token_stream(&mut out)?;
    Ok(())
}

fn generate_consts(data: &[CountryData]) -> Result<()> {
    let out_path = PathBuf::from("src/consts.rs");
    let mut out = io::BufWriter::new(File::create(out_path)?);
    writeln!(out, "// Auto generated file, please do not modify \n\n")?;
    writeln!(out, "use super::types::*;")?;

    for country in data {
        writeln!(out, "/// {}", country.name.official)?;
        write!(
            out,
            "pub const {}: &Country = ",
            country.cca2.to_uppercase()
        )?;
        country.to_value(&mut out)?;
        writeln!(out)?;
        writeln!(out, "/// {}", country.name.official)?;
        writeln!(
            out,
            "pub const {}: &Country = {};",
            country.cca3.to_uppercase(),
            country.cca2.to_uppercase()
        )?;
    }

    Ok(())
}

fn main() -> Result<()> {
    eprintln!("start build");
    let src_path = PathBuf::from("data.json");

    let src = File::open(src_path)?;
    let countries: Vec<CountryData> = serde_json::from_reader(src)?;
    generate_enums(&countries)?;
    generate_types()?;
    generate_consts(&countries)?;

    Ok(())
}
