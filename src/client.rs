//! The client module contains the implementations for the API client itself and wraps the OAuth process necessary
//! to make requests for a user's data.
//! If in doubt, this is the one to import.
//! Each model is provided its own plural-named method on ZohoClient for retrieving many entries.
//! Where possible there is also a singular-names method which takes an ID parameter to retrieve a single entry.
//! Some entities such as Activities are only available from the Zoho API as a list; in these cases a singular call has not been provided.

use crate::errors::*;
use crate::models::{
    activity, bug, category, event, forum, milestone, portal, project, status, task, tasklist,
    timesheet,
};
use crate::oauth;
use crate::request::RequestParameters;

/// ZohoClient initialises and manages the details with which we can make
/// a connection to the Zoho API. It holds the information required which
/// is then passed down into the various ModelRequests for use with a
/// specific endpoint.
#[derive(Debug, Clone)]
pub struct ZohoClient {
    oauth_credentials: oauth::Credentials,
    portal_id: Option<i64>,
    project_id: Option<i64>,
}

impl ZohoClient {
    /// Generate new ZohoClient which may be used to make requests
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        let credentials = oauth::Credentials::new(client_id, client_secret, None, None);

        let mut client = ZohoClient {
            oauth_credentials: credentials,
            portal_id: None,
            project_id: None,
        };

        client.initial_access_token();
        client
    }

    // Fetch API access token from OAuth
    fn initial_access_token(&mut self) -> String {
        self.oauth_credentials.access_token()
    }

    /// Returns the access token for this Client.
    ///
    /// # Panics
    ///
    /// Will panic if an access token has not been set for this client. As a ZohoClient
    /// cannot be constructed other than via [`new`] and that calls the
    /// necessary API endpoint to set the access token, this is not recoverable.
    ///
    /// [`new`]: #method.new
    pub fn access_token(&self) -> String {
        self.oauth_credentials
            .raw_access_token()
            .expect("Client was incompletely initialized")
    }

    /// Set the Portal to which this ZohoClient should make requests. This takes a Portal name;
    /// these are typically human-friendly strings and should be known by the Zoho Portal users.
    pub fn set_portal(mut self, portal_name: &str) -> Result<Self> {
        let portals = self.portals().get();
        let portal = match portals {
            Ok(Some(p_list)) => p_list.portals.into_iter().find(|p| p.name == portal_name),
            Ok(None) => return Err(Error::empty_entity_list("portal")),
            Err(e) => return Err(e),
        };

        if let Some(po) = portal {
            self.portal_id = Some(po.id)
        } else {
            return Err(Error::missing_entity_name(portal_name));
        };

        Ok(self)
    }

    /// Set the Project to which this ZohoClient should refer when making requests.
    /// All entities requested will be within this Project; if multiple Projects
    /// are to be queried this can be called a second time to re-use the client.
    /// The Project names are typically human-friendly strings and should be known
    /// by the Zoho Project users.
    pub fn set_project(mut self, project_name: &str) -> Result<Self> {
        let projects = self.projects().get();
        let project = match projects {
            Ok(Some(p_list)) => p_list.projects.into_iter().find(|p| p.name == project_name),
            Ok(None) => return Err(Error::empty_entity_list("project")),
            Err(e) => return Err(e),
        };

        if let Some(pr) = project {
            self.project_id = Some(pr.id)
        } else {
            return Err(Error::missing_entity_name(project_name));
        };

        Ok(self)
    }

    pub(crate) fn portal_id(&self) -> i64 {
        self.portal_id
            .expect("Portal context called without portal id set.")
    }

    pub(crate) fn project_id(&self) -> i64 {
        self.project_id
            .expect("Project context called without project id set.")
    }

    /// Construct a Request for retrieving multiple Activities. Activities cannot be
    /// requested singly, therefore there is no id-parameterised counterpart.
    pub fn activities(&self) -> activity::ActivityRequest {
        activity::ActivityRequest::new(
            &self.access_token(),
            &activity::model_path(self.portal_id(), self.project_id()),
        )
    }

    /// Construct a Request for retrieving a Bug by numeric ID
    pub fn bug(&self, id: i64) -> bug::BugRequest {
        bug::BugRequest::new(
            &self.access_token(),
            &bug::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    /// Construct a Request for retrieving multiple Bugs
    pub fn bugs(&self) -> bug::BugRequest {
        bug::BugRequest::new(
            &self.access_token(),
            &bug::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    /// Construct a Request for retrieving a Category by ID
    pub fn category(&self, id: i64) -> category::CategoryRequest {
        category::CategoryRequest::new(
            &self.access_token(),
            &category::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    /// Construct a Request for retrieving multiple Categories
    pub fn categories(&self) -> category::CategoryRequest {
        category::CategoryRequest::new(
            &self.access_token(),
            &category::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    /// Construct a Request for retrieving an Event by ID
    pub fn event(&self, id: i64) -> event::EventRequest {
        event::EventRequest::new(
            &self.access_token(),
            &event::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    /// Construct a Request for retrieving multiple Events
    pub fn events(&self) -> event::EventRequest {
        event::EventRequest::new(
            &self.access_token(),
            &event::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    /// Construct a Request for retrieving a Forum by ID
    pub fn forum(&self, id: i64) -> forum::ForumRequest {
        forum::ForumRequest::new(
            &self.access_token(),
            &forum::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    /// Construct a Request for retrieving multiple Fora
    pub fn forums(&self) -> forum::ForumRequest {
        forum::ForumRequest::new(
            &self.access_token(),
            &forum::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    /// Construct a Request for retrieving a Comment within a Forum, referencing the
    /// ID of both the Forum and Comment
    pub fn forum_comment(&self, forum_id: i64, id: i64) -> forum::comment::CommentRequest {
        forum::comment::CommentRequest::new(
            &self.access_token(),
            &forum::comment::model_path(self.portal_id(), self.project_id(), forum_id),
            Some(id),
        )
    }

    /// Construct a Request for retrieving multiple Comments within a Forum, referencing
    /// the Forum ID
    pub fn forum_comments(&self, forum_id: i64) -> forum::comment::CommentRequest {
        forum::comment::CommentRequest::new(
            &self.access_token(),
            &forum::comment::model_path(self.portal_id(), self.project_id(), forum_id),
            None,
        )
    }

    /// Construct a Request for retrieving a Milestone by ID
    pub fn milestone(&self, id: i64) -> milestone::MilestoneRequest {
        milestone::MilestoneRequest::new(
            &self.access_token(),
            &milestone::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    /// Construct a Request for retrieving multiple Milestones
    pub fn milestones(&self) -> milestone::MilestoneRequest {
        milestone::MilestoneRequest::new(
            &self.access_token(),
            &milestone::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    /// Construct a Request for retrieving the available Portals for this Client.
    /// Used when initializing a new Client.
    pub fn portals(&self) -> portal::PortalRequest {
        portal::PortalRequest::new(&self.access_token())
    }

    /// Construct a Request for retrieving the users with access to this Client's Portal.
    pub fn portal_users(&self) -> portal::user::PortalUserRequest {
        portal::user::PortalUserRequest::new(
            &self.access_token(),
            &portal::user::model_path(self.portal_id()),
            None,
        )
    }

    /// Construct a Request for retrieving a Project by ID
    pub fn project(&self, id: i64) -> project::ProjectRequest {
        project::ProjectRequest::new(
            &self.access_token(),
            &project::model_path(self.portal_id()),
            Some(id),
        )
    }

    /// Construct a Request for retrieving multiple Projects
    pub fn projects(&self) -> project::ProjectRequest {
        project::ProjectRequest::new(
            &self.access_token(),
            &project::model_path(self.portal_id()),
            None,
        )
    }

    /// Construct a Request for retrieving the Users with access to this Client's Project
    pub fn project_users(&self) -> project::user::ProjectUserRequest {
        project::user::ProjectUserRequest::new(
            &self.access_token(),
            &project::user::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    /// Construct a Request for retrieving multiple Statuses. There is no singular counterpart.
    pub fn statuses(&self) -> status::StatusRequest {
        status::StatusRequest::new(
            &self.access_token(),
            &status::model_path(self.portal_id(), self.project_id()),
        )
    }

    /// Construct a Request for retrieving a Task by ID
    pub fn task(&self, id: i64) -> task::TaskRequest {
        task::TaskRequest::new(
            &self.access_token(),
            &task::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    /// Construct a Request for retrieving multiple Tasks
    pub fn tasks(&self) -> task::TaskRequest {
        task::TaskRequest::new(
            &self.access_token(),
            &task::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    /// Construct a Request for retrieving a Tasklist by ID
    pub fn tasklist(&self, id: i64) -> tasklist::TasklistRequest {
        tasklist::TasklistRequest::new(
            &self.access_token(),
            &tasklist::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    /// Construct a Request for retrieving multiple Tasklists
    pub fn tasklists(&self) -> tasklist::TasklistRequest {
        tasklist::TasklistRequest::new(
            &self.access_token(),
            &tasklist::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    /// Construct a Request for retrieving a Task belonging to a specific Tasklist
    pub fn tasklist_task(
        &self,
        tasklist_id: usize,
        id: i64,
    ) -> tasklist::task::TasklistTaskRequest {
        tasklist::task::TasklistTaskRequest::new(
            &self.access_token(),
            &tasklist::task::model_path(self.portal_id(), self.project_id(), tasklist_id),
            Some(id),
        )
    }

    /// Construct a Request for retrieving Tasks belonging to a specific Tasklist
    pub fn tasklist_tasks(&self, tasklist_id: usize) -> tasklist::task::TasklistTaskRequest {
        tasklist::task::TasklistTaskRequest::new(
            &self.access_token(),
            &tasklist::task::model_path(self.portal_id(), self.project_id(), tasklist_id),
            None,
        )
    }

    /// Construct a Request for retrieving a Timesheet by ID
    pub fn timesheet(&self, id: i64) -> timesheet::TimesheetRequest {
        timesheet::TimesheetRequest::new(
            &self.access_token(),
            &timesheet::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    /// Construct a Request for retrieving multiple Timesheets
    pub fn timesheets(&self) -> timesheet::TimesheetRequest {
        timesheet::TimesheetRequest::new(
            &self.access_token(),
            &timesheet::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }
}
