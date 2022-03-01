use std::{fmt};
use axum::{Json, extract::Path};
use serde::{Serialize, Deserialize};
use mongodb::{bson::{doc, Uuid}};

const COLL_NAME: &str = "events";

use crate::{error, utils};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
  _id: Uuid,
  name: String,
  participants: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EventSummary {
  _id: Uuid,
  name: String,
  participants: Vec<Uuid>,
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

pub async fn create(Json(data): Json<CreateEventData>) -> Result<Json<Event>, error::PrError> {
  Ok::<Json<Event>, error::PrError>(Json(utils::create::<CreateEventData, Event>(COLL_NAME, data).await?))
}

pub async fn delete(Path(id): Path<String>) -> Result<(), error::PrError> {
  Ok::<(), error::PrError>(utils::delete::<Event>(COLL_NAME, id).await?)
}

pub async fn read(Path(id): Path<String>) -> Result<Json<Event>, error::PrError> {
  Ok::<Json<Event>, error::PrError>(Json(utils::read::<Event>(COLL_NAME, id).await?))
}

pub async fn read_all() -> Result<Json<Vec<Event>>, error::PrError> {
  Ok::<Json<Vec<Event>>, error::PrError>(Json(utils::read_all::<Event>(COLL_NAME).await?))
}
