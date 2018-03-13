use errors::*;
use client::ZohoClient;

#[derive(Debug)]
pub struct ForumFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoForums {
    #[serde(rename = "forums")]
    pub forums: Vec<Forum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "is_sticky_post")]
    pub is_sticky_post: bool,
    #[serde(rename = "is_announcement_post")]
    pub is_announcement_post: bool,
    #[serde(rename = "posted_by")]
    pub posted_by: String,
    #[serde(rename = "posted_person")]
    pub posted_person: String,
    #[serde(rename = "post_date")]
    pub post_date: String,
    #[serde(rename = "post_date_long")]
    pub post_date_long: i64,
    #[serde(rename = "link")]
    pub link: Link,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfLink {
    #[serde(rename = "url")]
    pub url: String,
}
