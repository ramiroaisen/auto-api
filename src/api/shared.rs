use garde::{Path, Report, Validate};
use normalize::Normalize;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shape::Shape;

pub const DEFAULT_LIMIT: u64 = 200;
pub const DEFAULT_SKIP: u64 = 0;

/// # Page
/// A page of items starting from `skip` and limited by `limit`
/// with the total number of records present in `total`
#[derive(Serialize, Deserialize, JsonSchema, Shape, Normalize)]
#[normalize(bound = "T: Normalize")]
pub struct Page<T> {
  #[normalize(skip)]
  #[garde(range(min = 0))]
  pub skip: u64,
  #[normalize(skip)]
  #[garde(range(min = 1))]
  pub limit: u64,
  #[normalize(skip)]
  #[garde(range(min = 0))]
  pub total: u64,
  #[normalize(dive)]
  #[garde(dive)]
  pub items: Vec<T>,
}

impl<T: Validate> Validate for Page<T> {
  type Context = T::Context;

  fn validate_into(
    &self,
    ctx: &Self::Context,
    parent: &mut dyn FnMut() -> Path,
    report: &mut Report,
  ) {
    ValidatePage {
      skip: self.skip,
      limit: self.limit,
      total: self.total,
      items: &self.items,
    }
    .validate_into(ctx, parent, report)
  }
}

/// temporary struct to do Page<T> validation with garde \
/// TODO: remove this when garde supports generics
#[derive(Validate)]
#[garde(context(T::Context))]
struct ValidatePage<'a, T: Validate> {
  #[garde(range(min = 0))]
  skip: u64,
  #[garde(range(min = 1))]
  limit: u64,
  #[garde(range(min = 0))]
  total: u64,
  #[garde(dive)]
  items: &'a Vec<T>,
}

const fn default_limit() -> u64 {
  DEFAULT_LIMIT
}

const fn default_skip() -> u64 {
  DEFAULT_SKIP
}

/// # Pagination Limit
/// How many records to return as maximum for the current query
#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, Shape, Normalize)]
pub struct Limit(
  #[normalize(skip)]
  #[garde(range(min = 1, max = 200))]
  #[schemars(default = "default_limit", example = "default_limit")]
  pub u64
);

impl Default for Limit {
  fn default() -> Self {
    Self(DEFAULT_LIMIT)
  }
}


/// # Pagination Skip
/// How many records to skip for the current query
#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, Shape, Normalize)]
pub struct Skip (
  #[normalize(skip)]
  #[garde(range(min = 0))]
  #[schemars(default = "default_skip", example = "default_skip")]
  pub u64
);

impl Default for Skip {
  fn default() -> Self {
    Self(DEFAULT_SKIP)
  }
}


