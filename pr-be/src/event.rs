use std::{fmt};
use axum::{Json, extract::Path};
use serde::{Serialize, Deserialize};
use mongodb::{bson::{doc, Uuid}, options::UpdateModifications};

const COLL_NAME: &str = "events";

use crate::{error::{self}, utils, user::{User}, dbutils};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullFillment {
  _id: Uuid,
  name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Requirement {
  name: String,
  fullfilled_by: Option<FullFillment>
}
pub type Requirements = Vec<Requirement>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
  _id: Uuid,
  name: String,
  participants: Vec<User>,
  description: String,
  requirements: Requirements
}

#[derive(Deserialize)]
pub struct CreateEventData {
  name: String,
  description: Option<String>,
  requirements: Option<Requirements>,
}

impl fmt::Display for Event {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} - {} with {} participants", self._id, self.name, self.participants.len())
  }
}

impl From<CreateEventData> for Event {
  fn from(ed: CreateEventData) -> Self {
    let description = ed.description.or_else(|| Some(String::from(""))).unwrap();
    let requirements = ed.requirements.or_else(|| Some(Vec::<Requirement>::new())).unwrap();
    Event {
      _id: Uuid::new(),
      name: ed.name,
      description,
      participants: vec![],
      requirements,
    }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullfillRequirement {
  requirement_name: String,
  user_id: Uuid,
  user_name: String
}

impl From<FullfillRequirement> for UpdateModifications {
  fn from(d: FullfillRequirement) -> Self {
    UpdateModifications::Document(doc! { "$set": { "requirements.$.fullfilled_by": { "_id": d.user_id, "name": d.user_name }}})
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnfullfillRequirement {
  requirement_name: String,
  user_id: String,
}

impl From<UnfullfillRequirement> for UpdateModifications {
  fn from(_d: UnfullfillRequirement) -> Self {
    UpdateModifications::Document(doc! { "$set": { "requirements.fullfilled_by": null }})
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

pub async fn fullfill_requirement(Path(id): Path<String>, Json(data): Json<FullfillRequirement>) -> Result<(), error::PrError> {
  let id = Uuid::parse_str(id)?;
  let filter = doc ! { "_id": id, "requirements.name": data.requirement_name.clone() };
  Ok::<(), error::PrError>(utils::put::<FullfillRequirement, Event>(COLL_NAME, filter, data).await?)
}

pub async fn unfullfill_requirement(Path(id): Path<String>, Json(data): Json<UnfullfillRequirement>) -> Result<(), error::PrError> {
  let id = Uuid::parse_str(id)?;
  let user_id = Uuid::parse_str(&data.user_id)?;
  let filter = doc ! { "_id": id, "requirements.name": data.requirement_name.clone(), "requirements.fullfilled_by._id": user_id };
  Ok::<(), error::PrError>(utils::put::<UnfullfillRequirement, Event>(COLL_NAME, filter, data).await?)
}

pub async fn add_user(Path(id): Path<String>, Json(data): Json<AddUserToEventData>) -> Result<(), error::PrError> {
  Ok::<(), error::PrError>(utils::put_by_id::<AddUserToEventData, Event>(COLL_NAME, id, data).await?)
}

pub async fn remove_user(Path((id, uid)): Path<(String, String)>) -> Result<(), error::PrError> {
  let id = Uuid::parse_str(id)?;
  let uid = Uuid::parse_str(uid)?;
  let data = RemoveUserFromEventData { _id: uid };
  let unfullfill_data = doc! { "$set": { "requirements.$[].fullfilled_by": null }};

  let coll = dbutils::get_collection::<Event>(COLL_NAME).await?;
  let _ = coll.update_one(doc ! { "_id": id.clone() }, data, None).await?;
  let _ = coll.update_many(doc ! { "_id": id, "requirements.fullfilled_by._id": uid }, unfullfill_data, None).await?;

  Ok::<(), error::PrError>(())
}

pub async fn read_all() -> Result<Json<Vec<Event>>, error::PrError> {
  Ok::<Json<Vec<Event>>, error::PrError>(Json(utils::read_all::<Event>(COLL_NAME).await?))
}
