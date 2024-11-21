use std::borrow::Cow;
use garde::Validate;
use axum::{
  async_trait, extract::{FromRequest, FromRequestParts, Path, Query, Request}, http::{
    header::CONTENT_TYPE, request::Parts, HeaderValue, Method, StatusCode
  }, response::{IntoResponse, Response}, Json
};

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

    let req_ctx = match self.ctx(&mut parts).await {
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
      let query = match Query::<Self::Query>::from_request_parts(&mut parts, &()).await {
        Ok(Query(query)) => query,
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
      
      let req = Request::from_parts(parts, body);

      let payload = match Json::<Self::Payload>::from_request(req, &()).await {
        Ok(Json(payload)) => payload,
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
      context: req_ctx,
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