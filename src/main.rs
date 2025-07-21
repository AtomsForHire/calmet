mod cli;
mod io;
mod metrics;

use clap::Parser;
use std::process;

fn main() {
    let command = cli::Cli::parse();

    if let Err(e) = command.sub_command.run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    };

    println!("Successfully processed data");
}
