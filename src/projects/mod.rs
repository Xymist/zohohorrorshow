use errors::*;
use RelativePath;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoProjects {
    #[serde(rename = "projects")]
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "custom_fields")]
    pub custom_fields: Vec<CustomField>,
    #[serde(rename = "created_date")]
    pub created_date: String,
    #[serde(rename = "IS_BUG_ENABLED")]
    pub is_bug_enabled: bool,
    #[serde(rename = "owner_name")]
    pub owner_name: String,
    #[serde(rename = "task_count")]
    pub task_count: Count,
    #[serde(rename = "start_date_long")]
    pub start_date_long: i64,
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
    pub end_date_format: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "id_string")]
    pub id_string: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    #[serde(rename = "end_date_long")]
    pub end_date_long: i64,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "start_date_format")]
    pub start_date_format: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
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
impl<'a> RelativePath<&'a str> for ZohoProjects {
    fn relative_path(portal_id: &str) -> Result<String> {
        Ok(format!("portal/{}/projects/", portal_id))
    }
}

// Return the Project for a Portal and Project ID. This actually returns a
// single-element ZohoProjects object, because whoever designed this API
// was insane.
impl<'a> RelativePath<[&'a str; 2]> for ZohoProjects {
    fn relative_path(portal_project: [&str; 2]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}/",
            portal_project[0], portal_project[1]
        ))
    }
}
