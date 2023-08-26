use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SpotData {
    pub id: u64,
    #[serde(rename = "timeStamp", with = "iso8601")]
    pub timestamp: DateTime<Utc>,
    pub comments: Option<String>,
    pub callsign: String,
    #[serde(rename = "activatorCallsign")]
    pub activator_callsign: String,
    #[serde(rename = "activatorName")]
    pub activator_name: String,
    pub frequency: String,
    pub mode: String,
    #[serde(rename = "summitDetails")]
    pub summit_details: String,
    #[serde(rename = "summitCode")]
    pub summit_code: String,
    #[serde(rename = "highlightColor")]
    pub highlight_color: String,
}
impl SpotData {
    pub fn time(&self) -> String {
        self.timestamp.format("%H:%m").to_string()
    }
}
pub type SpotDataCollection = Vec<SpotData>;

pub fn load_spots() -> SpotDataCollection {
    let body = reqwest::blocking::get("https://api2.sota.org.uk/api/spots/48/?filter=all")
        .unwrap()
        .text()
        .unwrap();

    serde_json::from_str(&body).unwrap()
}

mod iso8601 {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S.%f";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
