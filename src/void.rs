use std::any::{Any, TypeId};

pub trait Void: Sized +'static {
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

impl<T: Sized + 'static> Void for T {}
