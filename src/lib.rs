extern crate chrono;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate mockito;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
mod macros;

pub mod bugs;
pub mod categories;
pub mod client;
pub mod statuses;
pub mod activities;
pub mod events;
pub mod forums;
pub mod milestones;
pub mod portals;
pub mod projects;
pub mod tasklists;
pub mod tasks;
pub mod timesheets;
pub mod users;
pub mod errors;
