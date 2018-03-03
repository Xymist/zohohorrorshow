use errors::*;
use RelativePath;
use client::ZohoClient;

#[derive(Debug)]
pub struct StatusFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> StatusFragment<'a> {
    // Index number of the status.
    pub fn index(self, index: i64) -> StatusFragment<'a> {
        StatusFragment {
            client: self.client,
            path: format!("{}&index={}", self.path, index),
        }
    }
    // Range of the status.
    pub fn range(self, range: i64) -> StatusFragment<'a> {
        StatusFragment {
            client: self.client,
            path: format!("{}&range={}", self.path, range),
        }
    }
    // Execute the query against the Zoho API
    pub fn call(self) -> Result<Vec<Status>> {
        let status_list: ZohoStatuses = self.client.get_url(&self.path)?;
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

impl<'a> RelativePath<[i64; 2]> for ZohoStatuses {
    fn relative_path(params: [i64; 2]) -> String {
        format!("portal/{}/projects/{}/statuses/", params[0], params[1])
    }
}
