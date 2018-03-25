use client::ZohoClient;
use errors::*;
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
    query_strings!(CommentFragment; index, range);

    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Comment>> {
        let comment_list: ZohoComments = self.client.get(&self.path)?;
        Ok(comment_list.comments)
    }
    pub fn create(self, content: &str) -> Result<Comment> {
        let mut response: ZohoComments = self.client
            .post(&format!("{}&content={}", self.path, content), "")?;
        Ok(response.comments.remove(0))
    }
    // Delete a comment by ID
    pub fn delete(self, id: i64) -> Result<String> {
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        let response: Response = self.client
            .delete(&format!("{}{}/?{}", path_frags[0], id, path_frags[1]))?;
        Ok(response.response)
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    response: String,
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
