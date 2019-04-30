use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::from_str;
use std::collections::HashMap;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
) -> String {
    format!("portal/{}/projects/{}/bugs/", portal, project)
}

#[derive(Clone, Debug)]
pub struct BugRequest(RequestDetails);

impl BugRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        BugRequest(RequestDetails::new(access_token, model_path, id))
    }

    pub fn iter_get(self) -> BugIterator {
        BugIterator::new(self)
    }
}

impl ModelRequest for BugRequest {
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

impl RequestParameters for BugRequest {
    type ModelCollection = ZohoBugs;
    type NewModel = NewBug;
}

pub enum Flag {
    Internal,
    External,
}

impl Flag {
    pub fn value(&self) -> String {
        match self {
            Flag::External => "External".to_owned(),
            Flag::Internal => "Internal".to_owned(),
        }
    }
}

pub enum Filter {
    Index(usize),
    // Zoho only accepts ranges up to 100, no point in this being any bigger.
    Range(i8),
    StatusType(StatusType),
    CViewId(String),
    SortColumn(SortColumn),
    SortOrder(SortOrder),
    Flag(Flag),
    Status(Vec<i64>),
    Severity(Vec<i64>),
    Classification(Vec<i64>),
    Module(Vec<i64>),
    Milestone(Vec<i64>),
    Assignee(Vec<i64>),
    Escalation(Vec<i64>),
    Reporter(Vec<i64>),
    Affected(Vec<i64>),
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
            Filter::StatusType(_) => "statustype".to_owned(),
            Filter::CViewId(_) => "cview_id".to_owned(),
            Filter::SortColumn(_) => "sort_column".to_owned(),
            Filter::SortOrder(_) => "sort_order".to_owned(),
            Filter::Flag(_) => "flag".to_owned(),
            Filter::Status(_) => "status".to_owned(),
            Filter::Severity(_) => "severity".to_owned(),
            Filter::Classification(_) => "classification".to_owned(),
            Filter::Module(_) => "module".to_owned(),
            Filter::Milestone(_) => "milestone".to_owned(),
            Filter::Assignee(_) => "assignee".to_owned(),
            Filter::Escalation(_) => "escalation".to_owned(),
            Filter::Reporter(_) => "reporter".to_owned(),
            Filter::Affected(_) => "affected".to_owned(),
        }
    }

    fn value(&self) -> String {
        match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
            Filter::StatusType(status_type) => status_type.to_string(),
            Filter::CViewId(cview_id) => cview_id.to_owned(),
            Filter::SortColumn(sort_column) => sort_column.to_string(),
            Filter::SortOrder(sort_order) => sort_order.to_string(),
            Filter::Flag(flag) => flag.value(),
            Filter::Status(status) => status
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
            Filter::Severity(severity) => severity
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
            Filter::Classification(classification) => classification
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
            Filter::Module(module) => module
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
            Filter::Milestone(milestone) => milestone
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
            Filter::Assignee(assignee) => assignee
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
            Filter::Escalation(escalation) => escalation
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
            Filter::Reporter(reporter) => reporter
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
            Filter::Affected(affected) => affected
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(","),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ZohoBugs {
    #[serde(rename = "bugs")]
    pub bugs: Vec<Bug>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Bug {
    #[serde(rename = "module")]
    pub module: Module,
    #[serde(rename = "created_time_long")]
    pub created_time_long: i64,
    #[serde(rename = "customfields")]
    pub customfields: Option<Vec<Customfield>>,
    #[serde(rename = "status")]
    pub status: StrClassification,
    #[serde(rename = "reproducible")]
    pub reproducible: IntClassification,
    #[serde(rename = "link")]
    pub link: Link,
    #[serde(rename = "severity")]
    pub severity: IntClassification,
    #[serde(rename = "reported_person")]
    pub reported_person: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "flag")]
    pub flag: String,
    #[serde(rename = "assignee_name")]
    pub assignee_name: String,
    #[serde(rename = "reporter_id", deserialize_with = "from_str")]
    pub reporter_id: i64,
    #[serde(rename = "classification")]
    pub classification: IntClassification,
    #[serde(rename = "created_time_format")]
    pub created_time_format: String,
    #[serde(rename = "closed")]
    pub closed: bool,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "key")]
    pub key: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct NewBug {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct IntClassification {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "type")]
    pub classification_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct StrClassification {
    #[serde(rename = "id", deserialize_with = "from_str")]
    pub id: i64,
    #[serde(rename = "type")]
    pub classification_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Customfield {
    #[serde(rename = "label_name")]
    pub label_name: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "column_name")]
    pub column_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
    #[serde(rename = "timesheet")]
    pub timesheet: SelfLink,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct SelfLink {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Module {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn to_string(&self) -> String {
        match self {
            SortOrder::Ascending => "ascending".to_owned(),
            SortOrder::Descending => "descending".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SortColumn {
    CreatedTime,
    LastModifiedTime,
}

impl SortColumn {
    pub fn to_string(&self) -> String {
        match self {
            SortColumn::CreatedTime => "created_time".to_owned(),
            SortColumn::LastModifiedTime => "last_modified_time".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StatusType {
    Open,
    Closed,
}

impl StatusType {
    pub fn to_string(&self) -> String {
        match self {
            StatusType::Open => "open".to_owned(),
            StatusType::Closed => "closed".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BugIterator {
    pub items: <Vec<Bug> as IntoIterator>::IntoIter,
    pub last_full: bool,
    pub request: BugRequest,
    pub start_index: usize,
}

impl BugIterator {
    pub fn new(request: BugRequest) -> BugIterator {
        BugIterator {
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

    pub fn try_next(&mut self) -> Result<Option<Bug>> {
        // If there are still items in the local cache from the last request, use the next one of those.
        if let Some(bug) = self.items.next() {
            return Ok(Some(bug));
        }

        // If we didn't get a full 100 (the default number to retrieve) the last time, then we must have
        // run out in Zoho; don't request any more.
        if !self.last_full {
            return Ok(None);
        }

        let returned_tickets = self
            .request
            .clone()
            .filter(Filter::Index(self.start_index))
            .get()?;

        if let Some(ticket_list) = returned_tickets {
            self.last_full = ticket_list.bugs.len() as i8 == self.range();

            self.start_index += ticket_list.bugs.len();

            self.items = ticket_list.bugs.into_iter();

            Ok(self.items.next())
        } else {
            Ok(None)
        }
    }
}

impl Iterator for BugIterator {
    type Item = Result<Bug>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(val)) => Some(Ok(val)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
