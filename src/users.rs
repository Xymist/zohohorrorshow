use errors::*;
use client::ZohoClient;
use utils::from_str;

// A fragment of the path to call for the Zoho Users API. This carries
// with it a reference to the client which will be used to call it.
#[derive(Debug)]
pub struct UserFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoUsers {
    #[serde(rename = "users")]
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
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
