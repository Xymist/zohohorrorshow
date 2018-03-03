extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate zohohorrorshow;

use zohohorrorshow::errors::*;
use dotenv::dotenv;
use std::env;

fn run() -> Result<i32> {
    dotenv().ok();

    // Generate the client, with a valid auth token.
    let mut client = zohohorrorshow::client::create_client(&env::var("ZOHO_AUTHTOKEN")?)
        .chain_err(|| "Could not initialize; exiting")?;

    // Find a desired portal by its friendly name, so we don't need to go hunting for an ID
    let portal = client
        .portals()
        .by_name(&env::var("ZOHO_PORTAL_NAME")?)
        .call()?;

    // Set the client's portal_id from the portal we just found
    match portal {
        Some(p) => client.set_portal(p.id)?,
        None => bail!("No portal found with that name"),
    };

    // Repeat for finding the project we want to work with
    let project = client
        .projects()
        .by_name(&env::var("ZOHO_PROJECT_NAME")?)
        .call()?;

    // Assign the client's project_id
    match project {
        Some(p) => client.set_project(p.id)?,
        None => bail!("No project found with that name"),
    };

    // Display the created client
    println!("{:?}", client);

    Ok(0)
}

quick_main!(run);
