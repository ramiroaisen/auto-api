use axum::{http::{header::CONTENT_TYPE, HeaderValue, StatusCode}, response::{IntoResponse, Response}};
use serde::Serialize;

use crate::error::{ApiError, ApiErrorKind};

pub fn to_response<T: Serialize, E: IntoResponse>(result: Result<T, E>) -> Response {
  let out = match result {
    Ok(out) => out,
    Err(err) => return err.into_response(),
  };

  let body = match serde_json::to_vec(&out) {
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