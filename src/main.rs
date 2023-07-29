use core::panic;
use std::{
    fs, io,
    path::{self, Path},
    str::FromStr,
    sync::Arc,
};

use args::Commands;
use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod utils;
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::Args::parse();

    match &args.command {
        Commands::Print(args) => {
            println!("Output of {}", args.file)
        }
        Commands::Encode(args) => {
            if !Path::new(&args.file).exists() {
                panic!("'{}' file doesn't exists", &args.file);
            }

            match fs::read(args.file.clone()) {
                Ok(bytes) => match Png::try_from(bytes.as_slice()) {
                    Ok(mut png) => {
                        let chunk = Chunk::new(
                            ChunkType::from_str(args.chunk_type.as_str()).unwrap(),
                            args.message.as_bytes().to_vec(),
                        );

                        png.append_chunk(chunk);
                        let out_bytes = png.as_bytes();

                        let out_path = args.output_file.clone().unwrap_or(String::from("out.png"));
                        if let Err(e) = fs::write(&out_path, out_bytes) {
                            panic!("Failed to write {}", e);
                        }

                        println!("Wrote output to `{}`", &out_path)
                    }
                    Err(e) => panic!("PNG read failed with {}", e.to_string()),
                },
                Err(e) => panic!("{}", e.to_string()),
            }
        }
        Commands::Decode(args) => {
            if !Path::new(&args.file).exists() {
                panic!("'{}' file doesn't exists", &args.file);
            }

            match fs::read(args.file.clone()) {
                Ok(bytes) => match Png::try_from(bytes.as_slice()) {
                    Ok(png) => {
                        if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
                            let msg = chunk.data_as_string().unwrap();

                            println!("Message: {}", msg);
                        } else {
                            panic!("Chunk `{}` doesn't exists", &args.chunk_type);
                        }
                    }
                    Err(e) => panic!("PNG read failed with {}", e.to_string()),
                },
                Err(e) => panic!("{}", e.to_string()),
            }
        }
    }
    Ok(())
}
