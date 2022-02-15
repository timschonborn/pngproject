use std::fmt::Display;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]
pub enum Cmd {
  /// Encode a png file
  Encode,
  /// Decode a png file
  Decode,
  /// Remove a chunk from a png file
  Remove,
  /// Print a png file
  Print
}

impl Display for Cmd {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Cmd::Encode => write!(f, "encode"),
      Cmd::Decode => write!(f, "decode"),
      Cmd::Remove => write!(f, "remove"),
      Cmd::Print => write!(f, "print")
    }
  }
}

/// Testing Clap
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
  #[clap(subcommand)]
  command: Option<Cmd>
}

impl Args {
  pub fn command(&self) -> &Option<Cmd> {
    &self.command
  }
}

