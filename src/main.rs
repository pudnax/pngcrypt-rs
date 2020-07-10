use pngme::{args::Opt, decode, encode, print, remove, Result};
use structopt::StructOpt;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        let mut e: &dyn std::error::Error = &e;
        while let Some(source) = e.source() {
            eprintln!("  - caused by: {}", source);
            e = source;
        }
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Encode(encode_args) => encode(encode_args)?,
        Opt::Decode(decode_args) => decode(decode_args)?,
        Opt::Remove(remove_args) => remove(remove_args)?,
        Opt::Print(print_args) => print(print_args)?,
    }
    Ok(())
}
