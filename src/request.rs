use crate::client::BASE_URL;
use crate::errors::*;
use reqwest::Method::{self, Delete, Get, Post, Put};
use serde;
use std::default;

pub struct ZohoRequest {
    method: Method,
    url: String,
    data: Option<String>,
}

impl ZohoRequest {
    pub fn new(method: Method, url: &str, data: Option<&str>) -> Self {
        ZohoRequest {
            method,
            url: url.to_owned(),
            data: match data {
                Some(d) => Some(d.to_owned()),
                None => None,
            },
        }
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn data(&self) -> Option<String> {
        self.data.clone()
    }

    pub fn send<T>(&self) -> Result<T>
    where
        T: serde::de::DeserializeOwned + default::Default,
    {
        let req_client: reqwest::Client = reqwest::Client::new();
        let json_data: String = match self.data() {
            Some(d) => d,
            None => "".to_owned(),
        };
        let mut response = req_client
            .request(self.method(), &self.url())
            .json(&json_data)
            .send()?;
        if !response.status().is_success() {
            bail!("Server error: {:?}", response.status());
        };

        let res_obj: T = match response.status() {
            reqwest::StatusCode::NoContent => Default::default(),
            _ => response.json()?,
        };

        Ok(res_obj)
    }
}

pub struct RequestDetails {
    model_path: String,
    id: Option<i64>,
    name: Option<String>,
    params: Vec<(String, String)>,
}

impl RequestDetails {
    pub fn new(auth_token: &str, model_path: &str) -> Self {
        RequestDetails {
            model_path: model_path.to_owned(),
            id: None,
            name: None,
            params: vec![("authtoken".to_owned(), auth_token.to_owned())],
        }
    }
}

impl UriMaker for RequestDetails {
    fn uri(&mut self) -> String {
        let param_string: String = self
            .params
            .iter()
            .map(|pair| format!("{}={}", pair.0, pair.1))
            .collect::<Vec<String>>()
            .join("&");
        format!("{}/{}?{}", BASE_URL, self.model_path, param_string)
    }
}

pub trait FilterOptions {
    fn key(&self) -> String;
    fn value(&self) -> String;
}

pub trait UriMaker {
    fn uri(&mut self) -> String;
}

pub trait RequestParameters: UriMaker {
    type ModelCollection: serde::de::DeserializeOwned + default::Default;

    fn filter(self, param: impl FilterOptions) -> Self;

    fn get(&self, url: &str) -> Result<Self::ModelCollection> {
        ZohoRequest::new(Get, &self.uri(), None).send()
    }

    fn post(&self, url: &str, data: &str) -> Result<Self::ModelCollection> {
        ZohoRequest::new(Post, &self.uri(), Some(data)).send()
    }

    fn put(&self, url: &str, data: &str) -> Result<Self::ModelCollection> {
        ZohoRequest::new(Put, &self.uri(), Some(data)).send()
    }

    fn delete(&self, url: &str) -> Result<Self::ModelCollection> {
        ZohoRequest::new(Delete, &self.uri(), None).send()
    }
}
