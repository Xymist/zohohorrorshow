use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::{from_str, join_ids};
use serde::ser::{Serialize, Serializer};
use std::collections::HashMap;

pub fn model_path(portal: impl std::fmt::Display, project: impl std::fmt::Display) -> String {
    format!("portal/{}/projects/{}/events/", portal, project)
}

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
}

impl RequestParameters for EventRequest {
    type ModelCollection = ZohoEvents;
    type NewModel = NewEvent;
}

pub enum Filter {
    Index(i64),
    Range(i64),
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

    fn value(&self) -> String {
        match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
            Filter::Status(status) => status.clone(),
        }
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

#[derive(Clone, Debug, Deserialize)]
pub enum AmPm {
    Am,
    Pm,
}

impl Default for AmPm {
    fn default() -> AmPm {
        AmPm::Am
    }
}

impl AmPm {
    pub fn to_string(&self) -> String {
        match *self {
            AmPm::Am => "am".to_owned(),
            AmPm::Pm => "pm".to_owned(),
        }
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

impl Repeat {
    pub fn to_string(&self) -> String {
        match *self {
            Repeat::Once => "once".to_owned(),
            Repeat::EveryDay => "everyday".to_owned(),
            Repeat::EveryWeek => "everyweek".to_owned(),
            Repeat::EveryMonth => "everymonth".to_owned(),
            Repeat::EveryYear => "everyyear".to_owned(),
        }
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

impl RemindBefore {
    pub fn to_string(&self) -> String {
        match *self {
            RemindBefore::OnTime => "ontime".to_owned(),
            RemindBefore::FifteenMins => "15mins".to_owned(),
            RemindBefore::ThirtyMins => "30mins".to_owned(),
            RemindBefore::OneHour => "1hour".to_owned(),
            RemindBefore::TwoHours => "2hours".to_owned(),
            RemindBefore::SixHours => "6hours".to_owned(),
            RemindBefore::TwelveHours => "12hours".to_owned(),
            RemindBefore::OneDay => "1day".to_owned(),
        }
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
