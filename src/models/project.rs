use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::from_str;
use std::collections::HashMap;

pub fn model_path(portal: impl std::fmt::Display) -> String {
    format!("portal/{}/projects/", portal)
}

pub struct ProjectRequest(RequestDetails);

impl ProjectRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        ProjectRequest(RequestDetails::new(access_token, model_path, id))
    }
}

impl ModelRequest for ProjectRequest {
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

impl RequestParameters for ProjectRequest {
    type ModelCollection = ZohoProjects;
    type NewModel = NewProject;
}

pub enum Filter {
    Index(i64),
    Range(i64),
    Status(String),
    SortColumn(String),
    SortOrder(String),
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
            Filter::Status(_) => "status".to_owned(),
            Filter::SortColumn(_) => "sort_column".to_owned(),
            Filter::SortOrder(_) => "sort_order".to_owned(),
        }
    }

    fn value(&self) -> String {
        match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
            Filter::Status(status) => status.to_owned(),
            Filter::SortColumn(column) => column.to_owned(),
            Filter::SortOrder(order) => order.to_owned(),
        }
    }
}

// impl ProjectFragment {
//     // Fetch available custom fields (can be applied when creating projects)
//     pub fn customfields(self) -> Result<Option<Vec<CustomField>>> {
//         let mut path_frags = self.path.split('?').collect::<Vec<&str>>();
//         if path_frags[1].contains('&') {
//             let autht = path_frags.remove(1).split('&').collect::<Vec<&str>>()[0];
//             path_frags.push(autht)
//         }
//         println!("{:?}", self);
//         let fields: CustomFields = self.client.get(&format!(
//             "{}{}/?{}",
//             path_frags[0], "customfields", path_frags[1]
//         ))?;
//         Ok(Some(fields.fields))
//     }
// }

#[derive(Debug, Serialize, Clone)]
pub struct NewProject {
    name: String,
    owner: i64,
    description: String,
    template_id: i64,
    // [MM-DD-YYYY]
    start_date: String,
    // [MM-DD-YYYY]
    end_date: String,
    strict_project: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZohoProjects {
    #[serde(rename = "projects")]
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Project {
    #[serde(rename = "custom_fields")]
    pub custom_fields: Option<Vec<HashMap<String, String>>>,
    #[serde(rename = "created_date")]
    pub created_date: String,
    #[serde(rename = "IS_BUG_ENABLED")]
    pub is_bug_enabled: bool,
    #[serde(rename = "owner_name")]
    pub owner_name: String,
    #[serde(rename = "task_count")]
    pub task_count: Count,
    #[serde(rename = "start_date_long")]
    pub start_date_long: Option<i64>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "link")]
    pub link: HashMap<String, Link>,
    #[serde(rename = "created_date_format")]
    pub created_date_format: String,
    #[serde(rename = "workspace_id")]
    pub workspace_id: String,
    #[serde(rename = "milestone_count")]
    pub milestone_count: Count,
    #[serde(rename = "created_date_long")]
    pub created_date_long: i64,
    #[serde(rename = "end_date_format")]
    pub end_date_format: Option<String>,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "end_date")]
    pub end_date: Option<String>,
    #[serde(rename = "id_string")]
    pub id_string: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "owner_id", deserialize_with = "from_str")]
    pub owner_id: i64,
    #[serde(rename = "end_date_long")]
    pub end_date_long: Option<i64>,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "start_date_format")]
    pub start_date_format: Option<String>,
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Link {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Count {
    #[serde(rename = "open")]
    pub open: i64,
    #[serde(rename = "closed")]
    pub closed: i64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFields {
    #[serde(rename = "project_custom_fields")]
    pub fields: Vec<CustomField>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomField {
    #[serde(rename = "is_visible")]
    pub is_visible: bool,
    #[serde(rename = "field_name")]
    pub field_name: String,
    #[serde(rename = "field_type")]
    pub field_type: String,
    #[serde(rename = "default_value")]
    pub default_value: Option<String>,
    #[serde(rename = "field_id")]
    pub field_id: String,
}
