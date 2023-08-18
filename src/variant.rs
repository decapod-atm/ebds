//! Message variant types for crate messages.
//!
//! Allows returning a single type that wraps various message types from the crate.

mod command;
mod reply;

pub use command::*;
pub use reply::*;
