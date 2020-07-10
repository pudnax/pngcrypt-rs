#![feature(stmt_expr_attributes)]
use pngme::{args::Opt, decode, encode, print, remove, Decode, Encode, Print, Remove, Result};
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

    #[rustfmt::skip]
    match opt {
        Opt { input, commands: Encode(args) } => encode(input, args)?,
        Opt { input, commands: Decode(args) } => decode(input, args)?,
        Opt { input, commands: Remove(args) } => remove(input, args)?,
        Opt { input, commands: Print(_)     } => print(&input)?,
    }
    Ok(())
}
