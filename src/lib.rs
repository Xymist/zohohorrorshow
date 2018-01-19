#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(type_ascription)]

extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod bugs;
pub mod client;
pub mod dashboard;
pub mod errors;
pub mod events;
pub mod forums;
pub mod milestones;
pub mod portals;
pub mod projects;
pub mod tasklists;
pub mod tasks;
pub mod timesheets;
pub mod users;

use errors::*;

pub trait RelativePath<U> {
    fn relative_path(params: U) -> Result<String>;
}
