//! # Zohohorrorshow
//!
//! A library for interacting with the Zoho Projects API, because it's an awful nightmare of ambiguous fields,
//! optional fields, random casing and largely absent documentation. Xорошо́!

#![deny(unused_imports)]
// #![warn(missing_docs)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

pub mod client;
pub mod errors;
pub mod models;
pub mod prelude;
pub mod request;

mod oauth;
mod serializers;
