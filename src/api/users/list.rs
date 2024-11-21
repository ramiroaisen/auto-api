use axum::{async_trait, http::{request::Parts, Method}};
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use std::borrow::Cow;
use crate::{api::shared::{Limit, Skip}, endpoint::{Endpoint, EndpointError, ParsedRequest}};

use super::Item;
use crate::api::shared::Page;

pub struct E;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, TS)]
pub struct Query {
  #[garde(dive)]
  skip: Option<Skip>,
  #[garde(dive)]
  limit: Option<Limit>,
}

#[async_trait]
impl Endpoint for E {
  type Params = ();
  type Query = Query;
  type Payload = ();
  type Output = Page<Item>;
  type Ctx = ();

  fn path(&self) -> Cow<'static, str> {
    "/users".into()
  }

  fn method(&self) -> Method {
    Method::GET
  }

  async fn ctx(
    &self,
    _parts: &mut Parts,
  ) -> Result<Self::Ctx, Box<dyn EndpointError>> {
    Ok(())
  }

  async fn run(
    &self,
    ParsedRequest { query, .. }: ParsedRequest<Self::Ctx, Self::Params, Self::Query, Self::Payload>,
  ) -> Result<Self::Output, Box<dyn EndpointError>> {
    
    let Skip(skip) = query.skip.unwrap_or_default();
    let Limit(limit) = query.limit.unwrap_or_default();

    Ok(Page::<Item> {
      skip,
      limit,
      total: 0,
      items: vec![],
    })
  }
}

