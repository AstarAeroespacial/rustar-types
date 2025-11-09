use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetryMessage {
    pub ground_station_id: String,
    pub timestamp: DateTime<Utc>,
    pub payload: Vec<u8>,
}

impl TelemetryMessage {
    pub fn new(
        ground_station_id: impl Into<String>,
        timestamp: DateTime<Utc>,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            ground_station_id: ground_station_id.into(),
            timestamp,
            payload,
        }
    }
}
