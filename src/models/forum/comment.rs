use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters, ZohoRequest};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
    forum: impl std::fmt::Display,
) -> String {
    format!(
        "portal/{}/projects/{}/forums/{}/comments",
        portal, project, forum
    )
}

#[derive(Clone, Debug)]
pub struct CommentRequest(RequestDetails);

impl CommentRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        CommentRequest(RequestDetails::new(access_token, model_path, id))
    }

    pub fn iter_get(self) -> CommentIterator {
        CommentIterator::new(self)
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

    fn filter(mut self, param: (impl FilterOptions + std::fmt::Display)) -> Self {
        self.0 = self.0.filter(&param);
        self
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
    Index(usize),
    Range(i8),
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
        }
    }
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
        };

        write!(f, "{}", str_rep)
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

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct Response {
    response: String,
}

#[derive(Debug, Clone)]
pub struct CommentIterator {
    pub items: <Vec<Comment> as IntoIterator>::IntoIter,
    pub last_full: bool,
    pub request: CommentRequest,
    pub start_index: usize,
}

impl CommentIterator {
    pub fn new(request: CommentRequest) -> CommentIterator {
        CommentIterator {
            items: Vec::new().into_iter(),
            last_full: true,
            request,
            start_index: 0,
        }
    }

    fn range(&self) -> i8 {
        match self.request.params() {
            Some(params) => match params.get("range") {
                Some(range_string) => range_string.parse::<i8>().unwrap_or(100),
                None => 100,
            },
            None => 100,
        }
    }

    pub fn try_next(&mut self) -> Result<Option<Comment>> {
        // If there are still items in the local cache from the last request, use the next one of those.
        if let Some(comment) = self.items.next() {
            return Ok(Some(comment));
        }

        // If we didn't get a full 100 (the default number to retrieve) the last time, then we must have
        // run out in Zoho; don't request any more.
        if !self.last_full {
            return Ok(None);
        }

        let returned_comments = self
            .request
            .clone()
            .filter(Filter::Index(self.start_index))
            .get()?;

        if let Some(comment_list) = returned_comments {
            self.last_full = comment_list.comments.len() as i8 == self.range();

            self.start_index += comment_list.comments.len();

            self.items = comment_list.comments.into_iter();

            Ok(self.items.next())
        } else {
            Ok(None)
        }
    }
}

impl Iterator for CommentIterator {
    type Item = Result<Comment>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(val)) => Some(Ok(val)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
