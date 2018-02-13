use errors::*;
use RelativePath;

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
    pub status: Classification,
    #[serde(rename = "reproducible")]
    pub reproducible: Classification,
    #[serde(rename = "link")]
    pub link: Link,
    #[serde(rename = "severity")]
    pub severity: Classification,
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
    pub classification: Classification,
    #[serde(rename = "created_time_format")]
    pub created_time_format: String,
    #[serde(rename = "closed")]
    pub closed: bool,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "key")]
    pub key: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classification {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "type")]
    pub purple_type: String,
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
