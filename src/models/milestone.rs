use crate::client::ZohoClient;
use crate::errors::*;
use crate::serializers::from_str;

pub const ModelPath: &str = "portal/{}/projects/{}/milestones/";

pub fn milestones(cl: &ZohoClient) -> MilestoneFragment {
    let client = cl.clone();
    MilestoneFragment {
        path: client.make_uri(&format!(
            "portal/{}/projects/{}/milestones/",
            client.portal_id(),
            client.project_id()
        )),
        client,
    }
}

#[derive(Debug)]
pub struct MilestoneFragment {
    pub client: ZohoClient,
    pub path: String,
}

impl MilestoneFragment {
    query_strings!(index, range, status, display_type, flag);

    // Fetch a specific portal by id
    pub fn by_id(self, id: i64) -> MilestoneFilter {
        let mut path_frags = self.path.split('?').collect::<Vec<&str>>();
        if path_frags[1].contains('&') {
            let autht = path_frags.remove(1).split('&').collect::<Vec<&str>>()[0];
            path_frags.push(autht)
        }
        MilestoneFilter {
            client: self.client.clone(),
            path: format!("{}{}/?{}", path_frags[0], id, path_frags[1]),
            filter: Filter::ID(id),
        }
    }

    // Fetch a specific portal by name
    pub fn by_name(self, name: &str) -> MilestoneFilter {
        if self.path.contains('&') {
            panic!("Cannot both filter and find by name")
        }
        MilestoneFilter {
            client: self.client.clone(),
            path: self.path,
            filter: Filter::Name(name.to_owned()),
        }
    }

    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Milestone>> {
        let milestone_list: ZohoMilestones = self.client.get(&self.path)?;
        Ok(milestone_list.milestones)
    }
}

#[derive(Debug)]
enum Filter {
    ID(i64),
    Name(String),
}

#[derive(Debug)]
pub struct MilestoneFilter {
    client: ZohoClient,
    path: String,
    filter: Filter,
}

impl MilestoneFilter {
    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Option<Milestone>> {
        let milestone_list: ZohoMilestones = self.client.get(&self.path)?;
        let milestones = milestone_list.milestones;
        match self.filter {
            Filter::ID(id) => filter_by_id(milestones, id),
            Filter::Name(name) => filter_by_name(milestones, &name),
        }
    }
}

fn filter_by_id(milestones: Vec<Milestone>, id: i64) -> Result<Option<Milestone>> {
    let mut filtered = milestones
        .into_iter()
        .filter(|m| m.id == id)
        .collect::<Vec<Milestone>>();
    match filtered.len() {
        0 => Ok(None),
        _ => Ok(Some(filtered.remove(0))),
    }
}

fn filter_by_name(milestones: Vec<Milestone>, name: &str) -> Result<Option<Milestone>> {
    let mut filtered = milestones
        .into_iter()
        .filter(|m| m.name == name)
        .collect::<Vec<Milestone>>();
    match filtered.len() {
        0 => Ok(None),
        _ => Ok(Some(filtered.remove(0))),
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZohoMilestones {
    #[serde(rename = "milestones")]
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: SelfLink,
    #[serde(rename = "status")]
    pub status: SelfLink,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SelfLink {
    #[serde(rename = "url")]
    pub url: String,
}
