use clap::Parser;

use crate::lexer::create_tokens;

mod lexer;

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

    println!("Compiling file {}!", config.file);
    if config.debug {
        println!("Printing debug information!");
    }
    if let Some(output_file) = &config.output {
        println!("Outputting to file {output_file}!");
    }
    
    let tokens = create_tokens(String::from("let i = 1 + false;"));

    println!("Under construction!");
    Ok(())
}
