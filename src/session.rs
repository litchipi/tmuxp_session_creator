use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

use crate::create::WindowDescription;
use crate::create::TmuxpSessionCreation;

// List of commands
type TmuxPane = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct TmuxSession {
    session_name: String,
    start_directory: PathBuf,
    windows: Vec<TmuxWindow>,
    focus: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct TmuxWindow {
    window_name: String,
    layout: Option<String>,

    panes_focus: usize,
    panes: (TmuxPane, Vec<TmuxPane>),

    // options
    automatic_rename: bool,
}

impl TmuxWindow {
    pub fn default() -> TmuxWindow {
        TmuxWindow {
            window_name: String::from("default"),
            layout: None,
            panes_focus: 0,
            panes: (vec!["clear".to_string(), "bash".to_string()], vec![]),
            automatic_rename: false,
        }
    }
}

impl From<&WindowDescription> for TmuxWindow {
    // TODO Parse WindowDescription
    fn from(descr: &WindowDescription) -> TmuxWindow {
        TmuxWindow::default()
    }
}

impl Into<WindowDescription> for TmuxWindow {
    // TODO Parse WindowDescription
    fn into(self) -> WindowDescription {
        String::from("")
    }
}

impl From<&TmuxpSessionCreation> for TmuxSession {
    fn from(c: &TmuxpSessionCreation) -> TmuxSession {
        TmuxSession {
            session_name: c.session_name.clone(),
            start_directory: c.start_directory.clone(),
            windows: c.windows_description.iter().map(|el| el.into()).collect(),
            focus: c.focus,
        }
    }
}
