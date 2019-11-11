use super::ascii;
use super::binary;
use super::bone_naming;
use super::bone_naming::{BoneNaming, Converter};
use super::error_types::XpsError;
use super::types;
use super::types::ImportParameters;

pub fn open(
    filename: &str,
    bone_naming: super::bone_naming::BoneNaming,
    flip_uv: bool,
    reverse_winding: bool,
) -> Result<types::Data, XpsError> {
    let import_parameters = ImportParameters {
        flip_uv: flip_uv,
        reverse_winding: reverse_winding,
    };
    let mut model = {
        if filename.ends_with(".ascii") {
            {
                match ascii::read_xps_model(&filename.to_string(), import_parameters) {
                    Ok(x) => x,
                    Err(x) => return Err(x),
                }
            }
        } else if filename.ends_with(".mesh") || filename.ends_with(".xps") {
            {
                match binary::read_xps_model(&filename.to_string(), import_parameters) {
                    Ok(x) => x,
                    Err(x) => return Err(x),
                }
            }
        } else {
            return Err(XpsError::FileNotLoaded);
        }
    };

    match bone_naming {
        BoneNaming::Mecanim => {
            let conv = Converter::new();
            for x in 0..model.bones.len() {
                if let Some(bone_type) = conv
                    .bone_dictionary
                    .get(model.bones[x].name.to_str().unwrap())
                {
                    if let Ok(new_name) = std::ffi::CString::new(String::from(
                        bone_naming::bone_type_to_mecanim_name(bone_type.clone()),
                    )) {
                        model.bones[x].name = new_name;
                    }
                }
            }
        }
        _ => {}
    }
    Ok(model)
}
