use serde_json::Value;

use crate::errors::Errcode;

pub fn strval_to_string(val: &Value) -> Result<String, Errcode> {
    Ok(val.as_str()
        .ok_or(Errcode::JsonError("Attempt to convert non-string Value to String".to_string()))?
        .to_string().clone()
    )
}
