use errors::*;
use reqwest;
use serde;
use RelativePath;
use projects::{Project, ProjectFragment, ZohoProjects};
use portals::{Portal, PortalFragment, ZohoPortals};
use bugs::{BugFragment, ZohoBugs};
use milestones::{MilestoneFragment, ZohoMilestones};
use tasklists::{TasklistFragment, ZohoTasklists};
use tasks::{TaskFragment, ZohoTasks};

#[derive(Debug)]
pub struct ZohoClient {
    authtoken: String,
    client: reqwest::Client,
    portal_id: Option<i64>,
    project_id: Option<i64>,
}

impl ZohoClient {
    // Ensure we have a portal ID set before we do anything else. This has to be set after initialization because
    // we need a client to get the valid portal IDs.
    fn po_id(&self) -> i64 {
        match self.portal_id {
            Some(id) => id,
            None => panic!(
                "Please call client.set_portal with a valid portal ID before using this function
                You can use client.portals to find the available portal IDs for your auth token"
            ),
        }
    }

    // Ensure we have a portal ID set before we do anything else. This has to be set after initialization because
    // we need a client to get the valid portal IDs.
    fn pt_id(&self) -> i64 {
        match self.project_id {
            Some(id) => id,
            None => panic!(
                "Please call client.set_project with a valid portal ID before using this function
                You can use client.projects to find the available portal IDs for your auth token"
            ),
        }
    }

    fn make_uri(&self, relative_path: &str) -> String {
        format!(
            "https://projectsapi.zoho.com/restapi/{}{}",
            relative_path, self.authtoken
        )
    }

    pub fn get_url<T>(&self, url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut response = self.client.get(url).send()?;
        if !response.status().is_success() {
            bail!("Server error: {:?}", response.status());
        };
        let res_obj = response.json::<T>()?;
        Ok(res_obj)
    }

    pub fn get<T, U>(&self, params: U) -> Result<T>
    where
        T: serde::de::DeserializeOwned + RelativePath<U>,
    {
        let url: String = self.make_uri(&T::relative_path(params));
        self.get_url(&url)
    }

    pub fn set_portal(&mut self, portal_id: i64) -> Result<()> {
        self.portal_id = Some(portal_id);
        Ok(())
    }

    pub fn set_project(&mut self, project_id: i64) -> Result<()> {
        self.project_id = Some(project_id);
        Ok(())
    }

    pub fn portal(&self) -> Result<Option<Portal>> {
        let portal = self.portals().by_id(self.po_id()).call()?;
        Ok(portal)
    }

    pub fn project(&self) -> Result<Option<Project>> {
        let project = self.projects().by_id(self.pt_id()).call()?;
        Ok(project)
    }

    pub fn portals(&self) -> PortalFragment {
        PortalFragment {
            client: &self,
            path: self.make_uri(&ZohoPortals::relative_path(None)),
        }
    }

    pub fn projects(&self) -> ProjectFragment {
        ProjectFragment {
            client: &self,
            path: self.make_uri(&ZohoProjects::relative_path(self.po_id())),
        }
    }

    pub fn project_identifiers(&self) -> Result<Vec<(String, i64)>> {
        let projects = self.projects().call()?;
        Ok(projects
            .into_iter()
            .map(|p| (p.name, p.id))
            .collect::<Vec<(String, i64)>>())
    }

    pub fn bugs(&self) -> BugFragment {
        BugFragment {
            client: &self,
            path: self.make_uri(&ZohoBugs::relative_path([self.po_id(), self.pt_id()])),
        }
    }

    pub fn milestones(&self) -> MilestoneFragment {
        MilestoneFragment {
            client: &self,
            path: self.make_uri(&ZohoMilestones::relative_path([self.po_id(), self.pt_id()])),
        }
    }

    pub fn tasklists(&self) -> TasklistFragment {
        TasklistFragment {
            client: &self,
            path: self.make_uri(&ZohoTasklists::relative_path([self.po_id(), self.pt_id()])),
        }
    }
    pub fn tasks(&self) -> TaskFragment {
        TaskFragment {
            client: &self,
            path: self.make_uri(&ZohoTasks::relative_path([self.po_id(), self.pt_id()])),
        }
    }
}

pub fn create_client(auth_token: &str) -> Result<ZohoClient> {
    let new_client = ZohoClient {
        authtoken: format!("?authtoken={}", auth_token),
        client: reqwest::Client::new(),
        portal_id: None,
        project_id: None,
    };
    // If the provided auth token is invalid, this lib is useless, so return
    // an error instead of a client.
    let check_portal = new_client.portals().call();
    match check_portal {
        Ok(_) => Ok(new_client),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_invalid_client() {
        let client = create_client("bad-auth-token");
        assert!(client.is_err());
    }
}
