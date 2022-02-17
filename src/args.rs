use std::fmt::Display;
use std::path::PathBuf;
use std::ffi::OsString;
use clap::{AppSettings, Parser, Subcommand};

use crate::chunk_type::ChunkType;

impl Display for Cmd {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Cmd::Encode {file, chunk_type, message, output} => write!(f, "encode {:?} {}", output, chunk_type),
      Cmd::Decode {file, chunk_type} => write!(f, "decode"),
      Cmd::Remove => write!(f, "remove"),
      Cmd::Print => write!(f, "print")
    }
  }
}

/// Encode, decode, modify or print a png file
#[derive(Parser, Debug)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(author = "Tim Schoenborn <tim@schonborn.nl>", version, about)]
pub struct Args {
#[clap(subcommand)]
  pub command: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
  /// Encode a png file
  #[clap(setting(AppSettings::ArgRequiredElseHelp))]
  Encode {
    #[clap(parse(from_os_str), value_name = "FILE")]
    file: PathBuf,

    #[clap(parse(try_from_str), value_name="CHUNK_TYPE")]
    chunk_type: ChunkType,

    #[clap(value_name = "MESSAGE", default_value="")]
    message: String,

    #[clap(parse(from_os_str), value_name = "OUTPUT")]
    output: Option<PathBuf>,
  },
  /// Decode a png file
  #[clap(setting(AppSettings::ArgRequiredElseHelp))]
  Decode {
    #[clap(parse(from_os_str), value_name = "FILE")]
    file: PathBuf,

    #[clap(parse(try_from_str), value_name="CHUNK_TYPE")]
    chunk_type: ChunkType,
  },
  
  /// Remove a chunk from a png file
  #[clap(setting(AppSettings::ArgRequiredElseHelp))]
  Remove,
  /// Print a png file
  #[clap(setting(AppSettings::ArgRequiredElseHelp))]
  Print
}

