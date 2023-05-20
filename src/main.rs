use clap::Parser;

mod application;

use application::{search_from_file};

#[derive(Parser)]
pub struct Args {
    /// Absolute path to the file with the list of search strings
    #[arg(short, long)]
    path: std::path::PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    search_from_file(args.path)
}
