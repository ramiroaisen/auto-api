use garde::{Path, Report, Validate};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const DEFAULT_LIMIT: u64 = 200;
pub const DEFAULT_SKIP: u64 = 0;

#[derive(Serialize, Deserialize, JsonSchema, TS)]
pub struct Page<T> {
  #[garde(range(min = 0))]
  pub skip: u64,
  #[garde(range(min = 1))]
  pub limit: u64,
  #[garde(range(min = 0))]
  pub total: u64,
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, TS)]
pub struct Limit(
  #[garde(range(min = 1, max = 200))]
  #[schemars(default = "default_limit", example = "default_limit")]
  pub u64
);

impl Default for Limit {
  fn default() -> Self {
    Self(DEFAULT_LIMIT)
  }
}


#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, TS)]
pub struct Skip (
  #[garde(range(min = 0))]
  #[schemars(default = "default_skip", example = "default_skip")]
  pub u64
);

impl Default for Skip {
  fn default() -> Self {
    Self(DEFAULT_SKIP)
  }
}


