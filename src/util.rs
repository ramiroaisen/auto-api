use garde::Validate;

/// garde doesnt seems to support #[garde(inner(dive))] into Vec
pub fn validate_vec<T: Validate>(slice: &[T], ctx: &T::Context) -> Result<(), garde::error::Error>
where
  T::Context: Default,
{
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