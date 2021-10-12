use std::path::PathBuf;
use serde::{Serializer, Serialize, Deserialize};
use serde::ser::{SerializeStruct, SerializeSeq};
use serde_json::{to_string, from_str};

use crate::errors::Errcode;
use crate::create::WindowDescription;
use crate::create::TmuxpSessionCreation;

// List of commands
type TmuxPane = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
struct FocusedPane {
    shell_command: String,
    focus: bool
}

impl FocusedPane {
    pub fn from_cmd_vec(cmds: Vec<String>) -> FocusedPane {
        FocusedPane {
            shell_command: cmds.join(" && "),
            focus: true
        }
    }
}

#[derive(Debug)]
struct PaneSerializer {
    focused: FocusedPane,
    others: Vec<TmuxPane>,
    focused_index: usize
}

impl PaneSerializer{
    pub fn create(focused: FocusedPane, focused_index: usize, others: Vec<TmuxPane>) -> PaneSerializer {
        PaneSerializer {
            focused,
            focused_index,
            others
        }
    }
}

impl Serialize for PaneSerializer{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(1 + self.others.len()))?;

        let mut ind_back = 0;
        for i in 0..(1+self.others.len()){
            if i == self.focused_index {
                seq.serialize_element(&self.focused)?;
                ind_back = 1;
            } else {
                assert!(i - ind_back < self.others.len());
                seq.serialize_element(&self.others[i - ind_back])?;
            }
        }
        seq.end()

    }
}

#[derive(Debug)]
struct TmuxWindow {
    window_name: String,
    layout: Option<String>,
    focus: bool,

    pane_focused: usize,
    panes: PaneSerializer,

    // options
    automatic_rename: bool,
}

impl TmuxWindow {
    pub fn default() -> TmuxWindow {
        TmuxWindow {
            window_name: String::from("default"),
            layout: None,
            focus: false,
            pane_focused: 0,
            panes: PaneSerializer::create(
                FocusedPane::from_cmd_vec(vec!["clear".to_string(), "bash".to_string()]), 0,
                vec![]),
            automatic_rename: false,
        }
    }
}

impl Serialize for TmuxWindow{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let nfields = 4 + match self.layout {
            None => 0,
            Some(_) => 1,
        };

        let mut state = serializer.serialize_struct("TmuxWindow", nfields)?;
        state.serialize_field("window_name", &self.window_name)?;
        if let Some(_) = self.layout {
            state.serialize_field("layout", &self.layout)?;
        }
        state.serialize_field("focus", &self.focus)?;
        state.serialize_field("panes", &self.panes)?;
        state.end()
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

/* ----------- TMUX SESSION ---------- */
#[derive(Debug)]
pub struct TmuxSession {
    session_name: String,
    start_directory: PathBuf,
    windows: Vec<TmuxWindow>,
}

impl From<&TmuxpSessionCreation> for TmuxSession {
    fn from(c: &TmuxpSessionCreation) -> TmuxSession {
        let mut windows : Vec<TmuxWindow> = c.windows_description.iter()
                .map(|el| el.into()).collect();

        for (n, win) in windows.iter_mut().enumerate(){
            win.focus = n == c.focus;
        }

        TmuxSession {
            session_name: c.session_name.clone(),
            start_directory: c.start_directory.clone(),
            windows,
        }
    }
}

impl Serialize for TmuxSession{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TmuxSession", 3)?;
        state.serialize_field("session_name", &self.session_name)?;
        state.serialize_field("start_directory", &self.start_directory)?;
        state.serialize_field("windows", &self.windows)?;
        state.end()
    }
}
