use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;

pub enum PrError {
  DB(String),
  Server(String),
  NotFound(String)
}

impl IntoResponse for PrError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      PrError::Server(msg) => {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("{} - {}", "Internal Server Error", msg)
        )
      }
      PrError::DB(msg) => {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("{} - {}", "Internal Server Error - Database", msg)
        )
      }
      PrError::NotFound(msg) => {
        (
          StatusCode::NOT_FOUND,
          format!("{} - {}", "Not Found", msg)
        )
      }
    };
    let body = Json(json!({
      "error": error_message,
    }));

    (status, body).into_response()
  }
}

impl From<axum::Error> for PrError {
  fn from(e: axum::Error) -> Self {
      PrError::Server(e.to_string())
  }
}

impl From<mongodb::error::Error> for PrError {
  fn from(e: mongodb::error::Error) -> Self {
      PrError::DB(e.to_string())
  }
}

impl From<mongodb::bson::uuid::Error> for PrError {
  fn from(e: mongodb::bson::uuid::Error) -> Self {
    PrError::Server(e.to_string())
  }
}
