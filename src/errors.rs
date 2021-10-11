#[derive(Debug)]
pub enum Errcode {
    ArgValidationError(&'static str),
    JsonError(String),
    FileError(String),
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
