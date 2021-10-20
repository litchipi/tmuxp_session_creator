use structopt::StructOpt;

use crate::session::TmuxSession;
use crate::cli::CliSubCommand;
use crate::errors::Errcode;

#[derive(Debug, StructOpt)]
pub struct TmuxpSessionEdition {
    /// The name of the Tmuxp profile to edit
    #[structopt(short="n", long)]
    pub name: String,

    /// The window to modify
    #[structopt(short="i", long)]
    pub window_ind: usize,

    /// The layout to apply to the window
    #[structopt(short="l", long,)]
    pub layout: Option<String>,

    /// The window name to use
    #[structopt(short="w", long,)]
    pub window_name: Option<String>,

    /// The layout to apply to the window
    #[structopt(short="d", long,)]
    pub dump: bool,
}

impl CliSubCommand for TmuxpSessionEdition {

    fn execute_command(&self) -> Result<(), Errcode>{
        println!("Session name \"{}\"", self.name);
        let mut tmuxses = TmuxSession::load(&self.name)?;
        println!("Window {}", self.window_ind);

        let mut win = match tmuxses.get_window_ref(self.window_ind){
            Ok(w) => w,
            Err(_) => tmuxses.init_new_window()?,
        };

        if let Some(l) = &self.layout {
            win.set_layout(l)?;
        }

        if let Some(n) = &self.window_name {
            win.window_name = n.clone();
        }

        if self.dump {
            tmuxses.dump()?;
        } else {
            tmuxses.write_to_file()?;
        }
        Ok(())
    }

    fn validate_args(&self) -> Result<(), Errcode>{
        Ok(())
    }
}
