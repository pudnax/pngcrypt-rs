use crate::{
    args::{DecodeArgs, EncodeArgs, RemoveArgs},
    Chunk, ChunkType, Error, Png, Result,
};
use std::{
    convert::TryInto,
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

fn take_png<P: AsRef<Path>>(input: P) -> Result<Png> {
    let mut file = OpenOptions::new().write(true).read(true).open(input)?;
    let mut buffer = Vec::with_capacity(1000_000);

    file.read_to_end(&mut buffer)?;
    Ok(buffer.as_slice().try_into()?)
}

pub fn encode<S: AsRef<Path>>(input: S, args: EncodeArgs) -> Result<()> {
    let mut png = take_png(&input)?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.into_bytes(),
    ));

    let mut file = std::fs::File::create(input)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}

pub fn decode<S: AsRef<Path>>(input: S, args: DecodeArgs) -> Result<()> {
    let png = take_png(&input)?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!(
            "Hidden message in the chunk {}: '{}'",
            chunk.chunk_type().to_string(),
            chunk.data_as_string()?
        );
    } else {
        return Err(Error::Custom("Unable to decode chunk"));
    }
    Ok(())
}

pub fn remove<S: AsRef<Path>>(input: S, args: RemoveArgs) -> Result<()> {
    let mut png = take_png(&input)?;
    png.remove_chunk(&args.chunk_type)?;

    let mut file = std::fs::File::create(input)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}

pub fn print(input: &Path) -> Result<()> {
    let png = take_png(&input)?;
    println!("File: {}, Size: {}", input.display(), png.as_bytes().len());
    for (i, chunk) in png.chunks().iter().enumerate() {
        print!("\n({})", i + 1);
        print!("{}", chunk);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_copy_of_file<'a>() -> &'a Path {
        let input = Path::new("copy.png");
        std::fs::File::create(&input).unwrap();
        std::fs::copy("assets/pic.png", &input).unwrap();
        input
    }

    #[test]
    fn test_encode() {
        let input = make_copy_of_file();
        let chunk_type = "RuST".to_string();
        let message = "Message".to_string();
        let args = EncodeArgs {
            chunk_type,
            message,
        };
        let res = encode(input, args);
        assert!(res.is_ok());
    }

    #[test]
    fn test_decode() {
        let input = make_copy_of_file();
        let chunk_type = "RuST".to_string();
        let message = "Message".to_string();
        let args = EncodeArgs {
            chunk_type,
            message,
        };
        let res = encode(input, args);
        assert!(res.is_ok());
        let chunk_type = "RuST".to_string();
        let args = DecodeArgs { chunk_type };
        let res = decode(input, args);
        assert!(res.is_ok());
    }

    #[test]
    fn test_remove() {
        let input = make_copy_of_file();
        let chunk_type = "RuST".to_string();
        let message = "Message".to_string();
        let args = EncodeArgs {
            chunk_type,
            message,
        };
        let res = encode(input, args);
        assert!(res.is_ok());
        let chunk_type = "RuST".to_string();
        let args = RemoveArgs { chunk_type };
        let res = remove(input, args);
        assert!(res.is_ok());
    }

    #[test]
    fn test_print() {
        let input = make_copy_of_file();
        let res = print(&input);
        assert!(res.is_ok());
    }

    #[test]
    fn test_all_one() {
        let input = make_copy_of_file();
        let chunk_type = "RuST".to_string();
        let message = "Message".to_string();
        let args = EncodeArgs {
            chunk_type,
            message,
        };
        let res = encode(input, args);
        assert!(res.is_ok());
        let chunk_type = "RuST".to_string();
        let args = DecodeArgs { chunk_type };
        let res = decode(input, args);
        assert!(res.is_ok());
        let chunk_type = "RuST".to_string();
        let args = RemoveArgs { chunk_type };
        let res = remove(input, args);
        assert!(res.is_ok());
        let res = print(&input);
        assert!(res.is_ok());
    }
}
