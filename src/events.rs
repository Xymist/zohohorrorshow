use RelativePath;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoEvents {
    #[serde(rename = "events")]
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "scheduled_on")]
    pub scheduled_on: String,
    #[serde(rename = "scheduled_on_long")]
    pub scheduled_on_long: i64,
    #[serde(rename = "reminder")]
    pub reminder: String,
    #[serde(rename = "repeat")]
    pub repeat: String,
    #[serde(rename = "occurrence(s)")]
    pub occurrence_s: i64,
    #[serde(rename = "occurred")]
    pub occurred: i64,
    #[serde(rename = "duration_hour")]
    pub duration_hour: String,
    #[serde(rename = "duration_minutes")]
    pub duration_minutes: String,
    #[serde(rename = "is_open")]
    pub is_open: bool,
    #[serde(rename = "participants")]
    pub participants: Vec<Participant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    #[serde(rename = "participant_id")]
    pub participant_id: String,
    #[serde(rename = "participant_person")]
    pub participant_person: String,
}

impl<'a> RelativePath<[i64; 2]> for ZohoEvents {
    fn relative_path(params: [i64; 2]) -> String {
        format!("portal/{}/projects/{}/events/", params[0], params[1])
    }
}
