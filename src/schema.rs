use garde::Validate;
use normalize::Normalize;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use shape::Shape;

use crate::void::Void;

pub trait Schema: Serialize + DeserializeOwned + JsonSchema + Validate<Context = ()> + Shape + Normalize + Void + 'static {}
impl<T: Serialize + DeserializeOwned + JsonSchema + Validate<Context = ()> + Shape + Normalize + Void + 'static> Schema for T {}
