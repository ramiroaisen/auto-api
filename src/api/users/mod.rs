pub mod list;
pub mod get;

use garde::Validate;
use normalize::Normalize;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shape::Shape;

/// # User
/// A user record
#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate, Shape, Normalize)]
pub struct User {
  /// The unique id of the user
  #[normalize(skip)]
  #[garde(pattern("^[a-z0-9]+$"))]
  id: String,
  /// The email address of the user
  #[normalize(skip)]
  #[garde(email, length(max = 100))]
  email: String,
}