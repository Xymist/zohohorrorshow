use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::from_str;
use std::collections::HashMap;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
) -> String {
    format!("portal/{}/projects/{}/milestones/", portal, project)
}

#[derive(Clone, Debug)]
pub struct MilestoneRequest(RequestDetails);

impl MilestoneRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        MilestoneRequest(RequestDetails::new(access_token, model_path, id))
    }
}

impl ModelRequest for MilestoneRequest {
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

impl RequestParameters for MilestoneRequest {
    type ModelCollection = ZohoMilestones;
    type NewModel = NewMilestone;
}

pub enum Filter {
    Index(usize),
    Range(i8),
    Status(Status),
    DisplayType(DisplayType),
    Flag(Flag),
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
            Filter::Status(_) => "status".to_owned(),
            Filter::DisplayType(_) => "display_type".to_owned(),
            Filter::Flag(_) => "flag".to_owned(),
        }
    }
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
            Filter::Status(status) => status.to_string(),
            Filter::DisplayType(display_type) => display_type.to_string(),
            Filter::Flag(flag) => flag.to_string(),
        };

        write!(f, "{}", str_rep)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum Flag {
    #[serde(rename = "all")]
    #[default]
    AllFlag,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            Flag::AllFlag => "all",
            Flag::Internal => "internal",
            Flag::External => "external",
        };

        write!(f, "{}", str_rep)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum DisplayType {
    #[serde(rename = "all")]
    #[default]
    All,
    #[serde(rename = "upcoming")]
    Upcoming,
    #[serde(rename = "delayed")]
    Delayed,
}

impl std::fmt::Display for DisplayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            DisplayType::All => "all",
            DisplayType::Upcoming => "upcoming",
            DisplayType::Delayed => "delayed",
        };

        write!(f, "{}", str_rep)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum Status {
    #[serde(rename = "all")]
    #[default]
    All,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "notcompleted")]
    NotCompleted,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            Status::All => "all",
            Status::Completed => "completed",
            Status::NotCompleted => "notcompleted",
        };

        write!(f, "{}", str_rep)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZohoMilestones {
    #[serde(rename = "milestones")]
    pub milestones: Vec<Milestone>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Milestone {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "link")]
    pub link: Link,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "owner_name")]
    pub owner_name: String,
    #[serde(rename = "owner_id", deserialize_with = "from_str")]
    pub owner_id: i64,
    #[serde(rename = "flag")]
    pub flag: Flag,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "start_date_long")]
    pub start_date_long: i64,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "end_date_long")]
    pub end_date_long: i64,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "completed_date")]
    pub completed_date: Option<String>,
    #[serde(rename = "completed_date_long")]
    pub completed_date_long: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
    #[serde(rename = "status")]
    pub status: SelfLink,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SelfLink {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NewMilestone {
    name: String,
    start_date: String,
    end_date: String,
    owner: i64,
    flag: String,
}
