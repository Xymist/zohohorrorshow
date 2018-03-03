use errors::*;
use RelativePath;
use client::ZohoClient;

// A fragment of the path to call for the Zoho Tasks API. This carries
// with it a reference to the client which will be used to call it.
#[derive(Debug)]
pub struct TaskFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> TaskFragment<'a> {
    // Index number of the task.
    pub fn index(self, index: i64) -> TaskFragment<'a> {
        TaskFragment {
            client: self.client,
            path: format!("{}&index={}", self.path, index),
        }
    }
    // Range of the task.
    pub fn range(self, range: i64) -> TaskFragment<'a> {
        TaskFragment {
            client: self.client,
            path: format!("{}&range={}", self.path, range),
        }
    }
    // Owner of the task. Defaults to all.
    pub fn owner(self, owner: i64) -> TaskFragment<'a> {
        TaskFragment {
            client: self.client,
            path: format!("{}&owner={}", self.path, owner),
        }
    }
    // Status of the task. Accepts 'all', 'completed' or 'notcompleted'
    pub fn status(self, status: &str) -> TaskFragment<'a> {
        TaskFragment {
            client: self.client,
            path: format!("{}&status={}", self.path, status),
        }
    }
    // Time period of the task. Accepts 'all', 'overdue', 'today' or 'tomorrow'
    pub fn time(self, time: &str) -> TaskFragment<'a> {
        TaskFragment {
            client: self.client,
            path: format!("{}&time={}", self.path, time),
        }
    }
    // Priority of the task. Accepts 'all', 'none', 'low', 'medium', or 'high'.
    pub fn priority(self, priority: &str) -> TaskFragment<'a> {
        TaskFragment {
            client: self.client,
            path: format!("{}&priority={}", self.path, priority),
        }
    }
    // Tasklist_id of the task.
    pub fn tasklist_id(self, tasklist_id: i64) -> TaskFragment<'a> {
        TaskFragment {
            client: self.client,
            path: format!("{}&tasklist_id={}", self.path, tasklist_id),
        }
    }
    // The ID of a custom status for the task
    pub fn custom_status(self, custom_status: i64) -> TaskFragment<'a> {
        TaskFragment {
            client: self.client,
            path: format!("{}&custom_status={}", self.path, custom_status),
        }
    }
    // Fetch a specific task
    pub fn by_id(self, id: i64) -> TaskFragment<'a> {
        if self.path.contains('&') {
            panic!("Cannot both filter and find by ID")
        }
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        TaskFragment {
            client: self.client,
            path: format!("{}{}/?{}", path_frags[0], id, path_frags[1]),
        }
    }
    // Execute the query against the Zoho API
    pub fn call(self) -> Result<Vec<Task>> {
        let task_list: ZohoTasks = self.client.get_url(&self.path)?;
        Ok(task_list.tasks)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoTasks {
    #[serde(rename = "tasks")]
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "completed")]
    pub completed: bool,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "created_person")]
    pub created_person: String,
    #[serde(rename = "priority")]
    pub priority: String,
    #[serde(rename = "percent_complete")]
    pub percent_complete: String,
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    #[serde(rename = "start_date_long")]
    pub start_date_long: Option<i64>,
    #[serde(rename = "end_date")]
    pub end_date: Option<String>,
    #[serde(rename = "end_date_long")]
    pub end_date_long: Option<i64>,
    #[serde(rename = "custom_fields")]
    pub custom_fields: Option<Vec<CustomField>>,
    #[serde(rename = "dependency")]
    pub dependency: Option<Dependency>,
    #[serde(rename = "duration")]
    pub duration: Option<String>,
    #[serde(rename = "details")]
    pub details: Details,
    #[serde(rename = "link")]
    pub link: Link,
    #[serde(rename = "tasklist")]
    pub tasklist: Option<Tasklist>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomField {
    #[serde(rename = "column_name")]
    pub column_name: String,
    #[serde(rename = "label_name")]
    pub label_name: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    #[serde(rename = "successor")]
    pub successor: Vec<String>,
    #[serde(rename = "predecessor")]
    pub predecessor: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    #[serde(rename = "owners")]
    pub owners: Vec<Owner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
    #[serde(rename = "timesheet")]
    pub timesheet: SelfLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfLink {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub status_type: String,
    #[serde(rename = "color_code")]
    pub color_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tasklist {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}

impl<'a> RelativePath<[i64; 2]> for ZohoTasks {
    fn relative_path(params: [i64; 2]) -> String {
        format!("portal/{}/projects/{}/tasks/", params[0], params[1])
    }
}
