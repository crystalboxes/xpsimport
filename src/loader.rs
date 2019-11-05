use super::error_types::XpsError;
use super::types;
use super::read_ascii;
use super::read_bin;

pub fn open(filename: &str) -> Result<types::Data, XpsError> {
    if filename.ends_with(".ascii") {
        return Ok({
            match read_ascii::read_xps_model(&filename.to_string()) {
                Ok(x) => x,
                Err(x) => return Err(x),
            }
        });
    } else if filename.ends_with(".mesh") || filename.ends_with(".xps") {
        return Ok({
            match read_bin::read_xps_model(&filename.to_string()) {
                Ok(x) => x,
                Err(x) => return Err(x),
            }
        });
    }
    Err(XpsError::FileNotLoaded)
}
