use super::error_types::XpsError;
use super::file_input::FileStream;
use std::collections::HashMap;
use std::ffi::CString;
use std::path::Path;

use super::constants;

use super::types::{Bone, BonePose, BoneWeight, Data, Header, Mesh, Texture, Vertex};

pub fn split_values(line: &String) -> Vec<String> {
    line.replace("#", " ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

pub fn ignore_comment(line: &String) -> String {
    if let Some(x) = line.replace("#", " ").split_whitespace().next() {
        x.to_string()
    } else {
        "".to_string()
    }
}

pub fn ignore_string_comment(line: &String) -> String {
    if let Some(x) = line.split("#").next() {
        x.to_string()
    } else {
        "".to_string()
    }
}
pub fn get_float(value: &String) -> f32 {
    if let Ok(x) = value.parse() {
        x
    } else {
        std::f32::NAN
    }
}

pub fn get_int(value: &String) -> i32 {
    if let Ok(x) = value.parse() {
        x
    } else {
        0
    }
}

pub fn read_uv_vertex(file: &mut FileStream) -> [f32; 2] {
    let line = file.read_line_trim();
    let values = split_values(&line);
    [get_float(&values[0]), {
        let v = get_float(&values[1]);
        if constants::FLIP_UV {
            1_f32 - v
        } else {
            v
        }
    }]
}

pub fn read_xyz(file: &mut FileStream) -> [f32; 3] {
    let line = file.read_line_trim();
    let values = split_values(&line);
    [
        get_float(&values[0]),
        get_float(&values[1]),
        get_float(&values[2]),
    ]
}

pub fn fill_array(array: &mut Vec<String>, min_length: usize, value: String) {
    array.resize(min_length, value);
}

fn read_values(file: &mut FileStream, _: usize) -> Vec<String> {
    let line = file.read_line_trim();
    let mut values = split_values(&line);
    fill_array(&mut values, 4, "0".to_string());
    values
}

pub fn read_bone_weight(file: &mut FileStream) -> [f32; 4] {
    let values = read_values(file, 4);
    [
        get_float(&values[0]),
        get_float(&values[1]),
        get_float(&values[2]),
        get_float(&values[3]),
    ]
}

pub fn read_bone_ids(file: &mut FileStream) -> [i32; 4] {
    let values = read_values(file, 4);
    [
        get_int(&values[0]),
        get_int(&values[1]),
        get_int(&values[2]),
        get_int(&values[3]),
    ]
}

pub fn read_int4(file: &mut FileStream) -> (i32, i32, i32, i32) {
    let values = read_values(file, 4);
    (
        get_int(&values[0]),
        get_int(&values[1]),
        get_int(&values[2]),
        get_int(&values[3]),
    )
}

pub fn read_face_indices(file: &mut FileStream) -> (i32, i32, i32) {
    let line = file.read_line_trim();
    let values = split_values(&line);
    (
        get_int(&values[0]),
        get_int(&values[1]),
        get_int(&values[2]),
    )
}

pub fn read_bones(file: &mut FileStream) -> Vec<Bone> {
    let mut bones = vec![];
    let bone_count = file.read_int();
    for bone_id in 0..bone_count {
        let name = file.read_string();
        let parent = file.read_int();
        bones.push(Bone {
            id: bone_id as i16,
            name: CString::new(name).unwrap_or(CString::new("").unwrap()),
            co: read_xyz(file),
            parent_id: parent as i16,
        })
    }
    bones
}

pub fn read_meshes(file: &mut FileStream, has_bones: bool) -> Result<Vec<Mesh>, XpsError> {
    let mut meshes = vec![];
    let mesh_count = file.read_int();
    for _ in 0..mesh_count {
        let mut mesh_name = file.read_string();
        if mesh_name.len() == 0 {
            mesh_name = "xxx".to_string();
        }
        let uv_layer_count = file.read_int() as usize;
        let mut textures = vec![];
        let texture_count = file.read_int();
        for tex_id in 0..texture_count {
            let texture_file: String = {
                let filename = file.read_string();
                match Path::new(&filename).parent() {
                    Some(x) => {
                        if let Some(y) = x.to_str() {
                            filename.replace(y, "").replace("\\", "/").replace("/", "")
                        } else {
                            return Err(XpsError::PathToStr);
                        }
                    }
                    None => return Err(XpsError::PathGetParent),
                }
            };
            let uv_layer_id = file.read_int();
            textures.push(Texture {
                id: tex_id as u16,
                file: CString::new(texture_file).unwrap_or(CString::new("").unwrap()),
                uv_layer: uv_layer_id as u16,
            });
        }

        let vertex_count = file.read_int() as usize;

        let mut vertex = vec![Vertex::default(); vertex_count];
        for vtx in 0..vertex_count {
            vertex[vtx].position = read_xyz(file);
            vertex[vtx].normal = read_xyz(file);
            let vertex_color = read_int4(file);
            vertex[vtx].color = [
                vertex_color.0 as u8,
                vertex_color.1 as u8,
                vertex_color.2 as u8,
                vertex_color.3 as u8,
            ];
            for x in 0..uv_layer_count {
                vertex[vtx].uv[x] = read_uv_vertex(file);
            }

            if has_bones {
                let bone_idx = read_bone_ids(file);
                let bone_weight = read_bone_weight(file);
                for idx in 0..4 {
                    vertex[vtx].bone_weights[idx] = BoneWeight {
                        id: bone_idx[idx] as i16,
                        weight: bone_weight[idx],
                    };
                }
            }
        }
        let mut faces = vec![];
        let tri_count = file.read_int();
        for _ in 0..tri_count {
            let tri_idx = read_face_indices(file);
            faces.push(tri_idx.0 as u32);
            if constants::REVERSE_WINDING {
                faces.push(tri_idx.2 as u32);
                faces.push(tri_idx.1 as u32);
            } else {
                faces.push(tri_idx.1 as u32);
                faces.push(tri_idx.2 as u32);
            }
        }

        meshes.push(Mesh {
            name: CString::new(mesh_name).unwrap_or(CString::new("").unwrap()),
            textures: textures,
            vertices: vertex,
            faces: faces,
            uv_count: uv_layer_count as u16,
        });
    }
    Ok(meshes)
}

pub fn pose_data(string: &String) -> HashMap<String, BonePose> {
    let mut pose_data = HashMap::new();
    for bone_pose in string.split("\n") {
        if bone_pose.len() != 0 {
            let pose: Vec<String> = bone_pose.split(":").map(|x| x.to_string()).collect();
            let bone_name: String = pose[0].clone();
            let mut data_list = pose[1].split_whitespace().map(|x| x.to_string()).collect();
            fill_array(&mut data_list, 9, "1".to_string());
            pose_data.insert(
                bone_name.clone(),
                BonePose {
                    name: bone_name,
                    coordinate_delta: [
                        get_float(&data_list[0]),
                        get_float(&data_list[1]),
                        get_float(&data_list[2]),
                    ],
                    rotation_delta: [
                        get_float(&data_list[3]),
                        get_float(&data_list[4]),
                        get_float(&data_list[5]),
                    ],
                    scale: [
                        get_float(&data_list[6]),
                        get_float(&data_list[7]),
                        get_float(&data_list[8]),
                    ],
                },
            );
        }
    }
    pose_data
}

fn read_io_stream(filename: &String) -> Result<FileStream, String> {
    if let Some(x) = FileStream::new(filename, true) {
        Ok(x)
    } else {
        Err(String::from("File not found"))
    }
}

pub fn read_xps_model(filename: &String) -> Result<Data, XpsError> {
    if let Ok(mut io_stream) = read_io_stream(filename) {
        let bones = read_bones(&mut io_stream);
        if let Ok(meshes) = read_meshes(&mut io_stream, bones.len() > 0) {
            return Ok(Data {
                header: Header::default(),
                bones: bones,
                meshes: meshes,
                error: XpsError::None,
            });
        }
        return Err(XpsError::MeshReadAscii);
    } else {
        Err(XpsError::StreamNotOpened)
    }
}

// pub fn read_pose_file(file: &mut FileStream) -> String {
//   file.read_string()
// }

// pub fn bone_dict_data(string: &String) -> (HashMap<String, String>, HashMap<String, String>) {
//   let mut bone_dict_rename = HashMap::new();
//   let mut bone_dict_restore = HashMap::new();
//   for bone_pose in string.split("\n") {
//     if bone_pose.len() != 0 {
//       let pose: Vec<String> = bone_pose.split(';').map(|x| x.to_string()).collect();
//       let old_name = pose[0].clone();
//       let new_name = pose[1].clone();
//       bone_dict_rename.insert(old_name.clone(), new_name.clone());
//       bone_dict_restore.insert(new_name, old_name);
//     }
//   }
//   (bone_dict_rename, bone_dict_restore)
// }

// pub fn read_xps_pose(filename: &String) -> HashMap<String, BonePose> {
//   if let Ok(mut io_stream) = read_io_stream(filename) {
//     pose_data(&read_pose_file(&mut io_stream))
//   } else {
//     HashMap::new()
//   }
// }

// pub fn read_bone_dict(filename: &String) -> (HashMap<String, String>, HashMap<String, String>) {
//   if let Ok(mut io_stream) = read_io_stream(filename) {
//     bone_dict_data(&read_pose_file(&mut io_stream))
//   } else {
//     (HashMap::new(), HashMap::new())
//   }
// }
