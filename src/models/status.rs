use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};

pub const ModelPath: &str = "portal/{}/projects/{}/statuses/";

pub struct StatusRequest(RequestDetails);

impl StatusRequest {
    pub fn new(auth_token: &str, model_path: &str) -> Self {
        StatusRequest(RequestDetails::new(auth_token, model_path))
    }
}

impl ModelRequest for StatusRequest {
    fn uri(&self) -> String {
        self.0.uri()
    }
}

impl RequestParameters for StatusRequest {
    type ModelCollection = ZohoStatuses;
    type NewModel = NewStatus;

    fn put(&self, url: &str, data: &str) -> Result<Self::ModelCollection> {
        bail!("PUT requests are not supported for Statuses");
    }

    fn delete(&self, url: &str) -> Result<Self::ModelCollection> {
        bail!("DELETE requests are not supported for Statuses");
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

#[derive(Debug, Serialize)]
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
