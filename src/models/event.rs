use client::ZohoClient;
use errors::*;
use std::rc::Rc;
use utils::from_str;

pub fn events(cl: &Rc<ZohoClient>) -> EventFragment {
    let client = Rc::clone(cl);
    EventFragment {
        path: client.make_uri(&format!(
            "portal/{}/projects/{}/events/",
            client.portal_id(),
            client.project_id()
        )),
        client,
    }
}

#[derive(Debug)]
pub struct EventFragment {
    pub client: Rc<ZohoClient>,
    pub path: String,
}

impl EventFragment {
    query_strings!(EventFragment; index, range, status);

    // Delete an event by ID
    pub fn delete(self, id: i64) -> Result<String> {
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        let response: Response = self.client
            .delete(&format!("{}{}/?{}", path_frags[0], id, path_frags[1]))?;
        Ok(response.response)
    }

    pub fn update(mut self, id: i64, event_details: NewEvent) -> Result<Event> {
        let path = self.path.clone();
        let path_frags = path.split('?').collect::<Vec<&str>>();
        self.path = format!("{}{}/?{}", path_frags[0], id, path_frags[1]);
        self.create(event_details)
    }

    pub fn create(mut self, new_event: NewEvent) -> Result<Event> {
        let participants = new_event
            .participants
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(",");
        self.path = format!(
            "{}&title={}&date={}&hour={}&minutes={}&ampm={}&duration_hour={}&duration_mins={}&participants={}",
            self.path,
            new_event.title,
            new_event.date,
            new_event.hour,
            new_event.minutes,
            new_event.ampm.to_string(),
            new_event.duration_hour,
            new_event.duration_mins,
            participants
        );

        if let Some(rb) = new_event.remind_before {
            self.path = format!("{}&remind_before={}", self.path, rb.to_string());
        }
        if let Some(r) = new_event.repeat {
            self.path = format!("{}&repeat={}", self.path, r.to_string());
        }
        if let Some(nr) = new_event.nooftimes_repeat {
            self.path = format!("{}&noofimes_repeat={}", self.path, nr.to_i());
        }
        if let Some(l) = new_event.location {
            self.path = format!("{}&location={}", self.path, l.to_string());
        }

        let mut event: ZohoEvents = self.client.post(&self.path, "")?;
        Ok(event.events.remove(0))
    }

    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Event>> {
        if !self.path.contains("status") {
            return self.status("open").fetch();
        }
        let event_list: ZohoEvents = self.client.get(&self.path)?;
        Ok(event_list.events)
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
    pub participants: Vec<Participant>,
}

#[derive(Debug, Clone, Default)]
pub struct NewEvent {
    pub title: String,
    pub date: String,
    pub hour: String,
    pub minutes: String,
    pub ampm: AmPm,
    pub duration_hour: String,
    pub duration_mins: String,
    pub participants: Vec<i64>,
    pub remind_before: Option<RemindBefore>,
    pub repeat: Option<Repeat>,
    pub nooftimes_repeat: Option<NumRepeat>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
