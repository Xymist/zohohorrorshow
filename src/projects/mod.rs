use errors::*;
use RelativePath;
use client::ZohoClient;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ProjectFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> ProjectFragment<'a> {
    // Index number of the project.
    pub fn index(self, index: i64) -> ProjectFragment<'a> {
        ProjectFragment {
            client: self.client,
            path: format!("{}&index={}", self.path, index),
        }
    }
    // Range of the project.
    pub fn range(self, range: i64) -> ProjectFragment<'a> {
        ProjectFragment {
            client: self.client,
            path: format!("{}&range={}", self.path, range),
        }
    }
    // Status of the project - active, archive or template
    pub fn status(self, status: String) -> ProjectFragment<'a> {
        ProjectFragment {
            client: self.client,
            path: format!("{}&status={}", self.path, status),
        }
    }
    // Sort projects using the last modified time or time of creation.
    // created_time or last_modified_time
    pub fn sort_column(self, sort_column: String) -> ProjectFragment<'a> {
        ProjectFragment {
            client: self.client,
            path: format!("{}&sort_column={}", self.path, sort_column),
        }
    }
    // Sort order - ascending or descending
    pub fn sort_order(self, sort_order: String) -> ProjectFragment<'a> {
        ProjectFragment {
            client: self.client,
            path: format!("{}&sort_order={}", self.path, sort_order),
        }
    }
    // Fetch a specific portal
    pub fn by_id(self, id: i64) -> ProjectFragment<'a> {
        if self.path.contains("&") {
            panic!("Cannot both filter and find by ID")
        }
        let path_frags = self.path.split("?").collect::<Vec<&str>>();
        ProjectFragment {
            client: self.client,
            path: format!("{}{}/?{}", path_frags[0], id, path_frags[1]),
        }
    }
    // Execute the query against the Zoho API
    pub fn call(self) -> Vec<Project> {
        let project_list: ZohoProjects = self.client.get_url(&self.path).unwrap();
        project_list.projects
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
    pub custom_fields: Option<Vec<CustomField>>,
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
    #[serde(rename = "owner_id")]
    pub owner_id: String,
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
pub struct CustomField {
    #[serde(rename = "Template design")]
    pub template_design: Option<String>,
    #[serde(rename = "Promos per second")]
    pub promos_per_second: Option<String>,
    #[serde(rename = "Blog announcement")]
    pub blog_announcement: Option<String>,
    #[serde(rename = "Promo publish date")]
    pub promo_publish_date: Option<String>,
    #[serde(rename = "Content approval")]
    pub content_approval: Option<String>,
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

// Return all Projects for a Portal
impl RelativePath<i64> for ZohoProjects {
    fn relative_path(portal_id: i64) -> Result<String> {
        Ok(format!("portal/{}/projects/", portal_id))
    }
}
