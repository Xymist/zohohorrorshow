use errors::*;
use client::ZohoClient;

// A fragment of the path to call for the Zoho Tasks API. This carries
// with it a reference to the client which will be used to call it.
#[derive(Debug)]
pub struct TaskFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

#[derive(Debug)]
pub enum TaskStatus {
    All,
    Completed,
    NotCompleted,
}

impl TaskStatus {
    pub fn to_string(self) -> String {
        match self {
            TaskStatus::All => "all".to_string(),
            TaskStatus::Completed => "completed".to_string(),
            TaskStatus::NotCompleted => "notcompleted".to_string(),
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
    pub fn to_string(self) -> String {
        match self {
            TaskTimePeriod::All => "all".to_string(),
            TaskTimePeriod::Overdue => "overdue".to_string(),
            TaskTimePeriod::Today => "Today".to_string(),
            TaskTimePeriod::Tomorrow => "Tomorrow".to_string(),
        }
    }
}

impl<'a> TaskFragment<'a> {
    query_strings!(TaskFragment; index, range, owner, priority, tasklist_id, custom_status);

    // Status of the task. Defaults to All
    pub fn status(&mut self, status: TaskStatus) {
        self.path = format!("{}&status={}", self.path, status.to_string());
    }
    // Time period of the task. Defaults to All
    pub fn time(&mut self, time: TaskTimePeriod) {
        self.path = format!("{}&time={}", self.path, time.to_string());
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
        let task_list: ZohoTasks = self.client.get(&self.path)?;
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
