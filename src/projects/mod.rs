use errors::*;
use RelativePath;

#[derive(Debug, Deserialize)]
pub struct ZohoProjects {
    projects: Vec<Project>
}

#[derive(Debug, Deserialize)]
pub struct BugCount {
    closed: i32,
    open: i32,
}

#[derive(Debug, Deserialize)]
pub struct LinkList {
    activity: Link,
    document: Link,
    forum: Link,
    timesheet:  Link,
    task: Link,
    folder: Link,
    milestone: Link,
    bug: Link,
    #[serde(rename = "self")]
    self_link: Link,
    tasklist: Link,
    event: Link,
    user: Link,
    status: Link,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct MilestoneCount {
    closed: i32,
    open: i32,
}

#[derive(Debug, Deserialize)]
pub struct TaskCount {
    open: i32,
    closed: i32,
}

#[derive(Debug, Deserialize)]
pub struct LayoutDetails {
     task: TaskLayout
}

#[derive(Debug, Deserialize)]
pub struct TaskLayout {
    name: String,
    id: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    is_strict: String,
    role: String,
    bug_count: BugCount,
    #[serde(rename = "IS_BUG_ENABLED")]
    bug_enabled: bool,
    owner_id: String,
    link: LinkList,
    description: Option<String>,
    milestone_count: MilestoneCount,
    start_date_long: Option<i64>,
    updated_date_long: Option<i64>,
    task_count: TaskCount,
    updated_date_format: Option<String>,
    workspace_id: String,
    user_disabled_tab: Vec<String>,
    billing_status: String,
    id: i64,
    is_chat_enabled: bool,
    start_date: Option<String>,
    owner_name: String,
    created_date_long: i64,
    created_date_format: String,
    profile_id: i64,
    enabled_tabs: Option<Vec<String>>,
    name: String,
    is_public: String,
    id_string: String,
    created_date: String,
    updated_date: Option<String>,
    bug_prefix: String,
    layout_details: LayoutDetails,
    status: String,
}

impl<'a> RelativePath<&'a str> for ZohoProjects {
    fn relative_path(portal_id: &str) -> Result<String> {
        Ok(format!("portal/{}/projects/", portal_id))
    }
}

impl<'a> RelativePath<[&'a str; 2]> for Project {
    fn relative_path(portal_project: [&str; 2]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}",
            portal_project[0], portal_project[1]
        ))
    }
}
