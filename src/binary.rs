use super::ascii;
use super::constants;
use super::file_input::FileStream;
use super::types::{Bone, BonePose, BoneWeight, Data, Header, Mesh, Texture, Vertex};
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::Write;
use std::path::Path;
use std::string::String;

use super::error_types::XpsError;

pub fn round_to_multiple(num_to_round: usize, multiple: usize) -> usize {
  let remainder = num_to_round % multiple;
  if remainder == 0 {
    return num_to_round;
  }
  return num_to_round + multiple - remainder;
}

pub fn decode_bytes(bytes: &Vec<u8>) -> String {
  let mut put_string = String::new();
  for byte in bytes {
    if let Err(_) = put_string.write_char(*byte as char) {
      return String::new();
    }
  }
  put_string
}

fn read_files_string(file: &mut FileStream) -> String {
  let mut length_byte2 = 0;

  let length_byte1 = file.read_byte();

  if length_byte1 as usize >= constants::LIMIT {
    length_byte2 = file.read_byte();
  }
  let length =
    (length_byte1 as usize % constants::LIMIT) + (length_byte2 as usize * constants::LIMIT);

  file.read_string_bin(length)
}

fn read_xyz(file: &mut FileStream) -> [f32; 3] {
  [file.read_f32(), file.read_f32(), file.read_f32()]
}

fn has_tangent_header(header: &Header) -> bool {
  header.version_mayor <= 1 && header.version_minor <= 12
}

fn read_header(file: &mut FileStream) -> Header {
  let mut header = Header::default();

  let magic_number = file.read_u32();
  let version_mayor = file.read_u16();
  let version_minor = file.read_u16();
  let xna_aral = read_files_string(file);
  let settings_length = file.read_u32();
  let machine_name = read_files_string(file);
  let username = read_files_string(file);
  let files_string = read_files_string(file);
  let xps_pose_data = String::from("");

  if version_mayor <= 1 && version_minor <= 12 {
    let _ = file.read((settings_length * 4) as usize);
  } else {
    let mut values_read = 0;
    let _ = file.read_u32();
    values_read += 1 * 4;
    let items = file.read_u32();
    values_read += 1 * 4;
    for _ in 0..items {
      // 258
      let option_type = file.read_u32();
      values_read += 1 * 4;
      let opt_count = file.read_u32();
      values_read += 1 * 4;
      let opt_info = file.read_u32();
      values_read += 1 * 4;

      if option_type == 255 {
        read_none(file, opt_count as usize);
        values_read += opt_count * 2;
      } else if option_type == 2 {
        read_flags(file, opt_count as usize);
        values_read += opt_count * 2 * 4;
      } else if option_type == 1 {
        let _ = read_default_pose(file, opt_count, opt_info as usize);
        let read_count = round_to_multiple(opt_count as usize, constants::ROUND_MULTIPLE) as u32;
        values_read += read_count;
      } else {
        let loop_start = values_read / 4;
        let loop_finish = settings_length;
        for _ in loop_start..loop_finish {
          let _ = file.read_u32();
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

fn find_header(file: &mut FileStream) -> Result<Header, XpsError> {
  let number = file.read_u32();
  if let Err(_) = file.seek(0) {
    return Err(XpsError::Unknown);
  }

  if number as usize == constants::MAGIC_NUMBER {
    return Ok(read_header(file));
  }
  Err(XpsError::InvalidHeader)
}

fn read_none(file: &mut FileStream, opt_count: usize) {
  for _ in 0..opt_count {
    let _ = file.read_u32();
  }
}

fn read_flags(file: &mut FileStream, optcount: usize) {
  for _ in 0..(optcount * 2) {
    let _ = file.read_u32();
  }
}

fn read_bones(file: &mut FileStream) -> Vec<Bone> {
  let mut bones = vec![];
  let bone_count = file.read_u32();
  for bone_id in 0..bone_count {
    let bone_name = read_files_string(file);
    let parent_id = file.read_i16();
    let coords = read_xyz(file);

    let bone = Bone {
      id: bone_id as i16,
      name: CString::new(bone_name).unwrap_or(CString::new("").unwrap()),
      co: coords,
      parent_id: parent_id,
    };
    bones.push(bone)
  }
  return bones;
}

fn read_meshes(
  file: &mut FileStream,
  header: &Header,
  has_bones: bool,
  params: super::types::ImportParameters,
) -> Result<Vec<Mesh>, XpsError> {
  let mut meshes = vec![];
  let mesh_count = file.read_u32();
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
    let uv_layer_count = file.read_u32() as usize;
    let mut textures = vec![];
    let tex_count = file.read_u32();
    for tex_id in 0..tex_count {
      let texture_file = {
        let filename = read_files_string(file);
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
      let uv_layer_id = file.read_u32();

      textures.push(Texture {
        id: tex_id as u16,
        file: CString::new(texture_file).unwrap_or(CString::new("").unwrap()),
        uv_layer: uv_layer_id as u16,
      });
    }

    let mut vertex = vec![];
    let vertex_count = file.read_u32();

    for _ in 0..vertex_count {
      let coordinate = read_xyz(file);
      let normal = read_xyz(file);
      let vertex_color = [
        file.read_byte(),
        file.read_byte(),
        file.read_byte(),
        file.read_byte(),
      ];

      let mut uvs = [[0_f32; 2]; 3];
      for uvx in 0..uv_layer_count {
        uvs[uvx] = [file.read_f32(), {
          let v = file.read_f32();
          if params.flip_uv {
            1_f32 - v
          } else {
            v
          }
        }];

        if !has_header || has_tangent {
          let _ = [
            file.read_f32(),
            file.read_f32(),
            file.read_f32(),
            file.read_f32(),
          ];
        }
      }

      let mut bone_weights = [BoneWeight::default(); 4];
      if has_bones {
        let idx = [
          file.read_i16(),
          file.read_i16(),
          file.read_i16(),
          file.read_i16(),
        ];
        let weight = [
          file.read_f32(),
          file.read_f32(),
          file.read_f32(),
          file.read_f32(),
        ];
        for x in 0..idx.len() {
          bone_weights[x] = BoneWeight {
            id: idx[x],
            weight: weight[x],
          };
        }
      }
      vertex.push(Vertex {
        position: coordinate,
        normal: normal,
        color: vertex_color,
        uv: uvs,
        bone_weights: bone_weights,
        merged: false,
      });
    }

    let mut faces = vec![];
    let tri_count = file.read_u32();
    for _ in 0..tri_count {
      let idx = (file.read_u32(), file.read_u32(), file.read_u32());
      faces.push(idx.0);
      if params.reverse_winding {
        faces.push(idx.2);
        faces.push(idx.1);
      } else {
        faces.push(idx.1);
        faces.push(idx.2);
      }
    }
    meshes.push(Mesh {
      name: CString::new(mesh_name.clone()).unwrap_or(CString::new("").unwrap()),
      textures: textures,
      vertices: vertex,
      faces: faces,
      uv_count: uv_layer_count as u16,
      render_group: {
        let parser = super::mesh_name_parser::MeshNameParser::new(&mesh_name);
        super::material::RenderGroup::new(parser.get_render_group_number())
      },
    });
  }
  Ok(meshes)
}

fn read_io_stream(filename: &String) -> Result<FileStream, String> {
  if let Some(x) = FileStream::new(filename, false) {
    Ok(x)
  } else {
    Err(String::from("File not found"))
  }
}

pub fn read_xps_model(
  filename: &String,
  params: super::types::ImportParameters,
) -> Result<Data, XpsError> {
  if let Ok(mut io_stream) = read_io_stream(filename) {
    if let Ok(header) = find_header(&mut io_stream) {
      let bones = read_bones(&mut io_stream);
      let has_bones = bones.len() > 0;
      if let Ok(meshes) = read_meshes(&mut io_stream, &header, has_bones, params) {
        return Ok(Data {
          header: header,
          bones: bones,
          meshes: meshes,
          error: XpsError::None,
        });
      }
      return Err(XpsError::MeshReadBin);
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
      for byte in file.read_line().as_bytes() {
        pose_bytes.push(*byte);
      }
    }
  }
  let pose_length = round_to_multiple(pose_length_unround as usize, constants::ROUND_MULTIPLE);
  let empty = pose_length - pose_length_unround as usize;
  file.read(empty);
  let pose_string = decode_bytes(&pose_bytes);
  ascii::pose_data(&pose_string)
}
