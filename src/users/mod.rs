use errors::*;
use RelativePath;

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
