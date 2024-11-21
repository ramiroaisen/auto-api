pub mod shared;
pub mod users;

use crate::{error::ApiErrorPayload, registry::Registry};

pub fn registry() -> Registry {
  let mut registry = Registry::new::<ApiErrorPayload>();
  
  macro_rules! r {
    ($item:path) => {
      registry.register($item);
    };
  }

  r!(users::list::E);
  r!(users::get::E);
  
  registry
}
