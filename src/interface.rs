use super::error_types::XpsError;
use super::loader::open;
use super::types;
use std::alloc::{dealloc, Layout};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

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
pub extern "C" fn xps_load_model(
    filename: *const c_char,
    bone_naming_format: super::bone_naming::BoneNaming,
    flip_uv: i32,
    reverse_winding: i32,
) -> Box<types::Data> {
    let c_str = unsafe { CStr::from_ptr(filename) };
    let boxed = match c_str.to_str() {
        Ok(s) => match open(
            s,
            bone_naming_format,
            if flip_uv == 0 { false } else { true },
            if reverse_winding == 0 { false } else { true },
        ) {
            Ok(x) => Box::new(x),
            Err(x) => {
                let mut data = types::Data::default();
                data.error = x;
                Box::new(data)
            }
        },
        Err(_) => Box::new(types::Data::default()),
    };
    return boxed;
}

#[no_mangle]
pub extern "C" fn xps_get_error(model: *mut types::Data) -> XpsError {
    let mut _model = unsafe { &mut *model };
    _model.error
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
pub extern "C" fn xps_get_mesh_count(model: *mut types::Data) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.meshes.len() as i32
}

#[no_mangle]
pub extern "C" fn xps_get_bone_count(model: *mut types::Data) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.bones.len() as i32
}

#[no_mangle]
pub extern "C" fn xps_get_bone_name(model: *mut types::Data, index: i32) -> *const c_char {
    let mut _model = unsafe { &mut *model };
    _model.bones[index as usize].name.as_ptr()
}

#[no_mangle]
pub extern "C" fn xps_get_bone_parent_id(model: *mut types::Data, index: i32) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.bones[index as usize].parent_id as i32
}

#[no_mangle]
pub extern "C" fn xps_get_bone_position(model: *mut types::Data, index: i32) -> Vector3 {
    let mut _model = unsafe { &mut *model };
    Vector3 {
        x: _model.bones[index as usize].co[0],
        y: _model.bones[index as usize].co[1],
        z: _model.bones[index as usize].co[2],
    }
}

#[no_mangle]
pub extern "C" fn xps_get_mesh_name(model: *mut types::Data, mesh_index: i32) -> *const c_char {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].name.as_ptr()
}

#[no_mangle]
pub extern "C" fn xps_get_uv_layers(model: *mut types::Data, mesh_index: i32) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].uv_count as i32
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_count(model: *mut types::Data, mesh_index: i32) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].vertices.len() as i32
}

#[no_mangle]
pub extern "C" fn xps_get_texture_count(model: *mut types::Data, mesh_index: i32) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].textures.len() as i32
}

#[no_mangle]
pub extern "C" fn xps_get_texture_id(
    model: *mut types::Data,
    mesh_index: i32,
    texture_index: i32,
) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].textures[texture_index as usize].id as i32
}

#[no_mangle]
pub extern "C" fn xps_get_texture_filename(
    model: *mut types::Data,
    mesh_index: i32,
    texture_index: i32,
) -> *const c_char {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].textures[texture_index as usize]
        .file
        .as_ptr()
}

#[no_mangle]
pub extern "C" fn xps_get_texture_uv_layer(
    model: *mut types::Data,
    mesh_index: i32,
    texture_index: i32,
) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].textures[texture_index as usize].uv_layer as i32
}

#[no_mangle]
pub extern "C" fn xps_get_mesh_index_count(model: *mut types::Data, mesh_index: i32) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].faces.len() as i32
}

#[no_mangle]
pub extern "C" fn xps_get_mesh_index(
    model: *mut types::Data,
    mesh_index: i32,
    index_num: i32,
) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].faces[index_num as usize] as i32
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_position(
    model: *mut types::Data,
    mesh_index: i32,
    vertex_index: i32,
) -> Vector3 {
    let mut _model = unsafe { &mut *model };
    Vector3 {
        x: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].position[0],
        y: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].position[1],
        z: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].position[2],
    }
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_normal(
    model: *mut types::Data,
    mesh_index: i32,
    vertex_index: i32,
) -> Vector3 {
    let mut _model = unsafe { &mut *model };
    Vector3 {
        x: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].normal[0],
        y: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].normal[1],
        z: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].normal[2],
    }
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_color(
    model: *mut types::Data,
    mesh_index: i32,
    vertex_index: i32,
) -> Color {
    let mut _model = unsafe { &mut *model };
    Color {
        x: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].color[0],
        y: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].color[1],
        z: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].color[2],
        w: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].color[3],
    }
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_uv(
    model: *mut types::Data,
    mesh_index: i32,
    vertex_index: i32,
    layer_id: i32,
) -> Vector2 {
    let mut _model = unsafe { &mut *model };
    Vector2 {
        x: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].uv[layer_id as usize]
            [0],
        y: _model.meshes[mesh_index as usize].vertices[vertex_index as usize].uv[layer_id as usize]
            [1],
    }
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_bone_index(
    model: *mut types::Data,
    mesh_index: i32,
    vertex_index: i32,
    weight_id: i32,
) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].vertices[vertex_index as usize].bone_weights
        [weight_id as usize]
        .id as i32
}

#[no_mangle]
pub extern "C" fn xps_get_vertex_bone_weight(
    model: *mut types::Data,
    mesh_index: i32,
    vertex_index: i32,
    weight_id: i32,
) -> f32 {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].vertices[vertex_index as usize].bone_weights
        [weight_id as usize]
        .weight
}

#[no_mangle]
fn xps_get_render_group_alpha(model: *mut types::Data, mesh_index: i32) -> i32 {
    let mut _model = unsafe { &mut *model };
    if _model.meshes[mesh_index as usize].render_group.alpha {
        1
    } else {
        0
    }
}

#[no_mangle]
fn xps_get_render_group_posable(model: *mut types::Data, mesh_index: i32) -> i32 {
    let mut _model = unsafe { &mut *model };
    if _model.meshes[mesh_index as usize].render_group.posable {
        1
    } else {
        0
    }
}

#[no_mangle]
fn xps_get_render_group_bump1_rep(model: *mut types::Data, mesh_index: i32) -> i32 {
    let mut _model = unsafe { &mut *model };
    if _model.meshes[mesh_index as usize].render_group.bump1_rep {
        1
    } else {
        0
    }
}

#[no_mangle]
fn xps_get_render_group_bump2_rep(model: *mut types::Data, mesh_index: i32) -> i32 {
    let mut _model = unsafe { &mut *model };
    if _model.meshes[mesh_index as usize].render_group.bump2_rep {
        1
    } else {
        0
    }
}

#[no_mangle]
fn xps_get_render_group_spec1_rep(model: *mut types::Data, mesh_index: i32) -> i32 {
    let mut _model = unsafe { &mut *model };
    if _model.meshes[mesh_index as usize].render_group.spec1_rep {
        1
    } else {
        0
    }
}

#[no_mangle]
fn xps_get_render_group_texture_count(model: *mut types::Data, mesh_index: i32) -> i32 {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize].render_group.tex_count
}

#[no_mangle]
fn xps_get_render_group_texture_type(
    model: *mut types::Data,
    mesh_index: i32,
    texture_type_index: i32,
) -> *const c_char {
    let mut _model = unsafe { &mut *model };
    _model.meshes[mesh_index as usize]
        .render_group
        .texture_types[texture_type_index as usize]
        .as_ptr()
}
