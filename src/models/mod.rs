//! Wrapper module for all the Zoho models.
//! Each model (e.g. Tasks) is represented in its own module to maintain separation and allow
//! namespacing of similar methods and types, since each model requires (e.g.) a slightly different
//! implementation of Filter.

pub mod activity;
pub mod bug;
pub mod category;
pub mod event;
pub mod forum;
pub mod milestone;
pub mod portal;
pub mod project;
pub mod status;
pub mod task;
pub mod tasklist;
pub mod timesheet;

fn multi_filter_format(elements: &[impl std::fmt::Display]) -> String {
    let element_string = elements
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(",");
    format!("[{:?}]", element_string)
}
