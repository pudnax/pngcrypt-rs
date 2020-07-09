use crate::{Error, Result};
use std::{path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(Debug)]
enum Mode {
    Encode,
    Decode,
    Print,
}

impl FromStr for Mode {
    type Err = Error<'static>;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "encode" => Ok(Mode::Encode),
            "decode" => Ok(Mode::Decode),
            "print" => Ok(Mode::Print),
            _ => Err(Error::Custom("Parse error")),
        }
    }
}

/// Command line program that lets you hide secret messages in PNG files.
#[derive(Debug, StructOpt)]
#[structopt(name = "pngme")]
pub struct Opt {
    /// Application mode.
    mode: Mode,
    /// Path to file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    /// Chunk type
    chunk_type: Option<String>,
    /// Secret message
    message: Option<String>,
}
