use errors::*;
use RelativePath;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoPortals {
    pub login_id: String,
    pub portals: Vec<Portal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Portal {
    pub trial_enabled: bool,
    pub settings: PortalSettings,
    pub gmt_time_zone: String,
    pub project_count: ProjectCount,
    pub role: String,
    pub bug_singular: String,
    pub link: ProjectLink,
    pub bug_plan: String,
    pub locale: PortalLocale,
    pub available_projects: i32,
    pub default: bool,
    pub profile_id: i64,
    pub name: String,
    pub id_string: String,
    pub id: i32,
    pub bug_plural: String,
    pub plan: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortalLocale {
    pub country: String,
    pub code: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectLink {
    pub project: ProjectLinkProject,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectLinkProject {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectCount {
    pub active: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortalSettings {
    pub timelog_period: TimeLogPeriod,
    pub company_name: String,
    pub date_format: String,
    pub time_zone: String,
    pub startday_of_week: String,
    pub task_date_format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeLogPeriod {
    pub log_future_time: Permission,
    pub log_past_time: Permission,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub allowed: bool,
}

// For reasons best known to Zoho, retrieving one portal record actually
// returns an array of length 1, containing the record we actually want.
impl RelativePath<Option<i32>> for ZohoPortals {
    fn relative_path(param: Option<i32>) -> Result<String> {
        match param {
            Some(portal_id) => Ok(format!("portals/{}", portal_id)),
            None => Ok("portals/".to_string()),
        }
    }
}

impl RelativePath<Option<i32>> for Portal {
    fn relative_path(_param: Option<i32>) -> Result<String> {
        bail!(
            "This doesn't work; if you need a single Portal pass the id to a
             ZohoPortals struct and call .portals[0] on the result."
        )
    }
}
