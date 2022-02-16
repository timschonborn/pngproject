use std::{str::FromStr};
use crate::{Result, Error, chunk::Chunk};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChunkType {
  bytes: [u8; 4],
}

/// 4-bytes that identify the type of the chunk. As described
/// in the spec: http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
impl ChunkType {
  pub fn new(&self, bytes: [u8; 4]) -> Self {
    ChunkType {
        bytes
    }
  }

  /// Returns bytes of the chunk type
  pub fn bytes(&self) -> [u8; 4] {
    self.bytes
  }

  /// adheres to spec
  fn is_valid(&self) -> bool {
    if !self.bytes.iter().all(|&b| b.is_ascii_alphabetic()) {
        return false;
    }
    return self.is_reserved_bit_valid();
  }

  /// Ancillary bit (5th bit) 0 = critical, 1 = ancillary
  fn is_critical(&self) -> bool{
    self.bytes[0] & 0b00100000u8 != 0b00100000u8

  }

  /// Private bit (5th bit) 0 = public, 1 = private
  fn is_public(&self) -> bool {
    self.bytes[1] & 0b00100000u8 != 0b00100000u8
  }

  /// Reserved bit (5th bit) 0 = reserved, 1 = not reserved (must be 0)
  fn is_reserved_bit_valid(&self) -> bool {
    self.bytes[2] & 0b00100000u8 != 0b00100000u8
  }

  /// Safe-to-copy bit (5th bit) 0 = unsafe to copy, 1 = safe to copy
  fn is_safe_to_copy(&self) -> bool {
    self.bytes[3] & 0b00100000u8 == 0b00100000u8
  }
}


impl TryFrom<[u8; 4]> for ChunkType {
  type Error = Error;
  fn try_from(bytes: [u8; 4]) -> Result<Self> {
    for byte in &bytes {
      if !byte.is_ascii_alphabetic() {
          return Err(Box::new(std::fmt::Error));
      }
    }
    Ok(ChunkType {
      bytes
    })
  }
}  

impl TryFrom<&str> for ChunkType {
  type Error = Error;
  fn try_from(s: &str) -> Result<Self> {
    let bytes = s.as_bytes();
    if bytes.len() != 4 {
      return Err(Box::new(std::fmt::Error));
    }
    for byte in bytes {
      if !byte.is_ascii_alphabetic() {
          return Err(Box::new(std::fmt::Error));
      }
    }
    Ok(ChunkType {
      bytes: [bytes[0], bytes[1], bytes[2], bytes[3]]
    })
  }
}

impl FromStr for ChunkType {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    if s.len() != 4 {
      return Err(Box::new(std::fmt::Error));
    }
    for byte in s.as_bytes() {
      if !byte.is_ascii_alphabetic() {
          return Err(Box::new(std::fmt::Error));
      }
    }

    Ok(ChunkType {
      bytes: s.as_bytes().try_into().unwrap()
    })
  }
}

/// FIXME added this for making #[clap(parse(from_str))] work
/// but since this op can panic TryFrom is the better trait,
/// which I couldn't get working in parse()
// impl From<&str> for ChunkType {
//   fn from(s: &str) -> Self {
//     ChunkType {
//       bytes: s.as_bytes().try_into().unwrap()
//     }
//   }
// }

impl std::fmt::Display for ChunkType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", String::from_utf8_lossy(&self.bytes))
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
    println!("valid: {}", chunk.is_valid());
    assert!(!chunk.is_valid());

    let chunk = ChunkType::from_str("Ru1t");
    println!("valid: {:?}", chunk);
    assert!(chunk.is_err());
  }

  #[test]
  pub fn test_chunk_type_string() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert_eq!(&chunk.to_string(), "RuSt");
  }

  #[test]
  pub fn test_chunk_type_trait_impls() {
    let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    let _chunk_string = format!("{}", chunk_type_1);
    let _are_chunks_equal = chunk_type_1 == chunk_type_2;
  }
}
