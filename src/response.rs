use axum::{http::{header::CONTENT_TYPE, HeaderValue, StatusCode}, response::{IntoResponse, Response}};
use serde::Serialize;

use crate::error::{ApiError, ApiErrorKind};

pub fn into_json_response<T: Serialize>(v: T) -> Response {
  let body = match serde_json::to_vec(&v) {
    Ok(body) => body,
    Err(_err) => {
      return ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        kind: ApiErrorKind::Internal,
        message: String::from("error serializing response"),
      }.into_response()
    }
  };

  let mut res = Response::new(body.into());
  res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
  res

}