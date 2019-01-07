pub mod activity;
pub mod bug;
pub mod category;
pub mod comment;
pub mod event;
pub mod forum;
pub mod milestone;
pub mod portal;
pub mod project;
pub mod status;
pub mod task;
pub mod tasklist;
pub mod timesheet;
pub mod user;

// For convenience, lift all of these to this level to permit using (e.g.) `models::comments`
// to call the API.
pub use self::{
    category::categories,
    comment::comments,
    event::events,
    forum::forums,
    milestone::milestones,
    portal::portals,
    project::projects,
    status::statuses,
    task::tasks,
    tasklist::tasklists,
    timesheet::timesheets,
    user::{portal_users, project_users},
};
