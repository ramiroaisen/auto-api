use garde::{Path, Report, Validate};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::util::validate_vec;

#[derive(Serialize, Deserialize, JsonSchema, TS)]
pub struct Page<T> {
  #[garde(range(min = 0))]
  pub skip: usize,
  #[garde(range(min = 1))]
  pub limit: usize,
  #[garde(range(min = 0))]
  pub total: usize,
  #[garde(custom(validate_vec))]
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
  #[garde(custom(validate_vec))]
  items: &'a Vec<T>,
}
