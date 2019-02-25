use crate::errors::*;
use crate::request::{FilterOptions, ModelRequest, RequestDetails, RequestParameters};
use crate::serializers::from_str;
use std::collections::HashMap;

pub(crate) fn model_path(
    portal: impl std::fmt::Display,
    project: impl std::fmt::Display,
) -> String {
    format!("portal/{}/projects/{}/categories/", portal, project)
}

#[derive(Clone, Debug)]
pub struct CategoryRequest(RequestDetails);

impl CategoryRequest {
    pub fn new(access_token: &str, model_path: &str, id: Option<i64>) -> Self {
        CategoryRequest(RequestDetails::new(access_token, model_path, id))
    }

    pub fn iter_get(self) -> CategoryIterator {
        CategoryIterator::new(self)
    }
}

impl ModelRequest for CategoryRequest {
    fn uri(&self) -> String {
        self.0.uri()
    }

    fn params(&self) -> Option<HashMap<String, String>> {
        self.0.params()
    }

    fn access_token(&self) -> String {
        self.0.access_token()
    }

    fn filter(self, _param: impl FilterOptions) -> Self {
        self
    }
}

impl RequestParameters for CategoryRequest {
    type ModelCollection = ZohoCategories;
    type NewModel = NewCategory;

    fn put(&self, _data: Self::NewModel) -> Result<Option<Self::ModelCollection>> {
        bail!("PUT requests are not supported for Categories");
    }
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Response {
    response: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ZohoCategories {
    #[serde(rename = "categories")]
    pub categories: Vec<Category>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Category {
    #[serde(rename = "id", deserialize_with = "from_str")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct NewCategory {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct CategoryIterator {
    pub items: <Vec<Category> as IntoIterator>::IntoIter,
    pub last_full: bool,
    pub request: CategoryRequest,
}

impl CategoryIterator {
    pub fn new(request: CategoryRequest) -> Self {
        Self {
            items: Vec::new().into_iter(),
            last_full: true,
            request: request,
        }
    }

    pub fn try_next(&mut self) -> Result<Option<Category>> {
        // If there are still items in the local cache from the last request, use the next one of those.
        if let Some(category) = self.items.next() {
            return Ok(Some(category));
        }

        // If we didn't get a full 100 (the default number to retrieve) the last time, then we must have
        // run out in Zoho; don't request any more.
        if !self.last_full {
            return Ok(None);
        }

        let returned_categories = self.request.clone().get()?;

        if let Some(category_list) = returned_categories {
            self.last_full = category_list.categories.len() as i8 == 100;
            self.items = category_list.categories.into_iter();

            Ok(self.items.next())
        } else {
            Ok(None)
        }
    }
}

impl Iterator for CategoryIterator {
    type Item = Result<Category>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(val)) => Some(Ok(val)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
