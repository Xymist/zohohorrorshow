use reqwest;
use chrono::ParseError;
use std::env::VarError;
use serde_json;

error_chain!{
    foreign_links {
        Reqwest(reqwest::Error);
        Chrono(ParseError);
        EnvVar(VarError);
        Json(serde_json::Error);
    }
}
