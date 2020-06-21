use char;
use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;
use std::iter::Iterator;
use std::str;
use std::str::FromStr;

use anyhow::{Error, Result, anyhow};

#[derive(Debug, Eq, PartialEq)]
pub struct ChunkType {
    pub data: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        // TODO: Define cases where Error should be returned.
        return Ok(ChunkType { data: value });
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(anyhow!("String length is not equal to 4 bytes"));
        }

        let string_is_alpha: bool = s.chars().all(|c| (c as char).is_ascii_alphabetic());

        if !string_is_alpha {
            return Err(anyhow!("Valid ChunkType must contain only ASCII alphabetic characters"));
        }

        let mut byte_array: [u8; 4] = [0; 4];
        byte_array.copy_from_slice(s.as_bytes());

        return Ok(ChunkType { data: byte_array });
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}", str::from_utf8(&self.data).unwrap()))
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.data;
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        return (self.data[2] as char).is_ascii_uppercase();
    }

    pub fn is_critical(&self) -> bool {
        return (self.data[0] as char).is_ascii_uppercase();
    }

    pub fn is_public(&self) -> bool {
        return (self.data[1] as char).is_ascii_uppercase();
    }

    pub fn is_valid(&self) -> bool {
        return (self.data[2] as char).is_ascii_uppercase();
    }

    pub fn is_safe_to_copy(&self) -> bool {
        return !(self.data[3] as char).is_ascii_uppercase();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
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
    pub fn test_chunk_type_is_not_critcal() {
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

        let chunk = ChunkType::from_str("1111");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }
}
