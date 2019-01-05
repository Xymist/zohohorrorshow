use crate::errors::*;
use crate::request::{FilterOptions, RequestDetails, RequestParameters, ZohoRequest};

pub const ModelPath: &str = "portal/{}/projects/{}/activities/";

type ActivityDetails = RequestDetails;

impl RequestParameters for ActivityDetails {
    type ModelCollection = ZohoActivities;

    fn filter(self, param: impl FilterOptions) -> Self {
        let mut frp = ActivityDetails {
            model_path: self.model_path,
            id: self.id,
            name: self.name,
            params: self.params,
        };
        frp.params
            .push((param.key().to_owned(), param.value().to_owned()));
        frp
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
