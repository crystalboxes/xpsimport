use std::vec::Vec;
pub struct RenderGroup {
   pub alpha: bool,
   pub posable: bool,
   pub specular: String,
   pub bump1_rep: bool,
   pub bump2_rep: bool,
   pub spec1_rep: bool,
   pub tex_count: i32,
   pub texture_types: Vec<String>,
}

impl RenderGroup {
   pub fn new(render_group_num: i32) -> RenderGroup {
      let mut alpha = false;
      let mut posable = true;
      let mut specular = "Yes";
      let mut bump1_rep = true;
      let mut bump2_rep = true;
      let mut spec1_rep = false;
      let mut tex_count = 6;
      let mut texture_types = vec!["diffuse", "mask", "mask", "mask", "mask", "mask"];

      match render_group_num {
         1 => {
            alpha = false;
            posable = true;
            specular = "Yes";
            bump1_rep = true;
            bump2_rep = true;
            tex_count = 6;
            texture_types = vec!["diffuse", "lightmap", "bumpmap", "mask", "bump1", "bump2"];
         }
         2 => {
            alpha = false;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 3;
            texture_types = vec!["diffuse", "lightmap", "bumpmap"];
         }
         3 => {
            alpha = false;
            posable = true;
            specular = "No";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 2;
            texture_types = vec!["diffuse", "lightmap"];
         }
         4 => {
            alpha = false;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 2;
            texture_types = vec!["diffuse", "bumpmap"];
         }
         5 => {
            alpha = false;
            posable = true;
            specular = "No";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 1;
            texture_types = vec!["diffuse"];
         }
         6 => {
            alpha = true;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 2;
            texture_types = vec!["diffuse", "bumpmap"];
         }
         7 => {
            alpha = true;
            posable = true;
            specular = "No";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 1;
            texture_types = vec!["diffuse"];
         }
         8 => {
            alpha = true;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 3;
            texture_types = vec!["diffuse", "lightmap", "bumpmap"];
         }
         9 => {
            alpha = true;
            posable = true;
            specular = "No";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 2;
            texture_types = vec!["diffuse", "lightmap"];
         }
         10 => {
            alpha = false;
            posable = true;
            specular = "No";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 1;
            texture_types = vec!["diffuse"];
         }
         11 => {
            alpha = false;
            posable = false;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 2;
            texture_types = vec!["diffuse", "bumpmap"];
         }
         12 => {
            alpha = true;
            posable = false;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 2;
            texture_types = vec!["diffuse", "bumpmap"];
         }
         13 => {
            alpha = false;
            posable = false;
            specular = "No";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 1;
            texture_types = vec!["diffuse"];
         }
         14 => {
            alpha = false;
            posable = false;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 2;
            texture_types = vec!["diffuse", "bumpmap"];
         }
         15 => {
            alpha = true;
            posable = false;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 2;
            texture_types = vec!["diffuse", "bumpmap"];
         }
         16 => {
            alpha = false;
            posable = false;
            specular = "No";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 1;
            texture_types = vec!["diffuse"];
         }
         17 => {
            alpha = false;
            posable = false;
            specular = "No";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 2;
            texture_types = vec!["diffuse", "lightmap"];
         }
         18 => {
            alpha = true;
            posable = false;
            specular = "No";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 1;
            texture_types = vec!["diffuse"];
         }
         19 => {
            alpha = true;
            posable = false;
            specular = "No";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 2;
            texture_types = vec!["diffuse", "lightmap"];
         }
         20 => {
            alpha = true;
            posable = true;
            specular = "Yes";
            bump1_rep = true;
            bump2_rep = true;
            tex_count = 6;
            texture_types = vec!["diffuse", "lightmap", "bumpmap", "mask", "bump1", "bump2"];
         }
         21 => {
            alpha = true;
            posable = true;
            specular = "No";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 1;
            texture_types = vec!["diffuse"];
         }
         22 => {
            alpha = false;
            posable = true;
            specular = "Yes";
            bump1_rep = true;
            bump2_rep = true;
            tex_count = 7;
            texture_types = vec![
               "diffuse", "lightmap", "bumpmap", "mask", "bump1", "bump2", "specular",
            ];
         }
         23 => {
            alpha = true;
            posable = true;
            specular = "Yes";
            bump1_rep = true;
            bump2_rep = true;
            tex_count = 7;
            texture_types = vec![
               "diffuse", "lightmap", "bumpmap", "mask", "bump1", "bump2", "specular",
            ];
         }
         24 => {
            alpha = false;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 4;
            texture_types = vec!["diffuse", "lightmap", "bumpmap", "specular"];
         }
         25 => {
            alpha = true;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 4;
            texture_types = vec!["diffuse", "lightmap", "bumpmap", "specular"];
         }
         26 => {
            alpha = false;
            posable = true;
            specular = "Yes intensity";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 4;
            texture_types = vec!["diffuse", "bumpmap", "enviroment", "mask"];
         }
         27 => {
            alpha = true;
            posable = true;
            specular = "Yes intensity";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 4;
            texture_types = vec!["diffuse", "bumpmap", "enviroment", "mask"];
         }
         28 => {
            alpha = false;
            posable = true;
            specular = "Yes intensity";
            bump1_rep = true;
            bump2_rep = true;
            tex_count = 6;
            texture_types = vec!["diffuse", "bumpmap", "mask", "bump1", "bump2", "enviroment"];
         }
         29 => {
            alpha = true;
            posable = true;
            specular = "Yes intensity";
            bump1_rep = true;
            bump2_rep = true;
            tex_count = 6;
            texture_types = vec!["diffuse", "bumpmap", "mask", "bump1", "bump2", "enviroment"];
         }
         30 => {
            alpha = false;
            posable = true;
            specular = "Yes intensity";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 3;
            texture_types = vec!["diffuse", "bumpmap", "emission"];
         }
         31 => {
            alpha = true;
            posable = true;
            specular = "Yes intensity";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 3;
            texture_types = vec!["diffuse", "bumpmap", "emission"];
         }
         32 => {
            alpha = false;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 1;
            texture_types = vec!["diffuse"];
         }
         33 => {
            alpha = true;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 1;
            texture_types = vec!["diffuse"];
         }
         34 => {}
         35 => {}
         36 => {
            alpha = false;
            posable = true;
            specular = "Yes intensity";
            bump1_rep = true;
            bump2_rep = false;
            tex_count = 3;
            texture_types = vec!["diffuse", "bumpmap", "emission_mini_map"];
         }
         37 => {
            alpha = true;
            posable = true;
            specular = "Yes intensity";
            bump1_rep = true;
            bump2_rep = false;
            tex_count = 3;
            texture_types = vec!["diffuse", "bumpmap", "emission_mini_map"];
         }
         38 => {
            alpha = false;
            posable = true;
            specular = "Yes intensity";
            bump1_rep = true;
            bump2_rep = false;
            tex_count = 4;
            texture_types = vec!["diffuse", "bumpmap", "specular", "emission"];
         }
         39 => {
            alpha = true;
            posable = true;
            specular = "Yes intensity";
            bump1_rep = true;
            bump2_rep = false;
            tex_count = 4;
            texture_types = vec!["diffuse", "bumpmap", "specular", "emission"];
         }
         40 => {
            alpha = false;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 3;
            texture_types = vec!["diffuse", "bumpmap", "specular"];
         }
         41 => {
            alpha = true;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            tex_count = 3;
            texture_types = vec!["diffuse", "bumpmap", "specular"];
         }
         42 => {
            alpha = false;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            spec1_rep = true;
            tex_count = 3;
            texture_types = vec!["diffuse", "bumpmap", "specular"];
         }
         43 => {
            alpha = true;
            posable = true;
            specular = "Yes";
            bump1_rep = false;
            bump2_rep = false;
            spec1_rep = true;
            tex_count = 3;
            texture_types = vec!["diffuse", "bumpmap", "specular"];
         }
         _ => (),
      };
      RenderGroup {
         alpha: alpha,
         posable: posable,
         specular: specular.to_string(),
         bump1_rep: bump1_rep,
         bump2_rep: bump2_rep,
         spec1_rep: spec1_rep,
         tex_count: tex_count,
         texture_types: texture_types.into_iter().map(|x| x.to_string()).collect(),
      }
   }
}
