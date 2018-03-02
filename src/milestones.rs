use errors::*;
use RelativePath;
use client::ZohoClient;

#[derive(Debug)]
pub struct MilestoneFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> MilestoneFragment<'a> {
    // Index number of the milestone.
    pub fn index(self, index: i64) -> MilestoneFragment<'a> {
        MilestoneFragment {
            client: self.client,
            path: format!("{}&index={}", self.path, index),
        }
    }
    // Range of the milestones.
    pub fn range(self, range: i64) -> MilestoneFragment<'a> {
        MilestoneFragment {
            client: self.client,
            path: format!("{}&range={}", self.path, range),
        }
    }
    // Status of the milestone. Accepts 'completed' or 'notcompleted'.
    pub fn status(self, status: String) -> MilestoneFragment<'a> {
        MilestoneFragment {
            client: self.client,
            path: format!("{}&status={}", self.path, status),
        }
    }
    // Milestone type. Accepts 'upcoming' or 'delayed'.
    pub fn display_type(self, display_type: String) -> MilestoneFragment<'a> {
        MilestoneFragment {
            client: self.client,
            path: format!("{}&display_type={}", self.path, display_type),
        }
    }
    // Milestone flag. Accepts 'internal' or 'external'.
    pub fn flag(self, flag: String) -> MilestoneFragment<'a> {
        MilestoneFragment {
            client: self.client,
            path: format!("{}&flag={}", self.path, flag),
        }
    }
    // Execute the query against the Zoho API
    pub fn call(self) -> Vec<Milestone> {
        let milestone_list: ZohoMilestones = self.client.get_url(&self.path).unwrap();
        milestone_list.milestones
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
    #[serde(rename = "owner_id")]
    pub owner_id: String,
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

impl Milestone {
    pub fn completed(&self) -> bool {
        match self.completed_date {
            Some(_) => true,
            None => false,
        }
    }
    pub fn overdue(&self) -> bool {
        //  End date < current date?
        unimplemented!();
    }
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

impl<'a> RelativePath<[i64; 2]> for ZohoMilestones {
    fn relative_path(params: [i64; 2]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}/milestones/",
            params[0], params[1]
        ))
    }
}
