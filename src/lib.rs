use std::fs;

use clap::Parser;

use crate::lexer::create_tokens;

mod lexer;
mod output;

// clap generates cli parsing into this struct for us through macros

#[derive(Parser, Debug)]
#[clap(name = "hj compiler")]
#[clap(author = "karl2883 & Trubiso")]
#[clap(version = "0.0")]
#[clap(about = "Compiles hj source files into executables.")]
pub struct Config {
    /// The name of the output file
    #[clap(short, long)]
    pub output: Option<String>,

    /// Print debug information
    #[clap(short, long)]
    pub debug: bool,

    /// The name of the file to be compiled
    pub file: String,
}

pub fn run(config: Config) -> Result<(), ()> {

    output::print_process("Compiling", format!("file {}...", config.file).as_str());
    if config.debug {
        output::print_debug("Printing debug information!")
    }

    let source = match fs::read_to_string(&config.file) {
        Ok(src) => src,
        Err(e) => {
            output::print_error(format!("Error reading from source file \"{}\": {}", &config.file, e).as_str());
            return Err(());
        }
    };
    
    let tokens = create_tokens(source);

    output::print_debug("Tokens have been generated successfully!");
    Ok(())
}
