use std::{str::FromStr};

#[derive(PartialEq, Eq, Debug)]
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

    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_valid(&self) -> bool {
        if !self.bytes.iter().all(|&b| b.is_ascii_alphabetic()) {
            return false;
        }
        return self.is_reserved_bit_valid();
    }

    fn is_critical(&self) -> bool{
        self.bytes[0] & 0b00100000u8 != 0b00100000u8

    }

    fn is_public(&self) -> bool {
        self.bytes[1] & 0b00100000u8 != 0b00100000u8
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2] & 0b00100000u8 != 0b00100000u8
    }

    fn is_safe_to_copy(&self) -> bool {
        self.bytes[3] & 0b00100000u8 == 0b00100000u8
    }
}


impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in &bytes {
            if !byte.is_ascii_alphabetic() {
                return Err("Invalid value");
            }
        }
        Ok(ChunkType {
            bytes
        })
    }
}   

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("Invalid length");
        }
        for byte in s.as_bytes() {
            if !byte.is_ascii_alphabetic() {
                return  Err("Invalid value");
            }
        }

        Ok(ChunkType {
            // this cant be right haha
            bytes: s.bytes().into_iter().collect::<Vec<u8>>().as_slice().try_into().unwrap()
        
        })
    }
}

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
