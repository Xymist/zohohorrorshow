use chrono::ParseError;
use reqwest;
use serde_json;
use std::env::VarError;
use std::num::ParseIntError;

error_chain!{
    foreign_links {
        Reqwest(reqwest::Error);
        Chrono(ParseError);
        EnvVar(VarError);
        Json(serde_json::Error);
        ParseInt(ParseIntError);
    }
}
