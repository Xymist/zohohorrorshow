use errors::*;
use RelativePath;

// Represents a group of projects. If you pass the ID of the project you want,
// will return that project as the only element of the .projects vector.
#[derive(Debug, Deserialize, Clone)]
pub struct ZohoProjects {
    pub projects: Vec<Project>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BugCount {
    pub closed: i32,
    pub open: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LinkList {
    pub activity: Link,
    pub document: Link,
    pub forum: Link,
    pub timesheet: Link,
    pub task: Link,
    pub folder: Link,
    pub milestone: Link,
    pub bug: Link,
    #[serde(rename = "self")]
    pub self_link: Link,
    pub tasklist: Link,
    pub event: Link,
    pub user: Link,
    pub status: Link,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MilestoneCount {
    pub closed: i32,
    pub open: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TaskCount {
    pub open: i32,
    pub closed: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LayoutDetails {
    pub task: TaskLayout,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TaskLayout {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Project {
    pub is_strict: String,
    pub role: String,
    pub bug_count: BugCount,
    #[serde(rename = "IS_BUG_ENABLED")]
    pub bug_enabled: bool,
    pub owner_id: String,
    pub link: LinkList,
    pub description: Option<String>,
    pub milestone_count: MilestoneCount,
    pub start_date_long: Option<i64>,
    pub updated_date_long: Option<i64>,
    pub task_count: TaskCount,
    pub updated_date_format: Option<String>,
    pub workspace_id: String,
    pub user_disabled_tab: Vec<String>,
    pub billing_status: String,
    pub id: i64,
    pub is_chat_enabled: bool,
    pub start_date: Option<String>,
    pub owner_name: String,
    pub created_date_long: i64,
    pub created_date_format: String,
    pub profile_id: i64,
    pub enabled_tabs: Option<Vec<String>>,
    pub name: String,
    pub is_public: String,
    pub id_string: String,
    pub created_date: String,
    pub updated_date: Option<String>,
    pub bug_prefix: String,
    pub layout_details: LayoutDetails,
    pub status: String,
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
