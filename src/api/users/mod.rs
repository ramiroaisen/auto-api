pub mod list;
pub mod get;

use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, TS)]
pub struct Item {
  #[garde(pattern("^[a-z0-9]+$"))]
  id: String,
  #[garde(email, length(max = 100))]
  email: String,
}