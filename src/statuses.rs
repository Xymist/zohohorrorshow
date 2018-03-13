use errors::*;
use client::ZohoClient;

#[derive(Debug)]
pub struct StatusFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> StatusFragment<'a> {
    query_strings!(StatusFragment; index, range);

    // Execute the query against the Zoho API
    pub fn call(self) -> Result<Vec<Status>> {
        let status_list: ZohoStatuses = self.client.get(&self.path)?;
        Ok(status_list.statuses)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoStatuses {
    #[serde(rename = "statuses")]
    pub statuses: Vec<Status>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "posted_by")]
    pub posted_by: String,
    #[serde(rename = "posted_person")]
    pub posted_person: String,
    #[serde(rename = "posted_time")]
    pub posted_time: String,
    #[serde(rename = "posted_time_long")]
    pub posted_time_long: i64,
}
