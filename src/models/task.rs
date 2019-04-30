use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::from_str;
use std::collections::HashMap;
use crate::errors::*;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
) -> String {
    format!("portal/{}/projects/{}/tasks/", portal, project)
}

#[derive(Clone, Debug)]
pub struct TaskRequest(RequestDetails);

impl TaskRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        TaskRequest(RequestDetails::new(access_token, model_path, id))
    }

    pub fn iter_get(self) -> TaskIterator {
        TaskIterator::new(self)
    }
}

impl ModelRequest for TaskRequest {
    fn uri(&self) -> String {
        self.0.uri()
    }

    fn params(&self) -> Option<HashMap<String, String>> {
        self.0.params()
    }

    fn access_token(&self) -> String {
        self.0.access_token()
    }

    fn filter(mut self, param: impl FilterOptions) -> Self {
        self.0 = self.0.filter(&param);
        self
    }
}

impl RequestParameters for TaskRequest {
    type ModelCollection = ZohoTasks;
    type NewModel = NewTask;
}

pub enum Filter {
    Index(usize),
    Range(i8),
    Owner(i64),
    Priority(String),
    TasklistId(i64),
    CustomStatus(String),
    Status(TaskStatus),
    Time(TaskTimePeriod),
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
            Filter::Owner(_) => "owner".to_owned(),
            Filter::Priority(_) => "priority".to_owned(),
            Filter::TasklistId(_) => "tasklist_id".to_owned(),
            Filter::CustomStatus(_) => "custom_status".to_owned(),
            Filter::Status(_) => "status".to_owned(),
            Filter::Time(_) => "time".to_owned(),
        }
    }

    fn value(&self) -> String {
        match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
            Filter::Owner(owner) => owner.to_string(),
            Filter::Priority(priority) => priority.clone(),
            Filter::TasklistId(tasklist_id) => tasklist_id.to_string(),
            Filter::CustomStatus(custom_status) => custom_status.clone(),
            Filter::Status(status) => status.to_string(),
            Filter::Time(time) => time.to_string(),
        }
    }
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZohoTasks {
    #[serde(rename = "tasks")]
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct NewTask {
    #[serde(rename = "name")]
    pub name: String,
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

#[derive(Debug, Clone)]
pub struct TaskIterator {
    pub items: <Vec<Task> as IntoIterator>::IntoIter,
    pub last_full: bool,
    pub request: TaskRequest,
    pub start_index: usize,
}

impl TaskIterator {
    pub fn new(request: TaskRequest) -> TaskIterator {
        TaskIterator {
            items: Vec::new().into_iter(),
            last_full: true,
            request,
            start_index: 0,
        }
    }

    fn range(&self) -> i8 {
        match self.request.params() {
            Some(params) => match params.get("range") {
                Some(range_string) => range_string.parse::<i8>().unwrap_or(100),
                None => 100,
            },
            None => 100,
        }
    }

    pub fn try_next(&mut self) -> Result<Option<Task>> {
        // If there are still items in the local cache from the last request, use the next one of those.
        if let Some(task) = self.items.next() {
            return Ok(Some(task));
        }

        // If we didn't get a full 100 (the default number to retrieve) the last time, then we must have
        // run out in Zoho; don't request any more.
        if !self.last_full {
            return Ok(None);
        }

        let returned_tasks = self
            .request
            .clone()
            .filter(Filter::Index(self.start_index))
            .get()?;

        if let Some(task_list) = returned_tasks {
            self.last_full = task_list.tasks.len() as i8 == self.range();

            self.start_index += task_list.tasks.len();

            self.items = task_list.tasks.into_iter();

            Ok(self.items.next())
        } else {
            Ok(None)
        }
    }
}

impl Iterator for TaskIterator {
    type Item = Result<Task>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(val)) => Some(Ok(val)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
