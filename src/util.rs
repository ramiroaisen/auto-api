use garde::Validate;

/// garde doesnt seems to support #[garde(inner(dive))] into Vec
pub fn dive_vec<T: Validate>(slice: &[T], ctx: &T::Context) -> Result<(), garde::error::Error> {
  // TODO: add index to path
  for item in slice {
    match item.validate_with(ctx) {
      Ok(_) => {}
      Err(report) => {
        return Err(garde::error::Error::new(report.to_string()));
      }
    }
  }

  Ok(())
}


/// garde doesnt seems to support #[garde(inner(dive))] into Option
pub fn dive_option<T: Validate>(opt: &Option<T>, ctx: &T::Context) -> Result<(), garde::error::Error> {
  // TODO: add index to path
  if let Some(item) = opt {
    match item.validate_with(ctx) {
      Ok(_) => {}
      Err(report) => {
        return Err(garde::error::Error::new(report.to_string()));
      }
    }
  };

  Ok(())
}