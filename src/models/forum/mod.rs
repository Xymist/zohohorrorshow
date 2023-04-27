use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters, ZohoRequest};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod comment;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
) -> String {
    format!("portal/{}/projects/{}/forums/", portal, project)
}

#[derive(Clone, Debug)]
pub struct ForumRequest(RequestDetails);

impl ForumRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        ForumRequest(RequestDetails::new(access_token, model_path, id))
    }

    pub fn iter_get(self) -> ForumIterator {
        ForumIterator::new(self)
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

    fn filter(self, _param: (impl FilterOptions + std::fmt::Display)) -> Self {
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

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct Response {
    response: String,
}

#[derive(Debug, Clone)]
pub struct ForumIterator {
    pub items: <Vec<Forum> as IntoIterator>::IntoIter,
    pub last_full: bool,
    pub request: ForumRequest,
}

impl ForumIterator {
    pub fn new(request: ForumRequest) -> ForumIterator {
        ForumIterator {
            items: Vec::new().into_iter(),
            last_full: true,
            request,
        }
    }

    pub fn try_next(&mut self) -> Result<Option<Forum>> {
        // If there are still items in the local cache from the last request, use the next one of those.
        if let Some(forum) = self.items.next() {
            return Ok(Some(forum));
        }

        // If we didn't get a full 100 (the default number to retrieve) the last time, then we must have
        // run out in Zoho; don't request any more.
        if !self.last_full {
            return Ok(None);
        }

        let returned_forums = self.request.clone().get()?;

        if let Some(forum_list) = returned_forums {
            self.last_full = forum_list.forums.len() as i8 == 100;

            self.items = forum_list.forums.into_iter();

            Ok(self.items.next())
        } else {
            Ok(None)
        }
    }
}

impl Iterator for ForumIterator {
    type Item = Result<Forum>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(val)) => Some(Ok(val)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
