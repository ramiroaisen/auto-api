pub mod list;
pub mod get;

use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// # User
/// A user record
#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, TS)]
pub struct User {
  /// The unique id of the user
  #[garde(pattern("^[a-z0-9]+$"))]
  id: String,
  /// The email address of the user
  #[garde(email, length(max = 100))]
  email: String,
}