extern crate dotenv;
extern crate zohohorrorshow;

use dotenv::dotenv;
use std::env;
use zohohorrorshow::errors::*;
use zohohorrorshow::prelude::*;

fn run() -> Result<i32> {
    dotenv().ok();

    // Generate the client, with a valid auth token.
    let mut client = ZohoClient::new(
        &env::var("ZOHO_CLIENT_ID")?,
        &env::var("ZOHO_CLIENT_SECRET")?,
    )
    .set_portal(&env::var("ZOHO_PORTAL_NAME")?)?
    .set_project(&env::var("ZOHO_PROJECT_NAME")?)
    .chain_err(|| "Could not initialize; exiting")?;

    let pjts = client.projects().get()?;
    println!("Existing projects: {:?}", pjts);

    // Commented until reimplemented
    // let custom_fields = client.projects().customfields().get()?;
    // println!("Existing custom fields: {:?}", custom_fields);

    Ok(0)
}

quick_main!(run);
