use client::ZohoClient;
use errors::*;
use std::rc::Rc;
use utils::from_str;

pub fn categories(client: Rc<ZohoClient>) -> CategoryFragment {
    CategoryFragment {
        client: Rc::clone(&client),
        path: client.make_uri(&format!(
            "portal/{}/projects/{}/categories/",
            client.portal_id(),
            client.project_id()
        )),
    }
}

#[derive(Debug)]
pub struct CategoryFragment {
    pub client: Rc<ZohoClient>,
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
        let response: Response = self.client
            .delete(&format!("{}{}/?{}", path_frags[0], id, path_frags[1]))?;
        Ok(response.response)
    }
    // Create a category by name
    pub fn create(self, name: &str) -> Result<Category> {
        let mut response: ZohoCategories = self.client
            .post(&format!("{}&name={}", self.path, name), "")?;
        Ok(response.categories.remove(0))
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    response: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoCategories {
    #[serde(rename = "categories")]
    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "id", deserialize_with = "from_str")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
}
