use crate::client::BASE_URL;
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
    params: Option<HashMap<String, String>>,
}

impl<T: serde::Serialize + Clone> ZohoRequest<T> {
    pub fn new(
        method: Method,
        url: &str,
        data: Option<T>,
        params: Option<HashMap<String, String>>,
    ) -> Self {
        ZohoRequest {
            method,
            url: url.to_owned(),
            data,
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

    pub fn send<U>(&self) -> Result<Option<U>>
    where
        U: serde::de::DeserializeOwned,
    {
        let req_client: reqwest::Client = reqwest::Client::new();
        let mut builder = req_client.request(self.method(), &self.url());
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
    pub params: HashMap<String, String>,
}

impl RequestDetails {
    pub fn new(auth_token: &str, model_path: &str, id: Option<i64>) -> Self {
        let mut params = HashMap::new();
        params.insert("authtoken".to_owned(), auth_token.to_owned());

        RequestDetails {
            model_path: model_path.to_owned(),
            id: id,
            name: None,
            params: params,
        }
    }

    pub fn filter(mut self, param: &impl FilterOptions) -> Self {
        self.params
            .insert(param.key().to_owned(), param.value().to_owned());
        self
    }

    pub fn uri(&self) -> String {
        match self.id {
            Some(model_id) => format!("{}/{}{}/", BASE_URL, self.model_path, model_id),
            None => format!("{}/{}", BASE_URL, self.model_path),
        }
    }

    pub fn params(&self) -> Option<HashMap<String, String>> {
        Some(self.params.clone())
    }
}

pub trait FilterOptions {
    fn key(&self) -> String;
    fn value(&self) -> String;
}

pub trait ModelRequest {
    fn uri(&self) -> String;
    fn params(&self) -> Option<HashMap<String, String>>;
}

pub trait RequestParameters: ModelRequest {
    type ModelCollection: serde::de::DeserializeOwned;
    type NewModel: serde::Serialize + Clone;

    fn get(&self) -> Result<Option<Self::ModelCollection>> {
        ZohoRequest::<Self::NewModel>::new(Method::GET, &self.uri(), None, self.params()).send()
    }

    fn post(&self, data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        ZohoRequest::<Self::NewModel>::new(Method::POST, &self.uri(), Some(data), self.params())
            .send()
    }

    fn put(&self, data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        ZohoRequest::<Self::NewModel>::new(Method::POST, &self.uri(), Some(data), self.params())
            .send()
    }

    fn delete(&self) -> Result<Option<Self::ModelCollection>> {
        ZohoRequest::<Self::NewModel>::new(Method::DELETE, &self.uri(), None, self.params()).send()
    }
}

#[derive(Deserialize, Clone)]
pub struct DeleteResponse {
    response: String,
}
