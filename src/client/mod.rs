use errors::*;
use reqwest;
use serde;
use RelativePath;
use projects::{Project, ZohoProjects, ProjectFragment};
use portals::{Portal, ZohoPortals, PortalFragment};
use bugs::{Bug, BugFragment, ZohoBugs};
use milestones::{ZohoMilestones, Milestone, MilestoneFragment};

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

    pub fn portal(&self) -> Result<Option<Portal>> {
        let mut portals = self.portals().by_id(self.pid()).call();
        match portals.len() {
            n if n > 0 => Ok(Some(portals.remove(0))),
            _ => Ok(None),
        }
    }

    pub fn portals(&self) -> PortalFragment {
        PortalFragment {
            client: &self,
            path: self.make_uri(
                ZohoPortals::relative_path(None)
                    .unwrap()
                    .as_ref(),
                None,
            ).unwrap(),
        }
    }

    pub fn projects(&self) -> ProjectFragment {
        ProjectFragment {
            client: &self,
            path: self.make_uri(
                ZohoProjects::relative_path(self.pid())
                    .unwrap()
                    .as_ref(),
                None,
            ).unwrap(),
        }
    }

    pub fn bugs(&self, project_id: &str) -> BugFragment {
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

    pub fn milestones(&self, project_id: &str) -> MilestoneFragment {
        MilestoneFragment {
            client: &self,
            path: self.make_uri(
                ZohoMilestones::relative_path([format!("{}", self.pid()).as_ref(), project_id])
                    .unwrap()
                    .as_ref(),
                None,
            ).unwrap(),
        }
    }
}

pub fn create_client(auth_token: &str) -> Result<ZohoClient> {
    let new_client = ZohoClient {
        authtoken: format!("?authtoken={}", auth_token),
        client: reqwest::Client::new(),
        portal_id: None,
    };
    // If the provided auth token is invalid, this lib is useless, so return
    // an error instead of a client.
    new_client
        .portals()
        .call();
    Ok(new_client)
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
