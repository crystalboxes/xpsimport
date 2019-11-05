use super::error_types::XpsError;
use super::types;
use super::read_ascii;
use super::read_bin;
use std::alloc::{dealloc, Layout};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

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

#[repr(C)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
pub struct Vector2 {
    x: f32,
    y: f32,
}
#[repr(C)]
pub struct Color {
    x: u8,
    y: u8,
    z: u8,
    w: u8,
}

#[no_mangle]
pub extern "C" fn xps_load_model(filename: *const c_char) -> Box<types::Data> {
    let c_str = unsafe { CStr::from_ptr(filename) };
    match c_str.to_str() {
        Ok(s) => match open(s) {
            Ok(x) => return Box::new(x),
            Err(x) => {
                let mut data = types::Data::default();
                data.error = x;
                return Box::new(data);
            }
        },
        Err(_) => {}
    }

    Box::new(types::Data::default())
}

#[no_mangle]
pub extern "C" fn xps_get_error(model: Box<types::Data>) -> XpsError {
    model.error
}

#[no_mangle]
pub extern "C" fn xps_delete_model(model: Box<types::Data>) {
    let p = Box::into_raw(model);
    unsafe {
        ptr::drop_in_place(p);
        dealloc(p as *mut u8, Layout::new::<types::Data>());
    }
}

#[no_mangle]
pub extern "C" fn xps_get_mesh_count(model: Box<types::Data>) -> i32 {
    model.meshes.len() as i32
}

#[no_mangle]
pub extern "C" fn xps_get_bone_count(model: Box<types::Data>) -> i32 {
    model.bones.len() as i32
}

#[no_mangle]
pub extern "C" fn xps_get_bone_name(model: Box<types::Data>, index: i32) -> *const u8 {
    model.bones[index as usize].name.as_ptr()
}

#[no_mangle]
pub extern "C" fn xps_get_bone_parent_id(model: Box<types::Data>, index: i32) -> i16 {
    model.bones[index as usize].parent_id
}

#[no_mangle]
pub extern "C" fn xps_get_bone_position(model: Box<types::Data>, index: i32) -> Vector3 {
    Vector3 {
        x: model.bones[index as usize].co.0,
        y: model.bones[index as usize].co.1,
        z: model.bones[index as usize].co.2,
    }
}

#[no_mangle]
pub extern "C" fn xps_get_mesh_name(model: Box<types::Data>, mesh_index: i32) -> *const u8 {
    model.meshes[mesh_index as usize].name.as_ptr()
}

#[no_mangle]
pub extern "C" fn xps_get_uv_layers(model: Box<types::Data>, mesh_index: i32) -> i32 {
    model.meshes[mesh_index as usize].uv_count as i32
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_count(model: Box<types::Data>, mesh_index: i32) -> i32 {
    model.meshes[mesh_index as usize].vertices.len() as i32
}

#[no_mangle]
pub extern "C" fn xps_get_texture_count(model: Box<types::Data>, mesh_index: i32) -> i32 {
    model.meshes[mesh_index as usize].textures.len() as i32
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_id(
    model: Box<types::Data>,
    mesh_index: i32,
    vertex_index: i32,
) -> i32 {
    model.meshes[mesh_index as usize].vertices[vertex_index as usize].id as i32
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_position(
    model: Box<types::Data>,
    mesh_index: i32,
    vertex_index: i32,
) -> Vector3 {
    Vector3 {
        x: model.meshes[mesh_index as usize].vertices[vertex_index as usize]
            .position
            .0,
        y: model.meshes[mesh_index as usize].vertices[vertex_index as usize]
            .position
            .1,
        z: model.meshes[mesh_index as usize].vertices[vertex_index as usize]
            .position
            .2,
    }
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_normal(
    model: Box<types::Data>,
    mesh_index: i32,
    vertex_index: i32,
) -> Vector3 {
    Vector3 {
        x: model.meshes[mesh_index as usize].vertices[vertex_index as usize]
            .normal
            .0,
        y: model.meshes[mesh_index as usize].vertices[vertex_index as usize]
            .normal
            .1,
        z: model.meshes[mesh_index as usize].vertices[vertex_index as usize]
            .normal
            .2,
    }
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_color(
    model: Box<types::Data>,
    mesh_index: i32,
    vertex_index: i32,
) -> Color {
    Color {
        x: model.meshes[mesh_index as usize].vertices[vertex_index as usize]
            .color
            .0,
        y: model.meshes[mesh_index as usize].vertices[vertex_index as usize]
            .color
            .1,
        z: model.meshes[mesh_index as usize].vertices[vertex_index as usize]
            .color
            .2,
        w: model.meshes[mesh_index as usize].vertices[vertex_index as usize]
            .color
            .3,
    }
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_uv(
    model: Box<types::Data>,
    mesh_index: i32,
    vertex_index: i32,
    layer_id: i32,
) -> Vector2 {
    Vector2 {
        x: model.meshes[mesh_index as usize].vertices[vertex_index as usize].uv[layer_id as usize]
            .0,
        y: model.meshes[mesh_index as usize].vertices[vertex_index as usize].uv[layer_id as usize]
            .1,
    }
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_bone_index(
    model: Box<types::Data>,
    mesh_index: i32,
    vertex_index: i32,
    weight_id: i32,
) -> i32 {
    model.meshes[mesh_index as usize].vertices[vertex_index as usize].bone_weights
        [weight_id as usize]
        .id as i32
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_bone_weight(
    model: Box<types::Data>,
    mesh_index: i32,
    vertex_index: i32,
    weight_id: i32,
) -> f32 {
    model.meshes[mesh_index as usize].vertices[vertex_index as usize].bone_weights
        [weight_id as usize]
        .weight
}
