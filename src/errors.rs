use reqwest;
use chrono::ParseError;
use std::env::VarError;

error_chain!{
    foreign_links {
        Reqwest(reqwest::Error);
        Chrono(ParseError);
        EnvVar(VarError);
    }
}
