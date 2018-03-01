use errors::*;
use RelativePath;
use client::ZohoClient;

#[derive(Debug)]
pub struct PortalFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> PortalFragment<'a> {
    // Fetch a specific portal
    pub fn by_id(self, id: i64) -> PortalFragment<'a> {
        if self.path.contains("&") {
            panic!("Cannot both filter and find by ID")
        }
        let path_frags = self.path.split("?").collect::<Vec<&str>>();
        PortalFragment {
            client: self.client,
            path: format!("{}{}/?{}", path_frags[0], id, path_frags[1]),
        }
    }

    // Execute the query against the Zoho API
    pub fn call(self) -> Vec<Portal> {
        let portal_list: ZohoPortals = self.client.get_url(&self.path).unwrap();
        portal_list.portals
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoPortals {
    #[serde(rename = "login_id")]
    pub login_id: String,
    #[serde(rename = "portals")]
    pub portals: Vec<Portal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Portal {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "default")]
    pub default_portal: bool,
    #[serde(rename = "gmt_time_zone")]
    pub gmt_time_zone: String,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "project_count")]
    pub project_count: ProjectCount,
    #[serde(rename = "settings")]
    pub settings: Settings,
    #[serde(rename = "locale")]
    pub locale: Locale,
    #[serde(rename = "link")]
    pub link: Link,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "project")]
    pub project: Project,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Locale {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "country")]
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectCount {
    #[serde(rename = "template")]
    pub template: Option<i64>,
    #[serde(rename = "archived")]
    pub archived: Option<i64>,
    #[serde(rename = "active")]
    pub active: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "company_name")]
    pub company_name: String,
    #[serde(rename = "website_url")]
    pub website_url: Option<String>,
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    #[serde(rename = "date_format")]
    pub date_format: String,
}

impl RelativePath<Option<i8>> for ZohoPortals {
    fn relative_path(_params: Option<i8>) -> Result<String> {
        Ok(String::from("portals/"))
    }
}
