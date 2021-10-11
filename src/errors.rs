#[derive(Debug)]
pub enum Errcode {
    ArgValidationError(&'static str)
}

pub fn handle_error(err: Errcode) -> i32 {
    println!("Error occured: {:?}", err);
    return 1;
}
