use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use std::collections::HashMap;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
) -> String {
    format!("portal/{}/projects/{}/activities/", portal, project)
}

#[derive(Clone, Debug)]
pub struct ActivityRequest(RequestDetails);

impl ActivityRequest {
    pub fn new(access_token: &str, model_path: &str) -> Self {
        ActivityRequest(RequestDetails::new(access_token, model_path, None))
    }

    pub fn iter_get(self) -> ActivityIterator {
        ActivityIterator::new(self)
    }
}

impl ModelRequest for ActivityRequest {
    fn uri(&self) -> String {
        self.0.uri()
    }

    fn params(&self) -> Option<HashMap<String, String>> {
        self.0.params()
    }

    fn access_token(&self) -> String {
        self.0.access_token()
    }

    fn filter(mut self, param: impl FilterOptions) -> Self {
        self.0 = self.0.filter(&param);
        self
    }
}

impl RequestParameters for ActivityRequest {
    type ModelCollection = ZohoActivities;
    type NewModel = NewActivity;

    fn post(&self, _data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        bail!("POST requests are not supported for Activities");
    }

    fn put(&self, _data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        bail!("PUT requests are not supported for Activities");
    }

    fn delete(&self) -> Result<Option<Self::ModelCollection>> {
        bail!("DELETE requests are not supported for Activities");
    }
}

/// Filters available for Activities, to restrict the records returned
pub enum Filter {
    /// Index of first record to return. Defaults to 0.
    Index(usize),

    /// Number of records to return, if possible. Maximum 100, defaults to 100.
    /// If fewer than this number of records are available (after adjusting for any
    /// Index filter) then all records will be returned.
    Range(i8),
}

impl FilterOptions for Filter {
    fn key(&self) -> String {
        match self {
            Filter::Index(_) => "index".to_owned(),
            Filter::Range(_) => "range".to_owned(),
        }
    }

    fn value(&self) -> String {
        match self {
            Filter::Index(index) => index.to_string(),
            Filter::Range(range) => range.to_string(),
        }
    }
}

/// Wrapper struct representing the object returned by the Zoho API containing a list of
/// Activities.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZohoActivities {
    /// All Activity records corresponding to the applied filters
    #[serde(rename = "activities")]
    pub activities: Vec<Activity>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

/// Unconstructable enum representing a theoretical new Activity.
/// Zoho Projects does not permit creating new Activities through the API.
#[derive(Clone, Serialize, Deserialize)]
pub enum NewActivity {}

#[derive(Debug, Clone)]
pub struct ActivityIterator {
    pub items: <Vec<Activity> as IntoIterator>::IntoIter,
    pub last_full: bool,
    pub request: ActivityRequest,
    pub start_index: usize,
}

impl ActivityIterator {
    pub fn new(request: ActivityRequest) -> Self {
        Self {
            items: Vec::new().into_iter(),
            last_full: true,
            request: request,
            start_index: 0,
        }
    }

    fn range(&self) -> i8 {
        match self.request.params() {
            Some(params) => match params.get("range") {
                Some(range_string) => range_string.parse::<i8>().unwrap_or(100),
                None => 100,
            },
            None => 100,
        }
    }

    pub fn try_next(&mut self) -> Result<Option<Activity>> {
        // If there are still items in the local cache from the last request, use the next one of those.
        if let Some(activity) = self.items.next() {
            return Ok(Some(activity));
        }

        // If we didn't get a full 100 (the default number to retrieve) the last time, then we must have
        // run out in Zoho; don't request any more.
        if !self.last_full {
            return Ok(None);
        }

        let returned_activities = self
            .request
            .clone()
            .filter(Filter::Index(self.start_index))
            .get()?;

        if let Some(activity_list) = returned_activities {
            self.last_full = activity_list.activities.len() as i8 == self.range();

            self.start_index += activity_list.activities.len();

            self.items = activity_list.activities.into_iter();

            Ok(self.items.next())
        } else {
            Ok(None)
        }
    }
}

impl Iterator for ActivityIterator {
    type Item = Result<Activity>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(val)) => Some(Ok(val)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
