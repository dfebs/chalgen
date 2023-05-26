use chalgen::{file_processor, cli};
use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    file_processor::output_challenges(args);
}