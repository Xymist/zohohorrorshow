//! The client module contains the implementations for the API client itself and wraps the OAuth process necessary
//! to make requests for a user's data.
//! If in doubt, this is the one to import.
//! Each model is provided its own plural-named method on ZohoClient for retrieving many entries.
//! Where possible there is also a singular-names method which takes an ID parameter to retrieve a single entry.

use crate::errors::*;
use crate::models::{
    activity, bug, category, event, forum, milestone, portal, project, status, task, tasklist,
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
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        let credentials = oauth::Credentials::new(client_id, client_secret, None, None);

        ZohoClient {
            oauth_credentials: credentials,
            portal_id: None,
            project_id: None,
        }
    }

    pub fn access_token(&mut self) -> String {
        self.oauth_credentials.access_token()
    }

    pub fn set_portal(mut self, portal_name: &str) -> Result<Self> {
        let portals = self.portals().get();
        let portal = match portals {
            Ok(Some(p_list)) => p_list.portals.into_iter().find(|p| p.name == portal_name),
            Err(_) | Ok(None) => return Err("Failed to fetch portals from Zoho".into()),
        };

        if let Some(po) = portal {
            self.portal_id = Some(po.id)
        } else {
            return Err(format!("Could not find portal with name {}", portal_name).into());
        };

        Ok(self)
    }

    pub fn set_project(mut self, project_name: &str) -> Result<Self> {
        let projects = self.projects().get();
        let project = match projects {
            Ok(Some(p_list)) => p_list.projects.into_iter().find(|p| p.name == project_name),
            Err(_) | Ok(None) => return Err("Failed to fetch portals from Zoho".into()),
        };

        if let Some(pr) = project {
            self.project_id = Some(pr.id)
        } else {
            return Err(format!("Could not find project with name {}", project_name).into());
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

    pub fn activities(&mut self) -> activity::ActivityRequest {
        activity::ActivityRequest::new(
            &self.access_token(),
            &activity::model_path(self.portal_id(), self.project_id()),
        )
    }

    pub fn bug(&mut self, id: i64) -> bug::BugRequest {
        bug::BugRequest::new(
            &self.access_token(),
            &bug::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn bugs(&mut self) -> bug::BugRequest {
        bug::BugRequest::new(
            &self.access_token(),
            &bug::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn category(&mut self, id: i64) -> category::CategoryRequest {
        category::CategoryRequest::new(
            &self.access_token(),
            &category::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn categories(&mut self) -> category::CategoryRequest {
        category::CategoryRequest::new(
            &self.access_token(),
            &category::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn event(&mut self, id: i64) -> event::EventRequest {
        event::EventRequest::new(
            &self.access_token(),
            &event::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn events(&mut self) -> event::EventRequest {
        event::EventRequest::new(
            &self.access_token(),
            &event::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn forum(&mut self, id: i64) -> forum::ForumRequest {
        forum::ForumRequest::new(
            &self.access_token(),
            &event::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn forums(&mut self) -> forum::ForumRequest {
        forum::ForumRequest::new(
            &self.access_token(),
            &event::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn forum_comment(&mut self, forum_id: i64, id: i64) -> forum::comment::CommentRequest {
        forum::comment::CommentRequest::new(
            &self.access_token(),
            &forum::comment::model_path(self.portal_id(), self.project_id(), forum_id),
            Some(id),
        )
    }

    pub fn forum_comments(&mut self, forum_id: i64) -> forum::comment::CommentRequest {
        forum::comment::CommentRequest::new(
            &self.access_token(),
            &forum::comment::model_path(self.portal_id(), self.project_id(), forum_id),
            None,
        )
    }

    pub fn milestone(&mut self, id: i64) -> milestone::MilestoneRequest {
        milestone::MilestoneRequest::new(
            &self.access_token(),
            &milestone::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn milestones(&mut self) -> milestone::MilestoneRequest {
        milestone::MilestoneRequest::new(
            &self.access_token(),
            &milestone::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn portals(&mut self) -> portal::PortalRequest {
        portal::PortalRequest::new(&self.access_token())
    }

    pub fn project(&mut self, id: i64) -> project::ProjectRequest {
        project::ProjectRequest::new(
            &self.access_token(),
            &project::model_path(self.portal_id()),
            Some(id),
        )
    }

    pub fn projects(&mut self) -> project::ProjectRequest {
        project::ProjectRequest::new(
            &self.access_token(),
            &project::model_path(self.portal_id()),
            None,
        )
    }

    pub fn statuses(&mut self) -> status::StatusRequest {
        status::StatusRequest::new(
            &self.access_token(),
            &status::model_path(self.portal_id(), self.project_id()),
        )
    }

    pub fn task(&mut self, id: i64) -> task::TaskRequest {
        task::TaskRequest::new(
            &self.access_token(),
            &task::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn tasks(&mut self) -> task::TaskRequest {
        task::TaskRequest::new(
            &self.access_token(),
            &task::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }

    pub fn tasklist(&mut self, id: i64) -> tasklist::TasklistRequest {
        tasklist::TasklistRequest::new(
            &self.access_token(),
            &tasklist::model_path(self.portal_id(), self.project_id()),
            Some(id),
        )
    }

    pub fn tasklists(&mut self) -> tasklist::TasklistRequest {
        tasklist::TasklistRequest::new(
            &self.access_token(),
            &tasklist::model_path(self.portal_id(), self.project_id()),
            None,
        )
    }
}
