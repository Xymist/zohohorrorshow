use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters, ZohoRequest};
use reqwest::Method;
use std::collections::HashMap;

pub mod comment;

pub fn model_path(portal: impl std::fmt::Display, project: impl std::fmt::Display) -> String {
    format!("portal/{}/projects/{}/forums/", portal, project)
}

pub struct ForumRequest(RequestDetails);

impl ForumRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        ForumRequest(RequestDetails::new(access_token, model_path, id))
    }
}

impl ModelRequest for ForumRequest {
    fn uri(&self) -> String {
        self.0.uri()
    }

    fn params(&self) -> Option<HashMap<String, String>> {
        self.0.params()
    }

    fn access_token(&self) -> String {
        self.0.access_token()
    }

    fn filter(self, _param: impl FilterOptions) -> Self {
        self
    }
}

impl RequestParameters for ForumRequest {
    type ModelCollection = ZohoForums;
    type NewModel = NewForum;
}

impl ForumRequest {
    pub fn follow(&self) -> Result<Option<Response>> {
        let mut url = self.uri();
        url.push_str("follow");

        ZohoRequest::<NewForum>::new(Method::POST, &url, None, self.access_token(), self.params())
            .send()
    }

    pub fn unfollow(&self) -> Result<Option<Response>> {
        let mut url = self.uri();
        url.push_str("unfollow");

        ZohoRequest::<NewForum>::new(Method::POST, &url, None, self.access_token(), self.params())
            .send()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZohoForums {
    #[serde(rename = "forums")]
    pub forums: Vec<Forum>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SelfLink {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct NewForum {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "is_sticky_post")]
    pub sticky: bool,
    #[serde(rename = "is_announcement_post")]
    pub announcement: bool,
    #[serde(rename = "category_id")]
    pub category_id: i64,
    //TODO(Xymist): notify, flag, type, file
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Response {
    response: String,
}
