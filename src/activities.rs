use errors::*;
use RelativePath;
use client::ZohoClient;

#[derive(Debug)]
pub struct ActivityFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> ActivityFragment<'a> {
    // Index number of the activity.
    pub fn index(self, index: i64) -> ActivityFragment<'a> {
        ActivityFragment {
            client: self.client,
            path: format!("{}&index={}", self.path, index),
        }
    }
    // Range of the activity.
    pub fn range(self, range: i64) -> ActivityFragment<'a> {
        ActivityFragment {
            client: self.client,
            path: format!("{}&range={}", self.path, range),
        }
    }
    // Execute the query against the Zoho API
    pub fn call(self) -> Vec<Activity> {
        let activity_list: ZohoActivities = self.client.get_url(&self.path).unwrap();
        activity_list.activities
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoActivities {
    #[serde(rename = "activities")]
    pub activities: Vec<Activity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "activity_for")]
    pub activity_for: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "activity_by")]
    pub activity_by: String,
    #[serde(rename = "time_long")]
    pub time_long: i64,
    #[serde(rename = "display_time")]
    pub display_time: String,
    #[serde(rename = "time")]
    pub time: String,
}

impl<'a> RelativePath<[i64; 2]> for ZohoActivities {
    fn relative_path(params: [i64; 2]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}/activities/",
            params[0], params[1]
        ))
    }
}
