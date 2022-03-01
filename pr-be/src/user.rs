use axum::{Json, extract::Path};
use serde::{Serialize, Deserialize};
use mongodb::{bson::{doc, Uuid}};

use crate::{error, utils, refdata::Dispositions};

const COLL_NAME: &str = "users";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  _id: Uuid,
  name: String,
  dispositions: Dispositions
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserData {
  name: String,
  dispositions: Dispositions
}

impl From<CreateUserData> for User {
  fn from(ud: CreateUserData) -> Self {
    User {
      _id: Uuid::new(),
      name: ud.name,
      dispositions: ud.dispositions
    }
  }
}

pub async fn create(Json(data): Json<CreateUserData>) -> Result<Json<User>, error::PrError> {
  Ok::<Json<User>, error::PrError>(Json(utils::create::<CreateUserData, User>(COLL_NAME, data).await?))
}

pub async fn delete(Path(id): Path<String>) -> Result<(), error::PrError> {
  Ok::<(), error::PrError>(utils::delete::<User>(COLL_NAME, id).await?)
}

pub async fn read(Path(id): Path<String>) -> Result<Json<User>, error::PrError> {
  Ok::<Json<User>, error::PrError>(Json(utils::read::<User>(COLL_NAME, id).await?))
}

pub async fn read_all() -> Result<Json<Vec<User>>, error::PrError> {
  Ok::<Json<Vec<User>>, error::PrError>(Json(utils::read_all::<User>(COLL_NAME).await?))
}
