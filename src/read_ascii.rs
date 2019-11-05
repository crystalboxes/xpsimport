use super::ascii_ops;
use super::error_types::XpsError;
use super::file_stream::FileStream;
use std::collections::HashMap;
use std::path::Path;

use super::types::{Bone, BonePose, BoneWeight, Data, Header, Mesh, Texture, Vertex};

pub fn read_uv_vertex(file: &mut FileStream) -> (f32, f32) {
  let line = ascii_ops::readline(file);
  let values = ascii_ops::split_values(&line);
  (
    ascii_ops::get_float(&values[0]),
    ascii_ops::get_float(&values[1]),
  )
}

pub fn read_xyz(file: &mut FileStream) -> (f32, f32, f32) {
  let line = ascii_ops::readline(file);
  let values = ascii_ops::split_values(&line);
  (
    ascii_ops::get_float(&values[0]),
    ascii_ops::get_float(&values[1]),
    ascii_ops::get_float(&values[2]),
  )
}

pub fn fill_array(array: &mut Vec<String>, min_length: usize, value: String) {
  array.resize(min_length, value);
}

fn read_values(file: &mut FileStream, _: usize) -> Vec<String> {
  let line = ascii_ops::readline(file);
  let mut values = ascii_ops::split_values(&line);
  fill_array(&mut values, 4, "0".to_string());
  values
}

pub fn read_float4(file: &mut FileStream) -> (f32, f32, f32, f32) {
  let values = read_values(file, 4);
  (
    (ascii_ops::get_float(&values[0])),
    (ascii_ops::get_float(&values[1])),
    (ascii_ops::get_float(&values[2])),
    (ascii_ops::get_float(&values[3])),
  )
}

pub fn read_bone_weight(file: &mut FileStream) -> [f32; 4] {
  let values = read_values(file, 4);
  [
    ascii_ops::get_float(&values[0]),
    ascii_ops::get_float(&values[1]),
    ascii_ops::get_float(&values[2]),
    ascii_ops::get_float(&values[3]),
  ]
}

pub fn read_bone_ids(file: &mut FileStream) -> [i32; 4] {
  let values = read_values(file, 4);
  [
    ascii_ops::get_int(&values[0]),
    ascii_ops::get_int(&values[1]),
    ascii_ops::get_int(&values[2]),
    ascii_ops::get_int(&values[3]),
  ]
}

pub fn read_int4(file: &mut FileStream) -> (i32, i32, i32, i32) {
  let values = read_values(file, 4);
  (
    ascii_ops::get_int(&values[0]),
    ascii_ops::get_int(&values[1]),
    ascii_ops::get_int(&values[2]),
    ascii_ops::get_int(&values[3]),
  )
}

pub fn read_face_indices(file: &mut FileStream) -> (i32, i32, i32) {
  let line = ascii_ops::readline(file);
  let values = ascii_ops::split_values(&line);
  (
    ascii_ops::get_int(&values[0]),
    ascii_ops::get_int(&values[1]),
    ascii_ops::get_int(&values[2]),
  )
}

pub fn read_bones(file: &mut FileStream) -> Vec<Bone> {
  let mut bones = vec![];
  let bone_count = ascii_ops::read_int(file);
  for bone_id in 0..bone_count {
    bones.push(Bone {
      id: bone_id as i16,
      name: ascii_ops::read_string(file),
      co: read_xyz(file),
      parent_id: ascii_ops::read_int(file) as i16,
    })
  }
  bones
}

pub fn read_meshes(file: &mut FileStream, has_bones: bool) -> Result<Vec<Mesh>, XpsError> {
  let mut meshes = vec![];
  let mesh_count = ascii_ops::read_int(file);
  for _ in 0..mesh_count {
    let mut mesh_name = ascii_ops::read_string(file);
    if mesh_name.len() == 0 {
      mesh_name = "xxx".to_string();
    }
    let uv_layer_count = ascii_ops::read_int(file);
    let mut textures = vec![];
    let texture_count = ascii_ops::read_int(file);
    for tex_id in 0..texture_count {
      let texture_file: String = {
        match Path::new(&ascii_ops::read_string(file)).parent() {
          Some(x) => {
            if let Some(y) = x.to_str() {
              y.to_string()
            } else {
              return Err(XpsError::PathToStr);
            }
          }
          None => return Err(XpsError::PathGetParent),
        }
      };
      let uv_layer_id = ascii_ops::read_int(file);
      textures.push(Texture {
        id: tex_id as u16,
        file: texture_file,
        uv_layer: uv_layer_id as u16,
      });
    }

    let mut vertex = vec![];
    let vertex_count = ascii_ops::read_int(file);
    for vertex_id in 0..vertex_count {
      let coordinate = read_xyz(file);
      let normal = read_xyz(file);
      let vertex_color = read_int4(file);
      let vertex_color = (
        vertex_color.0 as u8,
        vertex_color.1 as u8,
        vertex_color.2 as u8,
        vertex_color.3 as u8,
      );
      let mut uvs = vec![];
      for _ in 0..uv_layer_count {
        uvs.push(read_uv_vertex(file));
      }
      let mut bone_weights = vec![];
      if has_bones {
        let bone_idx = read_bone_ids(file);
        let bone_weight = read_bone_weight(file);
        for idx in 0..4 {
          bone_weights.push(BoneWeight {
            id: bone_idx[idx] as i16,
            weight: bone_weight[idx],
          });
        }
      }
      vertex.push(Vertex {
        id: vertex_id as u32,
        position: coordinate,
        normal: normal,
        color: vertex_color,
        uv: uvs,
        bone_weights: bone_weights,
        merged: false,
      });
    }
    let mut faces = vec![];
    let tri_count = ascii_ops::read_int(file);
    for _ in 0..tri_count {
      let tri_idx = read_face_indices(file);
      faces.push(tri_idx.0 as u32);
      faces.push(tri_idx.1 as u32);
      faces.push(tri_idx.2 as u32);
    }

    meshes.push(Mesh {
      name: mesh_name,
      textures: textures,
      vertices: vertex,
      faces: faces,
      uv_count: uv_layer_count as u16,
    });
  }
  Ok(meshes)
}

pub fn read_pose_file(file: &mut FileStream) -> String {
  file.read_string()
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
          coordinate_delta: (
            ascii_ops::get_float(&data_list[0]),
            ascii_ops::get_float(&data_list[1]),
            ascii_ops::get_float(&data_list[2]),
          ),
          rotation_delta: (
            ascii_ops::get_float(&data_list[3]),
            ascii_ops::get_float(&data_list[4]),
            ascii_ops::get_float(&data_list[5]),
          ),
          scale: (
            ascii_ops::get_float(&data_list[6]),
            ascii_ops::get_float(&data_list[7]),
            ascii_ops::get_float(&data_list[8]),
          ),
        },
      );
    }
  }
  pose_data
}

pub fn bone_dict_data(string: &String) -> (HashMap<String, String>, HashMap<String, String>) {
  let mut bone_dict_rename = HashMap::new();
  let mut bone_dict_restore = HashMap::new();
  for bone_pose in string.split("\n") {
    if bone_pose.len() != 0 {
      let pose: Vec<String> = bone_pose.split(';').map(|x| x.to_string()).collect();
      let old_name = pose[0].clone();
      let new_name = pose[1].clone();
      bone_dict_rename.insert(old_name.clone(), new_name.clone());
      bone_dict_restore.insert(new_name, old_name);
    }
  }
  (bone_dict_rename, bone_dict_restore)
}

fn read_io_stream(filename: &String) -> Result<FileStream, String> {
  if let Some(x) = FileStream::new(filename) {
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

pub fn read_xps_pose(filename: &String) -> HashMap<String, BonePose> {
  if let Ok(mut io_stream) = read_io_stream(filename) {
    pose_data(&read_pose_file(&mut io_stream))
  } else {
    HashMap::new()
  }
}

pub fn read_bone_dict(filename: &String) -> (HashMap<String, String>, HashMap<String, String>) {
  if let Ok(mut io_stream) = read_io_stream(filename) {
    bone_dict_data(&read_pose_file(&mut io_stream))
  } else {
    (HashMap::new(), HashMap::new())
  }
}
