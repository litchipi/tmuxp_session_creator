use structopt::StructOpt;
use std::path::PathBuf;

use crate::cli::CliSubCommand;
use crate::session::TmuxpSession;
use crate::errors::Errcode;

pub type WindowDescription = String;

#[derive(Debug, StructOpt)]
pub struct TmuxpSessionCreation {
    #[structopt(short="n", long)]
    session_name: String,

    #[structopt(short="d", long="directory")]
    start_directory: PathBuf,
    
    #[structopt(short, long, default_value = "0")]
    focus: usize,

    #[structopt(short, long, default_value = "")]
    windows_description: Vec<WindowDescription>,
}

impl CliSubCommand for TmuxpSessionCreation {
    fn execute_command(&self) -> Result<(), Errcode>{
        Ok(())
    }

    fn validate_args(&self) -> Result<(), Errcode>{
        if !self.start_directory.is_dir(){
            return Err(Errcode::ArgValidationError("start directory"))
        }
        Ok(())
    }
}
