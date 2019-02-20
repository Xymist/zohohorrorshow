extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate zohohorrorshow;

use dotenv::dotenv;
use std::env;
use zohohorrorshow::{
    client::ZohoClient,
    errors::*,
    models::bug::{Bug, Filter, SortColumn, SortOrder},
    request::{ModelRequest, RequestParameters},
};

#[derive(Debug, Clone)]
pub struct BugIterator {
    pub items: <Vec<Bug> as IntoIterator>::IntoIter,
    pub last_full: bool,
    pub client: ZohoClient,
    pub start_index: i64,
}

impl BugIterator {
    pub fn new(client: &ZohoClient) -> BugIterator {
        BugIterator {
            items: Vec::new().into_iter(),
            last_full: true,
            client: client.clone(),
            start_index: 0,
        }
    }

    pub fn try_next(&mut self) -> Result<Option<Bug>> {
        if let Some(bug) = self.items.next() {
            return Ok(Some(bug));
        }

        if !self.last_full {
            return Ok(None);
        }

        let req = self.client.bugs();
        let returned_tickets = req
            .filter(Filter::SortColumn(SortColumn::LastModifiedTime))
            .filter(Filter::SortOrder(SortOrder::Descending))
            .filter(Filter::Index(self.start_index))
            .get()?;

        if let Some(ticket_list) = returned_tickets {
            self.last_full = match ticket_list.bugs.len() {
                100 => true,
                _ => false,
            };

            self.start_index += ticket_list.bugs.len() as i64;

            self.items = ticket_list.bugs.into_iter();

            Ok(self.items.next())
        } else {
            Ok(None)
        }
    }
}

impl Iterator for BugIterator {
    type Item = Result<Bug>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(val)) => Some(Ok(val)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

fn run() -> Result<i32> {
    dotenv().ok();

    let client = ZohoClient::new(
        &env::var("ZOHO_CLIENT_ID")?,
        &env::var("ZOHO_CLIENT_SECRET")?,
    )
    .set_portal(&env::var("ZOHO_PORTAL_NAME")?)?
    .set_project(&env::var("ZOHO_PROJECT_NAME")?)
    .chain_err(|| "Could not initialize; exiting")?;

    let mut all_bugs = BugIterator::new(&client);
    println!("{:?}", all_bugs.next().unwrap().unwrap());
    Ok(0)
}

quick_main!(run);
