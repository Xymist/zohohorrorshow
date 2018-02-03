use errors::*;
use RelativePath;

#[derive(Deserialize, Debug, Clone)]
pub struct ZohoBugs {
    pub milestone: Option<String>,
    pub issue_labels: Option<Vec<String>>,
    pub bugs: Option<Vec<Bug>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Bug {
    pub key: Option<String>,
    pub title: Option<String>,
    pub milestone: Option<BugMilestone>,
    pub customfields: Option<Vec<CustomField>>,
    pub status: Option<Status>,
    pub classification: Option<Classification>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Classification {
    pub id: Option<u64>,
    #[serde(rename = "type")]
    pub type_name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Status {
    pub color_code: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CustomField {
    pub column_name: String,
    pub label_name: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BugMilestone {
    pub name: String,
    pub id: String,
}

impl<'a> RelativePath<[&'a str; 2]> for ZohoBugs {
    fn relative_path(params: [&'a str; 2]) -> Result<String> {
        Ok(format!("portal/{}/projects/{}/bugs/", params[0], params[1]))
    }
}
