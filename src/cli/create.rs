use std::path::PathBuf;
use std::convert::TryFrom;

use structopt::StructOpt;

use crate::cli::CliSubCommand;
use crate::session::TmuxSession;
use crate::errors::Errcode;
use crate::window::WindowDescription;

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

    /// The window description,
    /// can be passed multiple time to create multiple windows
    /// 
    /// format:      NAME:STARTDIR:AUTORENAME:FOCUSED_PANE:PANE0:<PANE1>:<etc...>
    /// 
    /// Example:    code:./src/:off:0:nvim:cargo-watch -c:clear && bash
    #[structopt(short, long, default_value = "")]
    pub windows_description: Vec<WindowDescription>,

    /// Create a default "bash" tmux session
    #[structopt(short="D", long)]
    pub default: bool,

    /// Dump the content to stdout instead of writing it to the file
    #[structopt(short="o", long="dump")]
    pub dump: bool,
}

impl CliSubCommand for TmuxpSessionCreation {
    fn execute_command(&self) -> Result<(), Errcode>{
        let tmuxses = match TmuxSession::try_from(self){
            Ok(ses) => ses,
            Err(e) => {
                println!("Error while creating TmuxSession from commandline arguments");
                return Err(e);
            }
        };
        
        if self.dump {
            tmuxses.dump()?;
        } else {
            tmuxses.write_to_file()?;
        }
        Ok(())
    }

    fn validate_args(&self) -> Result<(), Errcode>{
        if !self.start_directory.is_dir(){
            return Err(Errcode::ArgValidationError("start directory"))
        }
        Ok(())
    }
}
