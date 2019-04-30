use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::from_str;
use std::collections::HashMap;

pub mod user;

pub(crate) fn model_path() -> String {
    "portals/".to_owned()
}

/// Request for Portal(s) from Zoho
#[derive(Clone, Debug)]
pub struct PortalRequest(RequestDetails);

impl PortalRequest {
    /// Constructor for a new PortalRequest
    pub(crate) fn new(access_token: &str) -> Self {
        PortalRequest(RequestDetails::new(access_token, &model_path(), None))
    }
}

impl ModelRequest for PortalRequest {
    fn uri(&self) -> String {
        self.0.uri()
    }

    fn params(&self) -> Option<HashMap<String, String>> {
        self.0.params()
    }

    fn access_token(&self) -> String {
        self.0.access_token()
    }

    fn filter(self, _param: impl FilterOptions) -> Self {
        self
    }
}

impl RequestParameters for PortalRequest {
    type ModelCollection = ZohoPortals;
    type NewModel = NewPortal;

    fn post(&self, _data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        Err(Error::disallowed_method("POST", "Portal"))
    }

    fn put(&self, _data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        Err(Error::disallowed_method("PUT", "Portal"))
    }

    fn delete(&self) -> Result<Option<Self::ModelCollection>> {
        Err(Error::disallowed_method("DELETE", "Portal"))
    }
}

/// Unconstructable enum representing a theoretical new Portal record.
/// The Zoho Projects API does not permit creating Portals.
#[derive(Debug, Serialize, Clone)]
pub enum NewPortal {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZohoPortals {
    #[serde(rename = "login_id", deserialize_with = "from_str")]
    pub login_id: i64,
    #[serde(rename = "portals")]
    pub portals: Vec<Portal>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Portal {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "default")]
    pub default_portal: bool,
    #[serde(rename = "gmt_time_zone")]
    pub gmt_time_zone: String,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "project_count")]
    pub project_count: ProjectCount,
    #[serde(rename = "settings")]
    pub settings: Settings,
    #[serde(rename = "locale")]
    pub locale: Locale,
    #[serde(rename = "link")]
    pub link: Link,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Link {
    #[serde(rename = "project")]
    pub project: Project,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Project {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Locale {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "country")]
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectCount {
    #[serde(rename = "template")]
    pub template: Option<i64>,
    #[serde(rename = "archived")]
    pub archived: Option<i64>,
    #[serde(rename = "active")]
    pub active: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    #[serde(rename = "company_name")]
    pub company_name: String,
    #[serde(rename = "website_url")]
    pub website_url: Option<String>,
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    #[serde(rename = "date_format")]
    pub date_format: String,
}
