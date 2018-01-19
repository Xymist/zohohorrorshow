use errors::*;
use reqwest;
use serde;
use RelativePath;

#[derive(Debug)]
pub struct ZohoClient {
    authtoken: String,
    client: reqwest::Client,
}

impl ZohoClient {
    fn make_uri(&self, relative_path: &str, query: Option<&str>) -> Result<String> {
        match query {
            Some(query_string) => Ok(format!(
                "https://projectsapi.zoho.com/restapi/{}{}{}",
                relative_path, self.authtoken, query_string
            )),
            None => Ok(format!(
                "https://projectsapi.zoho.com/restapi/{}{}",
                relative_path, self.authtoken
            )),
        }
    }

    fn get_url<T>(&self, url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut response = self.client.get(url).send()?;
        if !response.status().is_success() {
            bail!("Server error: {:?}", response.status());
        };
        let res_obj = response.json::<T>()?;
        Ok(res_obj)
    }

    // Under some circumstances you may want to pass a query string to an
    // endpoint. This should take the format "&attribute=value", repeated
    // if necessary.
    pub fn get_with_query<T, U>(&self, params: U, query: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned + RelativePath<U>,
    {
        let url: String = self.make_uri(&T::relative_path(params)?, Some(query))?;
        self.get_url(&url)
    }

    pub fn get<T, U>(&self, params: U) -> Result<T>
    where
        T: serde::de::DeserializeOwned + RelativePath<U>,
    {
        let url: String = self.make_uri(&T::relative_path(params)?, None)?;
        self.get_url(&url)
    }
}

pub fn create_client(auth_token: &str) -> Result<ZohoClient> {
    Ok(ZohoClient {
        authtoken: format!("?authtoken={}", auth_token),
        client: reqwest::Client::new(),
    })
}
