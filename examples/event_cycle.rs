extern crate dotenv;
extern crate zohohorrorshow;

use dotenv::dotenv;
use std::env;
use zohohorrorshow::{errors::*, prelude::*};

fn run() -> Result<i32> {
    dotenv().ok();

    // Generate the client, with a valid auth token.
    let client = ZohoClient::new(
        &env::var("ZOHO_CLIENT_ID")?,
        &env::var("ZOHO_CLIENT_SECRET")?,
    )
    .set_portal(&env::var("ZOHO_PORTAL_NAME")?)?
    .set_project(&env::var("ZOHO_PROJECT_NAME")?)?;

    let mut event = zoho_event::NewEvent {
        title: "TestEvent".to_string(),
        date: "01-01-2020".to_string(),
        hour: "02".to_string(),
        minutes: "00".to_string(),
        ampm: zoho_event::AmPm::Am,
        duration_hour: "01".to_string(),
        duration_mins: "05".to_string(),
        participants: Some(vec![3_928_929]),
        remind_before: None,
        repeat: None,
        nooftimes_repeat: None,
        location: None,
    };

    let new_event = &client.events().post(event.clone())?.unwrap().events[0];
    let ne_id = new_event.id;

    event.title = "TestEvent - Updated".to_string();
    client.event(ne_id).put(event)?;

    let updated_events = client.events().get()?;
    println!("Updated events: {:?}", updated_events);

    let destroyed_event = client.event(ne_id).delete()?;
    println!("Delete response: {:?}", destroyed_event);

    Ok(0)
}

fn main() {
    ::std::process::exit(match run() {
        Ok(_) => {
            println!("Goodbye");
            0
        }
        Err(err) => {
            eprintln!("Error occurred while running: {:?}", err);
            1
        }
    });
}
