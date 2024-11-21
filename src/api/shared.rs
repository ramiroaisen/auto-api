use garde::{Path, Report, Validate};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::util::dive_vec;

#[derive(Serialize, Deserialize, JsonSchema, TS)]
pub struct Page<T> {
  #[garde(range(min = 0))]
  pub skip: usize,
  #[garde(range(min = 1))]
  pub limit: usize,
  #[garde(range(min = 0))]
  pub total: usize,
  #[garde(custom(dive_vec))]
  pub items: Vec<T>,
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

// temporary struct to do Page<T> validation with garde
#[derive(Validate)]
struct PageValidate<'a, T: Validate<Context = ()>> {
  #[garde(range(min = 0))]
  skip: usize,
  #[garde(range(min = 1))]
  limit: usize,
  #[garde(range(min = 0))]
  total: usize,
  #[garde(custom(dive_vec))]
  items: &'a Vec<T>,
}

fn default_limit() -> usize {
  200
}

fn default_skip() -> usize {
  0
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, TS)]
pub struct Limit(
  #[garde(range(min = 1, max = 200))]
  #[schemars(default = "default_limit")]
  pub usize
);

impl Default for Limit {
  fn default() -> Self {
    Self(default_limit())
  }
}


#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, TS)]
pub struct Skip (
  #[garde(range(min = 0))]
  #[schemars(default = "default_skip")]
  pub usize
);

impl Default for Skip {
  fn default() -> Self {
    Self(default_skip())
  }
}


