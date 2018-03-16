use errors::*;
use client::ZohoClient;
use utils::from_str;

#[derive(Debug)]
pub struct CommentFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoComments {
  comments: Vec<Comment>,
}

impl<'a> CommentFragment<'a> {
    // Execute the query against the Zoho API
    pub fn call(self) -> Result<Vec<Comment>> {
        let comment_list: ZohoComments = self.client.get(&self.path)?;
        Ok(comment_list.comments)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "posted_by", deserialize_with = "from_str")]
    pub posted_by: i64,
    #[serde(rename = "posted_person")]
    pub posted_person: String,
    #[serde(rename = "posted_time")]
    pub posted_time: String,
    #[serde(rename = "posted_time_long")]
    pub posted_time_long: i64,
}