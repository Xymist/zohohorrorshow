use activities::ActivityFragment;
use bugs::BugFragment;
use categories::CategoryFragment;
use comments::CommentFragment;
use errors::*;
use events::EventFragment;
use forums::ForumFragment;
use milestones::MilestoneFragment;
use portals::PortalFragment;
use projects::ProjectFragment;
use reqwest;
use reqwest::Method::{self, Delete, Get, Post, Put};
use serde;
use statuses::StatusFragment;
use tasklists::TasklistFragment;
use tasks::TaskFragment;
use timesheets::TimesheetFragment;
use users::UserFragment;

#[cfg(test)]
use mockito;

#[cfg(not(test))]
const BASE_URL: &str = "https://projectsapi.zoho.com/restapi";

#[cfg(test)]
const BASE_URL: &str = mockito::SERVER_URL;

lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[derive(Debug)]
pub struct ZohoClient {
    auth_token: String,
    context: Context,
}

#[derive(Debug)]
struct Context {
    portal_id: Option<i64>,
    project_id: Option<i64>,
    forum_id: Option<i64>,
}

impl ZohoClient {
    // Generates a client from an auth_token, a portal name and a project name, searching for the latter two in that order.
    // The portal and project names are optional; if either is missing then it will be populated with the first in the
    // list from Zoho, which is often the only one (in the case of a portal) or the oldest one chronologically (in the case
    // of a project)
    pub fn new(
        auth_token: &str,
        portal_name: Option<&str>,
        project_name: Option<&str>,
    ) -> Result<ZohoClient> {
        let mut client = ZohoClient {
            auth_token: auth_token.to_string(),
            context: Context {
                portal_id: None,
                project_id: None,
                forum_id: None,
            },
        };
        let portal = match portal_name {
            Some(name) => client.portals().by_name(name).fetch()?,
            None => {
                let mut ptls = client.portals().fetch()?;
                match ptls.len() {
                    0 => None,
                    _ => Some(ptls.remove(0)),
                }
            }
        };
        if let Some(p) = portal {
            client.context.portal_id = Some(p.id)
        };
        let project = match project_name {
            Some(name) => client.projects().by_name(name).fetch()?,
            None => {
                let mut pjts = client.projects().fetch()?;
                match pjts.len() {
                    0 => None,
                    _ => Some(pjts.remove(0)),
                }
            }
        };
        if let Some(p) = project {
            client.context.project_id = Some(p.id)
        };
        Ok(client)
    }

    fn portal_id(&self) -> i64 {
        self.context.portal_id.expect(
            "Portal context called without portal id set.
            Hint: call ZohoClient::portal() with the ID of a valid portal to set the context.",
        )
    }

    fn project_id(&self) -> i64 {
        self.context.project_id.expect(
            "Project context called without project id set.
            Hint: call ZohoClient::project() with the ID of a valid project to set the context.",
        )
    }

    fn forum_id(&self) -> i64 {
        self.context.project_id.expect(
            "Forum context called without forum id set.
            Hint: call ZohoClient::forum() with the ID of a valid forum to set the context.",
        )
    }

    pub fn portal(&mut self, id: i64) {
        self.context.portal_id = Some(id)
    }

    pub fn project(&mut self, id: i64) {
        self.context.project_id = Some(id)
    }

    pub fn forum(&mut self, id: i64) {
        self.context.forum_id = Some(id)
    }

    fn make_uri(&self, relative_path: &str) -> String {
        format!(
            "{}/{}?authtoken={}",
            BASE_URL, relative_path, self.auth_token
        )
    }

    pub fn call_api<T>(&self, method: Method, url: &str, data: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut response = CLIENT.request(method, url).json(&data).send()?;
        if !response.status().is_success() {
            bail!("Server error: {:?}", response.status());
        };
        let res_obj = response.json::<T>()?;
        Ok(res_obj)
    }

    pub fn get<T>(&self, url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.call_api(Get, url, "")
    }

    pub fn post<T>(&self, url: &str, data: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.call_api(Post, url, data)
    }

    pub fn put<T>(&self, url: &str, data: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.call_api(Put, url, data)
    }

    pub fn delete<T>(&self, url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.call_api(Delete, url, "")
    }

    pub fn portals(&self) -> PortalFragment {
        PortalFragment {
            client: &self,
            path: self.make_uri("portals/"),
        }
    }

    pub fn projects(&self) -> ProjectFragment {
        ProjectFragment {
            client: &self,
            path: self.make_uri(&format!("portal/{}/projects/", self.portal_id())),
        }
    }

    pub fn portal_users(&self) -> UserFragment {
        UserFragment {
            client: &self,
            path: self.make_uri(&format!("portal/{}/users/", self.portal_id())),
        }
    }

    pub fn project_users(&self) -> UserFragment {
        UserFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/users/",
                self.portal_id(),
                self.project_id()
            )),
        }
    }

    pub fn bugs(&self) -> BugFragment {
        BugFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/bugs/",
                self.portal_id(),
                self.project_id()
            )),
        }
    }

    pub fn milestones(&self) -> MilestoneFragment {
        MilestoneFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/milestones/",
                self.portal_id(),
                self.project_id()
            )),
        }
    }

    pub fn tasklists(&self) -> TasklistFragment {
        TasklistFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/tasklists/",
                self.portal_id(),
                self.project_id()
            )),
        }
    }
    pub fn tasks(&self) -> TaskFragment {
        TaskFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/tasks/",
                self.portal_id(),
                self.project_id()
            )),
        }
    }

    pub fn activities(&self) -> ActivityFragment {
        ActivityFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/activities/",
                self.portal_id(),
                self.project_id()
            )),
        }
    }

    pub fn statuses(&self) -> StatusFragment {
        StatusFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/statuses/",
                self.portal_id(),
                self.project_id()
            )),
        }
    }

    pub fn timesheets(&self) -> TimesheetFragment {
        TimesheetFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/logs/",
                self.portal_id(),
                self.project_id()
            )),
        }
    }

    pub fn forums(&self) -> ForumFragment {
        ForumFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/forums/",
                self.portal_id(),
                self.project_id()
            )),
        }
    }

    pub fn categories(&self) -> CategoryFragment {
        CategoryFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/categories/",
                self.portal_id(),
                self.project_id()
            )),
        }
    }

    pub fn events(&self) -> EventFragment {
        EventFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/events/",
                self.portal_id(),
                self.project_id()
            )),
        }
    }

    pub fn comments(&self) -> CommentFragment {
        CommentFragment {
            client: &self,
            path: self.make_uri(&format!(
                "portal/{}/projects/{}/forums/{}/comments",
                self.portal_id(),
                self.project_id(),
                self.forum_id()
            )),
        }
    }
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
        let client = ZohoClient::new("bad-auth-token", None, None);
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

        let client = ZohoClient::new(
            "abc123",
            Some("zillum"),
            Some("Promotional banner for women's day"),
        ).unwrap();

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

        let client = ZohoClient::new("abc123", None, None).unwrap();

        assert_eq!(client.auth_token, "abc123");
        assert_eq!(client.portal_id(), 2063927);
        assert_eq!(client.project_id(), 170876000003152069);
    }
}
