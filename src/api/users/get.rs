
use std::borrow::Cow;
use axum::{async_trait, http::{request::Parts, Method}};
use crate::endpoint::{Endpoint, EndpointError, ParsedRequest};
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::Item;

pub struct E;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema, Validate, TS)]
pub struct Params {
  #[garde(pattern("^[a-z0-9]+$"))]
  id: String
}

#[async_trait]
impl Endpoint for E {
  type Params = Params;
  type Query = ();
  type Payload = ();
  type Output = Item;
  type Ctx = ();

  fn path(&self) -> Cow<'static, str> {
    "/users/:id".into()
  }

  fn method(&self) -> Method {
    Method::GET
  }
  
  async fn ctx(
    &self,
    _parts: &mut Parts,
  ) -> Result<<Self as Endpoint>::Ctx, Box<dyn EndpointError>> {
    Ok(())
  }

  async fn run(
    &self,
    ParsedRequest { .. }: ParsedRequest<Self::Ctx, Self::Params, Self::Query, Self::Payload>,
  ) -> Result<<Self as Endpoint>::Output, Box<dyn EndpointError>> {
    Ok(Item {
      id: "123".to_string(),
      email: "test@test.com".to_string(),
    })
  }
}
