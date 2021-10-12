#![allow(unused_imports, dead_code, unused_variables)]
use structopt::StructOpt;
use std::process::exit;

mod errors;
mod session;
mod create;
mod cli;
mod pane;
mod window;

use errors::{Errcode, handle_error};
use cli::Commands;

fn main() {
    match Commands::from_args().start() {
        Ok(_) => exit(0),
        Err(e) => exit(handle_error(e)),
    }
}
