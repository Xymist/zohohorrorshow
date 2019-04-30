extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate zohohorrorshow;

use dotenv::dotenv;
use zohohorrorshow::errors::*;

fn run() -> Result<i32> {
    dotenv().ok();

    // // Generate the client, with a valid auth token.
    // let client = ZohoClient::new(&env::var("ZOHO_CLIENT_ID")?, &env::var("ZOHO_CLIENT_SECRET")?)
    //     .set_portal(&env::var("ZOHO_PORTAL_NAME")?)?
    //     .set_project(&env::var("ZOHO_PROJECT_NAME")?)
    //     .chain_err(|| "Could not initialize; exiting")?;

    // let new_category = categories(&client).create("Test Category")?;
    // println!("New category: {:?}", new_category);

    // let ctgs = categories(&client).fetch()?;
    // println!("Existing categories: {:?}", ctgs);

    // let nc_id = new_category.id;

    // let destroyed_category = categories(&client).delete(nc_id)?;
    // println!("Delete response: {:?}", destroyed_category);

    Ok(0)
}

quick_main!(run);
