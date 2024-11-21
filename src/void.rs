use std::any::{type_name, Any, TypeId};

pub trait Void {
  fn is_void() -> bool where Self: 'static {
    TypeId::of::<Self>() == TypeId::of::<()>()
  }

  /// Create the () value from the Self = () type \
  /// This function will panic if !Self::is_void() meaning Self is not the type ()
  fn void() -> Self where Self: Sized + 'static {
    if Self::is_void() {
      // this doesn't really allocate, because the size of () is 0
      let v = Box::new(()) as Box<dyn Any>;
      *v.downcast::<Self>().unwrap()
    } else {
      panic!("cannot create void value out of non void type {}", type_name::<Self>())
    }
  }
}

impl<T> Void for T {}
