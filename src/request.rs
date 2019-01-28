use crate::errors::*;
use reqwest::{Method, StatusCode};
use serde;
use std::collections::HashMap;

pub struct ZohoRequest<T>
where
    T: serde::Serialize + Clone,
{
    method: Method,
    url: String,
    data: Option<T>,
    access_token: String,
    params: Option<HashMap<String, String>>,
}

impl<T: serde::Serialize + Clone> ZohoRequest<T> {
    pub fn new(
        method: Method,
        url: &str,
        data: Option<T>,
        access_token: String,
        params: Option<HashMap<String, String>>,
    ) -> Self {
        ZohoRequest {
            method,
            url: url.to_owned(),
            data,
            access_token,
            params,
        }
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn data(&self) -> Option<T> {
        self.data.clone()
    }

    pub fn params(&self) -> Option<HashMap<String, String>> {
        self.params.clone()
    }

    fn access_token(&self) -> String {
        self.access_token.clone()
    }

    pub fn send<U>(&self) -> Result<Option<U>>
    where
        U: serde::de::DeserializeOwned,
    {
        let req_client: reqwest::Client = reqwest::Client::new();
        let mut builder = req_client.request(self.method(), &self.url());
        builder = builder.header("Authorization", format!("Bearer {}", self.access_token()));
        if self.params.is_some() {
            builder = builder.query(&self.params().unwrap());
        }
        if self.data.is_some() {
            builder = builder.query(&self.data().unwrap());
        }
        let mut response = builder.send()?;
        if !response.status().is_success() {
            bail!("Server error: {:?}", response.status());
        };

        // TODO(Xymist): This should probably be an enum for types of response, rather than just an Option
        if self.method() == Method::DELETE {
            let _res_obj: DeleteResponse = match response.status() {
                StatusCode::NO_CONTENT => bail!("No content received"),
                _ => response.json()?,
            };
            Ok(None)
        } else {
            let res_obj: U = match response.status() {
                StatusCode::NO_CONTENT => bail!("No content received"),
                _ => response.json()?,
            };
            Ok(Some(res_obj))
        }
    }
}

pub struct RequestDetails {
    pub model_path: String,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub access_token: String,
    pub params: HashMap<String, String>,
}

impl RequestDetails {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        RequestDetails {
            model_path: model_path.to_owned(),
            id: id,
            name: None,
            access_token: access_token.to_owned(),
            params: HashMap::new(),
        }
    }

    pub fn filter(mut self, param: &impl FilterOptions) -> Self {
        self.params.insert(param.key(), param.value());
        self
    }

    pub fn uri(&self) -> String {
        let base_url = "https://projectsapi.zoho.com/restapi";

        match self.id {
            Some(model_id) => format!("{}/{}{}/", base_url, self.model_path, model_id),
            None => format!("{}/{}", base_url, self.model_path),
        }
    }

    pub fn params(&self) -> Option<HashMap<String, String>> {
        Some(self.params.clone())
    }

    pub fn access_token(&self) -> String {
        self.access_token.clone()
    }
}

pub trait FilterOptions {
    fn key(&self) -> String;
    fn value(&self) -> String;
}

pub trait ModelRequest {
    fn uri(&self) -> String;
    fn params(&self) -> Option<HashMap<String, String>>;
    fn access_token(&self) -> String;
}

pub trait RequestParameters: ModelRequest {
    type ModelCollection: serde::de::DeserializeOwned;
    type NewModel: serde::Serialize + Clone;

    fn get(&self) -> Result<Option<Self::ModelCollection>> {
        ZohoRequest::<Self::NewModel>::new(
            Method::GET,
            &self.uri(),
            None,
            self.access_token(),
            self.params(),
        )
        .send()
    }

    fn post(&self, data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        ZohoRequest::<Self::NewModel>::new(
            Method::POST,
            &self.uri(),
            Some(data),
            self.access_token(),
            self.params(),
        )
        .send()
    }

    fn put(&self, data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        ZohoRequest::<Self::NewModel>::new(
            Method::POST,
            &self.uri(),
            Some(data),
            self.access_token(),
            self.params(),
        )
        .send()
    }

    fn delete(&self) -> Result<Option<Self::ModelCollection>> {
        ZohoRequest::<Self::NewModel>::new(
            Method::DELETE,
            &self.uri(),
            None,
            self.access_token(),
            self.params(),
        )
        .send()
    }
}

#[derive(Deserialize, Clone)]
pub struct DeleteResponse {
    response: String,
}
