use crate::client::BASE_URL;
use crate::errors::*;
use reqwest::Method::{self, Delete, Get, Post};
use serde;
use serde_json;
use std::collections::HashMap;

pub struct ZohoRequest<T>
where
    T: serde::Serialize + Clone,
{
    method: Method,
    url: String,
    data: Option<T>,
}

impl<T: serde::Serialize + Clone> ZohoRequest<T> {
    pub fn new(method: Method, url: &str, data: Option<T>) -> Self {
        ZohoRequest {
            method,
            url: url.to_owned(),
            data: data,
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

    // TODO(Xymist): WTF? This is terrible, replace it.
    pub fn data_params(&self) -> String {
        match self.data() {
            None => "".to_owned(),
            Some(d) => {
                let mut json_data = String::new();
                match serde_json::to_string(&d) {
                    Ok(d) => {
                        // Using .unwrap() here is probably safe, as long as we assume Serde
                        // generates valid JSON.
                        // This is pretty crap though, so what do I know?
                        let hsh: HashMap<String, String> = serde_json::from_str(&d).unwrap();
                        for (key, val) in hsh.iter() {
                            json_data += &format!("&{}={}", key, val);
                        }
                    }
                    _ => return "".to_owned(),
                };
                json_data
            }
        }
    }

    pub fn send<U>(&self) -> Result<U>
    where
        U: serde::de::DeserializeOwned,
    {
        let req_client: reqwest::Client = reqwest::Client::new();
        let json_data: String = serde_json::to_string(&self.data()).unwrap_or_else(|_| "".to_owned());
        let mut response = req_client
            .request(self.method(), &self.url())
            .json(&json_data)
            .send()?;
        if !response.status().is_success() {
            bail!("Server error: {:?}", response.status());
        };

        let res_obj: U = match response.status() {
            reqwest::StatusCode::NoContent => bail!("No content received"),
            _ => response.json()?,
        };

        Ok(res_obj)
    }
}

pub struct RequestDetails {
    pub model_path: String,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub params: Vec<(String, String)>,
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

    pub fn filter(mut self, param: impl FilterOptions) -> Self {
        self.params
            .push((param.key().to_owned(), param.value().to_owned()));
        self
    }

    pub fn uri(&self) -> String {
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

pub trait ModelRequest {
    fn uri(&self) -> String;
}

pub trait RequestParameters: ModelRequest {
    type ModelCollection: serde::de::DeserializeOwned;
    type NewModel: serde::Serialize + Clone;

    fn get(&self) -> Result<Self::ModelCollection> {
        ZohoRequest::<Self::NewModel>::new(Get, &self.uri(), None).send()
    }

    fn post(&self, data: Self::NewModel) -> Result<Self::ModelCollection> {
        ZohoRequest::<Self::NewModel>::new(Post, &self.uri(), Some(data)).send()
    }

    fn delete(&self) -> Result<Self::ModelCollection> {
        ZohoRequest::<Self::NewModel>::new(Delete, &self.uri(), None).send()
    }
}
