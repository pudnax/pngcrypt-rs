pub mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
pub mod png;

pub use args::Commands::{Decode, Encode, Print, Remove};
pub use chunk::Chunk;
pub use chunk_type::ChunkType;
pub use commands::{decode, encode, print, remove};
pub use png::Png;

pub const BYTE_SIZE: usize = 4;

pub use error::{Error, Result};
