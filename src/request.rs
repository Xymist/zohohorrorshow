//! Principally a wrapper module for Reqwest, this abstracts the details necessary for actually
//! creating and making a request to the Zoho API.

use crate::errors::*;
use reqwest::{Method, StatusCode};
use serde::{self, Deserialize};
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

    fn access_token(&self) -> String {
        self.access_token.clone()
    }

    /// The .send<U> method uses the details provided to the ZohoRequest to make a request
    /// against the Zoho Projects API. This is generic over the various ZohoModels.
    pub(crate) fn send<U>(&self) -> Result<Option<U>>
    where
        U: serde::de::DeserializeOwned,
    {
        let req_client = reqwest::blocking::Client::new();
        let mut builder = req_client.request(self.method(), self.url());
        builder = builder.header("Authorization", format!("Bearer {}", self.access_token()));
        if let Some(ref params) = self.params {
            builder = builder.query(params);
        }
        if let Some(ref data) = self.data {
            builder = builder.query(data);
        }

        let response = builder.send()?;
        if !response.status().is_success() {
            return Err(Error::server_error(response.status().to_string()));
        };

        // TODO(Xymist): This should probably be an enum for types of response, rather than just an Option
        if self.method() == Method::DELETE {
            let _res_obj: DeleteResponse = match response.status() {
                StatusCode::NO_CONTENT => return Err(Error::no_content()),
                _ => response.json()?,
            };
            Ok(None)
        } else {
            let res_obj: U = match response.status() {
                StatusCode::NO_CONTENT => return Err(Error::no_content()),
                _ => response.json()?,
            };
            Ok(Some(res_obj))
        }
    }
}

/// Wrapper for the details used when making a single request to the Zoho API
#[derive(Clone, Debug)]
pub struct RequestDetails {
    /// Path required to generate URI, which references a particular model in the Zoho DB
    pub model_path: String,
    /// ID of the model instance to be referenced, if any. Irrelevant for POST requests.
    pub id: Option<i64>,
    /// TODO(Xymist): Fix this
    pub name: Option<String>,
    /// Token identifying the caller of the API.
    pub access_token: String,
    /// Any parameters to be passed in the request, such as filters for retrieval or fields for creation.
    pub params: HashMap<String, String>,
}

impl RequestDetails {
    /// Constructor method for new RequestDetails structs
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        RequestDetails {
            model_path: model_path.to_owned(),
            id,
            name: None,
            access_token: access_token.to_owned(),
            params: HashMap::new(),
        }
    }

    /// Setter method for request parameters. Utilises the Filter enums from the various models
    /// to ensure valid input.
    pub fn filter(mut self, param: &(impl FilterOptions + std::fmt::Display)) -> Self {
        self.params.insert(param.key(), param.to_string());
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
}

/// All Model Requests need to implement a small number of convenience methods for accessing the Zoho API
pub trait ModelRequest {
    /// Find or create the URI to call for this model
    fn uri(&self) -> String;
    /// Set parameters to send with this request, in the standard `&key=value` format
    fn params(&self) -> Option<HashMap<String, String>>;
    /// Find or create the access token with which we can authenticate the request
    fn access_token(&self) -> String;
    /// For GET requests, set filters to reduce the number of hits returned
    fn filter(self, param: (impl FilterOptions + std::fmt::Display)) -> Self;
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

    /// Send an HTTP GET request to the model
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

    /// Send an HTTP POST request to the model
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

    /// Send an HTTP PUT request to the model
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

    /// Send an HTTP DELETE request to the model
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
#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct DeleteResponse {
    response: String,
}
