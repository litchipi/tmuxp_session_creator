use serde::{Serializer, Serialize, Deserialize, Deserializer};
use serde::ser::SerializeStruct;
use serde::de::{Visitor, MapAccess};
use serde_json::Value;

use std::fmt;
use std::str::FromStr;
use std::path::PathBuf;
use std::cmp::Ordering;
use std::convert::TryFrom;

use text_io::{read, scan};

use nom::IResult;
use nom::bytes::complete::take_until;


use crate::errors::Errcode;
use crate::pane::{PaneSerializer, FocusedPane};
use crate::serialisation::strval_to_string;

pub type WindowDescription = String;

#[derive(Debug)]
pub struct TmuxWindow {
    pub window_name: String,
    pub layout: Option<String>,
    pub focus: bool,
    pub start_directory: PathBuf,

    pub panes: PaneSerializer,

    // options
    automatic_rename: bool,
}

impl TryFrom<&WindowDescription> for TmuxWindow {
    type Error = Errcode;

    fn try_from(descr: &WindowDescription) -> Result<TmuxWindow, Errcode> {
        let mut win = TmuxWindow::default(PathBuf::from_str("/tmp/").unwrap());
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

impl TmuxWindow {
    pub fn default(start_directory: PathBuf) -> TmuxWindow {
        TmuxWindow {
            window_name: String::from("bash"),
            layout: None,
            focus: false,
            panes: PaneSerializer::create(
                FocusedPane::from_cmd("clear && bash".to_string()), 0,
                vec![]),
            automatic_rename: true,
            start_directory
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

    // Format:      NAME:STARTDIR:AUTORENAME:FOCUSED_PANE:PANE0:<PANE1>:<etc...>
    fn parse_windescr(&mut self, input: &str) -> Result<(), Errcode> {
        let (input, window_name) = until_sep(input)?;
        self.window_name = window_name.to_string();
        
        let (input, startdir) = until_sep(input)?;
        self.start_directory = PathBuf::from_str(startdir)
            .or(Err(
                Errcode::ParsingError("Failed to get path from window description".to_string())
                ))?
            .canonicalize()?;

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




    /*          Window Configuration modifiers          */
    pub fn set_layout(&mut self, layout: &String) -> Result<(), Errcode> {
        let n = get_npane_from_layout(layout)?;
        
        let npanes = self.panes.nb_panes();
        let cmds = match n.cmp(&npanes){
            Ordering::Less => { self.drop_cmds(npanes - n)? },
            Ordering::Equal => { self.panes.get_panes_cmds()? },
            Ordering::Greater => { self.new_cmds(n - npanes)? },
        };
        println!("Commands to set to panes: {:?}", cmds);
        self.panes.set_panes_cmds(&cmds);

        self.layout = Some(layout.clone());
        Ok(())
    }

    pub fn drop_cmds(&mut self, ndrop: usize) -> Result<Vec<String>, Errcode> {
        let mut cmds = self.panes.get_panes_cmds()?;
        for _ in 0..ndrop{
            println!("\n\nCommands in panes: ");
            for (n, c) in cmds.iter().enumerate(){
                println!("\t{}: {}", n, c);
            }
            println!("Enter the number of the command to drop: ");
            let dropped : usize = read!();
            let dropped_cmd = cmds.remove(dropped);
            println!("Dropping command \"{}\"", dropped_cmd);
            //TODO  Allow to undo drops (keep the whole history)
        }
        Ok(cmds)
    }

    pub fn new_cmds(&mut self, nnew: usize) -> Result<Vec<String>, Errcode> {
        let mut cmds = self.panes.get_panes_cmds()?;
        println!("Asking for {} new commands", nnew);
        for _ in 0..nnew{
            println!("\n\nCommands in panes: ");
            for (n, c) in cmds.iter().enumerate(){
                println!("\t{}: {}", n, c);
            }
            println!("Enter a new command: ");
            let cmd: String;
            scan!("{}\n", cmd);
            cmds.push(cmd);
            println!("Added command \"{}\"", cmds.last().unwrap());
            //TODO  Allow to undo adds (keep the whole history)
        }
        Ok(cmds)
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

        state.serialize_field("start_directory", &self.start_directory.to_str())?;
        if self.focus {
            state.serialize_field("focus", &self.focus.to_string())?;
        }

        state.serialize_field("panes", &self.panes)?;
        state.serialize_field("options", &serde_json::json!(
            {
                "automatic-rename": if self.automatic_rename { "on" } else { "off" },
            }
        ))?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for TmuxWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>, {
        deserializer.deserialize_map(TmuxWindowVisitor::new())
    }
}


struct TmuxWindowVisitor{
    _data: u8
}

impl TmuxWindowVisitor{
    pub fn new() -> TmuxWindowVisitor{
        TmuxWindowVisitor{
            _data: 0
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
        let mut window = TmuxWindow::default(PathBuf::from_str("/tmp/").unwrap());

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
        match key.as_ref() {
            "options" => self.load_json_options(value)?,
            "panes" => self.load_json_panes(value)?,

            "window_name" => self.window_name = strval_to_string(&value)?,
            "layout" => self.layout = Some(strval_to_string(&value)?),
            "focus" => self.focus = strval_to_string(&value)? == "true",
            "start_directory" => self.start_directory = PathBuf::from(strval_to_string(&value)?),
            
            // TODO  Warning log here
            _ => println!("Unknown JSON key \"{}\" for Window loading", key),
        }
        Ok(())
    }
}

pub type WindowLayout = str;

pub fn get_npane_from_layout(orig_layout: &WindowLayout) -> Result<usize, Errcode> {
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
