use garde::{Path, Report, Validate};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::registry::Registry;

fn validate_vec<T: Validate>(slice: &[T], ctx: &T::Context) -> Result<(), garde::error::Error>
where
  T::Context: Default,
{
  // TODO: add index to path
  for item in slice {
    match item.validate_with(ctx) {
      Ok(_) => {}
      Err(report) => {
        return Err(garde::error::Error::new(report.to_string()));
      }
    }
  }

  Ok(())
}

// temporary struct to do Page<T> validation with garde
#[derive(Validate)]
struct PageValidate<'a, T: Validate<Context = ()>> {
  #[garde(range(min = 0))]
  skip: usize,
  #[garde(range(min = 1))]
  limit: usize,
  #[garde(range(min = 0))]
  total: usize,
  #[garde(custom(validate_vec))]
  items: &'a Vec<T>,
}

#[derive(Serialize, Deserialize, JsonSchema, TS)]
pub struct Page<T> {
  #[garde(range(min = 0))]
  skip: usize,
  #[garde(range(min = 1))]
  limit: usize,
  #[garde(range(min = 0))]
  total: usize,
  #[garde(custom(validate_vec))]
  items: Vec<T>,
}

impl<T: Validate<Context = ()>> Validate for Page<T> {
  type Context = ();

  fn validate_into(
    &self,
    ctx: &Self::Context,
    parent: &mut dyn FnMut() -> Path,
    report: &mut Report,
  ) {
    PageValidate {
      skip: self.skip,
      limit: self.limit,
      total: self.total,
      items: &self.items,
    }
    .validate_into(ctx, parent, report)
  }
}

pub mod users {
  use super::*;
  
  #[derive(Serialize, Deserialize, JsonSchema, Validate, TS)]
  pub struct Item {
    #[garde(pattern("^[a-z0-9]+$"))]
    id: String,
    #[garde(email, length(max = 100))]
    email: String,
  }

  pub mod list {
    use super::*;
    use axum::{async_trait, http::{request::Parts, Method}};
    use std::borrow::Cow;
    use crate::{Endpoint, EndpointError, ParsedRequest};

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
        _req: ParsedRequest<
          <Self as Endpoint>::Ctx,
          <Self as Endpoint>::Params,
          <Self as Endpoint>::Query,
          <Self as Endpoint>::Payload,
        >,
      ) -> Result<<Self as Endpoint>::Output, Box<dyn EndpointError>> {
        Ok(Page::<Item> {
          skip: 0,
          limit: 0,
          total: 0,
          items: vec![],
        })
      }
    }
  }

  pub mod get {
    use super::*;
    use std::borrow::Cow;
    use axum::{async_trait, http::{request::Parts, Method}};
    use crate::{Endpoint, EndpointError, ParsedRequest};


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
        _req: ParsedRequest<
          <Self as Endpoint>::Ctx,
          <Self as Endpoint>::Params,
          <Self as Endpoint>::Query,
          <Self as Endpoint>::Payload,
        >,
      ) -> Result<<Self as Endpoint>::Output, Box<dyn EndpointError>> {
        Ok(Item {
          id: "123".to_string(),
          email: "test@test.com".to_string(),
        })
      }
    }
  }
}


pub fn get_registry() -> Registry {
  let mut registry = Registry::new();
  
  macro_rules! r {
    ($item:expr) => {
      registry.register($item);
    };
  }

  r!(users::list::E);
  r!(users::get::E);
  
  registry
}
