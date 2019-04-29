use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use std::collections::HashMap;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
) -> String {
    format!("portal/{}/projects/{}/logs/", portal, project)
}

#[derive(Clone, Debug)]
pub struct TimesheetRequest(RequestDetails);

impl TimesheetRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        TimesheetRequest(RequestDetails::new(access_token, model_path, id))
    }
}

impl ModelRequest for TimesheetRequest {
    fn uri(&self) -> String {
        self.0.uri()
    }

    fn params(&self) -> Option<HashMap<String, String>> {
        self.0.params()
    }

    fn access_token(&self) -> String {
        self.0.access_token()
    }

    fn filter(mut self, param: impl FilterOptions) -> Self {
        self.0 = self.0.filter(&param);
        self
    }
}

impl RequestParameters for TimesheetRequest {
    type ModelCollection = ZohoTimeLogs;
    type NewModel = NewTimeLog;
}

pub enum Filter {
    Index(usize),
    Range(i8),
    Date(String),
    Users(Option<Vec<i64>>),
    ViewType(ViewType),
    ComponentType(ComponentType),
    BillStatus(BillStatus),
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
            Filter::Date(_) => "date".to_owned(),
            Filter::Users(_) => "users".to_owned(),
            Filter::ViewType(_) => "view_type".to_owned(),
            Filter::ComponentType(_) => "component_type".to_owned(),
            Filter::BillStatus(_) => "bill_status".to_owned(),
        }
    }

    fn value(&self) -> String {
        match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
            Filter::Date(date) => date.to_owned(),
            Filter::Users(users) => match users {
                Some(u) => u
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                None => "all".to_owned(),
            },
            Filter::ViewType(view_type) => view_type.to_string(),
            Filter::ComponentType(component_type) => component_type.to_string(),
            Filter::BillStatus(bill_status) => bill_status.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ViewType {
    Day,
    Week,
    Month,
}

impl ViewType {
    pub fn to_string(&self) -> String {
        match *self {
            ViewType::Day => "day".to_owned(),
            ViewType::Month => "month".to_owned(),
            ViewType::Week => "week".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BillStatus {
    All,
    Billable,
    NonBillable,
}

impl BillStatus {
    pub fn to_string(&self) -> String {
        match *self {
            BillStatus::All => "all".to_owned(),
            BillStatus::Billable => "billable".to_owned(),
            BillStatus::NonBillable => "non_billable".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ComponentType {
    Task,
    Bug,
    General,
}

impl ComponentType {
    pub fn to_string(&self) -> String {
        match *self {
            ComponentType::Task => "task".to_owned(),
            ComponentType::Bug => "bug".to_owned(),
            ComponentType::General => "general".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZohoTimeLogs {
    #[serde(rename = "timelogs")]
    pub timelogs: TimeLogs,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TimeLogs {
    #[serde(rename = "grandtotal")]
    pub grandtotal: String,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "timelog")]
    pub timelog: TimeLog,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TimeLog {
    #[serde(rename = "date")]
    pub date: Vec<DateLog>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NewTimeLog {}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DateLog {
    #[serde(rename = "date_long")]
    pub date_long: i64,
    #[serde(rename = "display_format")]
    pub display_format: String,
    #[serde(rename = "totalhours")]
    pub totalhours: String,
    #[serde(rename = "buglogs")]
    pub buglogs: Vec<BugLog>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BugLog {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "notes")]
    pub notes: String,
    #[serde(rename = "hours")]
    pub hours: i64,
    #[serde(rename = "minutes")]
    pub minutes: i64,
    #[serde(rename = "hour_display")]
    pub hour_display: String,
    #[serde(rename = "total_minutes")]
    pub total_minutes: i64,
    #[serde(rename = "owner_name")]
    pub owner_name: String,
    #[serde(rename = "bill_status")]
    pub bill_status: String,
    #[serde(rename = "project")]
    pub project: Project,
    #[serde(rename = "bug")]
    pub bug: Bug,
    #[serde(rename = "link")]
    pub link: Link,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Bug {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SelfLink {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Project {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}
