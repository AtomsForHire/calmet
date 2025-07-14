mod cli;
mod io;
mod metrics;

use clap::Parser;

fn main() {
    let command = cli::Cli::parse();

    match command.sub_command.run() {
        Ok(_) => {
            println!("Finished");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
