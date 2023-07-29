use clap::{Parser, Subcommand};

use crate::commands::*;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Print(PrintArgs),
    Encode(EncodeArgs),
    Decode(DecodeArgs),
}
