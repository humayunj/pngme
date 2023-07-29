#![allow(unused_variables)]

use std::fmt::{format, Display};

use crc::{Crc, CRC_16_NRSC_5, CRC_32_CKSUM, CRC_32_ISO_HDLC, CRC_32_MPEG_2};

use crate::{chunk_type::ChunkType, Error};
#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let mut bytes = (&chunk_type.bytes()).to_vec();

        bytes.extend_from_slice(&data[..]);

        let crc = Chunk::compute_crc(&bytes[..]);
        Chunk {
            chunk_type,
            length: data.len() as u32,
            chunk_data: data,
            crc,
        }
    }

    pub fn compute_crc(bytes: &[u8]) -> u32 {
        Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(bytes)
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data[..]
    }
    pub fn data_as_string(&self) -> crate::Result<String> {
        let v = String::from_utf8(self.data().to_vec());
        match v {
            Err(err) => Err(Error::from(err.to_string())),
            Ok(o) => Ok(o),
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect::<Vec<u8>>()
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Length: {}", &self.length).as_str())
            .unwrap();
        f.write_str(format!("Chunk Type: {}", &self.chunk_type.to_string()).as_str())
            .unwrap();
        f.write_str(format!("CRC: {}", &self.crc).as_str()).unwrap();
        Ok(())
    }
}
impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 4 {
            return Err(Error::from("invalid length"));
        }

        let mut len: u32 = 0;
        len |= (value[0] as u32) << 24;
        len |= (value[1] as u32) << 16;
        len |= (value[2] as u32) << 8;
        len |= (value[3] as u32) << 0;

        let mut type_bytes: [u8; 4] = [0, 0, 0, 0];
        for i in 0..4 {
            type_bytes[i] = value[4 + i];
        }

        let chunk_type = ChunkType::try_from(type_bytes);
        if chunk_type.is_err() {
            return Err(chunk_type.err().unwrap());
        }

        let chunk_type = chunk_type.ok().unwrap();

        // check remaining bytes
        if value.len() < 8 + len as usize {
            return Err(Error::from(format!(
                "insufficient bytes required: {}",
                value.len() + 8 + len as usize
            )));
        }

        let data = value[8..(8 + len as usize)].to_vec();

        if data.len() != len as usize {
            return Err(Error::from(format!(
                "Invalid length {} expected {}",
                len,
                data.len()
            )));
        }

        let mut bytes = (&chunk_type.bytes()).to_vec();

        bytes.extend_from_slice(&data[..]);

        let crc: u32 = Chunk::compute_crc(&bytes[..]);

        let crc_bytes: &[u8] = &value[(8 + len as usize)..(8 + len as usize + 4)];

        let mut provided_crc: u32 = 0;
        provided_crc |= (crc_bytes[0] as u32) << 24;
        provided_crc |= (crc_bytes[1] as u32) << 16;
        provided_crc |= (crc_bytes[2] as u32) << 8;
        provided_crc |= (crc_bytes[3] as u32) << 0;

        if provided_crc != crc {
            return Err(Error::from(format!(
                "Provided CRC {} mismatched {}",
                provided_crc, crc
            )));
        }

        Ok(Chunk {
            length: len,
            chunk_type,
            chunk_data: data,
            crc,
        })
    }
}

#[cfg(test)]
mod tests {
    use crc::CRC_32_AUTOSAR;

    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
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

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);

        assert_eq!(chunk.crc(), 2882656334);
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

    #[test]
    pub fn test_chunk_trait_impls() {
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

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
