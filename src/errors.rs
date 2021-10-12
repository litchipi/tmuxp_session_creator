#[derive(Debug)]
pub enum Errcode {
    ArgValidationError(&'static str),
    JsonError(String),
    FileError(String),
    ParsingError(String),
    EnvError(u8),
}

pub fn handle_error(err: Errcode) -> i32 {
    println!("Error occured: {:?}", err);
    return 1;
}

impl From<std::io::Error> for Errcode {
    fn from(e: std::io::Error) -> Errcode { Errcode::FileError(format!("{:?}", e)) }
}

impl From<serde_json::Error> for Errcode {
    fn from(e: serde_json::Error) -> Errcode { Errcode::JsonError(format!("{:?}", e)) }
}

impl From<nom::Err<nom::error::Error<&str>>> for Errcode {
    fn from(e: nom::Err<nom::error::Error<&str>>) -> Errcode { Errcode::ParsingError(format!("{:?}", e)) }
}
