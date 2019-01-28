use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use std::collections::HashMap;

pub fn model_path(portal: impl std::fmt::Display, project: impl std::fmt::Display) -> String {
    format!("portal/{}/projects/{}/activities/", portal, project)
}

pub struct ActivityRequest(RequestDetails);

impl ActivityRequest {
    pub fn new(access_token: &str, model_path: &str) -> Self {
        ActivityRequest(RequestDetails::new(access_token, model_path, None))
    }
}

impl ModelRequest for ActivityRequest {
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

impl RequestParameters for ActivityRequest {
    type ModelCollection = ZohoActivities;
    type NewModel = NewActivity;

    fn post(&self, _data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        bail!("POST requests are not supported for Activities");
    }

    fn put(&self, _data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        bail!("PUT requests are not supported for Activities");
    }

    fn delete(&self) -> Result<Option<Self::ModelCollection>> {
        bail!("DELETE requests are not supported for Activities");
    }
}

pub enum Filter {
    Index(i64),
    Range(i64),
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
        }
    }

    fn value(&self) -> String {
        match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZohoActivities {
    #[serde(rename = "activities")]
    pub activities: Vec<Activity>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

#[derive(Debug, Serialize, Clone)]
pub struct NewActivity {}
