use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn get_db() -> mongodb::error::Result<Client> {
    let client_uri = env::var("MONGO_URI").unwrap();
    let client_options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}