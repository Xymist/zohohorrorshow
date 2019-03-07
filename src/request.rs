//! Principally a wrapper module for Reqwest, this abstracts the details necessary for actually
//! creating and making a request to the Zoho API.

use crate::errors::*;
use reqwest::{Method, StatusCode};
use serde;
use std::collections::HashMap;

// TODO(Xymist): Split this; POST/PUT requests should carry data, GET and DELETE should not.
pub(crate) struct ZohoRequest<T>
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
    pub(crate) fn new(
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

    pub(crate) fn method(&self) -> Method {
        self.method.clone()
    }

    pub(crate) fn url(&self) -> String {
        self.url.clone()
    }

    pub(crate) fn data(&self) -> Option<T> {
        self.data.clone()
    }

    pub(crate) fn params(&self) -> Option<HashMap<String, String>> {
        self.params.clone()
    }

    fn access_token(&self) -> String {
        self.access_token.clone()
    }

    /// The .send<U> method uses the details provided to the ZohoRequest to make a request
    /// against the Zoho Projects API. This is generic over the various ZohoModels.
    pub(crate) fn send<U>(&self) -> Result<Option<U>>
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

#[derive(Clone, Debug)]
pub struct RequestDetails {
    pub model_path: String,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub access_token: String,
    pub params: HashMap<String, String>,
}

impl RequestDetails {
    /// Constructor method for new RequestDetails structs
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        RequestDetails {
            model_path: model_path.to_owned(),
            id: id,
            name: None,
            access_token: access_token.to_owned(),
            params: HashMap::new(),
        }
    }

    /// Setter method for request parameters. Utilises the Filter enums from the various models
    /// to ensure valid input.
    pub fn filter(mut self, param: &impl FilterOptions) -> Self {
        self.params.insert(param.key(), param.value());
        self
    }

    /// Constructor for URI string, to be used for making requests to the Zoho Projects API
    pub fn uri(&self) -> String {
        let base_url = "https://projectsapi.zoho.com/restapi";

        match self.id {
            Some(model_id) => format!("{}/{}{}/", base_url, self.model_path, model_id),
            None => format!("{}/{}", base_url, self.model_path),
        }
    }

    /// Accessor method for parameter hash
    pub fn params(&self) -> Option<HashMap<String, String>> {
        Some(self.params.clone())
    }

    /// Accessor method for access token
    pub fn access_token(&self) -> String {
        self.access_token.clone()
    }
}

/// A trait which defines access functions for turning Filters into parameters which the Zoho system will accept.
/// This is defined for all module::Filter enums, where a request is filterable.
pub trait FilterOptions {
    /// Returns the String to use as the 'key' part of a key=value parameter pair for a given Filter
    fn key(&self) -> String;
    /// Returns the String to use as the 'value' part of a key=value parameter pair for a given Filter
    fn value(&self) -> String;
}

pub trait ModelRequest {
    fn uri(&self) -> String;
    fn params(&self) -> Option<HashMap<String, String>>;
    fn access_token(&self) -> String;
    fn filter(self, param: impl FilterOptions) -> Self;
}

/// Trait with global implementations for issuing requests of each Method.
/// Implemented for each type of ModelRequest, overridden where a specific
/// request Method is not available for that model.
pub trait RequestParameters: ModelRequest {

    /// ModelCollection must be a "ZohoModels" struct, which contains just Vec<ZohoModel>.
    /// The Zoho Projects API always returns an object containing a JSONArray of whatever model
    /// is being requested, even if requested by ID and therefore returning either a single item
    /// or an error.
    type ModelCollection: serde::de::DeserializeOwned;

    /// NewModel is a struct containing the fields which the Zoho API accepts to create a new item
    /// of the model type. For those models which Zoho does not accept creation events though the API
    /// this is an empty struct.
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

/// On deleting an item the return value is an object with a response String in it, not the deleted item or
/// an empty list as might usually be expected. This struct packages that so that there is a valid return
/// type available for Delete responses to be parsed into.
/// This type will never be returned; if a Delete request is successfully made the response
/// from this wrapper is Ok(None).
#[derive(Deserialize, Clone)]
struct DeleteResponse {
    response: String,
}