use mongodb::{Client, options::ClientOptions, Collection};
use serde::Serialize;

use super::error::PrError;

pub async fn get_collection<T>(collection_name: &str) -> Result<Collection<T>, PrError>
where T: Serialize
{
  let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
  let client = Client::with_options(client_options)?;
  Ok(client.database("pantharei").collection::<T>(collection_name))
}
