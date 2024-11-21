use axum::{http::{header::CONTENT_TYPE, HeaderValue}, response::{IntoResponse, Response}};
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, TS)]
pub struct ApiError {
  #[garde(range(min = 400, max = 599))]
  pub status: u16,
  #[serde(flatten)]
  #[garde(skip)]
  pub kind: ApiErrorKind,
  #[garde(skip)]
  pub message: String
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(tag = "kind", content = "meta", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiErrorKind {
  Internal,
  
  ResourceNotFound,
  RecordNotFound,

  InvalidParamsParse,
  InvalidParamsValidate,
  
  InvalidQueryParse,
  InvalidQueryValidate,

  InvalidPayloadParse,
  InvalidPayloadValidate,
}


pub trait IntoApiError {
  fn into_api_error(self: Box<Self>) -> ApiError;
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, TS)]
pub struct ApiErrorPayload {
  #[garde(skip)]
  pub error: ApiError,
}

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    let status = self.status;
    let payload = ApiErrorPayload {
      error: self,
    };
    let body = serde_json::to_vec(&payload).expect("error serializing api error");
    let mut res = Response::new(body.into());
    *res.status_mut() = status.try_into().expect("invalid status code in api error");
    res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json")); 
    res
  }
}