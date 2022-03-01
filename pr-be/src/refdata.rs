use axum::{Json};
use serde::{Serialize, Deserialize};
use mongodb::{bson::{doc, Uuid}};
use futures::stream::{StreamExt};

const COLL_NAME: &str = "ref_data";

use crate::{error::{self, PrError}, dbutils, utils};

pub type Dispositions = Vec<String>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefData {
  _id: Uuid,
  dispositions: Dispositions,
}

async fn read_ref_data() -> Result<RefData, PrError> {
  let coll = dbutils::get_collection::<RefData>(COLL_NAME).await?;
  let mut cur = coll.find(None, None).await?;
  match cur.next().await.transpose()? {
    Some(d) => Ok(d),
    None => Err(PrError::NotFound(String::from("entry not found")))
  }
}

pub async fn read()  -> Result<Json<RefData>, error::PrError> {
  Ok::<Json<RefData>, error::PrError>(Json(read_ref_data().await?))
}

pub async fn read_all() -> Result<Json<Vec<RefData>>, error::PrError> {
  Ok::<Json<Vec<RefData>>, error::PrError>(Json(utils::read_all::<RefData>(COLL_NAME).await?))
}
