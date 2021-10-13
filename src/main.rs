#![allow(unused_imports, dead_code, unused_variables)]
use structopt::StructOpt;
use std::process::exit;

mod errors;
mod session;
mod create;
mod cli;
mod pane;
mod window;
mod edit;

use errors::{Errcode, handle_error};
use cli::Commands;

// TODO  Setup log for entire project
fn main() {
    match Commands::from_args().start() {
        Ok(_) => exit(0),
        Err(e) => exit(handle_error(e)),
    }
}
