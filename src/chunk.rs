use std::convert::TryFrom;

use std::convert::TryInto;
use std::fmt::{self, Display};

use crate::{
    chunk_type::ChunkType,
    error::{Error, Result},
    BYTE_SIZE,
};

// TODO(#3): Does it make sence too use Vec?
// TODO(#5): Do we need to use #[repr(C)] to proper alighment?
#[repr(C)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let hashing_data = [chunk_type.bytes(), data.as_slice()].concat();
        let crc = crc::crc32::checksum_ieee(&hashing_data);
        Chunk {
            length: data.len() as u32,
            chunk_type,
            chunk_data: data,
            crc,
        }
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn length(&self) -> usize {
        self.length as usize
    }

    fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    // TODO(#4): Delete public idetifier
    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.chunk_data.clone())?)
    }

    // TODO(#2): Make my own implementation of crc hashing
    fn crc(&self) -> u32 {
        self.crc
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [
            self.length.to_be_bytes().as_ref(),
            self.chunk_type.bytes(),
            self.chunk_data.as_slice(),
            self.crc.to_be_bytes().as_ref(),
        ]
        .concat()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error<'static>;

    fn try_from(value: &[u8]) -> Result<Self> {
        let length = u32::from_be_bytes(value[0..BYTE_SIZE].try_into()?) as usize;
        if value.len() != length + 3 * BYTE_SIZE {
            return Err(Error::Custom("ChunkData does not contain enough bytes"));
        }
        let data: [u8; BYTE_SIZE] = value[BYTE_SIZE..BYTE_SIZE + BYTE_SIZE].try_into()?;
        let chunk_type: ChunkType = data.try_into()?;
        let chunk_data = value[2 * BYTE_SIZE..2 * BYTE_SIZE + length].to_vec();

        let hashing_data = [chunk_type.bytes(), chunk_data.as_slice()].concat();
        let crc = crc::crc32::checksum_ieee(&hashing_data);
        let used_offset = 2 * BYTE_SIZE + length;

        if crc != u32::from_be_bytes(value[used_offset..used_offset + BYTE_SIZE].try_into()?) {
            return Err(Error::Custom("assert on crc checksums comparation"));
        }

        Ok(Chunk {
            length: length as u32,
            chunk_type,
            chunk_data,
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data: Vec<u8> = "This is where your secret message will be!"
            .bytes()
            .collect();
        Chunk::new(chunk_type, data)
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }
}
