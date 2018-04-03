use client::ZohoClient;
use errors::*;
use std::rc::Rc;

pub fn activities(cl: &Rc<ZohoClient>) -> ActivityFragment {
    let client = Rc::clone(cl);
    ActivityFragment {
        path: client.make_uri(&format!(
            "portal/{}/projects/{}/activities/",
            client.portal_id(),
            client.project_id()
        )),
        client,
    }
}

#[derive(Debug)]
pub struct ActivityFragment {
    pub client: Rc<ZohoClient>,
    pub path: String,
}

impl ActivityFragment {
    query_strings!(ActivityFragment; index, range);

    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Activity>> {
        let activity_list: ZohoActivities = self.client.get(&self.path)?;
        Ok(activity_list.activities)
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
