use crate::errors::*;
use crate::models::{activity, portal, project};
use crate::request::RequestParameters;
use reqwest;
use serde;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
pub const BASE_URL: &str = "https://projectsapi.zoho.com/restapi";

#[cfg(test)]
pub const BASE_URL: &str = mockito::SERVER_URL;

#[derive(Debug, Clone)]
pub struct ZohoClient {
    auth_token: String,
    portal_id: Option<i64>,
    project_id: Option<i64>,
}

impl ZohoClient {
    pub fn new(auth_token: &str) -> Self {
        ZohoClient {
            auth_token: auth_token.to_owned(),
            portal_id: None,
            project_id: None,
        }
    }

    pub fn set_portal(mut self, portal_name: &str) -> Result<Self> {
        let portals = self.portals().get();
        let portal = match portals {
            Ok(p_list) => p_list.portals.into_iter().find(|p| p.name == portal_name),
            Err(_) => return Err("Failed to fetch portals from Zoho".into()),
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
            Ok(p_list) => p_list.projects.into_iter().find(|p| p.name == project_name),
            Err(_) => return Err("Failed to fetch portals from Zoho".into()),
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

    pub fn activities(&self) -> activity::ActivityRequest {
        activity::ActivityRequest::new(
            &self.auth_token.clone(),
            &activity::model_path(self.portal_id(), self.project_id()),
        )
    }

    pub fn portals(&self) -> portal::PortalRequest {
        portal::PortalRequest::new(&self.auth_token.clone())
    }

    pub fn projects(&self) -> project::ProjectRequest {
        project::ProjectRequest::new(
            &self.auth_token.clone(),
            &project::model_path(self.portal_id()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    const PORTAL: &str = "{
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

    const PROJECT: &str = "{
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
        let client = ZohoClient::try_new("bad-auth-token", None, None);
        assert!(client.is_err());
    }

    #[test]
    fn force_populate_client() {
        let _portals = mock("GET", "/portals/?authtoken=abc123")
            .with_status(201)
            .with_body(PORTAL)
            .create();

        let _projects = mock("GET", "/portal/2063927/projects/?authtoken=abc123")
            .with_status(201)
            .with_body(PROJECT)
            .create();

        let client = ZohoClient::try_new(
            "abc123",
            Some("zillum"),
            Some("Promotional banner for women's day"),
        )
        .unwrap();

        assert_eq!(client.auth_token, "abc123");
        assert_eq!(client.portal_id(), 2063927);
        assert_eq!(client.project_id(), 170876000003152069);
    }

    #[test]
    fn autopopulate_client() {
        let _portals = mock("GET", "/portals/?authtoken=abc123")
            .with_status(201)
            .with_body(PORTAL)
            .create();

        let _projects = mock("GET", "/portal/2063927/projects/?authtoken=abc123")
            .with_status(201)
            .with_body(PROJECT)
            .create();

        let client = ZohoClient::try_new("abc123", None, None).unwrap();

        assert_eq!(client.auth_token, "abc123");
        assert_eq!(client.portal_id(), 2063927);
        assert_eq!(client.project_id(), 170876000003152069);
    }
}
