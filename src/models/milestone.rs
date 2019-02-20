use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::from_str;
use std::collections::HashMap;

pub fn model_path(portal: impl std::fmt::Display, project: impl std::fmt::Display) -> String {
    format!("portal/{}/projects/{}/milestones/", portal, project)
}

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

    fn filter(mut self, param: impl FilterOptions) -> Self {
        self.0 = self.0.filter(&param);
        self
    }
}

impl RequestParameters for MilestoneRequest {
    type ModelCollection = ZohoMilestones;
    type NewModel = NewMilestone;
}

pub enum Filter {
    Index(i64),
    Range(i64),
    Status(String),
    DisplayType(String),
    Flag(String),
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

    fn value(&self) -> String {
        match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
            Filter::Status(status) => status.clone(),
            Filter::DisplayType(display_type) => display_type.clone(),
            Filter::Flag(flag) => flag.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZohoMilestones {
    #[serde(rename = "milestones")]
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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
    pub flag: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "start_date_long")]
    pub start_date_long: i64,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "end_date_long")]
    pub end_date_long: i64,
    #[serde(rename = "status")]
    pub status: String,
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
