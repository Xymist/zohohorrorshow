use errors::*;
use RelativePath;

#[derive(Deserialize, Debug)]
pub struct ZohoMilestones {
    pub milestones: Option<Vec<Milestone>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Milestone {
    pub name: String,
    pub id: u64,
}

impl<'a> RelativePath<[&'a str; 2]> for ZohoMilestones {
    fn relative_path(params: [&'a str; 2]) -> Result<String> {
        Ok(format!(
            "portal/{}/projects/{}/milestones/",
            params[0], params[1]
        ))
    }
}
