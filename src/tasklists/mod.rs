use errors::*;
use RelativePath;
use tasks::Task;

#[derive(Debug, Deserialize, Clone)]
pub struct ZohoTasklists {
    pub tasklists: Vec<Tasklist>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tasklist {
    pub id: u64,
    pub name: String,
    pub milestone: TaskListMilestone,
    pub completed: bool,
    pub created_time: String,
    pub created_time_long: u64,
    pub rolled: bool,
    pub sequence: i32,
    pub view_type: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TaskListMilestone {
    pub id: u64,
    pub link: TaskListMilestoneLink,
    pub name: String,
    pub owner_name: String,
    pub owner_id: String,
    pub flag: String,
    pub start_date: String,
    pub start_date_long: u64,
    pub end_date: String,
    pub end_date_long: u64,
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TaskListMilestoneLink {
    #[serde(rename = "self")]
    self_link: Option<Link>,
    status: Link,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TasklistLink {
    #[serde(rename = "self")]
    self_link: Option<Link>,
    task: Link,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    pub url: String,
}

// Requires a query of either &flag=internal or &flag=external
impl<'a> RelativePath<[&'a str; 2]> for ZohoTasklists {
    fn relative_path(params: [&'a str; 2]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}/tasklists/",
            params[0], params[1]
        ))
    }
}
