use structopt::StructOpt;

use crate::session::TmuxSession;
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
    pub layout: Option<String>,
}

impl CliSubCommand for TmuxpSessionEdition {

    fn execute_command(&self) -> Result<(), Errcode>{
        println!("Session name \"{}\"", self.name);
        let mut tmuxses = TmuxSession::load(&self.name)?;
        println!("Window {}", self.window_ind);

        if let Some(l) = &self.layout {
            let n = get_npane_from_layout(l)?;
            println!("{} panes", n);

            let lenwin = tmuxses.windows.len();
            tmuxses.windows
                .get_mut(self.window_ind)
                .ok_or(Errcode::WindowNotFound(self.window_ind, lenwin))?
                .layout = Some(l.clone());
        }
        tmuxses.dump()?;
        //tmuxses.write_to_file()?;
        Ok(())
    }

    fn validate_args(&self) -> Result<(), Errcode>{
        Ok(())
    }
}
