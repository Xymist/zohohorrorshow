use client::ZohoClient;
use errors::*;

#[derive(Debug)]
pub struct ForumFragment<'a> {
    pub client: &'a ZohoClient,
    pub path: String,
}

impl<'a> ForumFragment<'a> {
    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Vec<Forum>> {
        let forum_list: ZohoForums = self.client.get(&self.path)?;
        Ok(forum_list.forums)
    }
    // Fetch a specific forum by id
    pub fn by_id(self, id: i64) -> ForumFilter<'a> {
        if self.path.contains('&') {
            panic!("Cannot both filter and find by ID")
        }
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        ForumFilter {
            client: self.client,
            path: format!("{}{}/?{}", path_frags[0], id, path_frags[1]),
            filter: Filter::ID(id),
        }
    }
    // Fetch a specific forum by id
    pub fn by_name(self, name: &'a str) -> ForumFilter<'a> {
        if self.path.contains('&') {
            panic!("Cannot both filter and find by Name")
        }
        let path_frags = self.path.split('?').collect::<Vec<&str>>();
        ForumFilter {
            client: self.client,
            path: format!("{}{}/?{}", path_frags[0], name, path_frags[1]),
            filter: Filter::Name(name),
        }
    }
}

#[derive(Debug)]
enum Filter<'a> {
    ID(i64),
    Name(&'a str),
}

#[derive(Debug)]
pub struct ForumFilter<'a> {
    client: &'a ZohoClient,
    path: String,
    filter: Filter<'a>,
}

impl<'a> ForumFilter<'a> {
    // Execute the query against the Zoho API
    pub fn fetch(self) -> Result<Option<Forum>> {
        let forum_list: ZohoForums = self.client.get(&self.path)?;
        let forums = forum_list.forums;
        match self.filter {
            Filter::ID(id) => filter_by_id(forums, id),
            Filter::Name(name) => filter_by_name(forums, name),
        }
    }
}

fn filter_by_id(forums: Vec<Forum>, id: i64) -> Result<Option<Forum>> {
    let mut filtered = forums
        .into_iter()
        .filter(|f| f.id == id)
        .collect::<Vec<Forum>>();
    match filtered.len() {
        0 => Ok(None),
        _ => Ok(Some(filtered.remove(0))),
    }
}

fn filter_by_name(forums: Vec<Forum>, name: &str) -> Result<Option<Forum>> {
    let mut filtered = forums
        .into_iter()
        .filter(|f| f.name == name)
        .collect::<Vec<Forum>>();
    match filtered.len() {
        0 => Ok(None),
        _ => Ok(Some(filtered.remove(0))),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoForums {
    #[serde(rename = "forums")]
    pub forums: Vec<Forum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "is_sticky_post")]
    pub is_sticky_post: bool,
    #[serde(rename = "is_announcement_post")]
    pub is_announcement_post: bool,
    #[serde(rename = "posted_by")]
    pub posted_by: String,
    #[serde(rename = "posted_person")]
    pub posted_person: String,
    #[serde(rename = "post_date")]
    pub post_date: String,
    #[serde(rename = "post_date_long")]
    pub post_date_long: i64,
    #[serde(rename = "link")]
    pub link: Link,
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
