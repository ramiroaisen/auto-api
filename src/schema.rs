use std::any::{Any, TypeId};
use garde::Validate;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use ts_rs::TS;

pub trait Schema: Serialize + DeserializeOwned + JsonSchema + Validate<Context = ()> + TS + 'static {
  fn is_void() -> bool {
    TypeId::of::<Self>() == TypeId::of::<()>()
  }

  fn void() -> Self {
    if Self::is_void() {
      let v = Box::new(()) as Box<dyn Any>;
      *v.downcast::<Self>().unwrap()
    } else {
      panic!("cannot create void value of non void schema")
    }
  }
}

impl<T: Serialize + DeserializeOwned + JsonSchema + Validate<Context = ()> + TS + 'static> Schema for T {}
