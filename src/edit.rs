use structopt::StructOpt;

use crate::cli::CliSubCommand;
use crate::errors::Errcode;
use crate::window::{WindowDescription, WindowLayout, get_npane_from_layout};

#[derive(Debug, StructOpt)]
pub struct TmuxpSessionEdition {
    /// The name of the Tmuxp profile to edit
    #[structopt(short="n", long)]
    pub name: String,

    /// The window to modify
    #[structopt(short="w", long)]
    pub window_ind: usize,

    /// The layout to apply to the window
    #[structopt(short, long,)]
    pub layout: String,
}

impl CliSubCommand for TmuxpSessionEdition {
    fn execute_command(&self) -> Result<(), Errcode>{
        let n = get_npane_from_layout(self.layout.as_ref())?;
        println!("Session name \"{}\"", self.name);
        println!("Window {}", self.window_ind);
        println!("{} panes", n);
        Ok(())
    }

    fn validate_args(&self) -> Result<(), Errcode>{
        Ok(())
    }
}
