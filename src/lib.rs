pub mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
pub mod png;

pub const BYTE_SIZE: usize = 4;

pub use error::{Error, Result};
