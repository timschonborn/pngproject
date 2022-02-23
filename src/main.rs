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

fn main() -> Result<()> {
  let args = args::Args::parse();

  if let Some(command) = &args.command {
    match command {
      args::Cmd::Encode {file, chunk_type, message, output} => commands::encode(file, chunk_type, message, output), // commands::encode::encode(args.input, args.output)?,
      args::Cmd::Decode {file, chunk_type} => commands::decode(file, chunk_type), // commands::decode::decode(args.input, args.output)?,
      args::Cmd::Remove {file, chunk_type} => commands::remove(file, chunk_type), // commands::remove::remove(args.input, args.output, args.chunk_type)?,
      args::Cmd::Print {file} => commands::print(file), // commands::print::print(args.input, args.output)?,
    }
  }
  Ok(())
}
