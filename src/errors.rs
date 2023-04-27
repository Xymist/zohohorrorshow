//! Error wrapper module utilising ThisError to unify error handling.

use chrono;
use reqwest;
use std::env;

use thiserror::Error;

/// A type alias for handling errors throughout ZohoHorrorshow.
pub type Result<T> = std::result::Result<T, Error>;

/// Varieties of error which may be encountered
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    /// Wrapper for an error thrown by Reqwest
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Wrapper for an error thrown by Chrono
    #[error("Chrono error: {0}")]
    Chrono(#[from] chrono::ParseError),
    /// Wrapper for a std::env::VarError
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] env::VarError),
    /// Failure in JSON parsing
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// Failure in Int parsing
    #[error("Int parsing error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    /// Server returned no content status code
    #[error("StatusCode::NOCONTENT received when making request")]
    NoContent,
    /// Server returned error code
    #[error("Server returned error code: {0}")]
    ServerError(String),
    /// Client attempted to make a request with invalid HTTP method
    #[error("Disallowed request method: {method} for {model}")]
    DisallowedRequestMethod {
        /// The disallowed method which was called
        method: String,
        /// The model on which this method was called
        model: String,
    },
    /// Entity sought by ID returned no results
    #[error("Failed to find entity with ID {0}")]
    MissingEntityId(usize),
    /// Entity sought by name returned no results
    #[error("Failed to find entity with name {0}")]
    MissingEntityName(String),
    /// Model sought en masse returned no results
    #[error("No entries found for {0}")]
    EmptyList(String),
}

impl Error {
    pub fn no_content() -> Error {
        Error::NoContent
    }

    pub fn server_error(status: String) -> Error {
        Error::ServerError(status)
    }

    pub fn disallowed_method(method: &str, model: &str) -> Error {
        Error::DisallowedRequestMethod {
            method: method.to_owned(),
            model: model.to_owned(),
        }
    }

    pub fn missing_entity_id(id: usize) -> Error {
        Error::MissingEntityId(id)
    }

    pub fn missing_entity_name(name: &str) -> Error {
        Error::MissingEntityName(name.to_owned())
    }

    pub fn empty_entity_list(model: &str) -> Error {
        Error::EmptyList(model.to_owned())
    }
}
