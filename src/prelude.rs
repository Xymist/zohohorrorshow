//! Convenience module for easy import. Exports the various models and the Zoho client.

pub use crate::client::ZohoClient;
pub use crate::models::{
    activity as zoho_activity, bug as zoho_bug, category as zoho_category, event as zoho_event,
    milestone as zoho_milestone, portal as zoho_portal, portal::user as zoho_portal_user,
    project as zoho_project, project::user as zoho_project_user, status as zoho_status,
    task as zoho_task, tasklist as zoho_tasklist, timesheet as zoho_timesheet,
};
pub use crate::request as zoho_request;
pub use crate::request::{ModelRequest, RequestParameters};
