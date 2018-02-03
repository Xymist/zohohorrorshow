use errors::*;
use RelativePath;

#[derive(Debug, Deserialize, Clone)]
pub struct ZohoTasks {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ZohoTasklistTasks {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub completed: bool,
    pub created_by: String,
    pub created_person: String,
    pub priority: String,
    pub percent_complete: String,
    pub start_date: String,
    pub start_date_long: u64,
    pub end_date: String,
    pub end_date_long: u64,
    pub custom_fields: Option<Vec<CustomField>>,
    pub dependency: Option<Dependency>,
    pub duration: String,
    pub details: Details,
    pub link: TaskLink,
    pub tasklist: Option<Tasklist>,
    pub status: Status,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomField {
    pub column_name: String,
    pub label_name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dependency {
    pub successor: Vec<String>,
    pub predecessor: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Details {
    pub owners: Vec<Owner>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Owner {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tasklist {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Status {
    pub name: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub color_code: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TaskLink {
    pub self_link: Option<Link>,
    pub timesheet: Link,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    pub url: String,
}

impl<'a> RelativePath<[&'a str; 2]> for ZohoTasks {
    fn relative_path(params: [&'a str; 2]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}/tasks/",
            params[0], params[1]
        ))
    }
}

impl<'a> RelativePath<[&'a str; 3]> for ZohoTasklistTasks {
    fn relative_path(params: [&'a str; 3]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}/tasklists/{}/tasks/",
            params[0], params[1], params[2]
        ))
    }
}
