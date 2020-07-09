use pngme::{args::Opt, Result};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    Ok(())
}
