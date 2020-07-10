pub mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
pub mod png;

pub use chunk::Chunk;
pub use chunk_type::ChunkType;
pub use png::Png;

pub const BYTE_SIZE: usize = 4;

pub use error::{Error, Result};
