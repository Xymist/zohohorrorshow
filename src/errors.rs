//! Error wrapper module utilising ErrorChain to unify error handling.
//! This probably ought to be replaced either with Failure or a custom implementation with a bit more clarity.

use chrono::ParseError;
use reqwest;
use serde_json;
use std::{env::VarError, num::ParseIntError};

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        Chrono(ParseError);
        EnvVar(VarError);
        Json(serde_json::Error);
        ParseInt(ParseIntError);
    }
}
