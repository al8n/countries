// Auto generated file, please do not modify

use super::StaticMap;

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

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
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

/// Driving side
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum DrivingSide {
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

#[cfg(feature = "serde")]
impl serde::Serialize for DrivingSide {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DrivingSide::Left => serializer.serialize_str("left"),
            DrivingSide::Right => serializer.serialize_str("right"),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DrivingSide {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).and_then(|s| match s.as_str() {
            "left" | "Left" | "l" | "L" => Ok(DrivingSide::Left),
            "right" | "Right" | "r" | "R" => Ok(DrivingSide::Right),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown driving side: {}",
                s
            ))),
        })
    }
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

impl core::fmt::Display for DistanceUint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DistanceUint::Kilometer => write!(f, "kilometer"),
            DistanceUint::Mile => write!(f, "mile"),
        }
    }
}

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DistanceUint {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).and_then(|s| match s.as_str() {
            "kilometer" | "km" | "Kilometer" | "Km" | "KM" => Ok(DistanceUint::Kilometer),
            "mile" | "mi" | "Mile" | "Mi" | "MI" => Ok(DistanceUint::Mile),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown distance unit: {}",
                s
            ))),
        })
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
    /// Mixed,
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

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
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

/// Locale
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Locale {
    ietf: &'static str,
    date_formats: &'static super::StaticMap<&'static str, &'static str>,
    measurement_system: MeasurementSystem,
    hour_clock: HourClock,
    driving_side: DrivingSide,
    distance_uint: DistanceUint,
    temperature_uint: TemperatureUint,
    week_start_on: Day,
}

impl Locale {
    /// Returns the IETF locale code (e.g. `en-US`)
    #[inline]
    pub const fn ietf(&self) -> &'static str {
        self.ietf
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
    pub const fn date_formats(&self) -> &'static super::StaticMap<&'static str, &'static str> {
        self.date_formats
    }

    /// Returns system of measurement in use. see [`MeasurementSystem`]
    #[inline]
    pub const fn measurement_system(&self) -> MeasurementSystem {
        self.measurement_system
    }

    /// Returns the type of clock used. see [`HourClock`]
    #[inline]
    pub const fn hour_clock(&self) -> HourClock {
        self.hour_clock
    }

    /// Returns the side of the road traffic drives on. see [`DrivingSide`]
    #[inline]
    pub const fn driving_side(&self) -> DrivingSide {
        self.driving_side
    }

    /// Returns the unit of distance used (kilometer or mile). see [`DistanceUint`]
    #[inline]
    pub const fn distance_uint(&self) -> DistanceUint {
        self.distance_uint
    }

    /// Returns the unit of temperature (celsius or fahrenheit). see [`TemperatureUint`]
    #[inline]
    pub const fn temperature_uint(&self) -> TemperatureUint {
        self.temperature_uint
    }

    /// Returns which day is the first day of the week on the calendar. see [`Day`]
    #[inline]
    pub const fn week_start_on(&self) -> Day {
        self.week_start_on
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

#[cfg(feature = "serde")]
impl serde::Serialize for TimezoneType {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TimezoneType::Link => serializer.serialize_str("link"),
            TimezoneType::Canonical => serializer.serialize_str("canonical"),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TimezoneType {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).and_then(|s| match s.as_str() {
            "link" | "Link" => Ok(TimezoneType::Link),
            "canonical" | "Canonical" => Ok(TimezoneType::Canonical),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown timezone type: {}",
                s
            ))),
        })
    }
}

/// Timezone info, reference: [tz database timezones].
///
/// [tz database timezones]: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    /// Returns the name of the timezone this timezone is linked to
    #[inline]
    pub const fn linked_to(&self) -> Option<&'static str> {
        self.linked_to
    }

    /// Returns the UTC offset of the timezone
    #[inline]
    pub const fn utc_offset(&self) -> &'static str {
        self.utc_offset
    }

    /// Returns the DST offset of the timezone
    #[inline]
    pub const fn dst_offset(&self) -> &'static str {
        self.dst_offset
    }
}

/// [International dialing direct] info.
///
/// [International dialing direct]: https://en.wikipedia.org/wiki/List_of_country_calling_codes
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Geography {
    latitude: f64,
    longitude: f64,
    land_locked: bool,
    capital: &'static [&'static str],
    area: f64,
    region: &'static str,
    subregion: &'static str,
    border_countries: &'static [super::CCA2],
}

impl Geography {
    /// Returns the latitude
    #[inline]
    pub const fn latitude(&self) -> f64 {
        self.latitude
    }

    /// Returns the longitude
    #[inline]
    pub const fn longitude(&self) -> f64 {
        self.longitude
    }

    /// Returns whether or not the country is landlocked (not bordering the ocean)
    #[inline]
    pub const fn is_landlocked(&self) -> bool {
        self.land_locked
    }

    /// Returns the name of the capital cities
    #[inline]
    pub const fn capitals(&self) -> &'static [&'static str] {
        self.capital
    }

    /// Returns the land area of the country
    #[inline]
    pub const fn area(&self) -> f64 {
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
    pub const fn border_countries(&self) -> &'static [super::CCA2] {
        self.border_countries
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Currency {
    name: &'static str,
    short_name: Option<&'static str>,
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
    /// Returns the name of the currency
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the short name of the currency
    #[inline]
    pub const fn short_name(&self) -> Option<&'static str> {
        self.short_name
    }

    /// Returns the [ISO 4217] currency code
    ///
    /// [ISO 4217]: https://en.wikipedia.org/wiki/ISO_4217
    #[inline]
    pub const fn iso4217(&self) -> &'static str {
        self.iso_4217
    }

    /// Returns the [ISO 4217 numeric] currency code
    ///
    /// [ISO 4217 numeric]: https://en.wikipedia.org/wiki/ISO_4217#cite_note-ISO4217-1
    #[inline]
    pub const fn iso_numeric(&self) -> Option<u16> {
        self.iso_numeric
    }

    /// Returns the currency symbol
    #[inline]
    pub const fn symbol(&self) -> &'static str {
        self.symbol
    }

    /// Returns the name of the subunit of the currency
    #[inline]
    pub const fn subunit(&self) -> Option<&'static str> {
        self.subunit
    }

    /// Returns the prefix of the currency symbol
    #[inline]
    pub const fn prefix(&self) -> Option<&'static str> {
        self.prefix
    }

    /// Returns the suffix of the currency symbol
    #[inline]
    pub const fn suffix(&self) -> Option<&'static str> {
        self.suffix
    }

    /// Returns the decimal mark of the currency
    #[inline]
    pub const fn decimal_mark(&self) -> Option<char> {
        self.decimal_mark
    }

    /// Returns the number of decimal places of the currency
    #[inline]
    pub const fn decimal_places(&self) -> u8 {
        self.decimal_places
    }

    /// Returns the thousands separator of the currency
    #[inline]
    pub const fn thousands_separator(&self) -> Option<char> {
        self.thousands_separator
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubdivisionMeta {
    official: &'static str,
    common: Option<&'static str>,
    native: Option<&'static str>,
}

impl SubdivisionMeta {
    /// Returns the official name of the subdivision
    #[inline]
    pub const fn official(&self) -> &'static str {
        self.official
    }

    /// Returns the common name of the subdivision
    #[inline]
    pub const fn common(&self) -> Option<&'static str> {
        self.common
    }

    /// Returns the native name of the subdivision
    #[inline]
    pub const fn native(&self) -> Option<&'static str> {
        self.native
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Subdivision {
    iso: &'static str,
    ty: &'static str,
    meta: &'static super::StaticMap<&'static str, SubdivisionMeta>,
}

impl Subdivision {
    /// Returns the [ISO 3166-2 code] of the subdivision
    ///
    /// [ISO 3166-2]: https://en.wikipedia.org/wiki/ISO_3166-2
    #[inline]
    pub const fn iso_code(&self) -> &'static str {
        self.iso
    }

    /// Returns the type of the subdivision
    #[inline]
    pub const fn subdivision_type(&self) -> &'static str {
        self.ty
    }

    /// Returns the meta of the subdivision
    #[inline]
    pub const fn meta(&self) -> &'static super::StaticMap<&'static str, SubdivisionMeta> {
        self.meta
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Language {
    name: &'static str,
    native_name: Option<&'static str>,
    iso_639_3: &'static str,
    bcp_47: &'static str,
    iso_15924: &'static str,
    iana: &'static [&'static str],
    extinct: bool,
    spurious: bool,
}

impl Language {
    /// Returns the name of the language
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the native name of the language
    #[inline]
    pub const fn native_name(&self) -> Option<&'static str> {
        self.native_name
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

    /// Returns whether the language is extinct
    #[inline]
    pub const fn is_extinct(&self) -> bool {
        self.extinct
    }

    /// Returns whether the language is spurious
    #[inline]
    pub const fn is_spurious(&self) -> bool {
        self.spurious
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountryName {
    common: &'static str,
    official: &'static str,
}

impl CountryName {
    /// Returns the common name of the country
    #[inline]
    pub const fn common(&self) -> &'static str {
        self.common
    }

    /// Returns the official name of the country
    #[inline]
    pub const fn official(&self) -> &'static str {
        self.official
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountryMeta {
    common: &'static str,
    official: &'static str,
    native: &'static super::StaticMap<&'static str, &'static CountryName>,
    alternates: &'static [&'static str],
}

impl CountryMeta {
    /// Returns the common name of the country
    #[inline]
    pub const fn common(&self) -> &'static str {
        self.common
    }

    /// Returns the official name of the country
    #[inline]
    pub const fn official(&self) -> &'static str {
        self.official
    }

    /// Returns the name of the country in native languages
    #[inline]
    pub const fn native(&self) -> &'static super::StaticMap<&'static str, &'static CountryName> {
        self.native
    }

    /// Returns the alternate names of the country
    #[inline]
    pub const fn alternates(&self) -> &'static [&'static str] {
        self.alternates
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Country {
    name: &'static CountryMeta,
    flag: &'static str,
    cca2: &'static str,
    cca3: &'static str,
    ccn3: &'static str,
    ioc: Option<&'static str>,
    tld: &'static [&'static str],
    locale: &'static Locale,
    idd: &'static IDD,
    geography: &'static Geography,
    languages: &'static Language,
    currencies: &'static [&'static Currency],
    subdivisions: &'static StaticMap<&'static str, &'static Subdivision>,
}

impl Country {
    /// Returns the name metadata of the country
    #[inline]
    pub const fn name(&self) -> &'static CountryMeta {
        self.name
    }

    /// Returns the country's flag
    #[inline]
    pub const fn flag(&self) -> &'static str {
        self.flag
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
    pub const fn ioc(&self) -> Option<&'static str> {
        self.ioc
    }

    /// Returns list of [Country Code Top Level Domain (ccTLD)] used
    ///
    /// [Country Code Top Level Domain (ccTLD)]: https://en.wikipedia.org/wiki/Country_code_top-level_domain#Lists
    #[inline]
    pub const fn tld(&self) -> &'static [&'static str] {
        self.tld
    }

    /// Returns the country's locale information
    #[inline]
    pub const fn locale(&self) -> &'static Locale {
        self.locale
    }

    /// Returns the country's [international dialing direct] information
    ///
    /// [international dialing direct]: https://en.wikipedia.org/wiki/List_of_country_calling_codes
    #[inline]
    pub const fn idd(&self) -> &'static IDD {
        self.idd
    }

    /// Returns the country's geographical information
    #[inline]
    pub const fn geography(&self) -> &'static Geography {
        self.geography
    }

    /// Returns the country's language information
    #[inline]
    pub const fn languages(&self) -> &'static Language {
        self.languages
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
}
