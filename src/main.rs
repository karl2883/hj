use hj::Config;
use clap::Parser;


fn main() {
    let args = Config::parse();

    if let Err(()) = hj::run(args) {
        println!("Unable to compile (see errors above)!");
    }
}
