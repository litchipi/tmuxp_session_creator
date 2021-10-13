use serde::{Serializer, Serialize, Deserialize, Deserializer};
use serde::ser::{SerializeStruct, SerializeSeq};
use serde::de::{Visitor, MapAccess};
use serde_json::Value;

use std::fmt;
use std::str::FromStr;
use std::path::PathBuf;
use std::convert::TryFrom;
use std::collections::HashMap;

use nom::IResult;
use nom::bytes::complete::take_until;

use crate::errors::Errcode;
use crate::pane::{PaneSerializer, FocusedPane};

pub type WindowDescription = String;

#[derive(Debug)]
pub struct TmuxWindow {
    window_name: String,
    pub layout: Option<String>,
    pub focus: bool,
    start_directory: PathBuf,

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
                FocusedPane::from_cmd("clear && bash".to_string()), 0,
                vec![]),
            automatic_rename: false,
            start_directory: PathBuf::from("/tmp/")
        }
    }

    fn load_json_options(&mut self, value: Value) -> Result<(), Errcode> {
        if let Value::Object(opt) = value {
            for (key, val) in opt.iter(){
                match key.as_ref() {
                    "automatic-rename" => self.automatic_rename = val.to_string() == "on",
                    _ => return Err(Errcode::OptionNotFound(val.to_string())),
                }
            }
        } else {
            return Err(Errcode::JsonError("Json loading options".to_string()))
        }
        Ok(())
    }

    fn load_json_panes(&mut self, value: Value) -> Result<(), Errcode> {
        let mut nfoc = 0;
        let mut panes_cmd = vec![];
        let mut foc: Option<FocusedPane> = None;

        if let Value::Array(panes) = value {
            for (nb, pane) in panes.iter().enumerate() {
                match pane {
                    Value::String(cmd) => panes_cmd.push(cmd.clone()),
                    Value::Object(val) => {
                        nfoc = nb;
                        foc = Some(FocusedPane::from_json(val)?)
                    },
                    _ => return Err(Errcode::JsonError("Json loading panes inner values".to_string())),

                }
            }
        } else {
            return Err(Errcode::JsonError("Json loading panes".to_string()))
        }
        
        if let Some(f) = foc {
            self.panes = PaneSerializer::create(f, nfoc, panes_cmd);
        } else {
            return Err(Errcode::JsonError("Missing data for pane loading from Json".to_string()));
        }

        Ok(())
    }

    // Format:      NAME:AUTORENAME:FOCUSED_PANE:PANE0:<PANE1>:<etc...>
    fn parse_windescr(&mut self, input: &str) -> Result<(), Errcode> {
        let (input, window_name) = until_sep(input)?;
        self.window_name = window_name.to_string();
        
        let (input, autorename_str) = until_sep(input)?;
        self.automatic_rename = autorename_str == "on";
        
        let (input, focused_str) = until_sep(input)?;
        let pane_focused = usize::from_str(focused_str)?;
        
        let mut panes_cmd = vec![];
        let mut input = input;
        while input.len() > 0 {
            let (new_input, panecmd) = until_sep(input)?;
            let cmd = if new_input.len() == 0 {
                if panecmd.contains(LAYOUT_DESCR_TAG){
                    let (layout_str, lastcmd) = take_until(LAYOUT_DESCR_TAG)(panecmd)?;
                    self.layout = Some(layout_str.to_string());
                    lastcmd
                } else {
                    panecmd
                }
            } else {
                panecmd
            };
            panes_cmd.push(cmd.to_string());
            input = new_input;
        }

        self.panes = PaneSerializer::create(
            FocusedPane::from_cmd(panes_cmd[pane_focused].clone()), pane_focused,
            panes_cmd.iter().enumerate()
            .filter(|(n, _)| *n != pane_focused)
            .map(|(_, el)| el.clone()).collect()
        );

        Ok(())
    }
}

impl TryFrom<&WindowDescription> for TmuxWindow {
    type Error = Errcode;

    fn try_from(descr: &WindowDescription) -> Result<TmuxWindow, Errcode> {
        let mut win = TmuxWindow::default();
        win.parse_windescr(descr)?;
        Ok(win)
    }
}

impl Into<WindowDescription> for TmuxWindow {
    // TODO Parse WindowDescription
    fn into(self) -> WindowDescription {
        String::from("")
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







/* ------------- Serde Serialization ------------- */
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

impl<'de> Deserialize<'de> for TmuxWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of MyMap.
        deserializer.deserialize_map(TmuxWindowVisitor::new())
    }
}


struct TmuxWindowVisitor{
    data: u8
}

impl TmuxWindowVisitor{
    pub fn new() -> TmuxWindowVisitor{
        TmuxWindowVisitor{
            data: 0
        }
    }
}

impl<'de> Visitor<'de> for TmuxWindowVisitor {
    type Value = TmuxWindow;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map containing all the informations for a tmuxp session")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut window = TmuxWindow::default();

        while let Some((key, value)) = access.next_entry()? {
            if let Err(e) = window.load_entry(key, value){
                println!("Error while deserializing TmuxWindow: {:?}", e); 
            }
        }

        Ok(window)
    }
}

trait TmuxWindowBuilder<V>{
    fn load_entry(&mut self, key: String, value: V) -> Result<(), Errcode>;
}

impl TmuxWindowBuilder<Value> for TmuxWindow{
    fn load_entry(&mut self, key: String, value: Value) -> Result<(), Errcode> {
        println!("Load entry {}: {}", key, value.to_string());
        match key.as_ref() {
            "options" => self.load_json_options(value)?,

            // TODO Simplify here, generalize this solution for each Value::String to String
            // conversion to avoid the \" poison
            "window_name" => self.window_name = value.as_str().ok_or(Errcode::JsonError("Window name".to_string()))?.to_string(),
            "layout" => self.layout = Some(value.to_string()),
            "panes" => self.load_json_panes(value)?,
            "focus" => self.focus = value.to_string() == "true",
            "start_directory" => self.start_directory = PathBuf::from(value.to_string()),
            _ => println!("Unknown JSON key \"{}\" for Window loading", key),
                    // TODO  Warning log here
        }
        Ok(())
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
