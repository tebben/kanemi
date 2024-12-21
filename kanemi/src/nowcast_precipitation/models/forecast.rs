use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PrecipitationForecast {
    pub datetime: String,
    pub values: Vec<PrecipitationForecastValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrecipitationForecastValue {
    pub datetime: String,
    pub value: f64,
}
