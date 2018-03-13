use errors::*;
use client::ZohoClient;
use std::fmt::Display;

#[derive(Debug)]
pub struct BugFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> BugFragment<'a> {
    query_strings!(BugFragment; index, range, status_type, cview_id, sort_column, sort_order, flag);
    query_groups!(BugFragment; status, severity, classification, module, milestone, assignee, escalation, reporter, affected);
    // Fetch a specific bug
    pub fn by_id(self, id: i64) -> BugFragment<'a> {
        if self.path.contains('&') {
            panic!("Cannot both filter and find by ID")
        }
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        BugFragment {
            client: self.client,
            path: format!("{}{}/?{}", path_frags[0], id, path_frags[1]),
        }
    }
    // Execute the query against the Zoho API
    pub fn call(self) -> Result<Vec<Bug>> {
        let bug_list: ZohoBugs = self.client.get(&self.path)?;
        Ok(bug_list.bugs)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoBugs {
    #[serde(rename = "bugs")]
    pub bugs: Vec<Bug>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
    #[serde(rename = "reporter_id")]
    pub reporter_id: String,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IntClassification {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "type")]
    pub classification_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StrClassification {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub classification_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Customfield {
    #[serde(rename = "label_name")]
    pub label_name: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "column_name")]
    pub column_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
    #[serde(rename = "timesheet")]
    pub timesheet: SelfLink,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SelfLink {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Module {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}
