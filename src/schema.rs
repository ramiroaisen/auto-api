use garde::Validate;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use ts_rs::TS;

use crate::void::Void;

pub trait Schema: Serialize + DeserializeOwned + JsonSchema + Validate<Context = ()> + TS + Void + 'static {}
impl<T: Serialize + DeserializeOwned + JsonSchema + Validate<Context = ()> + TS + Void + 'static> Schema for T {}
