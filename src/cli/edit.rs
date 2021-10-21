use structopt::StructOpt;

use std::path::PathBuf;

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

    //TODO FIXME    Keep track of the order in which commands are passed
    /// The commands to pass to each pane. Can be passed multiple times
    #[structopt(short="c", long="command")]
    pub commandlist: Vec<String>,
    
    /// The pane / command to focus on the window
    #[structopt(short="f", long)]
    pub focus: Option<usize>,

    /// Wether the window should be the focused one in the session
    #[structopt(short="F", long)]
    pub window_focused: bool,

    /// Change the start directory of the window
    #[structopt(short="d", long)]
    pub start_directory: Option<PathBuf>,

    /// The layout to apply to the window
    #[structopt(short="D", long,)]
    pub dump: bool,
}

impl CliSubCommand for TmuxpSessionEdition {
    fn execute_command(&self) -> Result<(), Errcode>{
        let mut tmuxses = TmuxSession::load(&self.name)?;

        let mut win = match tmuxses.get_window_ref(self.window_ind){
            Ok(w) => w,
            Err(_) => tmuxses.init_new_window()?,
        };

        let cmdslen = self.commandlist.len();
        if cmdslen > 0 {
            win.panes.set_panes_cmds(&self.commandlist);
        }

        if let Some(p) = &self.start_directory {
            win.start_directory = p.canonicalize()?.clone();
        }

        if let Some(f) = &self.focus {
            win.panes.set_focus(*f)?;
        }

        if let Some(l) = &self.layout {
            win.set_layout(l)?;
        }

        if let Some(n) = &self.window_name {
            win.window_name = n.clone();
        }

        if self.window_focused {
            tmuxses.set_window_focus(self.window_ind)?;
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
