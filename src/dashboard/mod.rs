use errors::*;
use RelativePath;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoDashboard {
    #[serde(rename = "activities")]
    pub activities: Vec<Activity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "activity_for")]
    pub activity_for: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "activity_by")]
    pub activity_by: String,
    #[serde(rename = "time_long")]
    pub time_long: i64,
    #[serde(rename = "display_time")]
    pub display_time: String,
    #[serde(rename = "time")]
    pub time: String,
}
