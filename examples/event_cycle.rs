extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate zohohorrorshow;

use dotenv::dotenv;
use std::env;
use zohohorrorshow::client::ZohoClient;
use zohohorrorshow::errors::*;
use zohohorrorshow::events::{AmPm, NewEvent};

fn run() -> Result<i32> {
    dotenv().ok();

    // Generate the client, with a valid auth token.
    let client = ZohoClient::new(
        &env::var("ZOHO_AUTHTOKEN")?,
        Some(&env::var("ZOHO_PORTAL_NAME")?),
        Some(&env::var("ZOHO_PROJECT_NAME")?),
    ).chain_err(|| "Could not initialize; exiting")?;

    let users = client.project_users().fetch()?;

    let mut event = NewEvent {
        title: "TestEvent".to_string(),
        date: "01-01-2020".to_string(),
        hour: "02".to_string(),
        minutes: "00".to_string(),
        ampm: AmPm::Am,
        duration_hour: "01".to_string(),
        duration_mins: "05".to_string(),
        participants: vec![users[0].id],
        remind_before: None,
        repeat: None,
        nooftimes_repeat: None,
        location: None,
    };

    let new_event = client.events().create(event.clone())?;
    println!("New event: {:?}", new_event);

    let events = client.events().fetch()?;
    println!("Existing events: {:?}", events);

    let ne_id = new_event.id;

    event.title = "TestEvent - Updated".to_string();
    client.events().update(ne_id, event)?;

    let updated_events = client.events().fetch()?;
    println!("Updated events: {:?}", updated_events);

    let destroyed_event = client.events().delete(ne_id)?;
    println!("Delete response: {:?}", destroyed_event);

    Ok(0)
}

quick_main!(run);
