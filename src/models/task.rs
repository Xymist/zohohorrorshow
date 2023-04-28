use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::from_str;
use std::collections::HashMap;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
) -> String {
    format!("portal/{}/projects/{}/tasks/", portal, project)
}

#[derive(Clone, Debug)]
pub struct TaskRequest {
    details: RequestDetails,
    with_subtasks: bool,
}

impl TaskRequest {
    /// Generate a new TaskRequest
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        TaskRequest {
            details: RequestDetails::new(access_token, model_path, id),
            with_subtasks: false,
        }
    }

    pub fn with_subtasks(mut self) -> Self {
        self.with_subtasks = true;
        self
    }

    /// Return a new TaskIterator, which allows batch iteration across grouped
    /// responses from the Zoho API
    pub fn iter_get(self) -> TaskIterator {
        TaskIterator::new(self)
    }
}

impl ModelRequest for TaskRequest {
    fn uri(&self) -> String {
        self.details.uri()
    }

    fn params(&self) -> Option<HashMap<String, String>> {
        self.details.params()
    }

    fn access_token(&self) -> String {
        self.details.access_token()
    }

    fn filter(mut self, param: (impl FilterOptions + std::fmt::Display)) -> Self {
        self.details = self.details.filter(&param);
        self
    }
}

impl RequestParameters for TaskRequest {
    type ModelCollection = ZohoTasks;
    type NewModel = NewTask;
}

/// Various fields by which a Zoho Task API response may be filtered
pub enum Filter {
    /// The index of the first record to be returned. Useful for pagination.
    Index(usize),
    /// The number of records to be returned, counting from the provided Index if any
    Range(i8),
    /// The owner/creator of the Tasks to be retrieved
    Owner(i64),
    /// The Priority of Tasks to be retrieved
    Priority(String),
    /// The ID of the Tasklist to which the retrieved Tasks must belong
    TasklistId(i64),
    /// The CustomStatus of Tasks to be retrieved
    CustomStatus(String),
    /// The Status of Tasks to be retrieved, from the predefined list
    /// See [TaskStatus] for details
    Status(TaskStatus),
    /// The Time status of the Tasks to be retrieved, from the predefined list
    /// See [TaskTimePeriod] for details
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
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
            Filter::Owner(owner) => owner.to_string(),
            Filter::Priority(priority) => priority.clone(),
            Filter::TasklistId(tasklist_id) => tasklist_id.to_string(),
            Filter::CustomStatus(custom_status) => custom_status.clone(),
            Filter::Status(status) => status.to_string(),
            Filter::Time(time) => time.to_string(),
        };

        write!(f, "{}", str_rep)
    }
}

/// Completion status of Tasks to be called from the API. Used for filtering.
#[derive(Debug)]
pub enum TaskStatus {
    /// All Tasks regardless of status
    All,
    /// Only Tasks marked as Completed
    Completed,
    /// Only Tasks not marked as Completed
    NotCompleted,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            TaskStatus::All => "all",
            TaskStatus::Completed => "completed",
            TaskStatus::NotCompleted => "notcompleted",
        };

        write!(f, "{}", str_rep)
    }
}

// When is this Task due to be completed?
#[derive(Debug)]
pub enum TaskTimePeriod {
    // Ignore this field, return everything
    All,
    // Task end date is already in the past
    Overdue,
    // Task end date is today
    Today,
    // Task end date is tomorrow
    Tomorrow,
}

impl std::fmt::Display for TaskTimePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            TaskTimePeriod::All => "all",
            TaskTimePeriod::Overdue => "overdue",
            TaskTimePeriod::Today => "Today",
            TaskTimePeriod::Tomorrow => "Tomorrow",
        };

        write!(f, "{}", str_rep)
    }
}

// Root node for what the various Task endpoints return.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZohoTasks {
    // A List of tasks, either in total or within a given Tasklist.
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
    #[serde(rename = "subtasks")]
    pub subtasks: bool,
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
    pub subtask_parent_ids: Vec<i64>,
    pub base_path: String,
    pub rate_limit_notified: bool,
}

impl TaskIterator {
    pub fn new(request: TaskRequest) -> TaskIterator {
        TaskIterator {
            items: Vec::new().into_iter(),
            last_full: true,
            start_index: 0,
            subtask_parent_ids: Vec::new(),
            base_path: request.details.model_path.clone(),
            request,
            rate_limit_notified: false,
        }
    }

    pub fn try_next(&mut self) -> Result<Option<Task>> {
        use std::{thread, time};

        // If there are still items in the local cache from the last request, use the next one of those.
        if let Some(task) = self.items.next() {
            // While we're looking at the task, check if we're fetching subtasks
            // and should be expecting subtasks for this task. If we do and are,
            // save the ID of this task for use in the URL later.
            if task.subtasks && self.request.with_subtasks {
                self.subtask_parent_ids.push(task.id)
            }
            return Ok(Some(task));
        }

        // If we didn't get a full 100 (the default number to retrieve) the last time, then we must have
        // run out of top level tasks in Zoho; don't request any more.
        if !self.last_full {
            // If we're not looking for subtasks, we can stop here.
            if !self.request.with_subtasks {
                return Ok(None);
            }

            let subtask_tasks = self.subtask_parent_ids.len();

            if subtask_tasks > 99 && !self.rate_limit_notified {
                eprintln!(
                    "{} tasks have subtasks. This would exceed Zoho's rate limit (100 requests per 120 seconds) \
                    if processed at full speed. Processing will be artificially slowed to that limit.",
                    subtask_tasks
                );
                self.rate_limit_notified = true;
            }

            // Otherwise, start popping IDs off our queue of subtask parents
            // and requesting their children.
            if let Some(id) = self.subtask_parent_ids.pop() {
                self.start_index = 0;
                // FIXME(Xymist): This will crash if this is a single task search.
                // I.e. if an ID was provided to the initial request.
                self.request.details.model_path = format!("{}{}/subtasks/", self.base_path, id)
            } else {
                // We have run out of not only top level tasks, but all subtasks.
                return Ok(None);
            }
        }

        if self.rate_limit_notified {
            thread::sleep(time::Duration::from_millis(1250));
        }

        let returned_tasks = self
            .request
            .clone()
            .filter(Filter::Index(self.start_index))
            .get();

        match returned_tasks {
            Ok(Some(task_list)) => {
                if task_list.tasks.is_empty() {
                    self.last_full = false;
                    return Ok(None);
                }

                self.last_full = task_list.tasks.len() as i8 == self.range();

                self.start_index += task_list.tasks.len();

                self.items = task_list.tasks.into_iter();

                Ok(self.items.next())
            }
            Ok(None) => {
                self.last_full = false;
                Ok(None)
            }
            Err(err) => {
                self.last_full = false;
                Err(dbg!(err))
            }
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
}

impl Iterator for TaskIterator {
    type Item = Result<Task>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(val)) => Some(Ok(val)),
            Ok(None) => None,
            Err(err) => {
                warn!("Fetching Tasks from Zoho experienced an error: {}", err);
                None
            }
        }
    }
}
