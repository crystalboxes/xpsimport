use std::string::String;

#[derive(Default)]
pub struct MeshNameParser {
  pub full_name: String,
  pub full_name_normalized: String,
  pub render_parameter_normalized: String,
  pub camera_targets_normalized: String,
  pub tokens: Vec<String>,
  pub mesh_name: String,
  pub item_name: String,
  pub item_mesh_part: String,
  pub render_group_number: String,
  pub specular_amount: f32,
  pub bump1_uv_scale: f32,
  pub bump2_uv_scale: f32,
  pub camera_name: String,
  pub camera_targets: Vec<String>,
  pub has_render_group: bool,
  pub has_parameter: bool,
  pub has_camera_targets: bool,
  pub has_specular_amount: bool,
  pub has_bump1_uv_scale: bool,
  pub has_bump2_uv_scale: bool,
  pub has_optional_items: bool,
  pub is_visible_by_default: bool,
}

impl MeshNameParser {
  pub fn new(mesh_part_name: &String) -> MeshNameParser {
    let mut mpn = MeshNameParser::default();
    mpn.clear();

    let mut mesh_part_name = mesh_part_name.to_string();

    if mesh_part_name.len() == 0 {
      mesh_part_name = String::from("null");
    }
    mpn.full_name = mesh_part_name.clone();
    mpn.tokens = mesh_part_name.split("_").map(|x| x.to_string()).collect();
    if mpn.tokens.len() < 2 {
      mpn.has_render_group = false;
      mpn.mesh_name = mesh_part_name.clone();
      mpn.item_name = mesh_part_name.clone();
      mpn.item_mesh_part = mesh_part_name;
      mpn.is_visible_by_default = false;
      mpn.render_group_number = String::from("0");
    } else {
      mpn.has_render_group = true;
      mpn.item_mesh_part = mpn.tokens[1].clone();
      mpn.mesh_name = mpn.tokens[1].clone();
      mpn.item_name = mpn.tokens[1].clone();
      mpn.is_visible_by_default = !(mpn.tokens[0] == "0");
      mpn.render_group_number = mpn.tokens[0].clone();
    }
    if mpn.tokens.len() > 2 {
      mpn.has_bump2_uv_scale = true;
      mpn.has_specular_amount = true;
      mpn.has_bump1_uv_scale = true;

      mpn.specular_amount = mpn.get_param(2);
      mpn.bump1_uv_scale = mpn.get_param(3);
      mpn.bump2_uv_scale = mpn.get_param(4);
    }
    if mpn.tokens.len() > 5 {
      mpn.get_camera();
    }
    if mpn.mesh_name.starts_with("+") || mpn.mesh_name.starts_with("-") {
      mpn.has_optional_items = true;
      mpn.is_visible_by_default = mpn.mesh_name.starts_with("+");
      let string_array: Vec<String> = mpn.mesh_name.split(".").map(|x| x.to_string()).collect();
      match string_array.len() {
        0 => {
          mpn.item_mesh_part = String::from(&mpn.mesh_name[..1]); // mpn.meshName.substring(1);
          mpn.item_name = mpn.item_mesh_part.clone();
        }
        1 => {
          mpn.item_mesh_part = String::from(&string_array[0][..1]);
          mpn.item_name = mpn.item_mesh_part.clone();
        }
        2 => {
          mpn.item_name = String::from(&string_array[0][..1]);
          mpn.item_mesh_part = string_array[1].clone();
        }
        _ => {
          mpn.item_name = String::from(&string_array[0][..1]);
          mpn.item_mesh_part = String::from(&mpn.mesh_name[..string_array[0].len()]);
        }
      }
    }
    mpn.normalize_full_name();

    mpn
  }

  fn normalize_full_name(&mut self) {
    self.normalize_render_parameter();
    self.normalize_camera_targets();
    let normalized_mesh_part_name = self.normalize_mesh_part_name();
    self.full_name_normalized = format!(
      "{}_{}_{}",
      self.render_group_number, normalized_mesh_part_name, self.render_parameter_normalized
    );
    if !self.has_camera_targets {
      return;
    }
    self.full_name_normalized = format!(
      "{}_{}{}",
      self.full_name_normalized, self.camera_name, self.camera_targets_normalized
    );
  }

  fn normalize_mesh_part_name(&mut self) -> String {
    let mut s = self.mesh_name.clone();
    if self.has_optional_items {
      s = format!(
        "{}{}.{}",
        {
          if self.is_visible_by_default {
            "+"
          } else {
            "-"
          }
        },
        self.item_name,
        self.item_mesh_part
      );
    }
    return s;
  }

  fn normalize_camera_targets(&mut self) {
    self.camera_targets_normalized = String::new();
    if !self.has_camera_targets {
      return;
    }
    for index in 0..self.camera_targets.len() {
      self.camera_targets_normalized = format!(
        "{}_{}",
        self.camera_targets_normalized, self.camera_targets[index]
      );
    }
  }

  fn normalize_render_parameter(&mut self) {
    self.render_parameter_normalized = format!(
      "{}_{}_{}",
      self.specular_amount, self.bump1_uv_scale, self.bump2_uv_scale
    );
  }

  fn clear(&mut self) {
    self.is_visible_by_default = true;
    self.has_optional_items = false;
    self.specular_amount = 0.1_f32;
    self.render_group_number = String::from("0");
  }

  fn get_camera(&mut self) {
    if self.tokens.len() <= 5 {
      return;
    }
    self.has_camera_targets = true;
    self.camera_name = self.tokens[5].clone();
    if self.tokens.len() == 6 {
      self.camera_targets = vec![String::from("root")];
    } else {
      self.camera_targets = vec![String::new(); self.tokens.len() - 6];
      for index in 0..self.tokens.len() - 6 {
        self.camera_targets[index] = self.tokens[index + 6].clone();
      }
    }
  }

  fn get_param(&mut self, index: i32) -> f32 {
    if let Ok(v) = self.tokens[index as usize].parse() {
      return v;
    } else {
      match index {
        2 => {
          self.has_specular_amount = false;
          self.specular_amount = 0.1;
          return 0.1;
        }
        3 => {
          self.has_bump1_uv_scale = false;
          self.bump1_uv_scale = 1_f32;
          return 0.0;
        }
        4 => {
          self.has_bump2_uv_scale = false;
          self.bump2_uv_scale = 1_f32;
          return 0.0;
        }
        _ => {
          return 0.0;
        }
      }
    }
  }

  pub fn get_full_name(&self) -> String {
    self.full_name_normalized.clone()
  }

  pub fn get_render_group_number(&self) -> i32 {
    if let Ok(x) = self.render_group_number.parse() {
      x
    } else {
      0
    }
  }
}
