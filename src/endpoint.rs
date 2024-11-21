use std::borrow::Cow;
use bytes::BytesMut;
use garde::Validate;
use axum::{
  async_trait, body::{Body, Bytes}, extract::{FromRequest, FromRequestParts, Path, Request}, http::{
    header::CONTENT_TYPE, request::Parts, HeaderValue, Method, StatusCode
  }, response::{IntoResponse, Response}, Json
};
use tokio_stream::StreamExt;

use crate::schema::Schema;
use crate::error::{ApiError, ApiErrorKind, IntoApiError};

pub struct ParsedRequest<Context, Params, Query, Payload> {
  pub context: Context,
  pub params: Params,
  pub query: Query,
  pub payload: Payload,
}

pub trait EndpointError: std::error::Error + IntoApiError {}

#[async_trait]
pub trait Endpoint: Send + Sync + 'static {
  type Ctx: Send;
  
  type Params: Schema + Send;
  type Query: Schema + Send;
  type Payload: Schema + Send;
  type Output: Schema + Send;

  fn path(&self) -> Cow<'static, str>;

  fn method(&self) -> Method;

  fn max_payload_size(&self) -> usize {
    2 * 1024 * 1024 // 2MB
  }

  async fn ctx(&self, parts: &mut Parts) -> Result<Self::Ctx, Box<dyn EndpointError>>;

  async fn run(
    &self,
    request: ParsedRequest<
      Self::Ctx,
      Self::Params,
      Self::Query,
      Self::Payload
    >
  ) -> Result<Self::Output, Box<dyn EndpointError>>;

  async fn handle(&self, req: Request) -> Response {
    let (mut parts, body) = req.into_parts();

    let ctx = match self.ctx(&mut parts).await {
      Ok(ctx) => ctx,
      Err(err) => {
        return err.into_api_error().into_response()
      }
    };

    let params = if Self::Params::is_empty() {
      Self::Params::empty()
    } else { 
      let params = match Path::<Self::Params>::from_request_parts(&mut parts, &()).await {
        Ok(Path(params)) => params,
        Err(err) => {
          return ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            kind: ApiErrorKind::InvalidParamsParse,
            message: format!("error parsing path parameters: {err}"),
          }.into_response()
        }
      };

      match params.validate() {
        Ok(()) => {},
        Err(report) => {
          return ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            kind: ApiErrorKind::InvalidParamsValidate,
            message: format!("error validating path parameters: {report}"),
          }.into_response()
        }
      }

      params
    };
    

    let query = if Self::Query::is_empty() {
      Self::Query::empty()
    } else {
      // axum extractor
      // use axum::extract::Query;
      // let query = match Query::<Self::Query>::from_request_parts(&mut parts, &()).await {
      //   Ok(Query(query)) => query,
      //   Err(err) => {
      //     return ApiError {
      //       status: StatusCode::BAD_REQUEST.as_u16(),
      //       kind: ApiErrorKind::InvalidQueryParse,
      //       message: format!("error parsing query parameters: {err}"),
      //     }.into_response()
      //   }
      // };

      // serde_qs
      let query = match serde_qs::from_str::<Self::Query>(parts.uri.query().unwrap_or("")) {
        Ok(query) => query,
        Err(err) => {
          return ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            kind: ApiErrorKind::InvalidQueryParse,
            message: format!("error parsing query parameters: {err}"),
          }.into_response()
        }
      };

      match query.validate() {
        Ok(()) => {},
        Err(report) => {
          return ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            kind: ApiErrorKind::InvalidQueryValidate,
            message: format!("error validating query parameters: {report}"),
          }.into_response()
        }
      }

      query
    };

    let payload = if Self::Payload::is_empty() {
      Self::Payload::empty()
    } else {

      // // axum
      // let req = Request::from_parts(parts, body);

      // let payload = match Json::<Self::Payload>::from_request(req, &()).await {
      //   Ok(Json(payload)) => payload,
      //   Err(err) => {
      //     return ApiError {
      //       status: StatusCode::BAD_REQUEST.as_u16(),
      //       kind: ApiErrorKind::InvalidPayloadParse,
      //       message: format!("error parsing payload: {err}"),
      //     }.into_response()
      //   }
      // };

      if !is_content_type_json(&parts) {
        return ApiError {
          status: StatusCode::BAD_REQUEST.as_u16(),
          kind: ApiErrorKind::PayloadContentType,
          message: String::from("content-type of request must be application/json"),
        }
        .into_response()
      }

      let buf = match read_body(self.max_payload_size(), body).await {
        Ok(buf) => buf,
        Err(err) => {
          return ApiError::from(err).into_response()
        }
      };

      let payload = match serde_json::from_slice::<Self::Payload>(&buf) {
        Ok(payload) => payload,
        Err(err) => {
          return ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            kind: ApiErrorKind::InvalidPayloadParse,
            message: format!("error parsing payload: {err}"),
          }.into_response()
        }
      };

      match payload.validate() {
        Ok(()) => {},
        Err(report) => {
          return ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            kind: ApiErrorKind::InvalidPayloadValidate,
            message: format!("error validating payload: {report}"),
          }.into_response()
        }
      }
      
      payload
    };
    
    let parsed = ParsedRequest {
      context: ctx,
      params,
      query,
      payload,
    };

    let out = match self.run(parsed).await {
      Ok(res) => res,
      Err(err) => {
        return err.into_api_error().into_response()
      }
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
}

async fn read_body(max_size: usize, body: Body) -> Result<Bytes, ReadBodyError> {
  let mut bytes = BytesMut::new();
  let mut stream = body.into_data_stream();
  while let Some(chunk) = stream.try_next().await? {
    let target_size = bytes.len() + chunk.len();
    if target_size > max_size {
      return Err(ReadBodyError::MaxSizeExceeded(max_size));
    } 
    bytes.extend_from_slice(&chunk);
  }
  Ok(bytes.freeze())
}

#[derive(Debug, thiserror::Error)]
pub enum ReadBodyError {
  #[error("body max size of {0} bytes exceeded")]
  MaxSizeExceeded(usize),
  #[error(transparent)]
  Read(#[from] axum::Error),
}

impl From<ReadBodyError> for ApiError {
  fn from(e: ReadBodyError) -> Self {
      ApiError {
        status: StatusCode::BAD_REQUEST.as_u16(),
        kind: ApiErrorKind::PayloadRead,
        message: format!("error reading payload: {e}"),
      }
  }
}

pub fn is_content_type_json(parts: &Parts) -> bool {
  let content_type = match parts.headers.get(CONTENT_TYPE) {
    Some(content_type) => content_type,
    None => return false,
  };

  let content_type = match content_type.to_str() {
    Ok(content_type) => content_type,
    Err(_) => return false,
  };

  let mime = match content_type.parse::<typed_headers::mime::Mime>() {
    Ok(mime) => mime,
    Err(_) => return false,
  };

  mime.essence_str() == "application/json"
}