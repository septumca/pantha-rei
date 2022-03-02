use serde::{Serialize, Deserialize, de::DeserializeOwned};
use mongodb::{bson::{doc, Uuid}, options::UpdateModifications};
use futures::stream::{StreamExt};

use super::dbutils;
use super::error::PrError;

pub async fn create<'de, D, T>(collection_name: &str, data: D) -> Result<T, PrError>
where D: Deserialize<'de>,
      T: From<D> + Serialize + Clone
{
  let coll = dbutils::get_collection::<T>(collection_name).await?;
  let entry = T::from(data);
  coll.insert_one(entry.clone(), None).await?;

  Ok(entry)
}

pub async fn read<T>(collection_name: &str, id: String) -> Result<T, PrError>
where T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync
{
  let coll = dbutils::get_collection::<T>(collection_name).await?;
  let id = Uuid::parse_str(id)?;
  coll
    .find_one(doc! { "_id": id }, None)
    .await?
    .ok_or_else(|| PrError::NotFound(String::from("entry not found")))
}

pub async fn read_all<T>(collection_name: &str) -> Result<Vec<T>, PrError>
where T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync
{
  let coll = dbutils::get_collection::<T>(collection_name).await?;
  let mut cur = coll.find(None, None).await?;
  let mut entries: Vec<T> = vec![];
  while let Some(e) = cur.next().await {
    entries.push(e.expect("should be an entries"));
  }

  Ok(entries)
}

pub async fn put<'de, D, T>(collection_name: &str, id: String, data: D) -> Result<(), PrError>
where D: Serialize + Deserialize<'de> + Into<UpdateModifications>,
      T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync
{
  let id = Uuid::parse_str(id)?;
  let coll = dbutils::get_collection::<T>(collection_name).await?;
  let _ = coll.update_one(doc! { "_id": id }, data, None).await?;

  Ok(())
}


pub async fn delete<T>(collection_name: &str, id: String) -> Result<(), PrError>
where T: Serialize + DeserializeOwned + Unpin + std::marker::Send + Sync
{
  let coll = dbutils::get_collection::<T>(collection_name).await?;
  let id = Uuid::parse_str(id)?;
  coll.delete_one(doc! { "_id": id }, None).await?;

  Ok(())
}
