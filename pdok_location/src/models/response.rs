use serde::{de, Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct FreeResponse {
    pub response: Reponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reponse {
    pub docs: Vec<Doc>,
    #[serde(rename = "maxScore")]
    pub max_score: f64,
    #[serde(rename = "numFound")]
    pub num_found: i32,
    #[serde(rename = "numFoundExact")]
    pub num_found_exact: bool,
    pub start: i32,
}

// ToDo parse separate types?
// weg
// gemeente

#[derive(Debug, Deserialize, Serialize)]
pub struct Doc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bron: Option<String>,
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
    pub doc_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weergavenaam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdf_seealso: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nwb_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openbareruimtetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straatnaam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub straatnaam_verkort: Option<String>,
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
    // Attempt to deserialize as an Option<String>
    let wkt: Option<String> = Option::deserialize(deserializer).unwrap_or(None);

    if let Some(wkt_string) = wkt {
        // Parse the WKT string into a Point
        parse_point_wkt(&wkt_string)
            .map(Some)
            .map_err(de::Error::custom)
    } else {
        // If the field is missing or null, return None
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
