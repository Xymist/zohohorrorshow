//! # Zohohorrorshow
//!
//! A library for interacting with the Zoho Projects API, because it's an awful nightmare of ambiguous fields,
//! optional fields, random casing and largely absent documentation. Xорошо́!

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;
mod utils;

pub mod client;
pub mod errors;
pub mod models;
