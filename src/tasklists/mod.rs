use errors::*;
use RelativePath;
use client::ZohoClient;

#[derive(Debug)]
pub struct TasklistFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> TasklistFragment<'a> {
    // Designate a specific tasklist. This cannot be used to fetch it,
    // but can be POSTed to in order to update or delete.
    pub fn by_id(self, id: i64) -> TasklistFragment<'a> {
        if self.path.contains("&") {
            panic!("Cannot both filter and find by ID")
        }
        let path_frags = self.path.split("?").collect::<Vec<&str>>();
        TasklistFragment {
            client: self.client,
            path: format!("{}{}/?{}", path_frags[0], id, path_frags[1]),
        }
    }

    // Execute the query against the Zoho API
    pub fn call(self) -> Vec<Tasklist> {
        let tasklist_list: ZohoTasklists = self.client.get_url(&self.path).unwrap();
        tasklist_list.tasklists
    }
}

#[derive(Serialize, Deserialize)]
pub struct ZohoTasklists {
    #[serde(rename = "tasklists")]
    pub tasklists: Vec<Tasklist>,
}

#[derive(Serialize, Deserialize)]
pub struct Tasklist {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "milestone")]
    pub milestone: Milestone,
    #[serde(rename = "completed")]
    pub completed: bool,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "created_time_long")]
    pub created_time_long: i64,
    #[serde(rename = "rolled")]
    pub rolled: bool,
    #[serde(rename = "sequence")]
    pub sequence: i64,
    #[serde(rename = "view_type")]
    pub view_type: String,
    #[serde(rename = "link")]
    pub link: TasklistLink,
}

#[derive(Serialize, Deserialize)]
pub struct TasklistLink {
    #[serde(rename = "self")]
    pub link: Link,
    #[serde(rename = "task")]
    pub task: Link,
}

#[derive(Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Milestone {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "link")]
    pub link: MilestoneLink,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "owner_name")]
    pub owner_name: String,
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    #[serde(rename = "flag")]
    pub flag: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "start_date_long")]
    pub start_date_long: i64,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "end_date_long")]
    pub end_date_long: i64,
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct MilestoneLink {
    #[serde(rename = "self")]
    pub link: Link,
    #[serde(rename = "status")]
    pub status: Link,
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
