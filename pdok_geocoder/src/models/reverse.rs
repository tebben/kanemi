use super::shared::DocType;
use super::shared::Reponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ReverseResponse {
    pub response: Reponse<Doc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Doc {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub doc_type: Option<DocType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weergavenaam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afstand: Option<f32>,
}
