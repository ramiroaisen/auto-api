use std::any::{Any, TypeId};
use garde::Validate;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use ts_rs::TS;

pub trait Schema: Serialize + DeserializeOwned + JsonSchema + Validate<Context = ()> + TS + 'static {
  fn is_empty() -> bool {
    TypeId::of::<Self>() == TypeId::of::<()>()
  }

  fn empty() -> Self {
    if Self::is_empty() {
      let v = Box::new(()) as Box<dyn Any>;
      *v.downcast::<Self>().unwrap()
    } else {
      panic!("cannot create empty value of non empty schema")
    }
  }
}

impl<T: Serialize + DeserializeOwned + JsonSchema + Validate<Context = ()> + TS + 'static> Schema for T {}
