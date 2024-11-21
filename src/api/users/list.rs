use axum::{async_trait, http::{request::Parts, Method}};
use std::borrow::Cow;
use crate::endpoint::{Endpoint, EndpointError, ParsedRequest};

use super::Item;
use crate::api::shared::Page;

pub struct E;

#[async_trait]
impl Endpoint for E {
  type Params = ();
  type Query = ();
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
  ) -> Result<<Self as Endpoint>::Ctx, Box<dyn EndpointError>> {
    Ok(())
  }

  async fn run(
    &self,
    ParsedRequest { .. }: ParsedRequest<Self::Ctx, Self::Params, Self::Query, Self::Payload>,
  ) -> Result<<Self as Endpoint>::Output, Box<dyn EndpointError>> {
    Ok(Page::<Item> {
      skip: 0,
      limit: 0,
      total: 0,
      items: vec![],
    })
  }
}

