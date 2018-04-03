use client::ZohoClient;
use errors::*;
use std::collections::HashMap;
use std::rc::Rc;
use utils::from_str;

pub fn projects(cl: &Rc<ZohoClient>) -> ProjectFragment {
    let client = Rc::clone(cl);
    ProjectFragment {
        path: client.make_uri(&format!("portal/{}/projects/", client.portal_id())),
        client,
    }
}

#[derive(Debug)]
pub struct ProjectFragment {
    pub client: Rc<ZohoClient>,
    pub path: String,
}

impl ProjectFragment {
    query_strings!(ProjectFragment; index, range, status, sort_column, sort_order);

    // Fetch available custom fields (can be applied when creating projects)
    pub fn customfields(self) -> Result<Option<Vec<CustomField>>> {
        let mut path_frags = self.path.split('?').collect::<Vec<&str>>();
        if path_frags[1].contains('&') {
            let autht = path_frags.remove(1).split('&').collect::<Vec<&str>>()[0];
            path_frags.push(autht)
        }
        println!("{:?}", self);
        let fields: CustomFields = self.client.get(&format!(
            "{}{}/?{}",
            path_frags[0], "customfields", path_frags[1]
        ))?;
        Ok(Some(fields.fields))
    }

    // Fetch a specific portal by id
    pub fn by_id(self, id: i64) -> ProjectFilter {
        let mut path_frags = self.path.split('?').collect::<Vec<&str>>();
        if path_frags[1].contains('&') {
            let autht = path_frags.remove(1).split('&').collect::<Vec<&str>>()[0];
            path_frags.push(autht)
        }
        ProjectFilter {
            client: Rc::clone(&self.client),
            path: format!("{}{}/?{}", path_frags[0], id, path_frags[1]),
            filter: Filter::ID(id),
        }
    }
    // Fetch a specific portal by name
    pub fn by_name(self, name: &str) -> ProjectFilter {
        if self.path.contains('&') {
            panic!("Cannot both filter and find by name")
        }
        ProjectFilter {
            client: Rc::clone(&self.client),
            path: self.path,
            filter: Filter::Name(name.to_owned()),
        }
    }
    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Project>> {
        let project_list: ZohoProjects = self.client.get(&self.path)?;
        Ok(project_list.projects)
    }
}

#[derive(Debug)]
enum Filter {
    ID(i64),
    Name(String),
}

#[derive(Debug)]
pub struct ProjectFilter {
    client: Rc<ZohoClient>,
    path: String,
    filter: Filter,
}

impl ProjectFilter {
    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Option<Project>> {
        let project_list: ZohoProjects = self.client.get(&self.path)?;
        let projects = project_list.projects;
        match self.filter {
            Filter::ID(id) => filter_by_id(projects, id),
            Filter::Name(name) => filter_by_name(projects, &name),
        }
    }
}

fn filter_by_id(projects: Vec<Project>, id: i64) -> Result<Option<Project>> {
    let mut filtered = projects
        .into_iter()
        .filter(|p| p.id == id)
        .collect::<Vec<Project>>();
    match filtered.len() {
        0 => Ok(None),
        _ => Ok(Some(filtered.remove(0))),
    }
}

fn filter_by_name(projects: Vec<Project>, name: &str) -> Result<Option<Project>> {
    let mut filtered = projects
        .into_iter()
        .filter(|p| p.name == name)
        .collect::<Vec<Project>>();
    match filtered.len() {
        0 => Ok(None),
        _ => Ok(Some(filtered.remove(0))),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoProjects {
    #[serde(rename = "projects")]
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    #[serde(rename = "open")]
    pub open: i64,
    #[serde(rename = "closed")]
    pub closed: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFields {
    #[serde(rename = "project_custom_fields")]
    pub fields: Vec<CustomField>,
}

#[derive(Debug, Serialize, Deserialize)]
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
