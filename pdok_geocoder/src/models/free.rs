use super::shared::{DocType, Reponse};
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct FreeResponse {
    pub response: Reponse<Doc>,
}

#[derive(Debug)]
pub enum Bron {
    Bag,
    Nwb,
    BagNwb,
    Dkk,
    BestuurlijkeGrenzen,
    Cbs,
    Hwh,
    Unknown(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Doc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bron: Option<Bron>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gemeentecode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gemeentenaam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identificatie: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provincieafkorting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provinciecode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provincienaam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub doc_type: Option<DocType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weergavenaam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdf_seealso: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nwb_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openbareruimtetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openbareruimte_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straatnaam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub woonplaatsnaam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub woonplaatscode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wijkcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wijknaam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buurtnaam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub huisnummer: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub huis_nlt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waterschapcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waterschapsnaam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adresseerbaarobject_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nummeraanduiding_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straatnaam_verkort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gekoppeld_perceel: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "point_from_wkt"
    )]
    pub centroide_ll: Option<Point>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "point_from_wkt"
    )]
    pub centroide_rd: Option<Point>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

fn point_from_wkt<'de, D>(deserializer: D) -> Result<Option<Point>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let wkt: Option<String> = Option::deserialize(deserializer).unwrap_or(None);

    if let Some(wkt_string) = wkt {
        parse_point_wkt(&wkt_string)
            .map(Some)
            .map_err(de::Error::custom)
    } else {
        Ok(None)
    }
}

fn parse_point_wkt(s: &str) -> Result<Point, &'static str> {
    if let Some(wkt) = s.strip_prefix("POINT(") {
        if let Some(wkt) = wkt.strip_suffix(')') {
            let parts: Vec<&str> = wkt.split_whitespace().collect();
            if parts.len() == 2 {
                let x = f64::from_str(parts[0]).map_err(|_| "Invalid X coordinate")?;
                let y = f64::from_str(parts[1]).map_err(|_| "Invalid Y coordinate")?;
                return Ok(Point { x, y });
            }
        }
    }
    Err("Invalid WKT format")
}

impl<'de> Deserialize<'de> for Bron {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct BronVisitor;

        impl Visitor<'_> for BronVisitor {
            type Value = Bron;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a valid bron value or an unknown type")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value.to_lowercase().as_str() {
                    "bag" => Ok(Bron::Bag),
                    "nwb" => Ok(Bron::Nwb),
                    "bag/nwb" => Ok(Bron::BagNwb),
                    "dkk" => Ok(Bron::Dkk),
                    "bestuurlijke grenzen" => Ok(Bron::BestuurlijkeGrenzen),
                    "cbs" => Ok(Bron::Cbs),
                    "hwh" => Ok(Bron::Hwh),
                    _ => Ok(Bron::Unknown(value.to_string())), // Catch unknown types
                }
            }
        }

        deserializer.deserialize_str(BronVisitor)
    }
}

impl Serialize for Bron {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Bron::Bag => serializer.serialize_str("BAG"),
            Bron::Nwb => serializer.serialize_str("NWB"),
            Bron::BagNwb => serializer.serialize_str("BAG/NWB"),
            Bron::Dkk => serializer.serialize_str("DKK"),
            Bron::BestuurlijkeGrenzen => serializer.serialize_str("Bestuurlijke Grenzen"),
            Bron::Cbs => serializer.serialize_str("CBS"),
            Bron::Hwh => serializer.serialize_str("HWH"),
            Bron::Unknown(ref s) => serializer.serialize_str(s), // Serialize unknown as string
        }
    }
}
