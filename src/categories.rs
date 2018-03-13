use errors::*;
use client::ZohoClient;

#[derive(Debug)]
pub struct CategoryFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoCategories {
    #[serde(rename = "forums")]
    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "id")]
    pub id: i64,
    pub name: String,
}