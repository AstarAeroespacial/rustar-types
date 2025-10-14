use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TelemetryMessage {
    ground_station_id: String,
    timestamp: DateTime<Utc>,
    payload: Vec<u8>,
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
