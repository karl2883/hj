use std::fs;

use clap::Parser;

use crate::lexer::create_tokens;

mod lexer;
mod parser;
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
    
    let tokens = match create_tokens(source) {
        Ok(t) => t,
        Err(e) => {
            output::print_error(e.as_str());
            return Err(());
        }
    };

    if config.debug {
        output::print_debug("Tokens have been generated successfully!");
        let token_str = tokens.iter().map(|token| token.debug_str()).collect::<Vec<String>>().join(",\n");
        output::print_debug(format!("Tokens: {}", token_str).as_str());
    }

    let mut parser = parser::Parser::new(tokens);
    let scope_node = match parser.parse() {
        Ok(node) => node,
        Err(e) => {
            output::print_error(e.as_str());
            return Err(());
        }
    };

    if config.debug {
        output::print_debug("The AST has been generated successfully!");
        let ast_str = scope_node.debug_str();
        output::print_debug(&ast_str);
    }

    Ok(())
}
