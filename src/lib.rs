#![cfg_attr(not(feature = "std"), no_std)]

mod generated;
use core::borrow::Borrow;

pub use generated::*;
#[cfg(feature = "enums")]
pub mod enums;
#[cfg(feature = "enums")]
pub use enums::{CCA2, CCA3};

mod types;
pub use types::*;

// #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
// pub enum ParseCountryError {
//     UnknownCountry,
//     UnknownAlpha2,
//     UnknownAlpha3,
//     UnknownCountryCode,
//     Unkonwn3166_2,
// }

// impl core::fmt::Display for ParseCountryError {
//     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//         match self {
//             ParseCountryError::UnknownCountry => write!(f, "unknown country name"),
//             ParseCountryError::UnknownAlpha2 => write!(f, "unknown country alpha2 code"),
//             ParseCountryError::UnknownAlpha3 => write!(f, "unknown country alpha3 code"),
//             ParseCountryError::UnknownCountryCode => write!(f, "unknown country code"),
//             ParseCountryError::Unkonwn3166_2 => write!(f, "unknown ISO 3166-2"),
//         }
//     }
// }

// #[cfg(feature = "std")]
// impl std::error::Error for ParseCountryError {}

// impl core::fmt::Display for Country {
//     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//         write!(f, "{}", self.name())
//     }
// }

// impl core::str::FromStr for Country {
//     type Err = ParseCountryError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Country::from_alpha2(s)
//             .or_else(|_| Country::from_alpha3(s))
//             .or_else(|_| Country::from_iso(s))
//             .or_else(|_| Country::from_country_code_str(s))
//             .or_else(|_| Country::from_name(s))
//     }
// }

// impl core::convert::TryFrom<String> for Country {
//     type Error = ParseCountryError;

//     fn try_from(s: String) -> Result<Self, Self::Error> {
//         <Country as core::str::FromStr>::from_str(&s)
//     }
// }

// #[cfg(feature = "serde")]
// impl serde::Serialize for Country {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_str(self.alpha2())
//     }
// }

// #[cfg(feature = "serde")]
// impl<'de> serde::Deserialize<'de> for Country {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         struct CountryVisitor;

//         impl<'de> serde::de::Visitor<'de> for CountryVisitor {
//             type Value = Country;

//             fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
//                 formatter.write_str("a ISO2")
//             }

//             fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
//             where
//                 E: serde::de::Error,
//             {
//                 Country::from_alpha2(value).map_err(|e| E::custom(e))
//             }
//         }

//         deserializer.deserialize_str(CountryVisitor)
//     }
// }

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StaticMap<K: 'static, V: 'static> {
    map: &'static [(K, V)],
}

impl<K: 'static, V: 'static> Clone for StaticMap<K, V> {
    fn clone(&self) -> Self {
        Self { map: self.map }
    }
}

impl<K: 'static, V: 'static> Copy for StaticMap<K, V> {}

impl<K: 'static + Eq, V: 'static> core::ops::Index<K> for StaticMap<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.map
            .iter()
            .find(|(k, _)| k.eq(&index))
            .map(|(_, v)| v)
            .expect("key not found")
    }
}

impl<K: 'static + Eq, V: 'static> core::ops::Index<&K> for StaticMap<K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.map
            .iter()
            .find(|(k, _)| k.eq(index))
            .map(|(_, v)| v)
            .expect("key not found")
    }
}

impl<K: 'static, V: 'static> StaticMap<K, V> {
    #[inline]
    pub const fn new(map: &'static [(K, V)]) -> Self {
        Self { map }
    }

    pub fn iter(&self) -> core::slice::Iter<'_, (K, V)> {
        self.map.iter()
    }
}

impl<K: 'static + Eq, V: 'static> StaticMap<K, V> {
    pub fn get<Q>(&self, k: &Q) -> Option<&'static V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        self.map
            .iter()
            .find(|(key, _)| key.borrow().eq(k))
            .map(|(_, v)| v)
    }

    pub fn contains<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq,
    {
        self.map.iter().any(|(key, _)| key.borrow().eq(k))
    }
}

impl<K: 'static, V: 'static> core::iter::IntoIterator for StaticMap<K, V> {
    type Item = &'static (K, V);

    type IntoIter = core::slice::Iter<'static, (K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct NameMeta {
    common: &'static str,
    official: &'static str,
}

impl NameMeta {
    /// Returns the common name (in English)
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.common
    }

    /// Returns officially recognized name (in English)
    #[inline]
    pub const fn official(&self) -> &'static str {
        self.official
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct OfficialLanguage {
    name: &'static str,
    native: &'static str,
    iso_639_3: &'static str,
    bcp_47: &'static str,
    iso_15924: &'static str,
    iana: &'static [&'static str],
    extinct: bool,
    spurious: bool,
}

impl OfficialLanguage {
    /// Returns the name of the language in English
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the name of the language in that language
    #[inline]
    pub const fn native(&self) -> &'static str {
        self.native
    }

    /// Returns the [ISO 639-3] language code.
    ///
    /// [ISO 639-3]: https://en.wikipedia.org/wiki/ISO_639-3
    #[inline]
    pub const fn iso639_3(&self) -> &'static str {
        self.iso_639_3
    }

    /// Returns the [BCP 47] tag.
    ///
    /// [BCP 47]: https://en.wikipedia.org/wiki/IETF_language_tag
    #[inline]
    pub const fn bcp47(&self) -> &'static str {
        self.bcp_47
    }

    /// Returns the [ISO 15924] script code.
    ///
    /// [ISO 15924]: https://en.wikipedia.org/wiki/ISO_15924
    #[inline]
    pub const fn iso15924(&self) -> &'static str {
        self.iso_15924
    }

    /// Returns array of assigned [IANA] tags.
    ///
    /// [IANA]: https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
    // TODO: add IANA struct which contains the information, and replace str with that struct
    #[inline]
    pub const fn iana(&self) -> &'static [&'static str] {
        self.iana
    }

    /// Returns true if the language is extinct
    #[inline]
    pub const fn is_extinct(&self) -> bool {
        self.extinct
    }

    /// Returns true if the language is spurious
    #[inline]
    pub const fn is_spurious(&self) -> bool {
        self.spurious
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Language {
    official: &'static [&'static OfficialLanguage],
    spoken: &'static [&'static str],
}

impl Language {
    /// Returns array of official languages
    #[inline]
    pub const fn official(&self) -> &'static [&'static OfficialLanguage] {
        self.official
    }

    /// Returns array of spoken languages
    #[inline]
    pub const fn spoken(&self) -> &'static [&'static str] {
        self.spoken
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Geography {
    lat: f64,
    long: f64,
    land_locked: bool,
    capital: &'static str,
    area: usize,
    region: &'static str,
    subregion: &'static str,
    borders_cca2: &'static [CCA2],
    borders_cca3: &'static [CCA3],
}

impl Geography {
    /// Returns the latitude of the country
    #[inline]
    pub const fn latitude(&self) -> f64 {
        self.lat
    }

    /// Returns the longitude of the country
    #[inline]
    pub const fn longitude(&self) -> f64 {
        self.long
    }

    /// Returns whether or not the country is landlocked (not bordering the ocean)
    #[inline]
    pub const fn is_landlocked(&self) -> bool {
        self.land_locked
    }

    /// Returns the name of the capital city
    #[inline]
    pub const fn capital(&self) -> &'static str {
        self.capital
    }

    /// Returns size of the country in km<sup>2</sup>
    #[inline]
    pub const fn area(&self) -> usize {
        self.area
    }

    /// Returns the region of the country
    #[inline]
    pub const fn region(&self) -> &'static str {
        self.region
    }

    /// Returns the subregion of the country
    #[inline]
    pub const fn subregion(&self) -> &'static str {
        self.subregion
    }

    /// Returns list of countries by their [ISO 3166-1 alpha-2] codes that border the country
    ///
    /// [ISO 3166-1 alpha-2]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[inline]
    pub const fn borders_cca2(&self) -> &'static [CCA2] {
        self.borders_cca2
    }

    /// Returns list of countries by their [ISO 3166-1 alpha-3] codes that border the country
    ///
    /// [ISO 3166-1 alpha-3]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3
    #[inline]
    pub const fn borders_cca3(&self) -> &'static [CCA3] {
        self.borders_cca3
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Currency {
    name: &'static str,
    short_name: &'static str,
    iso_4217: &'static str,
    iso_numeric: Option<u16>,
    symbol: &'static str,
    subunit: Option<&'static str>,
    prefix: Option<&'static str>,
    suffix: Option<&'static str>,
    decimal_mark: Option<char>,
    decimal_places: u8,
    thousands_separator: Option<char>,
}

impl Currency {
    /// Returns the official currency name (in English)
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the name of the currency itself (e.g. 'dollar' as opposed to 'US Dollar')
    #[inline]
    pub const fn short_name(&self) -> &'static str {
        self.short_name
    }

    /// Returns the [ISO 4217] currency code
    ///
    /// [ISO 4217]: https://en.wikipedia.org/wiki/ISO_4217
    #[inline]
    pub const fn iso(&self) -> &'static str {
        self.iso_4217
    }

    /// Returns the [ISO 4217 numeric] currency code
    ///
    /// [ISO 4217 numeric]: https://en.wikipedia.org/wiki/ISO_4217#cite_note-ISO4217-1
    #[inline]
    pub const fn iso_numeric(&self) -> Option<u16> {
        self.iso_numeric
    }

    /// Returns the unicode symbol (e.g. '$')
    #[inline]
    pub const fn symbol(&self) -> &'static str {
        self.symbol
    }

    /// Returns the subunit to whole value (e.g. 'cent')
    #[inline]
    pub const fn subunit(&self) -> Option<&'static str> {
        self.subunit
    }

    /// Returns the symbol that prefixes a currency amount (e.g. '$1')
    #[inline]
    pub const fn prefix(&self) -> Option<&'static str> {
        self.prefix
    }

    /// Returns the symbol that suffixes a currency amount (e.g. '1€')
    #[inline]
    pub const fn suffix(&self) -> Option<&'static str> {
        self.suffix
    }

    /// Returns the symbol that denotes a decimal place
    #[inline]
    pub const fn decimal_mark(&self) -> Option<char> {
        self.decimal_mark
    }

    /// Returns the number of decimal places rounded to
    #[inline]
    pub const fn decimal_places(&self) -> u8 {
        self.decimal_places
    }

    /// Returns the symbol that denotes a thousands separator
    #[inline]
    pub const fn thousands_separator(&self) -> Option<char> {
        self.thousands_separator
    }
}

/// [International dialing direct] info.
///
/// [International dialing direct]: https://en.wikipedia.org/wiki/List_of_country_calling_codes
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct IDD {
    prefix: &'static str,
    suffixes: &'static [&'static str],
}

impl IDD {
    /// Returns the geographical code prefix (e.g. +1 for US)
    #[inline]
    pub const fn prefix(&self) -> &'static str {
        self.prefix
    }

    /// Returns the list of suffixes assigned (e.g. 201 in US)
    #[inline]
    pub const fn suffixes(&self) -> &'static [&'static str] {
        self.suffixes
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct SubdivisionName {
    official: &'static str,
    common: Option<&'static str>,
    native: Option<&'static str>,
}

impl SubdivisionName {
    /// Returns the official name of the subdivision
    #[inline]
    pub const fn official(&self) -> &'static str {
        self.official
    }

    /// Returns the locally used name variant
    #[inline]
    pub const fn common(&self) -> Option<&'static str> {
        self.common
    }

    /// Returns the official name in non-alphanumeric script language (e.g. arabic, cyrillic)
    #[inline]
    pub const fn native(&self) -> Option<&'static str> {
        self.native
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Subdivision {
    iso_code: &'static str,
    ty: &'static str,
    name: &'static StaticMap<&'static str, SubdivisionName>,
}

impl Subdivision {
    /// Returns the subdivision type
    #[inline]
    pub const fn subdivision_type(&self) -> &'static str {
        self.ty
    }

    /// Returns the [ISO 3166-2] code of the subdivision
    ///
    /// [ISO 3166-2]: https://en.wikipedia.org/wiki/ISO_3166-2
    #[inline]
    pub const fn iso_code(&self) -> &'static str {
        self.iso_code
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum TimezoneType {
    Link,
    Canonical,
}

impl core::fmt::Display for TimezoneType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TimezoneType::Link => write!(f, "link"),
            TimezoneType::Canonical => write!(f, "canonical"),
        }
    }
}

/// Timezone info, reference: [tz database timezones].
///
/// [tz database timezones]: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Timezone {
    name: &'static str,
    ty: TimezoneType,
    linked_to: Option<&'static str>,
    utc_offset: &'static str,
    dst_offset: &'static str,
}

impl Timezone {
    /// Returns the name of the timezone
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the type of timezone (primary or alias)
    #[inline]
    pub const fn timezone_type(&self) -> TimezoneType {
        self.ty
    }

    /// Returns (if alias) the primary timezone this timezone links to
    #[inline]
    pub const fn linked_to(&self) -> Option<&'static str> {
        self.linked_to
    }

    /// Returns the hours offset from UTC
    #[inline]
    pub const fn utc_offset(&self) -> &'static str {
        self.utc_offset
    }

    /// Returns the hours offset from UTC during DST (if country doesn't observe DST, this is the same value as UTC offset)
    #[inline]
    pub const fn dst_offset(&self) -> &'static str {
        self.dst_offset
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

/// Driving side
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum DrivingSide {
    /// Left-hand side
    Left,
    /// Right-hand side
    Right,
}

/// The unit of distance used (kilometer or mile)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum DistanceUint {
    /// Kilometer
    Kilometer,
    /// Mile
    Mile,
}

/// The unit of temperature (celsius or fahrenheit)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum TemperatureUint {
    /// Celsius
    Celsius,
    /// Fahrenheit
    Fahrenheit,
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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Locale {
    ietf: &'static [&'static str],
    measurement_system: MeasurementSystem,
    driving_side: DrivingSide,
    hour_clock: HourClock,
    timezones: &'static [&'static Timezone],
    date_formats: &'static StaticMap<&'static str, &'static str>,
    week_starts_on: Day,
    distance_uint: DistanceUint,
    temperature_uint: TemperatureUint,
}

impl Locale {
    /// Returns a list of [IETF] locale codes (e.g. `en-US`)
    ///
    /// [IETF]: https://en.wikipedia.org/wiki/IETF_language_tag
    #[inline]
    pub const fn ietf(&self) -> &'static [&'static str] {
        self.ietf
    }

    /// Returns system of measurement in use
    #[inline]
    pub const fn measurement_system(&self) -> MeasurementSystem {
        self.measurement_system
    }

    /// Returns the side of the road traffic drives on
    #[inline]
    pub const fn driving_side(&self) -> DrivingSide {
        self.driving_side
    }

    /// Returns the type of clock used
    #[inline]
    pub const fn hour_clock(&self) -> HourClock {
        self.hour_clock
    }

    /// Returns the list of [tz database timezones](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)
    #[inline]
    pub const fn timezones(&self) -> &'static [&'static Timezone] {
        self.timezones
    }

    /// Returns date formats for each IETF locale.
    ///
    /// - Key is the IETF code
    /// - Value is the date format, where:
    ///   - `G` = era
    ///   - `y` = year
    ///   - `M` = month
    ///   - `d` = day  
    #[inline]
    pub const fn date_formats(&self) -> &'static StaticMap<&'static str, &'static str> {
        self.date_formats
    }

    /// Returns which day is the first day of the week on the calendar
    #[inline]
    pub const fn week_starts_on(&self) -> Day {
        self.week_starts_on
    }

    /// Returns the unit of distance used (kilometer or mile)
    #[inline]
    pub const fn distance_unit(&self) -> DistanceUint {
        self.distance_uint
    }

    /// Returns the unit of temperature (celsius or fahrenheit)
    #[inline]
    pub const fn temperature_unit(&self) -> TemperatureUint {
        self.temperature_uint
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Country1 {
    name: &'static NameMeta,
    flag: &'static str,
    cca2: &'static str,
    cca3: &'static str,
    ccn3: &'static str,
    ioc: &'static str,
    native: &'static StaticMap<&'static str, &'static NameMeta>,
    alternatives: &'static [&'static str],
    tld: &'static [&'static str],
    geography: &'static Geography,
    currencies: &'static [&'static Currency],
    subdivisions: &'static StaticMap<&'static str, &'static Subdivision>,
    idd: &'static IDD,
    locale: &'static Locale,
}

impl Country1 {
    /// Returns array of alternate name spellings
    #[inline]
    pub const fn alternates(&self) -> &'static [&'static str] {
        self.alternatives
    }

    /// Returns list of names in the country's recognized languages
    #[inline]
    pub const fn native(&self) -> &'static StaticMap<&'static str, &'static NameMeta> {
        self.native
    }

    /// Returns [ISO 3166-1 alpha-2] code.
    ///
    /// [ISO 3166-1 alpha-2]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[inline]
    pub const fn cca2(&self) -> &'static str {
        self.cca2
    }

    /// Returns [ISO 3166-1 alpha-3] code.
    ///
    /// [ISO 3166-1 alpha-3]: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3
    #[inline]
    pub const fn cca3(&self) -> &'static str {
        self.cca3
    }

    /// Returns [ISO 3166-1 numeric] code.
    ///
    /// [ISO 3166-1 numeric]: https://en.wikipedia.org/wiki/ISO_3166-1_numeric
    #[inline]
    pub const fn ccn3(&self) -> &'static str {
        self.ccn3
    }

    /// Returns [International Olympic Committee] code.
    ///
    /// [International Olympic Committee]: https://en.wikipedia.org/wiki/International_Olympic_Committee
    #[inline]
    pub const fn ioc(&self) -> &'static str {
        self.ioc
    }

    /// Returns list of [Country Code Top Level Domain (ccTLD)] used
    ///
    /// [Country Code Top Level Domain (ccTLD)]: https://en.wikipedia.org/wiki/Country_code_top-level_domain#Lists
    #[inline]
    pub const fn tld(&self) -> &'static [&'static str] {
        self.tld
    }

    /// Returns the country's flag
    #[inline]
    pub const fn flag(&self) -> &'static str {
        self.flag
    }

    /// Returns the country's geographical information
    #[inline]
    pub const fn geography(&self) -> &'static Geography {
        self.geography
    }

    /// Returns the list of currencies used in the country
    #[inline]
    pub const fn currencies(&self) -> &'static [&'static Currency] {
        self.currencies
    }

    /// Returns the subdivisions (states, provinces, etc.) map whose key is [ISO 639-3] in the country
    ///
    /// [ISO 639-3]: https://en.wikipedia.org/wiki/ISO_639-3
    #[inline]
    pub const fn subdivisions(&self) -> &'static StaticMap<&'static str, &'static Subdivision> {
        self.subdivisions
    }

    /// Returns the [International dialing direct] info.
    ///
    /// [International dialing direct]: https://en.wikipedia.org/wiki/List_of_country_calling_codes
    #[inline]
    pub const fn idd(&self) -> &'static IDD {
        self.idd
    }

    /// Returns the country's locale information
    #[inline]
    pub const fn locale(&self) -> &'static Locale {
        self.locale
    }
}

#[test]
fn test() {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::fs::File;

    #[derive(Serialize, Deserialize)]
    struct CountryData {
        alpha2: String,
        continent: Option<String>,
        emoji: Option<String>,
        emojiU: Option<String>,
        languages: Option<Vec<String>>,
        locales: Vec<String>,
        default_locale: String,
        currency_name: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Country1 {
        name: String,
        #[serde(rename = "alpha-2")]
        alpha2: String,
        #[serde(rename = "alpha-3")]
        alpha3: String,
        #[serde(rename = "country-code")]
        country_code: String,
        #[serde(rename = "phone-code")]
        phone_code: String,
        #[serde(rename = "currency-code")]
        currency_code: String,
        capital: String,
        iso: String,
        region: String,
        #[serde(rename = "sub-region")]
        sub_region: String,
        #[serde(rename = "intermediate-region")]
        intermediate_region: String,
        #[serde(rename = "region-code")]
        region_code: String,
        #[serde(rename = "sub-region-code")]
        sub_region_code: String,
        #[serde(rename = "intermediate-region-code")]
        intermediate_region_code: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Country {
        name: String,
        #[serde(rename = "alpha2")]
        alpha2: String,
        #[serde(rename = "alpha3")]
        alpha3: String,
        #[serde(rename = "country_code")]
        country_code: String,
        #[serde(rename = "phone_code")]
        phone_code: String,
        #[serde(rename = "currency_code")]
        currency_code: String,
        #[serde(rename = "currency_name")]
        currency_name: String,
        capital: String,
        iso: String,
        continent: String,
        region: String,
        #[serde(rename = "sub_region")]
        sub_region: String,
        #[serde(rename = "intermediate_region")]
        intermediate_region: String,
        #[serde(rename = "region_code")]
        region_code: String,
        #[serde(rename = "sub_region_code")]
        sub_region_code: String,
        #[serde(rename = "intermediate_region_code")]
        intermediate_region_code: String,
        emoji: String,
        #[serde(rename = "emoji_unicodes")]
        emoji_u: Vec<String>,
        languages: Vec<String>,
        default_language: String,
        locales: Vec<String>,
        #[serde(rename = "default_locale")]
        default_locale: String,
    }

    let src = File::open("countries.json").unwrap();
    let country_data = serde_json::from_reader::<_, Vec<CountryData>>(src)
        .unwrap()
        .into_iter()
        .map(|country| (country.alpha2.clone(), country))
        .collect::<HashMap<String, CountryData>>();
    eprintln!("country_data: {}", country_data.len());

    let src = File::open("data.json").unwrap();
    let countries: Vec<Country1> = serde_json::from_reader(src).unwrap();

    let countries = countries
        .into_iter()
        .map(|c| Country {
            name: c.name,
            alpha2: c.alpha2.clone(),
            alpha3: c.alpha3,
            country_code: c.country_code,
            phone_code: c.phone_code,
            currency_code: c.currency_code,
            capital: c.capital,
            iso: c.iso,
            region: c.region,
            sub_region: c.sub_region,
            intermediate_region: c.intermediate_region,
            region_code: c.region_code,
            sub_region_code: c.sub_region_code,
            intermediate_region_code: c.intermediate_region_code,
            currency_name: country_data.get(&c.alpha2).unwrap().currency_name.clone(),
            continent: country_data
                .get(&c.alpha2)
                .unwrap()
                .continent
                .clone()
                .unwrap_or_default(),
            emoji: country_data
                .get(&c.alpha2)
                .unwrap()
                .emoji
                .clone()
                .unwrap_or_default(),
            emoji_u: country_data
                .get(&c.alpha2)
                .unwrap()
                .emojiU
                .clone()
                .unwrap_or_default()
                .split(' ')
                .map(ToString::to_string)
                .collect(),
            languages: country_data
                .get(&c.alpha2)
                .unwrap()
                .languages
                .clone()
                .unwrap_or_default(),
            locales: country_data.get(&c.alpha2).unwrap().locales.clone(),
            default_locale: country_data.get(&c.alpha2).unwrap().default_locale.clone(),
            default_language: country_data
                .get(&c.alpha2)
                .unwrap()
                .languages
                .clone()
                .unwrap_or_default()[0]
                .clone(),
        })
        .collect::<Vec<_>>();

    let mut f = File::create("data1.json").unwrap();
    serde_json::to_writer_pretty(&mut f, &countries).unwrap();
}
