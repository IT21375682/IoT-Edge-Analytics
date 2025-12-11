use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TelemetryData {
    pub temperature: f64,
    pub humidity: f64,
}
