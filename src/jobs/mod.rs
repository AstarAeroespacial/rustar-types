use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// TODO: use sgp4 elements instead of TLE DATA
// TODO: builder interface from TLE data and parses to elements

/// # Job
///
/// A job instructs the ground station to track a specific satellite pass,
/// specifying:
///
/// - **Job identification**: Unique identifier (`id`)
/// - **Satellite identification**: ID of the satellite (`satellite_id`)
/// - **Time window**: When tracking should start and end (`start`, `end`)
/// - **Satellite orbital data**: Two-Line Element set (`tle`)
/// - **Transceiver frequencies**: Downlink (`rx_frequency`) and uplink (`tx_frequency`)
/// - **Uplink data**: Optional data to transmit to the satellite (`uplink`)
///
/// Example JSON:
/// ```json
/// {
///   "id": 12345,
///   "satellite_id": "ISS (ZARYA)",
///   "start": "2025-09-19T12:00:00Z",
///   "end": "2025-09-19T12:15:00Z",
///   "tle": {
///     "tle0": "ISS (ZARYA)",
///     "tle1": "1 25544U 98067A   25235.75642456  .00011222  00000+0  20339-3 0  9993",
///     "tle2": "2 25544  51.6355 332.1708 0003307 260.2831  99.7785 15.50129787525648"
///   },
///   "rx_frequency": 145800000,
///   "tx_frequency": 437500000,
///   "uplink": [72, 101, 108, 108, 111]
/// }
/// ```
///
/// ## Notes:
/// - `id` must be a **unique 64-bit unsigned integer** identifying this job.
/// - `satellite_id` is the identifier of the satellite being tracked.
/// - `start` and `end` must be **UTC timestamps** in ISO-8601 format. (Use https://www.utctime.net/ for getting the current UTC timestamp.)
/// - `tle1` and `tle2` **must be exactly 69 characters long** with valid checksums.
/// - `rx_frequency` and `tx_frequency` are expressed in **Hertz**.
/// - `uplink` is **optional** and contains raw bytes to transmit to the satellite during the pass.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct Job {
    /// **Unique job identifier**.
    ///
    /// This ID is used to track the job status and correlate related events.
    ///
    /// Example: `12345`
    #[schema(example = 12345)]
    pub id: u64,

    /// **Satellite ID**.
    ///
    /// Identifier of the satellite to be tracked.
    ///
    /// Example: `"ISS (ZARYA)"`
    #[schema(example = "ISS (ZARYA)")]
    pub satellite_id: String,

    /// UTC timestamp for when the tracking should **begin**.
    ///
    /// This marks the *Acquisition of Signal* (AOS) time.
    ///
    /// Example: `"2025-09-19T12:00:00Z"`
    #[schema(value_type = String, format = "date-time", example = "2025-09-19T12:00:00Z")]
    pub start: DateTime<Utc>,

    /// UTC timestamp for when the tracking should **end**.
    ///
    /// This marks the *Loss of Signal* (LOS) time.
    ///
    /// Example: `"2025-09-19T12:15:00Z"`
    #[schema(value_type = String, format = "date-time", example = "2025-09-19T12:15:00Z")]
    pub end: DateTime<Utc>,

    /// Orbital data (Two-Line Element set) for the satellite to be tracked.
    ///
    /// - `tle0`: Human-readable satellite name or catalog identifier.
    /// - `tle1`: First TLE line (exactly 69 characters).
    /// - `tle2`: Second TLE line (exactly 69 characters).
    ///
    /// Example:
    /// ```json
    /// {
    ///   "tle0": "ISS (ZARYA)",
    ///   "tle1": "1 25544U 98067A   25235.75642456  .00011222  00000+0  20339-3 0  9993",
    ///   "tle2": "2 25544  51.6355 332.1708 0003307 260.2831  99.7785 15.50129787525648"
    /// }
    /// ```
    #[schema(
        example = json!({
            "tle0": "ISS (ZARYA)",
            "tle1": "1 25544U 98067A   25235.75642456  .00011222  00000+0  20339-3 0  9993",
            "tle2": "2 25544  51.6355 332.1708 0003307 260.2831  99.7785 15.50129787525648"
        })
    )]
    pub tle: TleData,

    /// **Receiver frequency** in Hertz (Hz).
    ///
    /// This is the **downlink frequency** for receiving telemetry or data
    /// from the satellite.
    ///
    /// Examples:
    /// - `145800000` → 145.8 MHz (VHF downlink, common for many CubeSats)
    /// - `437500000` → 437.5 MHz (UHF downlink, common for amateur satellites)
    #[schema(example = 145800000)]
    pub rx_frequency: f64,

    /// **Transmitter frequency** in Hertz (Hz).
    ///
    /// This is the **uplink frequency** for sending commands to the satellite.
    ///
    /// Examples:
    /// - `437500000` → 437.5 MHz (UHF uplink, common for many satellites)
    #[schema(example = 437500000)]
    pub tx_frequency: f64,

    /// **Optional uplink data** to transmit to the satellite.
    ///
    /// This field contains raw bytes that will be transmitted during the pass.
    /// If `None`, the ground station will only receive data (downlink only).
    ///
    /// Example: `[72, 101, 108, 108, 111]` (ASCII for "Hello")
    #[schema(example = json!([72, 101, 108, 108, 111]))]
    pub uplink: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub enum JobStatus {
    Received,
    Scheduled,
    Started,
    Completed,
    Error,
}

/// # Two-Line Element (TLE) Data
///
/// Represents the standard orbital elements used to define a satellite's orbit.
/// These three lines are required to accurately track a satellite pass.
///
/// You can find up to date TLE data at []
///
/// ## Example
/// ```text
/// ISS (ZARYA)
/// 1 25544U 98067A   25235.75642456  .00011222  00000+0  20339-3 0  9993
/// 2 25544  51.6355 332.1708 0003307 260.2831  99.7785 15.50129787525648
/// ```
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TleData {
    /// Satellite name or catalog ID (first line of a TLE set)
    #[schema(example = "ISS (ZARYA)")]
    pub tle0: String,
    /// The first data line of the TLE
    #[schema(example = "1 25544U 98067A   25235.75642456  .00011222  00000+0  20339-3 0  9993")]
    pub tle1: String,
    /// The second data line of the TLE
    #[schema(example = "2 25544  51.6355 332.1708 0003307 260.2831  99.7785 15.50129787525648")]
    pub tle2: String,
}

/// Error type for TLE parsing failures
#[derive(Debug, Clone)]
pub enum TleParseError {
    /// Not enough lines in the input (expected 3)
    InsufficientLines,
    /// TLE line 1 doesn't have the correct length (expected 69 characters)
    InvalidTle1Length,
    /// TLE line 2 doesn't have the correct length (expected 69 characters)
    InvalidTle2Length,
}

impl TryFrom<String> for TleData {
    type Error = TleParseError;

    /// Parses a TLE string into TleData.
    ///
    /// The input string should contain three lines separated by newlines:
    /// - Line 0: Satellite name or identifier
    /// - Line 1: First TLE data line (exactly 69 characters)
    /// - Line 2: Second TLE data line (exactly 69 characters)
    ///
    /// # Example
    /// ```
    /// use rustar_types::jobs::TleData;
    /// use std::convert::TryFrom;
    ///
    /// let tle_string = "ISS (ZARYA)
    /// 1 25544U 98067A   25235.75642456  .00011222  00000+0  20339-3 0  9993
    /// 2 25544  51.6355 332.1708 0003307 260.2831  99.7785 15.50129787525648".to_string();
    ///
    /// let tle_data = TleData::try_from(tle_string).unwrap();
    /// ```
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lines: Vec<&str> = value.lines().collect();

        if lines.len() < 3 {
            return Err(TleParseError::InsufficientLines);
        }

        let tle0 = lines[0].trim().to_string();
        let tle1 = lines[1].trim().to_string();
        let tle2 = lines[2].trim().to_string();

        // Validate TLE line lengths
        if tle1.len() != 69 {
            return Err(TleParseError::InvalidTle1Length);
        }

        if tle2.len() != 69 {
            return Err(TleParseError::InvalidTle2Length);
        }

        Ok(TleData { tle0, tle1, tle2 })
    }
}

impl Job {
    pub fn new(
        id: u64,
        satellite_id: impl Into<String>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        tle: TleData,
        rx_frequency: f64,
        tx_frequency: f64,
        uplink: Option<Vec<u8>>,
    ) -> Self {
        Job {
            id,
            satellite_id: satellite_id.into(),
            start,
            end,
            tle,
            rx_frequency,
            tx_frequency,
            uplink,
        }
    }
}
