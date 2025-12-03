use std::error::Error;

#[allow(dead_code)]
pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

