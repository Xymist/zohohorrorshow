use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::from_str;
use std::collections::HashMap;
use std::fmt;

pub(crate) fn model_path(portal: impl std::fmt::Display) -> String {
    format!("portal/{}/users/", portal)
}

#[derive(Clone, Debug)]
pub struct PortalUserRequest(RequestDetails);

impl PortalUserRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        PortalUserRequest(RequestDetails::new(access_token, model_path, id))
    }
}

impl ModelRequest for PortalUserRequest {
    fn uri(&self) -> String {
        self.0.uri()
    }

    fn params(&self) -> Option<HashMap<String, String>> {
        self.0.params()
    }

    fn access_token(&self) -> String {
        self.0.access_token()
    }

    fn filter(mut self, param: (impl FilterOptions + std::fmt::Display)) -> Self {
        self.0 = self.0.filter(&param);
        self
    }
}

impl RequestParameters for PortalUserRequest {
    type ModelCollection = ZohoUsers;
    type NewModel = NewUser;
}

// There is exactly one variant of this, "usertype", so the usual enum is
// overkill.
pub struct Filter(String);

impl FilterOptions for Filter {
    fn key(&self) -> String {
        "usertype".to_owned()
    }
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Response {
    response: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AvailCount {
    available_user_count: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZohoUsers {
    #[serde(rename = "users")]
    pub users: Vec<User>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct User {
    #[serde(rename = "id", deserialize_with = "from_str")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "role")]
    pub role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "role")]
    pub role: Role,
    #[serde(rename = "rate")]
    pub rate: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Role {
    Manager,
    Employee,
    Contractor,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role = match self {
            Role::Manager => "manager",
            Role::Employee => "employee",
            Role::Contractor => "contractor",
        };
        write!(f, "{}", role)
    }
}
