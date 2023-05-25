use clap::Parser;

mod application;

use application::websites_from_file;

#[derive(Parser)]
pub struct Args {
    /// Absolute path to the file with the list of website strings
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    websites_from_file(args.path);
}
