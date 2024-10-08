use std::{env, error::Error};

use mongodb::{options::ClientOptions, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options = ClientOptions::parse(&client_uri).await?;
    let client = Client::with_options(options)?;

    println!("Databases:");
    for name in client.list_database_names().await? {
        println!("- {name}");
    }

    Ok(())
}
