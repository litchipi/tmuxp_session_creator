use serde::{Serializer, Serialize, Deserialize};
use serde::ser::{SerializeStruct, SerializeSeq};

use std::str::FromStr;
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
    pub fn create(name: String, automatic_rename: bool, pane_focused: usize, panes_cmds: Vec<String>) -> TmuxWindow {
        assert!(pane_focused < panes_cmds.len());
        TmuxWindow {
            window_name: name,
            layout: None,
            focus: false,
            panes: PaneSerializer::create(
                FocusedPane::from_cmd(panes_cmds[pane_focused].clone()), pane_focused,
                panes_cmds.iter().enumerate()
                    .filter(|(n, _)| *n != pane_focused)
                    .map(|(_, el)| el.clone()).collect()
                ),
            pane_focused,
            automatic_rename,
        }
    }

    pub fn default() -> TmuxWindow {
        TmuxWindow {
            window_name: String::from("default"),
            layout: None,
            focus: false,
            pane_focused: 0,
            panes: PaneSerializer::create(
                FocusedPane::from_cmd("clear && bash".to_string()), 0,
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
        // TODO     Add options
        state.end()
    }
}


const LAYOUT_DESCR_TAG: &'static str = "#";
const WINDOWDESCR_PARSER_SEP: &'static str = ":";

fn until_sep(s: &str) -> IResult<&str, &str> {
    assert!(s.len() >= WINDOWDESCR_PARSER_SEP.len());
    let inp = if s.starts_with(WINDOWDESCR_PARSER_SEP) {
        s.strip_prefix(WINDOWDESCR_PARSER_SEP).unwrap()
    } else {
        s
    };
    if !inp.contains(WINDOWDESCR_PARSER_SEP) {
        Ok(("", inp))
    } else {
        take_until(WINDOWDESCR_PARSER_SEP)(inp)
    }
}

// Format:      NAME:AUTORENAME:FOCUSED_PANE:PANE0:<PANE1>:<etc...>
fn parse_windescr(input: &str) -> Result<(&str, TmuxWindow), Errcode> {
    let (input, window_name) = until_sep(input)?;
    
    let (input, autorename_str) = until_sep(input)?;
    let autorename = autorename_str == "on";
    
    let (input, focused_str) = until_sep(input)?;
    let focused = usize::from_str(focused_str)?;
    
    let mut panes_cmd = vec![];
    let mut input = input;
    while input.len() > 0 {
        let (new_input, panecmd) = until_sep(input)?;
        if new_input.len() == 0 {
            if panecmd.contains(LAYOUT_DESCR_TAG){
                let (layout, cmd) = take_until(LAYOUT_DESCR_TAG)(panecmd)?;
                let layout = layout.to_string();
                println!("Layout: {}", layout);
                println!("Cmd: {}", cmd);
            }
        }
        panes_cmd.push(panecmd.to_string());
        input = new_input;
    }

    Ok((input, TmuxWindow::create(window_name.to_string(), autorename, focused, panes_cmd)))
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

pub type WindowLayout = str;

pub fn get_npane_from_layout(orig_layout: &WindowLayout) -> Result<usize, Errcode> {
    //take_until(WINDOWDESCR_PARSER_SEP)(inp)
    let mut layout = orig_layout.clone();
    let mut npane = 0;
    let mut nel = 0;
    let mut alone = true;
    while layout.len() > 0{
        let (new_layout, content) = if layout.contains(","){
            take_until(",")(layout)?
        } else {
            ("", layout.clone())
        };
        nel += 1;
        
        if content.contains("{") || content.contains("["){
            if alone{
                assert_eq!(nel, 4);
            } else {
                assert_eq!(nel, 3);
            }
            nel = 1;
            alone = false;
        } else if nel == 5 && !alone {
            nel = 1;
            npane += 1;
        } else if content.contains("}") || content.contains("]"){
            assert_eq!(nel, 4);
            npane += 1;
            nel = 0;
        }

        if new_layout.len() == 0{
            if alone{
                assert_eq!(nel, 5);
                npane += 1;
            }
            break;
        }
        layout = &new_layout[1..];
    }
    Ok(npane)
}

#[test]
fn test_get_npane_from_layout(){
    let test_points = vec![
        ("5be4,211x62,0,0,15", 1),
        ("f93e,211x62,0,0[211x31,0,0,15,211x30,0,32,24]", 2),
        ("6669,211x62,0,0{105x62,0,0,15,105x62,106,0,25}", 2),
        ("dcbe,211x62,0,0{105x62,0,0[105x31,0,0,15,105x30,0,32,26],105x62,106,0,25}", 3),
        ("7303,211x62,0,0{105x62,0,0[105x31,0,0,15,105x30,0,32,26],105x62,106,0[105x31,106,0,25,105x30,106,32,27]}", 4),
        ("1bd3,211x62,0,0{105x62,0,0[105x31,0,0,15,105x30,0,32,26],105x62,106,0[105x31,106,0,25,105x15,106,32,27,105x14,106,48{52x14,106,48,28,26x14,159,48,29,25x14,186,48[25x7,186,48,30,25x6,186,56,31]}]}", 8)
    ];

    for (ntest, (layout, exp)) in test_points.iter().enumerate(){
        let got = get_npane_from_layout(layout).expect("Layout parsing raised error");
        println!("Layout {} expect {} panes, got {}", ntest, exp, got);
        assert_eq!(got, *exp);
    }
}
