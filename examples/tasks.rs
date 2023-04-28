use dotenv::dotenv;
use std::env;
use zohohorrorshow::errors::*;
use zohohorrorshow::prelude::*;

fn run() -> Result<i32> {
    dotenv().ok();

    // Generate the client, with a valid auth token.
    let client = ZohoClient::new(
        &env::var("ZOHO_CLIENT_ID")?,
        &env::var("ZOHO_CLIENT_SECRET")?,
    )
    .set_portal(&env::var("ZOHO_PORTAL_NAME")?)?
    .set_project(&env::var("ZOHO_PROJECT_NAME")?)?;

    let tasks = client
        .tasks()
        .with_subtasks()
        .iter_get()
        .filter(std::result::Result::is_ok)
        .map(std::result::Result::unwrap)
        .count();
    println!("Existing tasks and subtasks: {:#?}", tasks);

    Ok(0)
}

fn main() {
    tracing_subscriber::fmt::init();

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
