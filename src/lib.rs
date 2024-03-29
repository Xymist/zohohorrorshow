//! # Zohohorrorshow
//!
//! A library for interacting with the Zoho Projects API, because it's an awful nightmare of ambiguous fields,
//! optional fields, random casing and largely absent documentation. Xорошо́!

#![deny(
    unused_imports,
    rust_2018_idioms,
    rust_2018_compatibility,
    unsafe_code,
    clippy::all
)]
#![doc(html_root_url = "https://docs.rs/zohohorrorshow/0.7.4")]

pub mod client;
pub mod errors;
pub mod models;
pub mod prelude;
pub mod request;

mod oauth;
mod serializers;
