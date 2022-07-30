use clap::Parser;


#[derive(Parser, Debug)]
#[clap(name = "hj compiler")]
#[clap(author = "karl2883 & Trubiso")]
#[clap(version = "0.0")]
#[clap(about = "Compiles hj source files into executables")]
struct CliArgs {
    /// The name of the output file
    #[clap(short, long)]
    output: Option<String>,

    /// Print debug information
    #[clap(short, long)]
    debug: bool,

    /// The name of the file to be compiled
    file: String,
}

fn main() {
    let args = CliArgs::parse();

    println!("Compiling  file {}!", args.file);
    if args.debug {
        println!("Printing debug information!");
    }
    if let Some(output_file) = args.output {
        println!("Outputting to file {output_file}!");
    }
}
