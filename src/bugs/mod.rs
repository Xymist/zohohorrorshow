use errors::*;
use RelativePath;
use client::ZohoClient;

#[derive(Debug)]
pub struct BugFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

pub trait BugFragmentExt<'a> {
    fn index(self, index: i64) -> BugFragment<'a>;
    fn range(self, range: i64) -> BugFragment<'a>;
    fn status_type(self, status_type: String) -> BugFragment<'a>;
    fn cview_id(self, cview_id: i64) -> BugFragment<'a>;
    fn sort_column(self, sort_column: String) -> BugFragment<'a>;
    fn sort_order(self, sort_order: String) -> BugFragment<'a>;
    fn status(self, status: Vec<String>) -> BugFragment<'a>;
    fn severity(self, severity: Vec<String>) -> BugFragment<'a>;
    fn classification(self, classification: Vec<String>) -> BugFragment<'a>;
    fn module(self, module: Vec<String>) -> BugFragment<'a>;
    fn milestone(self, milestone: Vec<String>) -> BugFragment<'a>;
    fn flag(self, flag: String) -> BugFragment<'a>;
    fn assignee(self, assignee: Vec<String>) -> BugFragment<'a>;
    fn escalation(self, escalation: Vec<String>) -> BugFragment<'a>;
    fn reporter(self, reporter: Vec<String>) -> BugFragment<'a>;
    fn affected(self, affected: Vec<String>) -> BugFragment<'a>;
    fn call(self) -> Vec<Bug>;
}

impl<'a> BugFragmentExt<'a> for BugFragment<'a> {
    // Start index
    fn index(self, index: i64) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&index={}", self.path, index),
        }
    }
    // Number of records (bugs)
    fn range(self, range: i64) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&range={}", self.path, range),
        }
    }
    // Accepted values: open/closed
    fn status_type(self, status_type: String) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&statustype={}", self.path, status_type),
        }
    }
    // Custom View ID
    fn cview_id(self, cview_id: i64) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&cview_id={}", self.path, cview_id),
        }
    }
    // Accepted values: created_time/last_modified_time
    fn sort_column(self, sort_column: String) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&sort_column={}", self.path, sort_column),
        }
    }
    // Accepted values: ascending/descending
    fn sort_order(self, sort_order: String) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&sort_order={}", self.path, sort_order),
        }
    }
    // Status IDs
    fn status(self, status: Vec<String>) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&status=[{}]", self.path, status.join(",")),
        }
    }
    // Severity IDs
    fn severity(self, severity: Vec<String>) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&severity=[{}]", self.path, severity.join(",")),
        }
    }
    // Classification IDs
    fn classification(self, classification: Vec<String>) -> BugFragment<'a> {
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
    fn module(self, module: Vec<String>) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&module=[{}]", self.path, module.join(",")),
        }
    }
    // Milestone IDs
    fn milestone(self, milestone: Vec<String>) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&milestone=[{}]", self.path, milestone.join(",")),
        }
    }
    // Accepted values: Internal/External
    fn flag(self, flag: String) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&flag={}", self.path, flag),
        }
    }
    // Assignee IDs
    fn assignee(self, assignee: Vec<String>) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&assignee=[{}]", self.path, assignee.join(",")),
        }
    }
    // Escalation IDs
    fn escalation(self, escalation: Vec<String>) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&escalation=[{}]", self.path, escalation.join(",")),
        }
    }
    // Reporter IDs
    fn reporter(self, reporter: Vec<String>) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&reporter=[{}]", self.path, reporter.join(",")),
        }
    }
    // Affected milestone IDs
    fn affected(self, affected: Vec<String>) -> BugFragment<'a> {
        BugFragment {
            client: self.client,
            path: format!("{}&affected=[{}]", self.path, affected.join(",")),
        }
    }
    fn call(self) -> Vec<Bug> {
        let bug_list: ZohoBugs = self.client.get_url(&self.path).unwrap();
        bug_list.bugs
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoBugs {
    #[serde(rename = "bugs")]
    pub bugs: Vec<Bug>,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct IntClassification {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "type")]
    pub classification_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrClassification {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub classification_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customfield {
    #[serde(rename = "label_name")]
    pub label_name: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "column_name")]
    pub column_name: String,
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
pub struct Module {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}

impl<'a> RelativePath<[&'a str; 2]> for ZohoBugs {
    fn relative_path(params: [&'a str; 2]) -> Result<String> {
        Ok(format!("portal/{}/projects/{}/bugs/", params[0], params[1]))
    }
}
