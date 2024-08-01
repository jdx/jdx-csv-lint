use std::error::Error;
use std::process::exit;

use clap::Parser;
use log::error;

use cli::Cli;

mod linter;
mod cli;
mod logger;
mod lint_error;
mod linter_builder;
mod checks;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() {
    let cli = Cli::parse();
    if let Err(err) = cli.run() {
        error!("{err}");
        exit(1);
    }
}
