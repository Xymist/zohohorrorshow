use errors::*;
use RelativePath;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoTimelogs {
    #[serde(rename = "timelogs")]
    pub timelogs: Timelogs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timelogs {
    #[serde(rename = "grandtotal")]
    pub grandtotal: String,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "timelog")]
    pub timelog: Timelog,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timelog {
    #[serde(rename = "date")]
    pub date: Vec<Date>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    #[serde(rename = "date_long")]
    pub date_long: i64,
    #[serde(rename = "display_format")]
    pub display_format: String,
    #[serde(rename = "totalhours")]
    pub totalhours: String,
    #[serde(rename = "buglogs")]
    pub buglogs: Vec<Buglog>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Buglog {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "notes")]
    pub notes: String,
    #[serde(rename = "hours")]
    pub hours: i64,
    #[serde(rename = "minutes")]
    pub minutes: i64,
    #[serde(rename = "hour_display")]
    pub hour_display: String,
    #[serde(rename = "total_minutes")]
    pub total_minutes: i64,
    #[serde(rename = "owner_name")]
    pub owner_name: String,
    #[serde(rename = "bill_status")]
    pub bill_status: String,
    #[serde(rename = "project")]
    pub project: Project,
    #[serde(rename = "bug")]
    pub bug: Bug,
    #[serde(rename = "link")]
    pub link: Link,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bug {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfLink {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}

impl<'a> RelativePath<[i64; 2]> for ZohoTimelogs {
    fn relative_path(params: [i64; 2]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}/logs/",
            params[0], params[1]
        ))
    }
}
