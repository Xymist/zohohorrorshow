use crate::errors::*;
use crate::models::{activity, bug, category, event, forum, milestone, portal, project};
use crate::oauth;
use crate::request::RequestParameters;

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
        let projects = self.projects(None).get();
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

    pub fn bugs(&mut self, id: Option<i64>) -> bug::BugRequest {
        bug::BugRequest::new(
            &self.access_token(),
            &bug::model_path(self.portal_id(), self.project_id()),
            id,
        )
    }

    pub fn categories(&mut self, id: Option<i64>) -> category::CategoryRequest {
        category::CategoryRequest::new(
            &self.access_token(),
            &category::model_path(self.portal_id(), self.project_id()),
            id,
        )
    }

    pub fn events(&mut self, id: Option<i64>) -> event::EventRequest {
        event::EventRequest::new(
            &self.access_token(),
            &event::model_path(self.portal_id(), self.project_id()),
            id,
        )
    }

    pub fn forums(&mut self, id: Option<i64>) -> forum::ForumRequest {
        forum::ForumRequest::new(
            &self.access_token(),
            &event::model_path(self.portal_id(), self.project_id()),
            id,
        )
    }

    pub fn forum_comments(
        &mut self,
        forum_id: i64,
        id: Option<i64>,
    ) -> forum::comment::CommentRequest {
        forum::comment::CommentRequest::new(
            &self.access_token(),
            &forum::comment::model_path(self.portal_id(), self.project_id(), forum_id),
            id,
        )
    }

    pub fn milestones(&mut self, id: Option<i64>) -> milestone::MilestoneRequest {
        milestone::MilestoneRequest::new(
            &self.access_token(),
            &milestone::model_path(self.portal_id(), self.project_id()),
            id,
        )
    }

    pub fn portals(&mut self) -> portal::PortalRequest {
        portal::PortalRequest::new(&self.access_token())
    }

    pub fn projects(&mut self, id: Option<i64>) -> project::ProjectRequest {
        project::ProjectRequest::new(
            &self.access_token(),
            &project::model_path(self.portal_id()),
            id,
        )
    }
}
