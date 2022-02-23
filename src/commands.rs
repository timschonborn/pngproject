use std::{path::PathBuf, process::Output};
use std::io;
use std::io::prelude::*;
use crate::chunk::Chunk;
use crate::png::Png;
use crate::{Error, Result};
use crate::chunk_type::ChunkType;

/// FIXME to utils or chunks
fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk> {
  use std::str::FromStr;

  let chunk_type = ChunkType::from_str(chunk_type)?;
  let data: Vec<u8> = data.bytes().collect();

  Ok(Chunk::new(chunk_type, data))
}

/// Insert a chunk with type and message into the given png file and writhe to output file.
pub fn encode(file: &PathBuf, chunk_type: &ChunkType, message: &str, output: &Option<PathBuf>) {
  println!("{:?} {} {} {:?}", file, chunk_type, message, output);

  let mut file = std::fs::File::open(file).unwrap();
  let mut buffer = Vec::<u8>::new();
  file.read_to_end(&mut buffer).unwrap();

  // Create png from fs::File
  let mut png = Png::try_from(buffer.as_slice()).unwrap();

  // create chunk from type and message
  let chunk = chunk_from_strings(&chunk_type.to_string(), message).unwrap();

  // insert at second to last pos
  png.insert_chunk(chunk);

  std::fs::write("./test.png", png.as_bytes()).unwrap();
}

/// Read a chunk with chunk_type from given png and print chunk data.
pub fn decode(file: &PathBuf, chunk_type: &ChunkType) {
  println!("{:?} {}", file, chunk_type);

  let mut file = std::fs::File::open(file).unwrap();
  let mut buffer = Vec::<u8>::new();
  file.read_to_end(&mut buffer).unwrap();

  let mut png = Png::try_from(buffer.as_slice()).unwrap();

  let chunk = png.chunk_by_type(&chunk_type.to_string()).unwrap();

  println!("{}", chunk.data_as_string().unwrap());
}