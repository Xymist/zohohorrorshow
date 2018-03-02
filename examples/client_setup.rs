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
    let mut client = zohohorrorshow::client::create_client(&env::var("ZOHO_AUTHTOKEN").unwrap())
        .chain_err(|| "Could not initialize; exiting")?;

    // Find a desired portal by its friendly name, so we don't need to go hunting for an ID
    let portal = client
        .portals()
        .by_name(&env::var("ZOHO_PORTAL_NAME").unwrap())
        .call()
        .unwrap();

    // Set the client's portal_id from the portal we just found
    client.set_portal(portal.id)?;

    // Repeat for finding the project we want to work with
    let project = client
        .projects()
        .by_name(&env::var("ZOHO_PROJECT_NAME").unwrap())
        .call()
        .unwrap();

    // Assign the client's project_id
    client.set_project(project.id)?;

    // Display the created client
    println!("{:?}", client);

    Ok(0)
}

quick_main!(run);
