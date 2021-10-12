use serde::{Serializer, Serialize, Deserialize};
use serde::ser::{SerializeStruct, SerializeSeq};

use std::convert::TryFrom;

use nom::IResult;
use nom::bytes::complete::take_until;

use crate::errors::Errcode;
use crate::pane::{PaneSerializer, FocusedPane};

pub type WindowDescription = String;

#[derive(Debug)]
pub struct TmuxWindow {
    window_name: String,
    layout: Option<String>,
    pub focus: bool,

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


const WINDOWDESCR_PARSER_SEP: &'static str = ":";

fn until_sep(s: &str) -> IResult<&str, &str> {
  take_until(WINDOWDESCR_PARSER_SEP)(s)
}

fn parse_windescr(input: &str) -> IResult<&str, TmuxWindow> {
    /*
  let (input, _) = tag("#")(input)?;
  let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

  Ok((input, Color { red, green, blue }))
      */
    Ok((input, TmuxWindow::default()))
}


impl TryFrom<&WindowDescription> for TmuxWindow {
    type Error = Errcode;

    fn try_from(descr: &WindowDescription) -> Result<TmuxWindow, Errcode> {
        Ok(parse_windescr(descr)?.1)
    }
}

impl Into<WindowDescription> for TmuxWindow {
    // TODO Parse WindowDescription
    fn into(self) -> WindowDescription {
        String::from("")
    }
}
