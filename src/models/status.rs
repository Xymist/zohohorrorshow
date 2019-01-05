use crate::client::ZohoClient;
use crate::errors::*;

pub const ModelPath: &str = "portal/{}/projects/{}/statuses/";

pub fn statuses(cl: &ZohoClient) -> StatusFragment {
    let client = cl.clone();
    StatusFragment {
        path: client.make_uri(&format!(
            "portal/{}/projects/{}/statuses/",
            client.portal_id(),
            client.project_id()
        )),
        client,
    }
}

#[derive(Debug)]
pub struct StatusFragment {
    pub client: ZohoClient,
    pub path: String,
}

impl StatusFragment {
    query_strings!(index, range);

    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Status>> {
        let status_list: ZohoStatuses = self.client.get(&self.path)?;
        Ok(status_list.statuses)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZohoStatuses {
    #[serde(rename = "statuses")]
    pub statuses: Vec<Status>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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
