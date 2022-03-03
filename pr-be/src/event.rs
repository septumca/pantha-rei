use std::{fmt};
use axum::{Json, extract::Path};
use serde::{Serialize, Deserialize};
use mongodb::{bson::{doc, Uuid}, options::UpdateModifications};

const COLL_NAME: &str = "events";

use crate::{error::{self}, utils, user::{User}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
  _id: Uuid,
  name: String,
  participants: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EventSummary {
  _id: Uuid,
  name: String,
  participants: Vec<User>,
}

#[derive(Deserialize)]
pub struct CreateEventData {
  name: String,
}

impl fmt::Display for Event {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} - {} with {} participants", self._id, self.name, self.participants.len())
  }
}

impl Event {
    pub fn new(name: String) -> Self {
      Event {
        _id: Uuid::new(),
        name,
        participants: vec![]
      }
    }
}

impl From<CreateEventData> for Event {
  fn from(ed: CreateEventData) -> Self {
      Event::new(ed.name)
  }
}

pub type AddUserToEventData = User;

impl From<AddUserToEventData> for UpdateModifications {
  fn from(d: AddUserToEventData) -> Self {
    UpdateModifications::Document(doc! { "$push": { "participants": { "_id": d._id, "name": d.name, "dispositions": d.dispositions }}})
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveUserFromEventData {
  _id: Uuid
}

impl From<RemoveUserFromEventData> for UpdateModifications {
  fn from(d: RemoveUserFromEventData) -> Self {
    UpdateModifications::Document(doc! { "$pull": { "participants": { "_id": d._id }}})
  }
}

pub async fn create(Json(data): Json<CreateEventData>) -> Result<Json<Event>, error::PrError> {
  Ok::<Json<Event>, error::PrError>(Json(utils::create::<CreateEventData, Event>(COLL_NAME, data).await?))
}

pub async fn delete(Path(id): Path<String>) -> Result<(), error::PrError> {
  Ok::<(), error::PrError>(utils::delete::<Event>(COLL_NAME, id).await?)
}

pub async fn read(Path(id): Path<String>) -> Result<Json<Event>, error::PrError> {
  Ok::<Json<Event>, error::PrError>(Json(utils::read::<Event>(COLL_NAME, id).await?))
}

pub async fn add_user(Path(id): Path<String>, Json(data): Json<AddUserToEventData>) -> Result<(), error::PrError> {
  Ok::<(), error::PrError>(utils::put::<AddUserToEventData, Event>(COLL_NAME, id, data).await?)
}

pub async fn remove_user(Path((id, uid)): Path<(String, String)>) -> Result<(), error::PrError> {
  let uid = Uuid::parse_str(uid)?;
  let data = RemoveUserFromEventData { _id: uid };
  Ok::<(), error::PrError>(utils::put::<RemoveUserFromEventData, Event>(COLL_NAME, id, data).await?)
}

pub async fn read_all() -> Result<Json<Vec<Event>>, error::PrError> {
  Ok::<Json<Vec<Event>>, error::PrError>(Json(utils::read_all::<Event>(COLL_NAME).await?))
}
