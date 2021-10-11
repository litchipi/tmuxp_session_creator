use structopt::StructOpt;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use serde_json::to_string;

use crate::cli::CliSubCommand;
use crate::session::TmuxSession;
use crate::errors::Errcode;


pub type WindowDescription = String;

#[derive(Debug, StructOpt)]
pub struct TmuxpSessionCreation {
    #[structopt(short="n", long)]
    pub session_name: String,

    #[structopt(short="d", long="directory")]
    pub start_directory: PathBuf,
    
    #[structopt(short, long, default_value = "0")]
    pub focus: usize,

    #[structopt(short, long, default_value = "")]
    pub windows_description: Vec<WindowDescription>,
}

impl CliSubCommand for TmuxpSessionCreation {
    fn execute_command(&self) -> Result<(), Errcode>{
        let tmuxp_dir = PathBuf::from("~/.tmuxp/");
        let tmuxses = TmuxSession::from(self);
        let output_fname = tmuxp_dir.join(self.session_name.replace(" ", "_") + ".json");
        
        let mut file = File::create(output_fname)?;
        file.write_all(to_string(&tmuxses)?.as_bytes())?;
        Ok(())
    }

    fn validate_args(&self) -> Result<(), Errcode>{
        if !self.start_directory.is_dir(){
            return Err(Errcode::ArgValidationError("start directory"))
        }
        Ok(())
    }
}
