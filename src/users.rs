use errors::*;
use RelativePath;

type ProjectUsers = ZohoUsers;
type PortalUsers = ZohoUsers;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoUsers {
    #[serde(rename = "users")]
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "role")]
    pub role: String,
}

impl<'a> RelativePath<[i64; 2]> for ProjectUsers {
    fn relative_path(params: [i64; 2]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}/users/",
            params[0], params[1]
        ))
    }
}

impl<'a> RelativePath<i64> for PortalUsers {
    fn relative_path(params: i64) -> Result<String> {
        Ok(format!(
            "portal/{}/users/",
            params[0]
        ))
    }
}
