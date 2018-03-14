use errors::*;
use client::ZohoClient;
use utils::from_str;

#[derive(Debug)]
pub struct PortalFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> PortalFragment<'a> {
    pub fn by_id(self, id: i64) -> PortalFilter<'a> {
        PortalFilter {
            client: self.client,
            path: self.path,
            filter: Filter::ID(id),
        }
    }
    pub fn by_name(self, name: &'a str) -> PortalFilter<'a> {
        PortalFilter {
            client: self.client,
            path: self.path,
            filter: Filter::Name(name),
        }
    }
    // Execute the query against the Zoho API
    pub fn call(self) -> Result<Vec<Portal>> {
        let portal_list: ZohoPortals = self.client.get(&self.path)?;
        Ok(portal_list.portals)
    }
}

#[derive(Debug)]
enum Filter<'a> {
    ID(i64),
    Name(&'a str),
}

#[derive(Debug)]
pub struct PortalFilter<'a> {
    client: &'a ZohoClient,
    path: String,
    filter: Filter<'a>,
}

impl<'a> PortalFilter<'a> {
    // Execute the query against the Zoho API
    pub fn call(self) -> Result<Option<Portal>> {
        let portal_list: ZohoPortals = self.client.get(&self.path)?;
        let portals = portal_list.portals;
        match self.filter {
            Filter::ID(id) => filter_by_id(portals, id),
            Filter::Name(name) => filter_by_name(portals, name),
        }
    }
}

fn filter_by_id(portals: Vec<Portal>, id: i64) -> Result<Option<Portal>> {
    let mut filtered = portals
        .into_iter()
        .filter(|p| p.id == id)
        .collect::<Vec<Portal>>();
    match filtered.len() {
        0 => Ok(None),
        _ => Ok(Some(filtered.remove(0))),
    }
}

fn filter_by_name(portals: Vec<Portal>, name: &str) -> Result<Option<Portal>> {
    let mut filtered = portals
        .into_iter()
        .filter(|p| p.name == name)
        .collect::<Vec<Portal>>();
    match filtered.len() {
        0 => Ok(None),
        _ => Ok(Some(filtered.remove(0))),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoPortals {
    #[serde(rename = "login_id", deserialize_with = "from_str")]
    pub login_id: i64,
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
