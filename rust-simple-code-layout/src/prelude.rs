pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;
// Generic wrapper tuple struct for newtype pattern,mostly used for type alias
pub struct W<T>(pub T);
