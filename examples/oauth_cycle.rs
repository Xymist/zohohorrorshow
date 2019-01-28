extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate zohohorrorshow;

use dotenv::dotenv;
use std::env;
use zohohorrorshow::{errors::*, oauth};

fn run() -> Result<i32> {
    dotenv().ok();

    let creds = oauth::Credentials::new(
        &env::var("ZOHO_CLIENT_ID")?,
        &env::var("ZOHO_CLIENT_SECRET")?,
        None,
        None,
    );

    let mut client = oauth::client(creds);

    client.request_access();

    println!("Credentials retrieved: {:?}", client.credentials());

    Ok(0)
}

quick_main!(run);
