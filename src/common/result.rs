use std::result;

use super::error::Error;

pub type Result<'a, T> = result::Result<T, Error<'a>>;
