use axum::{http::{header::CONTENT_TYPE, HeaderValue}, response::{IntoResponse, Response}};
use garde::Validate;
use normalize::Normalize;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shape::Shape;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema, Validate, Shape, Normalize, thiserror::Error)]
#[error("ApiError: status = {status}, kind = {kind:?}, message = {message}")]
pub struct ApiError {
  #[normalize(skip)]
  #[garde(range(min = 400, max = 599))]
  pub status: u16,
  #[normalize(dive)]
  #[garde(skip)]
  #[serde(flatten)]
  pub kind: ApiErrorKind,
  #[normalize(trim)]
  #[garde(skip)]
  pub message: String
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema, Shape, Normalize)]
#[serde(tag = "kind", content = "meta", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiErrorKind {
  Internal,
  
  ResourceNotFound,
  RecordNotFound,

  InvalidParamsParse,
  InvalidParamsValidate,
  
  InvalidQueryParse,
  InvalidQueryValidate,

  PayloadRead,
  PayloadContentType,
  InvalidPayloadParse,
  InvalidPayloadValidate,
}


pub trait IntoApiError {
  fn into_api_error(self: Box<Self>) -> ApiError;
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, Shape, Normalize)]
pub struct ApiErrorPayload {
  #[normalize(dive)]
  #[garde(dive)]
  pub error: ApiError,
}

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    let status = self.status;
    let mut payload = ApiErrorPayload { error: self };
    payload.normalize();
    let body = serde_json::to_vec(&payload).expect("error serializing api error");
    let mut res = Response::new(body.into());
    *res.status_mut() = status.try_into().expect("invalid status code in api error");
    res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json")); 
    res
  }
}