pub mod shared;
pub mod users;

use crate::{error::ApiErrorPayload, registry::Registry};

pub fn registry() -> Registry {
  let mut registry = Registry::new::<ApiErrorPayload>();
  
  macro_rules! r {
    ($item:expr) => {
      registry.register($item);
    };

    ($item:expr, $($rest:expr,)*) => {
      r!($item);
      r!($($rest),*);
    };
  }

  r!(
    users::list::E,
    users::get::E,
  );
  
  registry
}
