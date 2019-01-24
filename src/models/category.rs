use crate::client::ZohoClient;
use crate::errors::*;
use crate::serializers::from_str;

pub const ModelPath: &str = "portal/{}/projects/{}/categories/";

pub fn categories(cl: &ZohoClient) -> CategoryFragment {
    let client = cl.clone();
    CategoryFragment {
        path: client.make_uri(&format!(
            "portal/{}/projects/{}/categories/",
            client.portal_id(),
            client.project_id()
        )),
        client,
    }
}

#[derive(Debug)]
pub struct CategoryFragment {
    pub client: ZohoClient,
    pub path: String,
}

impl CategoryFragment {
    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Category>> {
        let category_list: ZohoCategories = self.client.get(&self.path)?;
        Ok(category_list.categories)
    }
    // Delete a category by ID
    pub fn delete(self, id: i64) -> Result<String> {
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        let response: Response = self
            .client
            .delete(&format!("{}{}/?{}", path_frags[0], id, path_frags[1]))?;
        Ok(response.response)
    }
    // Create a category by name
    pub fn create(self, name: &str) -> Result<Category> {
        let mut response: ZohoCategories = self
            .client
            .post(&format!("{}&name={}", self.path, name), "")?;
        Ok(response.categories.remove(0))
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Response {
    response: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZohoCategories {
    #[serde(rename = "categories")]
    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Category {
    #[serde(rename = "id", deserialize_with = "from_str")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}
