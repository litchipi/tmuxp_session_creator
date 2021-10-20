use serde::{Serializer, Serialize, Deserialize};
use serde::ser::{SerializeStruct, SerializeSeq};
use serde_json::{Map, Value};

use crate::errors::Errcode;
use crate::serialisation::strval_to_string;

// List of commands
pub type TmuxPane = String;

#[derive(Deserialize, Debug)]
pub struct FocusedPane {
    shell_command: String,
    focus: bool
}

impl FocusedPane {
    pub fn from_cmd(cmd: TmuxPane) -> FocusedPane {
        FocusedPane {
            shell_command: cmd,
            focus: true
        }
    }

    pub fn from_json(val: &Map<String, Value>) -> Result<FocusedPane, Errcode> {
        let mut pane = FocusedPane::from_cmd("".to_string());
        for (key, val) in val.iter() {
            match key.as_ref() {
                "focus" => pane.focus = strval_to_string(val)? == "true",
                "shell_command" => pane.shell_command = strval_to_string(val)?,
                _ => return Err(Errcode::JsonError("FocusedPane from Json".to_string())),
            }
        }
        Ok(pane)
    }
}

impl Serialize for FocusedPane{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FocusedPane", 2)?;
        state.serialize_field("shell_command", &self.shell_command)?;
        state.serialize_field("focus", &self.focus.to_string())?;
        state.end()
    }
}

#[derive(Debug)]
pub struct PaneSerializer {
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

    pub fn nb_panes(&self) -> usize {
        1 + self.others.len()
    }

    pub fn set_panes_cmds(&mut self, cmds: &Vec<String>) {
        assert!(cmds.len() >= 1);
        let mut focused_passed = 0;
        self.others.resize(cmds.len()-1, "".to_string());
        
        for (n,c) in cmds.iter().enumerate() {
            if n == self.focused_index{
                self.focused.shell_command = c.clone();
                focused_passed = 1;
            } else { // if (n-focused_passed) < self.others.len() {
                *self.others.get_mut(n-focused_passed).unwrap() = c.clone();
            }
        }
    }

    pub fn set_focus(&mut self, focus: usize) -> Result<(), Errcode>{
        if focus == self.focused_index{
            return Ok(());
        }
        let mut cmds = self.get_panes_cmds()?;
        let focused = cmds.remove(focus);
        self.others = cmds;
        self.focused = FocusedPane::from_cmd(focused);
        Ok(())
    }

    pub fn get_panes_cmds(&mut self) -> Result<Vec<String>, Errcode>{
        let mut allcmds = self.others.clone();
        allcmds.insert(0, self.focused.shell_command.clone());
        Ok(allcmds)
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

