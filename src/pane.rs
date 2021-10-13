use serde::{Serializer, Serialize, Deserialize};
use serde::ser::{SerializeStruct, SerializeSeq};
use serde_json::{Map, Value};

use crate::errors::Errcode;

// List of commands
pub type TmuxPane = String;

#[derive(Serialize, Deserialize, Debug)]
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
                "focus" => pane.focus = val.to_string() == "true",
                "shell_command" => pane.shell_command = val.to_string(),
                _ => return Err(Errcode::JsonError("FocusedPane from Json".to_string())),
            }
        }
        Ok(pane)
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

