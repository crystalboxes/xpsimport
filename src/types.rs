use super::error_types::XpsError;
use super::material::RenderGroup;
use std::ffi::CString;

pub struct ImportParameters {
  pub flip_uv: bool,
  pub reverse_winding: bool,
}

pub struct Bone {
  pub id: i16,
  pub name: CString,
  pub co: [f32; 3],
  pub parent_id: i16,
}

pub struct BonePose {
  pub name: String,
  pub coordinate_delta: [f32; 3],
  pub rotation_delta: [f32; 3],
  pub scale: [f32; 3],
}

pub struct Mesh {
  pub name: CString,
  pub textures: Vec<Texture>,
  pub vertices: Vec<Vertex>,
  pub faces: Vec<u32>,
  pub uv_count: u16,
  pub render_group: RenderGroup,
}

#[derive(Default, Copy, Clone)]
pub struct BoneWeight {
  pub id: i16,
  pub weight: f32,
}

#[derive(Default, Clone, Copy)]
pub struct Vertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub color: [u8; 4],
  pub uv: [[f32; 2]; 3],
  pub bone_weights: [BoneWeight; 4],
  pub merged: bool,
}

pub struct Texture {
  pub id: u16,
  pub file: CString,
  pub uv_layer: u16,
}

#[derive(Default)]
pub struct Data {
  pub header: Header,
  pub bones: Vec<Bone>,
  pub meshes: Vec<Mesh>,
  pub error: XpsError,
}

pub struct Header {
  pub magic_number: u32,
  pub version_mayor: u16,
  pub version_minor: u16,
  pub aral: String,
  pub settings_length: u32,
  pub machine: String,
  pub user: String,
  pub file: String,
  pub settings: String,
  pub pose: String,
}

impl Default for Header {
  fn default() -> Header {
    Header {
      magic_number: 323232,
      version_mayor: 2,
      version_minor: 15,
      aral: String::from("XNAaraL"),
      settings_length: 275,
      machine: String::default(),
      user: String::default(),
      file: String::default(),
      settings: String::default(),
      pose: String::default(),
    }
  }
}
