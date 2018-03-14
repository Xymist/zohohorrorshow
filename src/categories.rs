use errors::*;
use client::ZohoClient;
use utils::from_str;

#[derive(Debug)]
pub struct CategoryFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> CategoryFragment<'a> {
    // Execute the query against the Zoho API
    pub fn call(self) -> Result<Vec<Category>> {
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
    pub fn create(self, name: &str) -> Result<ZohoCategories> {
        let response: ZohoCategories = self.client
            .post(&format!("{}&name={}", self.path, name), "")?;
        Ok(response)
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
