use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters, ZohoRequest};
use reqwest::Method;
use std::collections::HashMap;

pub fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
    forum: impl std::fmt::Display,
) -> String {
    format!(
        "portal/{}/projects/{}/forums/{}/comments",
        portal, project, forum
    )
}

pub struct CommentRequest(RequestDetails);

impl CommentRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        CommentRequest(RequestDetails::new(access_token, model_path, id))
    }
}

impl ModelRequest for CommentRequest {
    fn uri(&self) -> String {
        self.0.uri()
    }

    fn params(&self) -> Option<HashMap<String, String>> {
        self.0.params()
    }

    fn access_token(&self) -> String {
        self.0.access_token()
    }
}

impl RequestParameters for CommentRequest {
    type ModelCollection = ZohoComments;
    type NewModel = NewComment;
}

impl CommentRequest {
    pub fn mark_best(&self) -> Result<Option<Response>> {
        let mut url = self.uri();
        url.push_str("markbestanswer");

        ZohoRequest::<NewComment>::new(Method::POST, &url, None, self.access_token(), self.params())
            .send()
    }

    pub fn unmark_best(&self) -> Result<Option<Response>> {
        let mut url = self.uri();
        url.push_str("markbestanswer");

        ZohoRequest::<NewComment>::new(
            Method::DELETE,
            &url,
            None,
            self.access_token(),
            self.params(),
        )
        .send()
    }
}

pub enum Filter {
    Index(i64),
    Range(i64),
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
        }
    }

    fn value(&self) -> String {
        match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZohoComments {
    comments: Vec<Comment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    pub post_date_long: i64,
    pub is_best_answer: bool,
    pub attachments: Vec<Attachment>,
    pub level: String,
    #[serde(rename = "type")]
    pub comment_type: String,
    pub content: String,
    pub parent_posted_by: String,
    pub posted_person: String,
    pub parent_id: String,
    pub post_date: String,
    pub posted_by: String,
    pub root_id: String,
    pub post_date_format: String,
    pub id: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewComment {
    #[serde(rename = "type")]
    pub comment_type: String,
    pub content: String,
    pub parent_id: i64,
    //TODO(Xymist): uploadfile
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub file_url: String,
    pub file_name: String,
    pub is_image: bool,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Response {
    response: String,
}
