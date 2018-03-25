use errors::*;
use client::ZohoClient;

// A fragment of the path to call for the Zoho Timesheets API. This carries
// with it a reference to the client which will be used to call it.
#[derive(Debug)]
pub struct TimesheetFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> TimesheetFragment<'a> {
    query_strings!(TimesheetFragment; index, range, date);

    pub fn users_list(mut self, ids: Option<Vec<i64>>) -> TimesheetFragment<'a> {
        let users = match ids {
            Some(u) => u.into_iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(","),
            None => "all".to_string(),
        };
        self.path = format!("{}&users_list={}", self.path, users);
        self
    }

    pub fn view_type(mut self, view_type: ViewType) -> TimesheetFragment<'a> {
        self.path = format!("{}&view_type={}", self.path, view_type.to_string());
        self
    }

    pub fn component_type(mut self, component_type: ComponentType) -> TimesheetFragment<'a> {
        self.path = format!(
            "{}&component_type={}",
            self.path,
            component_type.to_string()
        );
        self
    }

    pub fn bill_status(mut self, bill_status: BillStatus) -> TimesheetFragment<'a> {
        self.path = format!("{}&bill_status={}", self.path, bill_status.to_string());
        self
    }

    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Timelogs> {
        if !self.path.contains("component_type") || !self.path.contains("bill_status")
            || !self.path.contains("users_list") || !self.path.contains("view_type")
            || !self.path.contains("date")
        {
            bail!(
                "More information needed; please specify at least date, view type,
                component type, billable status and users scope before searching time logs."
            )
        }
        let timelog_list: ZohoTimelogs = self.client.get(&self.path)?;
        Ok(timelog_list.timelogs)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ViewType {
    Day,
    Week,
    Month,
}

impl ViewType {
    pub fn to_string(self) -> String {
        match self {
            ViewType::Day => "day".to_string(),
            ViewType::Month => "month".to_string(),
            ViewType::Week => "week".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BillStatus {
    All,
    Billable,
    NonBillable,
}

impl BillStatus {
    pub fn to_string(self) -> String {
        match self {
            BillStatus::All => "all".to_string(),
            BillStatus::Billable => "billable".to_string(),
            BillStatus::NonBillable => "non_billable".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ComponentType {
    Task,
    Bug,
    General,
}

impl ComponentType {
    pub fn to_string(self) -> String {
        match self {
            ComponentType::Task => "task".to_string(),
            ComponentType::Bug => "bug".to_string(),
            ComponentType::General => "general".to_string(),
        }
    }
}

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
