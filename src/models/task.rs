use client::ZohoClient;
use errors::*;
use std::rc::Rc;
use utils::from_str;

pub fn tasks(cl: &Rc<ZohoClient>) -> TaskFragment {
    let client = Rc::clone(cl);
    TaskFragment {
        path: client.make_uri(&format!(
            "portal/{}/projects/{}/tasks/",
            client.portal_id(),
            client.project_id()
        )),
        client,
    }
}

// A fragment of the path to call for the Zoho Tasks API. This carries
// with it a reference to the client which will be used to call it.
#[derive(Debug)]
pub struct TaskFragment {
    pub client: Rc<ZohoClient>,
    pub path: String,
}

#[derive(Debug)]
pub enum TaskStatus {
    All,
    Completed,
    NotCompleted,
}

impl TaskStatus {
    pub fn to_string(&self) -> String {
        match *self {
            TaskStatus::All => "all".to_owned(),
            TaskStatus::Completed => "completed".to_owned(),
            TaskStatus::NotCompleted => "notcompleted".to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum TaskTimePeriod {
    All,
    Overdue,
    Today,
    Tomorrow,
}

impl TaskTimePeriod {
    pub fn to_string(&self) -> String {
        match *self {
            TaskTimePeriod::All => "all".to_owned(),
            TaskTimePeriod::Overdue => "overdue".to_owned(),
            TaskTimePeriod::Today => "Today".to_owned(),
            TaskTimePeriod::Tomorrow => "Tomorrow".to_owned(),
        }
    }
}

impl TaskFragment {
    query_strings!(TaskFragment; index, range, owner, priority, tasklist_id, custom_status);

    // Status of the task. Defaults to All
    pub fn status(&mut self, status: &TaskStatus) {
        self.path = format!("{}&status={}", self.path, status.to_string());
    }
    // Time period of the task. Defaults to All
    pub fn time(&mut self, time: &TaskTimePeriod) {
        self.path = format!("{}&time={}", self.path, time.to_string());
    }

    // Fetch a specific task
    pub fn by_id(self, id: i64) -> TaskFragment {
        if self.path.contains('&') {
            panic!("Cannot both filter and find by ID")
        }
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        TaskFragment {
            client: Rc::clone(&self.client),
            path: format!("{}{}/?{}", path_frags[0], id, path_frags[1]),
        }
    }
    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Task>> {
        let task_list: ZohoTasks = self.client.get(&self.path)?;
        Ok(task_list.tasks)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZohoTasks {
    #[serde(rename = "tasks")]
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Task {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "completed")]
    pub completed: bool,
    #[serde(rename = "last_updated_time")]
    pub last_updated_time: String,
    #[serde(rename = "last_updated_time_long")]
    pub last_updated_time_long: i64,
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
    #[serde(default, rename = "tasklist_id", deserialize_with = "from_str")]
    pub tasklist_id: i64,
    #[serde(rename = "tasklist")]
    pub tasklist: Option<Tasklist>,
    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CustomField {
    #[serde(rename = "column_name")]
    pub column_name: String,
    #[serde(rename = "label_name")]
    pub label_name: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dependency {
    #[serde(rename = "successor")]
    pub successor: Option<Vec<String>>,
    #[serde(rename = "predecessor")]
    pub predecessor: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Details {
    #[serde(rename = "owners")]
    pub owners: Vec<Owner>,
}

// Defaults are available here because in the event that a task has no owner, an owner object will be passed but with
// an absent ID and meaningless name.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Owner {
    #[serde(default = "owner_default_name", rename = "name")]
    pub name: String,
    #[serde(
        default = "owner_default_id",
        rename = "id",
        deserialize_with = "from_str"
    )]
    pub id: i64,
}

fn owner_default_name() -> String {
    "Unassigned".to_owned()
}

fn owner_default_id() -> i64 {
    0
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
    #[serde(rename = "timesheet")]
    pub timesheet: SelfLink,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SelfLink {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Status {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id", deserialize_with = "from_str")]
    pub id: i64,
    #[serde(rename = "type")]
    pub status_type: String,
    #[serde(rename = "color_code")]
    pub color_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Tasklist {
    #[serde(rename = "id", deserialize_with = "from_str")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}
