extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate zohohorrorshow;

use dotenv::dotenv;
use std::env;
use zohohorrorshow::errors::*;
use zohohorrorshow::{client::ZohoClient, models::projects};

fn run() -> Result<i32> {
    dotenv().ok();

    // Generate the client, with a valid auth token.
    let client = ZohoClient::new(
        &env::var("ZOHO_AUTHTOKEN")?,
        Some(&env::var("ZOHO_PORTAL_NAME")?),
        Some(&env::var("ZOHO_PROJECT_NAME")?),
    ).chain_err(|| "Could not initialize; exiting")?;

    let pjts = projects(&client).fetch()?;
    println!("Existing projects: {:?}", pjts);

    let custom_fields = projects(&client).customfields()?;
    println!("Existing custom fields: {:?}", custom_fields);

    Ok(0)
}

quick_main!(run);
