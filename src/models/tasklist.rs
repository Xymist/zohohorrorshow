use client::ZohoClient;
use errors::*;
use models::task::{Task, ZohoTasks};
use std::rc::Rc;
use utils::from_str;

pub fn tasklists(client: Rc<ZohoClient>) -> TasklistFragment {
    TasklistFragment {
        path: client.make_uri(&format!(
            "portal/{}/projects/{}/tasklists/",
            client.portal_id(),
            client.project_id()
        )),
        client,
    }
}

#[derive(Debug)]
pub struct TasklistFragment {
    pub client: Rc<ZohoClient>,
    pub path: String,
}

impl TasklistFragment {
    query_strings!(TasklistFragment; index, range, flag);

    // Designate a specific tasklist. This cannot be used to fetch it,
    // but can be POSTed to in order to update or delete.
    pub fn by_id(self, id: i64) -> TasklistPath {
        if self.path.contains('&') {
            panic!("Cannot both filter and find by ID")
        }
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        TasklistPath {
            client: Rc::clone(&self.client),
            path: format!("{}{}/?{}", path_frags[0], id, path_frags[1]),
        }
    }

    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Tasklist>> {
        if !self.path.contains("flag") {
            bail!(
                "The 'flag' parameter is mandatory. Please call '.flag()'
                with either 'internal' or 'external' before calling."
            )
        }
        let tasklist_list: ZohoTasklists = self.client.get(&self.path)?;
        Ok(tasklist_list.tasklists)
    }
}

#[derive(Debug)]
pub struct TasklistPath {
    pub client: Rc<ZohoClient>,
    pub path: String,
}

impl TasklistPath {
    // Designate a specific tasklist. This cannot be used to fetch it,
    // but can be POSTed to in order to update or delete.
    pub fn tasks(self) -> TasklistTasksPath {
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        TasklistTasksPath {
            client: Rc::clone(&self.client),
            path: format!("{}{}/?{}", path_frags[0], "tasks", path_frags[1]),
        }
    }

    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Option<Tasklist>> {
        let mut tasklist_list: ZohoTasklists = self.client.get(&self.path)?;
        match tasklist_list.tasklists.len() {
            0 => Ok(None),
            _ => Ok(Some(tasklist_list.tasklists.remove(0))),
        }
    }
}

#[derive(Debug)]
pub struct TasklistTasksPath {
    pub client: Rc<ZohoClient>,
    pub path: String,
}

impl TasklistTasksPath {
    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Task>> {
        let task_list: ZohoTasks = self.client.get(&self.path)?;
        Ok(task_list.tasks)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoTasklists {
    #[serde(rename = "tasklists")]
    pub tasklists: Vec<Tasklist>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub view_type: Option<String>,
    #[serde(rename = "link")]
    pub link: TasklistLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TasklistLink {
    #[serde(rename = "self")]
    pub link: Link,
    #[serde(rename = "task")]
    pub task: Link,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "link")]
    pub link: MilestoneLink,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "owner_name")]
    pub owner_name: String,
    #[serde(rename = "owner_id", deserialize_with = "from_str")]
    pub owner_id: i64,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MilestoneLink {
    #[serde(rename = "self")]
    pub link: Link,
    #[serde(rename = "status")]
    pub status: Link,
}
