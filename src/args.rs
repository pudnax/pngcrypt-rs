use std::path::PathBuf;
use structopt::StructOpt;

/// Command line program that lets you hide secret messages in PNG files.
#[derive(Debug, StructOpt)]
#[structopt(name = "pngme")]
pub enum Opt {
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
    /// Path to file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    /// Chunk type
    pub chunk_type: String,
    /// Secret message
    pub message: String,
}

#[derive(Debug, StructOpt)]
pub struct DecodeArgs {
    /// Path to file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    /// Chunk type
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct RemoveArgs {
    /// Path to file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    /// Chunk type
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct PrintArgs {
    /// Path to file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}
