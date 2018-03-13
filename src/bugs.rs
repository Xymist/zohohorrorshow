use errors::*;
use client::ZohoClient;

#[derive(Debug)]
pub struct BugFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> BugFragment<'a> {
    // Start index
    pub fn index(self, index: i64) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&index={}", self.path, index),
        }
    }
    // Number of records (bugs)
    pub fn range(self, range: i64) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&range={}", self.path, range),
        }
    }
    // Accepted values: open/closed
    pub fn status_type(self, status_type: &str) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&statustype={}", self.path, status_type),
        }
    }
    // Custom View ID
    pub fn cview_id(self, cview_id: i64) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&cview_id={}", self.path, cview_id),
        }
    }
    // Accepted values: created_time/last_modified_time
    pub fn sort_column(self, sort_column: &str) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&sort_column={}", self.path, sort_column),
        }
    }
    // Accepted values: ascending/descending
    pub fn sort_order(self, sort_order: &str) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&sort_order={}", self.path, sort_order),
        }
    }
    // Status IDs
    pub fn status(self, status: &[&str]) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&status=[{}]", self.path, status.join(",")),
        }
    }
    // Severity IDs
    pub fn severity(self, severity: &[&str]) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&severity=[{}]", self.path, severity.join(",")),
        }
    }
    // Classification IDs
    pub fn classification(self, classification: &[&str]) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!(
                "{}&classification=[{}]",
                self.path,
                classification.join(",")
            ),
        }
    }
    // Module IDs
    pub fn module(self, module: &[&str]) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&module=[{}]", self.path, module.join(",")),
        }
    }
    // Milestone IDs
    pub fn milestone(self, milestone: &[&str]) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&milestone=[{}]", self.path, milestone.join(",")),
        }
    }
    // Accepted values: Internal/External
    pub fn flag(self, flag: &str) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&flag={}", self.path, flag),
        }
    }
    // Assignee IDs
    pub fn assignee(self, assignee: &[&str]) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&assignee=[{}]", self.path, assignee.join(",")),
        }
    }
    // Escalation IDs
    pub fn escalation(self, escalation: &[&str]) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&escalation=[{}]", self.path, escalation.join(",")),
        }
    }
    // Reporter IDs
    pub fn reporter(self, reporter: &[&str]) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&reporter=[{}]", self.path, reporter.join(",")),
        }
    }
    // Affected milestone IDs
    pub fn affected(self, affected: &[&str]) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&affected=[{}]", self.path, affected.join(",")),
        }
    }
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
    pub customfields: Vec<Customfield>,
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
