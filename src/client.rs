//! The client module contains the implementations for the API client itself and wraps the OAuth process necessary
//! to make requests for a user's data.
//! If in doubt, this is the one to import.
//! Each model is provided its own plural-named method on ZohoClient for retrieving many entries.
//! Where possible there is also a singular-names method which takes an ID parameter to retrieve a single entry.

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

    /// Fetch API access token from OAuth
    fn initial_access_token(&mut self) -> String {
        self.oauth_credentials.access_token()
    }

    pub fn access_token(&self) -> String {
        self.oauth_credentials
            .raw_access_token()
            .expect("Client was incompletely initialized")
    }

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

    pub fn activities(&self) -> activity::ActivityRequest {
        activity::ActivityRequest::new(
            &self.access_token(),
            &activity::model_path(self.portal_id(), self.project_id()),
        )
    }

    pub fn bug(&self, id: i64) -> bug::BugRequest {
        bug::BugRequest::new(
            &self.access_token(),
            &bug::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn bugs(&self) -> bug::BugRequest {
        bug::BugRequest::new(
            &self.access_token(),
            &bug::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn category(&self, id: i64) -> category::CategoryRequest {
        category::CategoryRequest::new(
            &self.access_token(),
            &category::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn categories(&self) -> category::CategoryRequest {
        category::CategoryRequest::new(
            &self.access_token(),
            &category::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn event(&self, id: i64) -> event::EventRequest {
        event::EventRequest::new(
            &self.access_token(),
            &event::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn events(&self) -> event::EventRequest {
        event::EventRequest::new(
            &self.access_token(),
            &event::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn forum(&self, id: i64) -> forum::ForumRequest {
        forum::ForumRequest::new(
            &self.access_token(),
            &forum::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn forums(&self) -> forum::ForumRequest {
        forum::ForumRequest::new(
            &self.access_token(),
            &forum::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn forum_comment(&self, forum_id: i64, id: i64) -> forum::comment::CommentRequest {
        forum::comment::CommentRequest::new(
            &self.access_token(),
            &forum::comment::model_path(self.portal_id(), self.project_id(), forum_id),
            Some(id),
        )
    }

    pub fn forum_comments(&self, forum_id: i64) -> forum::comment::CommentRequest {
        forum::comment::CommentRequest::new(
            &self.access_token(),
            &forum::comment::model_path(self.portal_id(), self.project_id(), forum_id),
            None,
        )
    }

    pub fn milestone(&self, id: i64) -> milestone::MilestoneRequest {
        milestone::MilestoneRequest::new(
            &self.access_token(),
            &milestone::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn milestones(&self) -> milestone::MilestoneRequest {
        milestone::MilestoneRequest::new(
            &self.access_token(),
            &milestone::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn portals(&self) -> portal::PortalRequest {
        portal::PortalRequest::new(&self.access_token())
    }

    pub fn portal_users(&self) -> portal::user::PortalUserRequest {
        portal::user::PortalUserRequest::new(
            &self.access_token(),
            &portal::user::model_path(self.portal_id()),
            None,
        )
    }

    pub fn project(&self, id: i64) -> project::ProjectRequest {
        project::ProjectRequest::new(
            &self.access_token(),
            &project::model_path(self.portal_id()),
            Some(id),
        )
    }

    pub fn projects(&self) -> project::ProjectRequest {
        project::ProjectRequest::new(
            &self.access_token(),
            &project::model_path(self.portal_id()),
            None,
        )
    }

    pub fn project_users(&self) -> project::user::ProjectUserRequest {
        project::user::ProjectUserRequest::new(
            &self.access_token(),
            &project::user::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn statuses(&self) -> status::StatusRequest {
        status::StatusRequest::new(
            &self.access_token(),
            &status::model_path(self.portal_id(), self.project_id()),
        )
    }

    pub fn task(&self, id: i64) -> task::TaskRequest {
        task::TaskRequest::new(
            &self.access_token(),
            &task::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn tasks(&self) -> task::TaskRequest {
        task::TaskRequest::new(
            &self.access_token(),
            &task::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn tasklist(&self, id: i64) -> tasklist::TasklistRequest {
        tasklist::TasklistRequest::new(
            &self.access_token(),
            &tasklist::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn tasklists(&self) -> tasklist::TasklistRequest {
        tasklist::TasklistRequest::new(
            &self.access_token(),
            &tasklist::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

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

    pub fn tasklist_tasks(&self, tasklist_id: usize) -> tasklist::task::TasklistTaskRequest {
        tasklist::task::TasklistTaskRequest::new(
            &self.access_token(),
            &tasklist::task::model_path(self.portal_id(), self.project_id(), tasklist_id),
            None,
        )
    }

    pub fn timesheet(&self, id: i64) -> timesheet::TimesheetRequest {
        timesheet::TimesheetRequest::new(
            &self.access_token(),
            &timesheet::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn timesheets(&self) -> timesheet::TimesheetRequest {
        timesheet::TimesheetRequest::new(
            &self.access_token(),
            &timesheet::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }
}
