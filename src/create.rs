use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::convert::TryFrom;

use structopt::StructOpt;
use serde_json::to_string;
use dirs::home_dir;

use crate::cli::CliSubCommand;
use crate::session::TmuxSession;
use crate::errors::Errcode;
use crate::window::WindowDescription;

const TMUXP_DIR: &'static str = ".tmuxp/";

#[derive(Debug, StructOpt)]
pub struct TmuxpSessionCreation {
    /// The name of the Tmux session to create
    #[structopt(short="n", long)]
    pub session_name: String,

    /// The directory where the tmux will be launched
    #[structopt(short="d", long="directory")]
    pub start_directory: PathBuf,
    
    /// The number of the window to focus
    #[structopt(short, long, default_value = "0")]
    pub focus: usize,

    /// The window description, can be passed multiple time to create multiple windows
    #[structopt(short, long, default_value = "")]
    pub windows_description: Vec<WindowDescription>,

    /// Create a default "bash" tmux session
    #[structopt(short="D", long)]
    pub default: bool,
}

impl CliSubCommand for TmuxpSessionCreation {
    fn execute_command(&self) -> Result<(), Errcode>{
        let tmuxses = TmuxSession::try_from(self)?;
        let mut output_fname = PathBuf::from(home_dir().ok_or(Errcode::EnvError(0))?);
        output_fname.push(TMUXP_DIR);
        output_fname.push(self.session_name.replace(" ", "_"));
        output_fname.set_extension("json");
        
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
