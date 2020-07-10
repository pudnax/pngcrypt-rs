use std::path::PathBuf;
use structopt::StructOpt;

/// Command line program that lets you hide secret messages in PNG files.
#[derive(Debug, StructOpt)]
#[structopt(name = "pngme")]
pub struct Opt {
    /// Path to file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    /// Available commands
    #[structopt(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, StructOpt)]
pub enum Commands {
    /// Encodes a message into a PNG file and saves the result
    Encode(EncodeArgs),
    /// Searches for a message hidden in a PNG file and prints the message if one is found
    Decode(DecodeArgs),
    /// Removes a chunk from a PNG file and saves the result
    Remove(RemoveArgs),
    /// Prints all of the chunks in a PNG file
    Print(PrintArgs),
}

#[derive(Debug, StructOpt)]
pub struct EncodeArgs {
    /// Chunk type
    pub chunk_type: String,
    /// Secret message
    pub message: String,
}

#[derive(Debug, StructOpt)]
pub struct DecodeArgs {
    /// Chunk type
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct RemoveArgs {
    /// Chunk type
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct PrintArgs {}
