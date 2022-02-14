use std::string::FromUtf8Error;

use crate::chunk_type::ChunkType;

pub struct Chunk {
  length: u32,
  chunk_type: ChunkType,
  data: Vec<u8>,
  crc: u32,
}


// Copy the unit tests below and paste them at the bottom of your chunk.rs file.
// Write a Chunk struct with your implementation of PNG chunks.
// Implement TryFrom<&[u8]> for your Chunk.
// Implement Display for your Chunk.
// Required methods:
//     fn length(&self) -> u32
//     fn chunk_type(&self) -> &ChunkType
//     fn data(&self) -> &[u8]
//     fn crc(&self) -> u32
//     fn data_as_string(&self) -> Result<String>
//     fn as_bytes(&self) -> Vec<u8>
// Pass all of the unit tests.


impl Chunk {

  pub fn new(length: u32, chunk_type: ChunkType, data: Vec<u8>, crc: u32) -> Chunk {
    Chunk {
      length,
      chunk_type,
      data,
      crc,
    }
  }

  pub fn length(&self) -> u32 {
    self.length
  }

  pub fn chunk_type(&self) -> &ChunkType {
    &self.chunk_type
  }

  pub fn data(&self) -> &[u8] {
    &self.data
  }

  pub fn crc(&self) -> u32 {
    self.crc
  }

  pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
    String::from_utf8(self.data.clone())
  }

}

impl TryFrom<&[u8]> for Chunk {
  type Error = &'static str;
  fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
    if bytes.len() < 12 {
      return Err("Invalid length");
    }

    let checksum = crc32fast::hash(&bytes[4..bytes.len() - 4]);
    if checksum != u32::from_be_bytes(array4_from_slice(&bytes[bytes.len() - 4..])) {
      return Err("Invalid checksum");
    }
    
    Ok(Chunk {
      length: u32::from_be_bytes(array4_from_slice(&bytes[..4])),
      chunk_type: ChunkType::try_from(array4_from_slice(&bytes[4..8]))?,
      data: bytes[8..bytes.len() - 4].to_vec(),
      crc: u32::from_be_bytes(array4_from_slice(&bytes[bytes.len() - 4..])),
    })
  }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk {{ length: {}, chunk_type: {}, data: {:?}, crc: {} }}", 
          self.length, self.chunk_type, self.data, self.crc)
    }
}

/// Turn a slice of bytes into an array of length 4, needed for TryFrom.
/// This can probably be improved, but I'm not sure how
pub fn array4_from_slice(slice: &[u8]) -> [u8; 4] {
  let mut arr = [0; 4];
  for (i, &item) in slice.iter().enumerate() {
      arr[i] = item;
  }
  arr
}



#[cfg(test)]
mod tests {
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