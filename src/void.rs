use std::any::{Any, TypeId};

pub trait Void {
  fn is_void() -> bool where Self: 'static {
    TypeId::of::<Self>() == TypeId::of::<()>()
  }

  /// Create the () value from the Self = () type \
  fn void() -> Option<Self> where Self: Sized + 'static {
    // this doesn't really allocate, because the size of () is 0
    let boxed = Box::new(()) as Box<dyn Any>;
    match boxed.downcast::<Self>() {
      Ok(v) => Some(*v),
      Err(_) => None,
    }
  }
}

impl<T> Void for T {}
