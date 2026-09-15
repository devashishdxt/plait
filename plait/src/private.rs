//! Implementation details shared with the version-matched proc macros.
//! These types are not a supported application construction API.

pub struct Missing;
pub struct Provided<T>(pub T);
