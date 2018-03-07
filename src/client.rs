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

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const BASE_URL: &str = "https://projectsapi.zoho.com/restapi";

#[cfg(test)]
const BASE_URL: &str = mockito::SERVER_URL;

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
            "{}/{}?authtoken={}",
            BASE_URL, relative_path, self.authtoken
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
        authtoken: auth_token.to_string(),
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

// Generates a client from an auth_token, a portal name and a project name, searching for the latter two in that order.
// The portal and project names are optional; if either is missing then it will be populated with the first in the
// list from Zoho, which is often the only one (in the case of a portal) or the oldest one chronologically (in the case
// of a project)
pub fn create_populated_client(
    auth_token: &str,
    portal_name: Option<&str>,
    project_name: Option<&str>,
) -> Result<ZohoClient> {
    let mut client = create_client(auth_token)?;
    let portal = match portal_name {
        Some(name) => client.portals().by_name(name).call()?,
        None => {
            let mut ptls = client.portals().call()?;
            match ptls.len() {
                0 => None,
                _ => Some(ptls.remove(0)),
            }
        }
    };
    if let Some(p) = portal { client.set_portal(p.id)? };
    let project = match project_name {
        Some(name) => client.projects().by_name(name).call()?,
        None => {
            let mut pjts = client.projects().call()?;
            match pjts.len() {
                0 => None,
                _ => Some(pjts.remove(0)),
            }
        }
    };
    if let Some(p) = project { client.set_project(p.id)? };
    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    const PORTAL: &'static str = "{
        \"login_id\": \"2060758\",
        \"portals\": [{
            \"id\": 2063927,
            \"name\": \"zillum\",
            \"default\": true,
            \"gmt_time_zone\": \"(GMT 5:30) India Standard Time\",
            \"role\": \"admin\",
            \"project_count\":{
            \"template\":3,
            \"archived\":1,
            \"active\":30
            },
            \"settings\": {
                \"company_name\": \"Zillum Inc.\",
                \"website_url\": \"www.zillum.com\",
                \"time_zone\": \"Asia/Calcutta\",
                \"date_format\": \"MM-dd-yyyy hh:mm aaa\"
            },
            \"locale\": {
                \"code\": \"en_US\",
                \"language\": \"English\",
                \"country\": \"United States\"
            },
            \"link\": {
                \"project\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/2063927/projects\"
                }
            }
        }]
    }";

    const PROJECT: &'static str = "{
        \"projects\": [{
            \"custom_fields\": [{
                \"Template design\": \"Rightnav_temp\"
            }, {
                \"Promos per second\": \"2\"
            }, {
                \"Blog announcement\": \"02-25-2016\"
            }, {
                \"Promo publish date\": \"02-17-2016\"
            }, {
                \"Content approval\": \"Amritha Agrawal\"
            }],
            \"created_date\": \"02-16-2016\",
            \"IS_BUG_ENABLED\": true,
            \"owner_name\": \"Patricia Boyle\",
            \"task_count\": {
                \"open\": 0,
                \"closed\": 0
            },
            \"start_date_long\": -32401001,
            \"status\": \"active\",
            \"link\": {
                \"folder\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/folders/\"
                },
                \"milestone\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/milestones/\"
                },
                \"forum\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/forums/\"
                },
                \"document\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/documents/\"
                },
                \"status\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/statuses/\"
                },
                \"event\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/events/\"
                },
                \"task\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/tasks/\"
                },
                \"bug\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/bugs/\"
                },
                \"self\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/\"
                },
                \"timesheet\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/logs/\"
                },
                \"user\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/users/\"
                },
                \"tasklist\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/tasklists/\"
                },
                \"activity\": {
                    \"url\": \"https://projectsapi.zoho.com/restapi/portal/zillum/projects/170876000003152069/activities/\"
                }
            },
            \"created_date_format\": \"02-16-2016 03:20:43 AM\",
            \"workspace_id\": \"2ea4657bfe29202df4eda90dabc651e61b1d6\",
            \"milestone_count\": {
                \"open\": 0,
                \"closed\": 0
            },
            \"created_date_long\": 1455621643662,
            \"end_date_format\": \"12-31-1969 06:59:59 AM\",
            \"id\": 170876000003152069,
            \"end_date\": \"12-31-1969\",
            \"id_string\": \"170876000003152069\",
            \"description\": \"fdb df b\",
            \"name\": \"Promotional banner for women's day\",
            \"owner_id\": \"2060758\",
            \"end_date_long\": -32401001,
            \"role\": \"admin\",
            \"start_date_format\": \"12-31-1969 06:59:59 AM\",
            \"start_date\": \"12-31-1969\"
        }]
    }";

    #[test]
    fn create_invalid_client() {
        let client = create_client("bad-auth-token");
        assert!(client.is_err());
    }

    #[test]
    fn create_bare_client() {
        let _m = mock("GET", "/portals/?authtoken=abc123")
            .with_status(201)
            .with_body(PORTAL)
            .create();

        let client = create_client("abc123").unwrap();

        assert_eq!(client.authtoken, "abc123");
        assert_eq!(client.portal_id, None);
        assert_eq!(client.project_id, None);
    }

    #[test]
    fn populate_client() {
        let _portals = mock("GET", "/portals/?authtoken=abc123")
            .with_status(201)
            .with_body(PORTAL)
            .create();

        let _projects = mock("GET", "/portal/2063927/projects/?authtoken=abc123")
            .with_status(201)
            .with_body(PROJECT)
            .create();

        let client = create_populated_client("abc123", None, None).unwrap();

        assert_eq!(client.authtoken, "abc123");
        assert_eq!(client.portal_id, Some(2063927));
        assert_eq!(client.project_id, Some(170876000003152069));
    }
}
