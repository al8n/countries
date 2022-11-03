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

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Timezone {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "linkedTo")]
    linked_to: Option<String>,
    #[serde(rename = "utcOffset")]
    utc_offset: String,
    #[serde(rename = "dstOffset")]
    dst_offset: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Locale {
    ietf: Vec<String>,
    measurement_system: String,
    driving_side: String,
    hour_clock: String,
    timezones: Vec<Timezone>,
    date_formats: HashMap<String, String>,
    week_starts_on: String,
    #[serde(rename = "distanceMeasurement")]
    distance_uint: String,
    #[serde(rename = "temperatureMeasurement")]
    temperature_uint: String,
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
    String(String)
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

fn main() -> Result<()> {
    let out_path = PathBuf::from("src/generated.rs");
    let src_path = PathBuf::from("data.json");
    
    let mut out = io::BufWriter::new(File::create(&out_path)?);
    
    let src = File::open(&src_path)?;
    let countries: Vec<CountryData> = serde_json::from_reader(src)?;
    

    // writeln!(out, "// Auto generated file, please do not modify \n\n")?;
    // writeln!(out, "/// ISO-3166")?;
    // writeln!(out, "#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]")?;
    // writeln!(out, "#[repr(u8)]")?;
    // writeln!(out, "pub enum Country {{")?;
    // for country in &countries {
    //     writeln!(out, "\t{},", country.alpha2)?;
    // }
    // out.close()?;

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
