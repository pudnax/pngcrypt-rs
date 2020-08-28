use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

use crate::BYTE_SIZE;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType([u8; BYTE_SIZE]);

impl ChunkType {
    pub fn bytes(&self) -> &[u8; 4] {
        &self.0
    }

    fn at_char(&self, index: usize) -> char {
        (self.0)[index] as char
    }

    pub fn is_critical(&self) -> bool {
        self.at_char(0).is_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.at_char(1).is_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.at_char(2).is_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        !self.at_char(3).is_uppercase()
    }

    pub fn is_valid(&self) -> bool {
        !self.is_public() && self.is_reserved_bit_valid()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error<'static>;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        if bytes.iter().all(|&c| (c as char).is_ascii_alphabetic()) {
            Ok(ChunkType(bytes))
        } else {
            Err(Error::Custom("Invalid assii literals"))
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).map_err(Error::from)?)
    }
}

impl FromStr for ChunkType {
    type Err = Error<'static>;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() > BYTE_SIZE {
            return Err(Error::Custom("Too long lenght of chunk type"));
        }
        if !s.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(Error::Custom("Invalid assii literal"));
        }
        let mut chunk = [1; BYTE_SIZE];
        chunk.clone_from_slice(s.as_bytes());
        Ok(ChunkType(chunk))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = &[82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }
}
