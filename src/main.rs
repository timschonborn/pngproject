use std::path::PathBuf;

use chunk_type::ChunkType;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;


fn encode(file: &PathBuf, chunk_type: &ChunkType, message: &String, output: Option<&PathBuf>) -> Result<()> {
  todo!()  
  // let mut png = png::Png::from_file(file)?;
    // let chunk = chunk::Chunk::new(chunk_type, message)?;
    // png.add_chunk(chunk)?;
    // png.write_file(output)?;
    // Ok(())

  
}

fn decode(file: &PathBuf, chunk_type: &ChunkType) -> Result<()> {
  todo!()
}

fn main() -> Result<()> {
  let args = args::Args::parse();

  if let Some(command) = &args.command {
    match command {
      args::Cmd::Encode {file, chunk_type, message, output} => println!("{}", command), // commands::encode::encode(args.input, args.output)?,
      args::Cmd::Decode {file, chunk_type} => println!("{}", command), // commands::decode::decode(args.input, args.output)?,
      args::Cmd::Remove => println!("{}", command), // commands::remove::remove(args.input, args.output, args.chunk_type)?,
      args::Cmd::Print => println!("{}", command), // commands::print::print(args.input, args.output)?,
    }
  }
  Ok(())
}

