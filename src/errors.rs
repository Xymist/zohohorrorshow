//! Error wrapper module utilising Failure to unify error handling.

use chrono;
use reqwest;
use std::{env, fmt};

use failure::{Backtrace, Context, Fail};

/// A type alias for handling errors throughout ZohoHorrorshow.
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur while interacting with the Zoho API.
#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }

    pub(crate) fn reqwest(err: reqwest::Error) -> Error {
        Error::from(ErrorKind::Reqwest(err.to_string()))
    }

    pub(crate) fn chrono_parse(err: chrono::ParseError) -> Error {
        Error::from(ErrorKind::Chrono(err.to_string()))
    }

    pub(crate) fn env_var(err: env::VarError) -> Error {
        Error::from(ErrorKind::EnvVar(err.to_string()))
    }

    // pub(crate) fn json(err: serde_json::Error) -> Error {
    //     Error::from(ErrorKind::Json(err.to_string()))
    // }

    // pub(crate) fn parse_int(err: num::ParseIntError) -> Error {
    //     Error::from(ErrorKind::ParseInt(err.to_string()))
    // }

    pub(crate) fn no_content() -> Error {
        Error::from(ErrorKind::NoContent)
    }

    pub(crate) fn server_error(err: String) -> Error {
        Error::from(ErrorKind::ServerError(err))
    }

    pub(crate) fn disallowed_method(method: &str, model: &str) -> Error {
        Error::from(ErrorKind::DisallowedRequestMethod {
            method: method.to_owned(),
            model: model.to_owned(),
        })
    }

    // pub(crate) fn missing_entity_id(id: usize) -> Error {
    //     Error::from(ErrorKind::MissingEntityId(id))
    // }

    pub(crate) fn missing_entity_name(name: &str) -> Error {
        Error::from(ErrorKind::MissingEntityName(name.to_owned()))
    }

    pub(crate) fn empty_entity_list(model: &str) -> Error {
        Error::from(ErrorKind::EmptyList(model.to_owned()))
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

/// Varieties of error which may be encountered
#[non_exhaustive]
pub enum ErrorKind {
    /// Wrapper for an error thrown by Reqwest
    Reqwest(String),
    /// Wrapper for an error thrown by Chrono
    Chrono(String),
    /// Wrapper for a std::env::VarError
    EnvVar(String),
    /// Failure in JSON parsing
    Json(String),
    /// Failure in Int parsing
    ParseInt(String),
    /// Server returned no content status code
    NoContent,
    /// Server returned error code
    ServerError(String),
    /// Client attempted to make a request with invalid HTTP method
    DisallowedRequestMethod {
        /// The disallowed method which was called
        method: String,
        /// The model on which this method was called
        model: String,
    },
    /// Entity sought by ID returned no results
    MissingEntityId(usize),
    /// Entity sought by name returned no results
    MissingEntityName(String),
    /// Model sought en masse returned no results
    EmptyList(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorKind::Reqwest(ref msg) => write!(f, "{}", msg),
            ErrorKind::Chrono(ref msg) => write!(f, "{}", msg),
            ErrorKind::EnvVar(ref msg) => write!(f, "{}", msg),
            ErrorKind::Json(ref msg) => write!(f, "{}", msg),
            ErrorKind::ParseInt(ref msg) => write!(f, "{}", msg),
            ErrorKind::NoContent => write!(f, "StatusCode::NOCONTENT received when making request"),
            ErrorKind::ServerError(ref msg) => {
                write!(f, "Received error code from server: {}", msg)
            }
            ErrorKind::DisallowedRequestMethod {
                ref method,
                ref model,
            } => write!(f, "{} method is not permitted for {}", method, model),
            ErrorKind::MissingEntityId(id) => write!(f, "Failed to find entity with ID {}", id),
            ErrorKind::MissingEntityName(ref name) => {
                write!(f, "Failed to find entity with name {}", name)
            }
            ErrorKind::EmptyList(ref model) => write!(f, "No entries found for {}", model),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::reqwest(err)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Error {
        Error::chrono_parse(err)
    }
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error {
        Error::env_var(err)
    }
}
