use client::ZohoClient;
use errors::*;
use std::rc::Rc;
use utils::from_str;

pub fn milestones(client: Rc<ZohoClient>) -> MilestoneFragment {
    MilestoneFragment {
        client: Rc::clone(&client),
        path: client.make_uri(&format!(
            "portal/{}/projects/{}/milestones/",
            client.portal_id(),
            client.project_id()
        )),
    }
}

#[derive(Debug)]
pub struct MilestoneFragment {
    pub client: Rc<ZohoClient>,
    pub path: String,
}

impl MilestoneFragment {
    query_strings!(MilestoneFragment; index, range, status, display_type, flag);

    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Milestone>> {
        let milestone_list: ZohoMilestones = self.client.get(&self.path)?;
        Ok(milestone_list.milestones)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoMilestones {
    #[serde(rename = "milestones")]
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Milestone {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "link")]
    pub link: Link,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "owner_name")]
    pub owner_name: String,
    #[serde(rename = "owner_id", deserialize_with = "from_str")]
    pub owner_id: i64,
    #[serde(rename = "flag")]
    pub flag: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "start_date_long")]
    pub start_date_long: i64,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "end_date_long")]
    pub end_date_long: i64,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "completed_date")]
    pub completed_date: Option<String>,
    #[serde(rename = "completed_date_long")]
    pub completed_date_long: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
    #[serde(rename = "status")]
    pub status: SelfLink,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelfLink {
    #[serde(rename = "url")]
    pub url: String,
}