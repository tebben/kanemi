use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Reponse<T> {
    pub docs: Vec<T>,
    #[serde(rename = "maxScore")]
    pub max_score: f64,
    #[serde(rename = "numFound")]
    pub num_found: i32,
    #[serde(rename = "numFoundExact")]
    pub num_found_exact: bool,
    pub start: i32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DocType {
    Provincie,
    Gemeente,
    Woonplaats,
    Weg,
    Postcode,
    Adres,
    Perceel,
    Hectometerpaal,
    Wijk,
    Buurt,
    Waterschapsgrens,
    Appartementsrecht,
}
