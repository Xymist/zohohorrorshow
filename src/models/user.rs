use client::ZohoClient;
use errors::*;
use std::rc::Rc;
use utils::from_str;

pub fn portal_users(cl: &Rc<ZohoClient>) -> UserFragment {
    let client = Rc::clone(cl);
    UserFragment {
        path: client.make_uri(&format!("portal/{}/users/", client.portal_id())),
        client,
    }
}

pub fn project_users(cl: &Rc<ZohoClient>) -> UserFragment {
    let client = Rc::clone(cl);
    UserFragment {
        path: client.make_uri(&format!(
            "portal/{}/projects/{}/users/",
            client.portal_id(),
            client.project_id()
        )),
        client,
    }
}

// A fragment of the path to call for the Zoho Users API. This carries
// with it a reference to the client which will be used to call it.
#[derive(Debug)]
pub struct UserFragment {
    pub client: Rc<ZohoClient>,
    pub path: String,
}

impl UserFragment {
    query_strings!(UserFragment; usertype);

    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<User>> {
        let user_list: ZohoUsers = self.client.get(&self.path)?;
        Ok(user_list.users)
    }

    // Create a new user with specified details.
    // email: the user's email address
    // role: the user's role [manager, employee, contractor]
    // rate: the cost per hour for the employee if billing clients
    pub fn create(
        self,
        email: Option<&str>,
        role: Option<&str>,
        rate: Option<f64>,
    ) -> Result<User> {
        let mut details = self.path;
        if let Some(e) = email {
            details = details + &format!("&email={}", e)
        };
        if let Some(r) = role {
            details = details + &format!("&role={}", r)
        };
        if let Some(rt) = rate {
            details = details + &format!("&rate={}", rt)
        };

        let mut response: ZohoUsers = self.client.post(&details, "")?;
        Ok(response.users.remove(0))
    }

    // Update a user's role
    pub fn update(self, role: &str) -> Result<User> {
        if !self.path.contains("project") {
            return project_users(&self.client).update(role);
        };
        let mut response: ZohoUsers = self.client
            .post(&format!("{}&role={}", self.path, role), "")?;
        Ok(response.users.remove(0))
    }

    // Delete a user by ID
    pub fn delete(self, id: i64) -> Result<String> {
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        let response: Response = self.client
            .delete(&format!("{}{}/?{}", path_frags[0], id, path_frags[1]))?;
        Ok(response.response)
    }

    pub fn available(self) -> Result<String> {
        if self.path.contains("project") {
            return portal_users(&self.client).available();
        };
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        let avail_response: AvailCount = self.client
            .get(&format!("{}availcount/?{}", path_frags[0], path_frags[1]))?;
        Ok(avail_response.available_user_count)
    }

    pub fn activate(self, id: &str) -> Result<String> {
        if self.path.contains("project") {
            return portal_users(&self.client).activate(id);
        };
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        let response: Response = self.client.post(
            &format!(
                "{}activate/?{}&activate={}",
                path_frags[0], path_frags[1], id
            ),
            "",
        )?;
        Ok(response.response)
    }

    pub fn deactivate(self, id: &str) -> Result<String> {
        if self.path.contains("project") {
            return portal_users(&self.client).deactivate(id);
        };
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        let response: Response = self.client.post(
            &format!(
                "{}activate/?{}&deactivate={}",
                path_frags[0], path_frags[1], id
            ),
            "",
        )?;
        Ok(response.response)
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    response: String,
}

#[derive(Debug, Deserialize)]
pub struct AvailCount {
    available_user_count: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoUsers {
    #[serde(rename = "users")]
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id", deserialize_with = "from_str")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "role")]
    pub role: String,
}
