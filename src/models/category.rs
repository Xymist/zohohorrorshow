use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::from_str;
use std::collections::HashMap;

pub fn model_path(portal: impl std::fmt::Display, project: impl std::fmt::Display) -> String {
    format!("portal/{}/projects/{}/categories/", portal, project)
}

pub struct CategoryRequest(RequestDetails);

impl CategoryRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        CategoryRequest(RequestDetails::new(access_token, model_path, id))
    }
}

impl ModelRequest for CategoryRequest {
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

impl RequestParameters for CategoryRequest {
    type ModelCollection = ZohoCategories;
    type NewModel = NewCategory;

    fn put(&self, _data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        bail!("PUT requests are not supported for Activities");
    }
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Response {
    response: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZohoCategories {
    #[serde(rename = "categories")]
    pub categories: Vec<Category>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Category {
    #[serde(rename = "id", deserialize_with = "from_str")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct NewCategory {
    #[serde(rename = "name")]
    pub name: String,
}
