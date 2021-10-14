use std::path::PathBuf;
use std::convert::TryFrom;
use std::fs;
use std::io::prelude::*;

use serde::{Serializer, Serialize, Deserialize};
use serde::ser::{SerializeStruct, SerializeSeq};
use serde_json::{to_string_pretty, from_str};

use dirs::home_dir;

use crate::errors::Errcode;
use crate::create::TmuxpSessionCreation;

use crate::window::{WindowDescription, TmuxWindow};
use crate::pane::TmuxPane;

#[derive(Debug, Deserialize)]
pub struct TmuxSession {
    session_name: String,
    start_directory: PathBuf,
    pub windows: Vec<TmuxWindow>,
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

const TMUXP_DIR: &'static str = ".tmuxp/";

impl TmuxSession {
    pub fn get_session_fname(name: &String) -> Result<PathBuf, Errcode> {
        let mut output_fname = PathBuf::from(home_dir().ok_or(Errcode::EnvError(0))?);
        output_fname.push(TMUXP_DIR);
        output_fname.push(name.replace(" ", "_"));
        output_fname.set_extension("json");
        Ok(output_fname)
    }

    pub fn load(name: &String) -> Result<TmuxSession, Errcode> {
        let fname = Self::get_session_fname(name)?;
        println!("Reading session from file {} ...", fname.to_str().unwrap());
        let jsonses = fs::read_to_string(fname)?;
        let res = serde_json::from_str(&jsonses)?;
        Ok(res)
    }

    pub fn write_to_file(&self) -> Result<(), Errcode> {
        let output_fname = Self::get_session_fname(&self.session_name)?;
        println!("Writting configuration in {}", output_fname.to_str().unwrap());
        let mut file = fs::File::create(output_fname)?;
        file.write_all(to_string_pretty(self)?.as_bytes())?;
        Ok(())
    }

    pub fn dump(&self) -> Result<(), Errcode> {
        println!();
        println!("{}", to_string_pretty(self)?);
        Ok(())
    }

    pub fn apply_layout(&mut self, window_index: usize, layout: &String) -> Result<(), Errcode> {
        let win: &mut TmuxWindow = self.get_window_ref(window_index)?;
        win.set_layout(layout)
    }

    pub fn get_window_ref(&mut self, window_index: usize) -> Result<&mut TmuxWindow, Errcode> {
        let winlen = self.windows.len();
        self.windows.get_mut(window_index)
            .ok_or(Errcode::WindowNotFound(window_index, winlen))
    }
}
