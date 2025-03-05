#![allow(clippy::needless_return)]
#![allow(clippy::unused_unit)]

pub use common::*;
pub use fixed_point_value::*;
pub use precision::*;

mod common;
mod fixed_point_value;
mod precision;

mod test;