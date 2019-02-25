extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate zohohorrorshow;

use dotenv::dotenv;
use std::{collections::HashMap, env};
use zohohorrorshow::{
    client::ZohoClient,
    errors::*,
    models::bug::{Filter, SortColumn, SortOrder},
    request::ModelRequest,
};

fn run() -> Result<i32> {
    dotenv().ok();

    let mut client = ZohoClient::new(
        &env::var("ZOHO_CLIENT_ID")?,
        &env::var("ZOHO_CLIENT_SECRET")?,
    )
    .set_portal(&env::var("ZOHO_PORTAL_NAME")?)?
    .set_project(&env::var("ZOHO_PROJECT_NAME")?)
    .chain_err(|| "Could not initialize; exiting")?;

    let team_members = [
        "satyendra.s1".to_owned(),
        "James Duerden".to_owned(),
        "neelam.j".to_owned(),
        "Martin Nicholas".to_owned(),
        "ankit.ya".to_owned(),
        "ankit.k".to_owned(),
        "Deepesh Kakani".to_owned(),
        "vijay".to_owned(),
        "nitin.k".to_owned(),
        "surbhi.a".to_owned(),
        "Gaurav.j".to_owned(),
        "jeetendra.s".to_owned(),
        "Chris Barrett".to_owned(),
        "Shekhar.S".to_owned(),
    ];

    let open_statuses = [
        "Open".to_owned(),
        "Started".to_owned(),
        "Resolved".to_owned(),
        "To Be Tested".to_owned(),
    ];

    let all_bugs = client
        .bugs()
        .filter(Filter::SortColumn(SortColumn::LastModifiedTime))
        .filter(Filter::SortOrder(SortOrder::Descending))
        .filter(Filter::Range(100))
        .iter_get();

    let mut assigned_tickets: HashMap<String, Vec<String>> = HashMap::new();

    for bg in all_bugs
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .filter(|bg| !team_members.contains(&bg.assignee_name))
        .filter(|bg| open_statuses.contains(&bg.status.classification_type))
    {
        assigned_tickets
            .entry(bg.assignee_name)
            .or_insert_with(Vec::new)
            .push(bg.key)
    }

    for (user, tkts) in assigned_tickets {
        println!("{} has {} tickets: {:?}", user, tkts.len(), tkts);
    }

    Ok(0)
}

quick_main!(run);
