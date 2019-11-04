use super::bin_ops;
use super::file_stream::FileStream;
use super::read_ascii;
use super::constants;
use super::types::{
    BoneWeight, Bone, BonePose, Data, Header, Mesh, Texture, Vertex,
};
use std::collections::HashMap;
use std::io::Seek;
use std::io::SeekFrom;
use std::string::String;
use std::path::Path;
use std::fmt::Debug;
use core::fmt;

pub enum XpsError {
  StreamNotOpened,
  InvalidHeader,
}

impl Debug for XpsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        XpsError::StreamNotOpened => {
          write!(f, "StreamNotOpened")
        },
        XpsError::InvalidHeader => {
          write!(f, "InvalidHeader")

        },
      }
    }
}

fn read_files_string(file: &mut FileStream) -> String {
  let mut length_byte2 = 0;

  let length_byte1 = bin_ops::read_byte(file);

  if length_byte1 as usize >= constants::LIMIT {
    length_byte2 = bin_ops::read_byte(file);
  }
  let length =
    (length_byte1 as usize % constants::LIMIT) + (length_byte2 as usize * constants::LIMIT);

  bin_ops::read_string(file, length)
}

fn read_vertex_color(file: &mut FileStream) -> (u8, u8, u8, u8) {
  (
    bin_ops::read_byte(file),
    bin_ops::read_byte(file),
    bin_ops::read_byte(file),
    bin_ops::read_byte(file),
  )
}

fn read_uv_vert(file: &mut FileStream) -> (f32, f32) {
  (bin_ops::read_f32(file), bin_ops::read_f32(file))
}

fn read_xyz(file: &mut FileStream) -> (f32, f32, f32) {
  (
    bin_ops::read_f32(file),
    bin_ops::read_f32(file),
    bin_ops::read_f32(file),
  )
}

fn read_float4(file: &mut FileStream) -> (f32, f32, f32, f32) {
  (
    bin_ops::read_f32(file),
    bin_ops::read_f32(file),
    bin_ops::read_f32(file),
    bin_ops::read_f32(file),
  )
}

fn read_i16_4(file: &mut FileStream) -> (i16, i16, i16, i16) {
  (
    bin_ops::read_i16(file),
    bin_ops::read_i16(file),
    bin_ops::read_i16(file),
    bin_ops::read_i16(file),
  )
}

fn read_face_indices(file: &mut FileStream) -> (u32, u32, u32) {
  (
    bin_ops::read_u32(file),
    bin_ops::read_u32(file),
    bin_ops::read_u32(file),
  )
}

fn has_tangent_header(header: &Header) -> bool {
  header.version_mayor <= 1 && header.version_minor <= 12
}

fn read_header(file: &mut FileStream) -> Header {
  let mut header = Header::default();

  let magic_number = bin_ops::read_u32(file);
  let version_mayor = bin_ops::read_u16(file);
  let version_minor = bin_ops::read_u16(file);
  let xna_aral = read_files_string(file);
  let settings_length = bin_ops::read_u32(file);
  let machine_name = read_files_string(file);
  let username = read_files_string(file);
  let files_string = read_files_string(file);
  let xps_pose_data = String::from("");

  if version_mayor <= 1 && version_minor <= 12 {
    let _ = file.read((settings_length * 4) as usize);
  } else {
    let mut values_read = 0;
    let _ = bin_ops::read_u32(file);
    values_read += 1 * 4;
    let items = bin_ops::read_u32(file);
    values_read += 1 * 4;
    for _ in 0..items {
      let option_type = bin_ops::read_u32(file);
      values_read += 1 * 4;
      let opt_count = bin_ops::read_u32(file);
      values_read += 1 * 4;
      let opt_info = bin_ops::read_u32(file);
      values_read += 1 * 4;

      if option_type == 255 {
        read_none(file, opt_count as usize);
        values_read += opt_count * 2;
      } else if option_type == 2 {
        read_flags(file, opt_count as usize);
        values_read += opt_count * 2 * 4;
      } else if option_type == 1 {
        let _ = read_default_pose(file, opt_count, opt_info as usize);
        let read_count =
          bin_ops::round_to_multiple(opt_count as usize, constants::ROUND_MULTIPLE) as u32;
        values_read += read_count;
      } else {
        let loop_start = values_read;
        let loop_finish = settings_length;
        for _ in loop_start..loop_finish {
          let _ = bin_ops::read_u32(file);
        }
      }
    }
  }

  header.magic_number = magic_number;
  header.version_mayor = version_mayor;
  header.version_minor = version_minor;
  header.aral = xna_aral;
  header.settings_length = settings_length;
  header.machine = machine_name;
  header.user = username;
  header.file = files_string;
  header.pose = xps_pose_data;
  header
}

fn find_header(file: &mut FileStream) -> Option<Header> {
  let number = bin_ops::read_u32(file);
  file.file.seek(SeekFrom::Start(0)).unwrap();

  if number as usize == constants::MAGIC_NUMBER {
    return Some(read_header(file));
  }
  None
}

fn read_none(file: &mut FileStream, opt_count: usize) {
  for _ in 0..opt_count {
    let _ = bin_ops::read_u32(file);
  }
}

fn read_flags(file: &mut FileStream, optcount: usize) {
  for _ in 0..(optcount * 2) {
    let _ = bin_ops::read_u32(file);
  }
}

fn read_bones(file: &mut FileStream) -> Vec<Bone> {
  let mut bones = vec![];
  let bone_count = bin_ops::read_u32(file);
  for bone_id in 0..bone_count {
    let bone_name = read_files_string(file);
    let parent_id = bin_ops::read_i16(file);
    let coords = read_xyz(file);

    let bone = Bone {
      id: bone_id as i16,
      name: bone_name,
      co: coords,
      parent_id: parent_id,
    };
    bones.push(bone)
  }
  return bones;
}

fn read_meshes(file: &mut FileStream, header: &Header, has_bones: bool) -> Vec<Mesh> {
  let mut meshes = vec![];
  let mesh_count = bin_ops::read_u32(file);
  let has_header = true;
  let mut has_tangent = false;
  if has_header {
    has_tangent = has_tangent_header(header)
  }
  for _ in 0..mesh_count {
    let mut mesh_name = read_files_string(file);
    if mesh_name.len() == 0 {
      mesh_name = "unnamed".to_string();
    }
    let uv_layer_count = bin_ops::read_u32(file);
    let mut textures = vec![];
    let tex_count = bin_ops::read_u32(file);
    for tex_id in 0..tex_count {
      let texture_file = {
        Path::new(&read_files_string(file))
          .parent()
          .unwrap()
          .to_str()
          .unwrap()
          .to_string()
      };
      let uv_layer_id = bin_ops::read_u32(file);

      textures.push(Texture {
        id: tex_id as u16,
        file: texture_file,
        uv_layer: uv_layer_id as u16,
      });
    }

    let mut vertex = vec![];
    let vertex_count = bin_ops::read_u32(file);

    for vertex_id in 0..vertex_count {
      let coordinate = read_xyz(file);
      let normal = read_xyz(file);
      let vertex_color = read_vertex_color(file);

      let mut uvs = vec![];
      for _ in 0..uv_layer_count {
        let uv_vertex = read_uv_vert(file);
        uvs.push(uv_vertex);
        if !has_header || has_tangent {
          let _ = read_float4(file);
        }
      }

      let mut bone_weights = vec![];
      if has_bones {
        let idx = read_i16_4(file);
        let weight = read_float4(file);
        let idx = [idx.0, idx.1, idx.2, idx.3];
        let weight = [weight.0, weight.1, weight.2, weight.3];
        for x in 0..idx.len() {
          bone_weights.push(BoneWeight {
            id: idx[x],
            weight: weight[x],
          });
        }
      }
      vertex.push(Vertex {
        id: vertex_id,
        position: coordinate,
        normal: normal,
        color: vertex_color,
        uv: uvs,
        bone_weights: bone_weights,
        merged: false,
      });
    }

    let mut faces = vec![];
    let tri_count = bin_ops::read_u32(file);
    for _ in 0..tri_count {
      let idx = read_face_indices(file);
      faces.push(idx.0);
      faces.push(idx.1);
      faces.push(idx.2);
    }
    meshes.push(Mesh {
      name: mesh_name,
      textures: textures,
      vertices: vertex,
      faces: faces,
      uv_count: uv_layer_count as u16,
    });
  }
  meshes
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
    if let Some(header) = find_header(&mut io_stream) {
      let bones = read_bones(&mut io_stream);
      let has_bones = bones.len() > 0;
      let meshes = read_meshes(&mut io_stream, &header, has_bones);
      return Ok(Data {
        header: header,
        bones: bones,
        meshes: meshes,
      });
    } else {
      return Err(XpsError::InvalidHeader);
    }
  } else {
    return Err(XpsError::StreamNotOpened);
  }
}

fn read_default_pose(
  file: &mut FileStream,
  pose_length_unround: u32,
  pose_bones: usize,
) -> HashMap<String, BonePose> {
  let mut pose_bytes = vec![];
  if pose_length_unround > 0 {
    for _ in 0..pose_bones {
      for byte in file.readline().as_bytes() {
        pose_bytes.push(*byte);
      }
    }
  }
  let pose_length = bin_ops::round_to_multiple(pose_length_unround as usize, constants::ROUND_MULTIPLE);
  let empty = pose_length - pose_length_unround as usize;
  file.read(empty);
  let pose_string = bin_ops::decode_bytes(&pose_bytes);
  read_ascii::pose_data(&pose_string)
}
