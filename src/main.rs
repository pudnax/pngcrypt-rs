use pngme::{
    args::{DecodeArgs, EncodeArgs, Opt, PrintArgs, RemoveArgs},
    Chunk, ChunkType, Png, Result,
};
use std::{
    convert::TryInto,
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};
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

fn take_png<P: AsRef<Path>>(input: P) -> Result<Png> {
    let mut file = OpenOptions::new().write(true).read(true).open(input)?;
    let mut buffer = Vec::with_capacity(1000_000);

    file.read_to_end(&mut buffer)?;
    Ok(buffer.as_slice().try_into()?)
}

fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = take_png(&args.input)?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.into_bytes(),
    ));

    let mut file = std::fs::File::create(args.input)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
    let png = take_png(&args.input)?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!(
            "Hidden message in the chunk {}: '{}'",
            chunk.chunk_type().to_string(),
            chunk.data_as_string()?
        );
    } else {
        println!("Unable to decode chunk");
    }
    Ok(())
}

fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = take_png(&args.input)?;
    png.remove_chunk(&args.chunk_type)?;

    let mut file = std::fs::File::create(args.input)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}

fn print(args: PrintArgs) -> Result<()> {
    let png = take_png(&args.input)?;
    println!(
        "File: {}, Size: {}",
        args.input.display(),
        png.as_bytes().len()
    );
    for (i, chunk) in png.chunks().iter().enumerate() {
        print!("\n({})", i + 1);
        print!("{}", chunk);
    }
    Ok(())
}
