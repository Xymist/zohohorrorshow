use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use std::collections::HashMap;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
    id: usize,
) -> String {
    format!(
        "portal/{}/projects/{}/tasklists/{}/tasks",
        portal, project, id
    )
}

#[derive(Clone, Debug)]
pub struct TasklistTaskRequest(RequestDetails);

impl TasklistTaskRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        TasklistTaskRequest(RequestDetails::new(access_token, model_path, id))
    }
}

impl ModelRequest for TasklistTaskRequest {
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

impl RequestParameters for TasklistTaskRequest {
    type ModelCollection = crate::models::task::ZohoTasks;
    type NewModel = crate::models::task::NewTask;
}
