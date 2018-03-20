extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate zohohorrorshow;

use zohohorrorshow::errors::*;
use zohohorrorshow::client::ZohoClient;
use dotenv::dotenv;
use std::env;

fn run() -> Result<i32> {
    dotenv().ok();

    // Generate the client, with a valid auth token.
    let client = ZohoClient::new(
        &env::var("ZOHO_AUTHTOKEN")?,
        Some(&env::var("ZOHO_PORTAL_NAME")?),
        Some(&env::var("ZOHO_PROJECT_NAME")?),
    ).chain_err(|| "Could not initialize; exiting")?;

    let categories = client.categories().fetch()?;
    println!("Pre-existing categories: {:?}", categories);

    let new_category = client.categories().create("Test Category")?;
    println!("New category: {:?}", new_category);

    let nc_id = new_category.id;
    let destroyed_category = client.categories().delete(nc_id)?;
    println!("Delete response: {:?}", destroyed_category);

    Ok(0)
}

quick_main!(run);
