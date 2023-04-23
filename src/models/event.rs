use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::{from_str, join_ids};
use serde::ser::{Serialize, Serializer};
use std::collections::HashMap;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
) -> String {
    format!("portal/{}/projects/{}/events/", portal, project)
}

#[derive(Clone, Debug)]
pub struct EventRequest(RequestDetails);

impl EventRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        EventRequest(RequestDetails::new(access_token, model_path, id))
    }
}

impl ModelRequest for EventRequest {
    fn uri(&self) -> String {
        self.0.uri()
    }

    fn params(&self) -> Option<HashMap<String, String>> {
        self.0.params()
    }

    fn access_token(&self) -> String {
        self.0.access_token()
    }

    fn filter(mut self, param: (impl FilterOptions + std::fmt::Display)) -> Self {
        self.0 = self.0.filter(&param);
        self
    }
}

impl RequestParameters for EventRequest {
    type ModelCollection = ZohoEvents;
    type NewModel = NewEvent;
}

pub enum Filter {
    Index(usize),
    Range(i8),
    Status(String),
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
            Filter::Status(_) => "status".to_owned(),
        }
    }
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
            Filter::Status(status) => status.clone(),
        };

        write!(f, "{}", str_rep)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Response {
    response: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZohoEvents {
    #[serde(rename = "events")]
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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
    pub occurrence_s: Option<i64>,
    #[serde(rename = "occurred")]
    pub occurred: i64,
    #[serde(rename = "duration_hour")]
    pub duration_hour: String,
    #[serde(rename = "duration_minutes")]
    pub duration_minutes: String,
    #[serde(rename = "is_open")]
    pub is_open: Option<bool>,
    #[serde(rename = "participants")]
    pub participants: Option<Vec<Participant>>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct NewEvent {
    pub title: String,
    pub date: String,
    pub hour: String,
    pub minutes: String,
    pub ampm: AmPm,
    pub duration_hour: String,
    pub duration_mins: String,
    #[serde(serialize_with = "join_ids")]
    pub participants: Option<Vec<i64>>,
    pub remind_before: Option<RemindBefore>,
    pub repeat: Option<Repeat>,
    pub nooftimes_repeat: Option<NumRepeat>,
    pub location: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub enum AmPm {
    #[default]
    Am,
    Pm,
}

impl std::fmt::Display for AmPm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            AmPm::Am => "am",
            AmPm::Pm => "pm",
        };

        write!(f, "{}", str_rep)
    }
}

impl Serialize for AmPm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Repeat {
    Once,
    EveryDay,
    EveryWeek,
    EveryMonth,
    EveryYear,
}

impl std::fmt::Display for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            Repeat::Once => "once",
            Repeat::EveryDay => "everyday",
            Repeat::EveryWeek => "everyweek",
            Repeat::EveryMonth => "everymonth",
            Repeat::EveryYear => "everyyear",
        };

        write!(f, "{}", str_rep)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RemindBefore {
    OnTime,
    FifteenMins,
    ThirtyMins,
    OneHour,
    TwoHours,
    SixHours,
    TwelveHours,
    OneDay,
}

impl std::fmt::Display for RemindBefore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            RemindBefore::OnTime => "ontime",
            RemindBefore::FifteenMins => "15mins",
            RemindBefore::ThirtyMins => "30mins",
            RemindBefore::OneHour => "1hour",
            RemindBefore::TwoHours => "2hours",
            RemindBefore::SixHours => "6hours",
            RemindBefore::TwelveHours => "12hours",
            RemindBefore::OneDay => "1day",
        };

        write!(f, "{}", str_rep)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NumRepeat {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl NumRepeat {
    pub fn to_i(&self) -> i64 {
        match *self {
            NumRepeat::Two => 2,
            NumRepeat::Three => 3,
            NumRepeat::Four => 4,
            NumRepeat::Five => 5,
            NumRepeat::Six => 6,
            NumRepeat::Seven => 7,
            NumRepeat::Eight => 8,
            NumRepeat::Nine => 9,
            NumRepeat::Ten => 10,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Participant {
    #[serde(rename = "participant_id", deserialize_with = "from_str")]
    pub participant_id: i64,
    #[serde(rename = "participant_person")]
    pub participant_person: String,
}
