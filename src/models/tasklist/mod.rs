use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::from_str;
use std::collections::HashMap;

pub mod task;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
) -> String {
    format!("portal/{}/projects/{}/tasklists/", portal, project)
}

#[derive(Clone, Debug)]
pub struct TasklistRequest(RequestDetails);

impl TasklistRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        TasklistRequest(RequestDetails::new(access_token, model_path, id))
    }

    pub fn iter_get(self) -> TasklistIterator {
        TasklistIterator::new(self)
    }
}

impl ModelRequest for TasklistRequest {
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

impl RequestParameters for TasklistRequest {
    type ModelCollection = ZohoTasklists;
    type NewModel = NewTasklist;
}

pub enum Filter {
    Index(usize),
    Range(i8),
    Flag(Flag),
    Milestone(usize),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Flag {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
}

impl Flag {
    pub fn to_string(&self) -> String {
        match self {
            Flag::Internal => "internal".to_owned(),
            Flag::External => "external".to_owned(),
        }
    }
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
            Filter::Flag(_) => "flag".to_owned(),
            Filter::Milestone(_) => "milestone_id".to_owned(),
        }
    }

    fn value(&self) -> String {
        match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
            Filter::Flag(flag) => flag.to_string(),
            Filter::Milestone(id) => id.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZohoTasklists {
    #[serde(rename = "tasklists")]
    pub tasklists: Vec<Tasklist>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewTasklist {
    name: String,
    milestone_id: usize,
    flag: Flag,
}

// TODO(Xymist): Implement Tasklist::tasks() to create a new request to fetch all tasks for a tasklist
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TasklistLink {
    #[serde(rename = "self")]
    pub link: Link,
    #[serde(rename = "task")]
    pub task: Link,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Link {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MilestoneLink {
    #[serde(rename = "self")]
    pub link: Link,
    #[serde(rename = "status")]
    pub status: Link,
}

#[derive(Clone, Debug)]
pub struct TasklistIterator {
    pub items: <Vec<Tasklist> as IntoIterator>::IntoIter,
    pub last_full: bool,
    pub request: TasklistRequest,
    pub start_index: usize,
}

impl TasklistIterator {
    pub fn new(request: TasklistRequest) -> TasklistIterator {
        TasklistIterator {
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

    pub fn try_next(&mut self) -> Result<Option<Tasklist>> {
        // If there are still items in the local cache from the last request, use the next one of those.
        if let Some(tasklist) = self.items.next() {
            return Ok(Some(tasklist));
        }

        // If we didn't get a full 100 (the default number to retrieve) the last time, then we must have
        // run out in Zoho; don't request any more.
        if !self.last_full {
            return Ok(None);
        }

        let returned_tasklists = self
            .request
            .clone()
            .filter(Filter::Index(self.start_index))
            .get()?;

        if let Some(tasklist_list) = returned_tasklists {
            self.last_full = tasklist_list.tasklists.len() as i8 == self.range();

            self.start_index += tasklist_list.tasklists.len();

            self.items = tasklist_list.tasklists.into_iter();

            Ok(self.items.next())
        } else {
            Ok(None)
        }
    }
}

impl Iterator for TasklistIterator {
    type Item = Result<Tasklist>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(val)) => Some(Ok(val)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
