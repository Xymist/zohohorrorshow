extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate zohohorrorshow;

use dotenv::dotenv;
use std::env;
use zohohorrorshow::errors::*;
use zohohorrorshow::{client::ZohoClient, request::RequestParameters};

fn run() -> Result<i32> {
    dotenv().ok();

    // Generate the client, with a valid auth token.
    let client = ZohoClient::new(&env::var("ZOHO_AUTHTOKEN")?)
        .set_portal(&env::var("ZOHO_PORTAL_NAME")?)?
        .set_project(&env::var("ZOHO_PROJECT_NAME")?)
        .chain_err(|| "Could not initialize; exiting")?;

    let pjts = client.projects(None).get()?;
    println!("Existing projects: {:?}", pjts);

    // Commented until reimplemented
    // let custom_fields = client.projects().customfields().get()?;
    // println!("Existing custom fields: {:?}", custom_fields);

    Ok(0)
}

quick_main!(run);
