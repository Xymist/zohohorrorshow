use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use std::collections::HashMap;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
) -> String {
    format!("portal/{}/projects/{}/statuses/", portal, project)
}

#[derive(Clone, Debug)]
pub struct StatusRequest(RequestDetails);

impl StatusRequest {
    pub fn new(access_token: &str, model_path: &str) -> Self {
        StatusRequest(RequestDetails::new(access_token, model_path, None))
    }
}

impl ModelRequest for StatusRequest {
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

impl RequestParameters for StatusRequest {
    type ModelCollection = ZohoStatuses;
    type NewModel = NewStatus;

    fn put(&self, _data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        Err(Error::disallowed_method("PUT", "Status"))
    }

    fn delete(&self) -> Result<Option<Self::ModelCollection>> {
        Err(Error::disallowed_method("DELETE", "Status"))
    }
}

pub enum Filter {
    Index(usize),
    Range(i8),
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
        }
    }
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
        };

        write!(f, "{}", str_rep)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZohoStatuses {
    #[serde(rename = "statuses")]
    pub statuses: Vec<Status>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Status {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "posted_by")]
    pub posted_by: String,
    #[serde(rename = "posted_person")]
    pub posted_person: String,
    #[serde(rename = "posted_time")]
    pub posted_time: String,
    #[serde(rename = "posted_time_long")]
    pub posted_time_long: i64,
}

#[derive(Clone, Debug, Serialize)]
pub struct NewStatus {
    #[serde(rename = "content")]
    content: String,
}

impl NewStatus {
    pub fn new(content: &str) -> Self {
        NewStatus {
            content: content.to_owned(),
        }
    }
}
