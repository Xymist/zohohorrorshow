use errors::*;
use reqwest;
use serde;
use RelativePath;
use projects::{Project, ZohoProjects};
use portals::{Portal, ZohoPortals};
use bugs::{Bug, BugFragment, ZohoBugs};
use milestones::{Milestone, ZohoMilestones};

#[derive(Debug)]
pub struct ZohoClient {
    authtoken: String,
    client: reqwest::Client,
    portal_id: Option<i64>,
}

impl ZohoClient {
    // Ensure we have a portal ID set before we do anything else. This has to be set after initialization because
    // we need a client to get the valid portal IDs.
    fn pid(&self) -> i64 {
        match self.portal_id {
            Some(id) => id,
            None => panic!(
                "Please call client.set_portal with a valid portal ID before using this function
                You can use client.get_portals to find the available portal IDs for your auth token")
        }
    }

    fn make_uri(&self, relative_path: &str, query: Option<&str>) -> Result<String> {
        match query {
            Some(query_string) => Ok(format!(
                "https://projectsapi.zoho.com/restapi/{}{}{}",
                relative_path, self.authtoken, query_string
            )),
            None => Ok(format!(
                "https://projectsapi.zoho.com/restapi/{}{}",
                relative_path, self.authtoken
            )),
        }
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

    pub fn get<T, U>(&self, params: U, query: Option<&str>) -> Result<T>
    where
        T: serde::de::DeserializeOwned + RelativePath<U>,
    {
        let url: String = self.make_uri(&T::relative_path(params)?, query)?;
        self.get_url(&url)
    }

    pub fn set_portal(&mut self, portal_id: i64) -> Result<()> {
        self.portal_id = Some(portal_id);
        Ok(())
    }

    pub fn portals(&self) -> Result<Vec<Portal>> {
        let portals: Result<ZohoPortals> = self.get(None, None);
        let portal_list: ZohoPortals = portals.unwrap();
        Ok(portal_list.portals)
    }

    pub fn portal(&self) -> Result<Option<Portal>> {
        let portals: Result<ZohoPortals> = self.get(Some(self.pid()), None);
        let mut portal_list: ZohoPortals = portals.unwrap();
        match portal_list.portals.len() {
            n if n > 0 => Ok(Some(portal_list.portals.remove(0))),
            _ => Ok(None),
        }
    }

    pub fn projects(&self) -> Result<Vec<Project>> {
        let projects: Result<ZohoProjects> = self.get(self.pid(), None);
        let project_list: ZohoProjects = projects.unwrap();
        Ok(project_list.projects)
    }

    pub fn bugs(&self, project_id: &str) -> Result<Vec<Bug>> {
        let bugs: Result<ZohoBugs> =
            self.get([format!("{}", self.pid()).as_ref(), project_id], None);
        let bug_list: ZohoBugs = bugs.unwrap();
        Ok(bug_list.bugs)
    }

    pub fn filtered_bugs(&self, project_id: &str) -> BugFragment {
        BugFragment {
            client: &self,
            path: self.make_uri(
                ZohoBugs::relative_path([format!("{}", self.pid()).as_ref(), project_id])
                    .unwrap()
                    .as_ref(),
                None,
            ).unwrap(),
        }
    }
}

pub fn create_client(auth_token: &str) -> Result<ZohoClient> {
    Ok(ZohoClient {
        authtoken: format!("?authtoken={}", auth_token),
        client: reqwest::Client::new(),
        portal_id: None,
    })
}
