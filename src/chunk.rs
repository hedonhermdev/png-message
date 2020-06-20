use byteorder::{BigEndian, ByteOrder};
use crc::crc32;
use std;
use std::convert::TryFrom;
use std::io::{BufReader, Read};
use std::string::FromUtf8Error;
use std::u32;

use super::chunk_type::ChunkType;

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);

        let mut length_arr: [u8; 4] = [0; 4];
        let mut read_op = reader.read_exact(&mut length_arr);

        if read_op.is_err() {
            return Err("Cannot read chunk length");
        }

        let length = u32::from_be_bytes(length_arr);

        let mut chunk_type_arr: [u8; 4] = [0; 4];
        read_op = reader.read_exact(&mut chunk_type_arr);

        if read_op.is_err() {
            return Err("Cannot read chunk type");
        }

        let chunk_type = ChunkType::try_from(chunk_type_arr);

        if chunk_type.is_err() {
            return Err("ChunkType is Invalid");
        }

        let mut chunk_data: Vec<u8> = vec![0; length as usize];
        read_op = reader.read_exact(&mut chunk_data);

        if read_op.is_err() {
            return Err("Cannot read chunk data");
        }

        let mut crc_arr: [u8; 4] = [0; 4];

        read_op = reader.read_exact(&mut crc_arr);

        if read_op.is_err() {
            return Err("Cannot read chunk CRC");
        }

        let crc = u32::from_be_bytes(crc_arr);

        let calucated_crc: u32 = crc32::checksum_ieee(&value[4..(8 + length) as usize]);

        if crc != calucated_crc {
            return Err("CRC is incorrect");
        }

        return Ok(Chunk {
            length,
            chunk_type: chunk_type.unwrap(),
            chunk_data,
            crc,
        });
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Chunk {
        let length: u32 = chunk_data.len() as u32;
        let mut crc_vec: Vec<u8> = chunk_type.bytes().to_vec();
        crc_vec.extend(chunk_data.iter().cloned());
        let crc: u32 = crc32::checksum_ieee(&crc_vec);
        return Chunk {
            length,
            chunk_type,
            chunk_data,
            crc,
        };
    }
    pub fn length(&self) -> u32 {
        return self.length;
    }

    pub fn chunk_type(&self) -> &ChunkType {
        return &self.chunk_type;
    }

    pub fn data(&self) -> &[u8] {
        return &self.chunk_data;
    }

    pub fn data_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        return String::from_utf8(self.chunk_data.clone());
    }

    pub fn crc(&self) -> u32 {
        return self.crc;
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let mut length_arr: [u8; 4] = [0; 4];
        BigEndian::write_u32(&mut length_arr, self.length);
        // Length
        bytes.extend_from_slice(&length_arr);

        // Chunk type
        bytes.extend_from_slice(&self.chunk_type.bytes());

        // Data
        bytes.extend(self.chunk_data.clone());

        let mut crc_arr: [u8; 4] = [0; 4];
        BigEndian::write_u32(&mut crc_arr, self.crc);
        // CRC
        bytes.extend_from_slice(&crc_arr);

        return bytes;
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

    // #[test]
    // fn test_chunk_string() {
    //     let chunk = testing_chunk();
    //     let chunk_string = chunk.data_as_string().unwrap();
    //     let expected_chunk_string = String::from("This is where your secret message will be!");
    //     assert_eq!(chunk_string, expected_chunk_string);
    // }

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
