use std::path::PathBuf;
use std::convert::TryFrom;

use serde::{Serializer, Serialize, Deserialize};
use serde::ser::{SerializeStruct, SerializeSeq};
use serde_json::{to_string, from_str};

use crate::errors::Errcode;
use crate::create::TmuxpSessionCreation;

use crate::window::{WindowDescription, TmuxWindow};
use crate::pane::TmuxPane;

#[derive(Debug)]
pub struct TmuxSession {
    session_name: String,
    start_directory: PathBuf,
    windows: Vec<TmuxWindow>,
}

impl TryFrom<&TmuxpSessionCreation> for TmuxSession {
    type Error = Errcode;

    fn try_from(c: &TmuxpSessionCreation) -> Result<TmuxSession, Errcode> {
        let mut windows : Vec<TmuxWindow> = {
            if !c.default {
                let mut res = vec![];
                for windescr in c.windows_description.iter(){
                    res.push(TmuxWindow::try_from(windescr)?);
                }
                res
            } else {
                vec![TmuxWindow::default()]
            }
        };

        for (n, win) in windows.iter_mut().enumerate(){
            win.focus = n == c.focus;
        }

        Ok(TmuxSession {
            session_name: c.session_name.clone(),
            start_directory: c.start_directory.clone(),
            windows,
        })
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
