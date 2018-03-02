use errors::*;
use RelativePath;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoForums {
    #[serde(rename = "forums")]
    pub forums: Vec<Forum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoCategories {
    #[serde(rename = "forums")]
    pub categories: Vec<Category>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "id")]
    pub id: i64,
    pub name: String,
}

impl<'a> RelativePath<[i64; 2]> for ZohoForums {
    fn relative_path(params: [i64; 2]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}/forums/",
            params[0], params[1]
        ))
    }
}

impl<'a> RelativePath<[i64; 2]> for ZohoCategories {
    fn relative_path(params: [i64; 2]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}/categories/",
            params[0], params[1]
        ))
    }
}
