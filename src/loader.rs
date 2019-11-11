use super::error_types::XpsError;
use super::types;
use super::ascii;
use super::binary;

pub fn open(filename: &str) -> Result<types::Data, XpsError> {
    if filename.ends_with(".ascii") {
        return Ok({
            match ascii::read_xps_model(&filename.to_string()) {
                Ok(x) => x,
                Err(x) => return Err(x),
            }
        });
    } else if filename.ends_with(".mesh") || filename.ends_with(".xps") {
        return Ok({
            match binary::read_xps_model(&filename.to_string()) {
                Ok(x) => x,
                Err(x) => return Err(x),
            }
        });
    }
    Err(XpsError::FileNotLoaded)
}
