use super::bin_ops;
use super::constants;
use super::file_input::FileStream;
use super::read_ascii;
use super::types::{Bone, BonePose, BoneWeight, Data, Header, Mesh, Texture, Vertex};
use std::collections::HashMap;
use std::path::Path;
use std::string::String;
use std::ffi::CString;

use byteorder::{ByteOrder, NativeEndian};

use super::error_types::XpsError;


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
    [
        file.read_f32(),
        file.read_f32(),
        file.read_f32(),
    ]
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
        for _ in 0..items { // 258
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
                let read_count =
                    bin_ops::round_to_multiple(opt_count as usize, constants::ROUND_MULTIPLE) as u32;
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
                match Path::new(&read_files_string(file)).parent() {
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
            let uv_layer_id = file.read_u32();

            textures.push(Texture {
                id: tex_id as u16,
                file: CString::new(texture_file).unwrap_or(CString::new("").unwrap()),
                uv_layer: uv_layer_id as u16,
            });
        }

        let vertex_count = file.read_u32() as usize;


        let f32_4_size = 4 * 4;
        let f32_3_size = 4 * 3;
        let f32_2_size = 4 * 2;
        let u8_4_size = 4;
        let i16_4_size = 4 * 2;

        let garbage_size =
            if !has_header || has_tangent {
                f32_4_size
            } else {
                0
            };


        let v_normal_offset = f32_3_size;
        let v_color_offset = f32_3_size + f32_3_size;
        let v_uv_offset = v_color_offset + u8_4_size;
        let bones_indices_offset = v_uv_offset + garbage_size + uv_layer_count * f32_2_size;
        let bones_weights_offset = bones_indices_offset + i16_4_size;

        let vertex_size: usize = {
            f32_3_size // position
                + f32_3_size // position
                + u8_4_size // color
                + uv_layer_count * f32_2_size
                + garbage_size + {
                if has_bones {
                    i16_4_size
                        + f32_4_size
                } else { 0 }
            }
        };
        let mem_block_size = vertex_size * vertex_count;
        let m = file.read(mem_block_size);

        let mut vertex = vec![Vertex::default(); vertex_count];
        for vid in 0..vertex_count {
            let offset = vid * vertex_size;
            vertex[vid] = Vertex {
                // id: vid as u32,
                position: [
                    NativeEndian::read_f32(&m[offset + 4 * 0..offset + 4 * 1]),
                    NativeEndian::read_f32(&m[offset + 4 * 1..offset + 4 * 2]),
                    NativeEndian::read_f32(&m[offset + 4 * 2..offset + 4 * 3])
                ],
                normal: {
                    let offset = offset + v_normal_offset;
                    [
                        NativeEndian::read_f32(&m[offset + 4 * 0..offset + 4 * 1]),
                        NativeEndian::read_f32(&m[offset + 4 * 1..offset + 4 * 2]),
                        NativeEndian::read_f32(&m[offset + 4 * 2..offset + 4 * 3])
                    ]
                },
                color: {
                    let offset = offset + v_color_offset;
                    [
                        *&m[offset..offset + 4][0], *&m[offset..offset + 4][1], *&m[offset..offset + 4][2], *&m[offset..offset + 4][3]]
                },
                uv: {
                    let offset = offset + v_uv_offset;
                    let mut uv = [[0_f32; 2]; 3];
                    for u in 0..uv_layer_count {
                        let offset = offset + u * f32_2_size;
                        uv[u][0] = NativeEndian::read_f32(&m[offset + 4 * 0..offset + 4 * 1]);
                        uv[u][1] = NativeEndian::read_f32(&m[offset + 4 * 1..offset + 4 * 2]);
                    }
                    uv
                },
                bone_weights: {
                    let mut bone_weight = [BoneWeight::default(); 4];
                    if has_bones {
                        {
                            let offset = offset + bones_indices_offset;
                            bone_weight[0].id = NativeEndian::read_i16(&m[offset + 2 * 0..offset + 2 * 1]);
                            bone_weight[1].id = NativeEndian::read_i16(&m[offset + 2 * 1..offset + 2 * 2]);
                            bone_weight[2].id = NativeEndian::read_i16(&m[offset + 2 * 2..offset + 2 * 3]);
                            bone_weight[3].id = NativeEndian::read_i16(&m[offset + 2 * 3..offset + 2 * 4]);
                        }
                        {
                            let offset = offset + bones_weights_offset;
                            bone_weight[0].weight = NativeEndian::read_f32(&m[offset + 4 * 0..offset + 4 * 1]);
                            bone_weight[1].weight = NativeEndian::read_f32(&m[offset + 4 * 1..offset + 4 * 2]);
                            bone_weight[2].weight = NativeEndian::read_f32(&m[offset + 4 * 2..offset + 4 * 3]);
                            bone_weight[3].weight = NativeEndian::read_f32(&m[offset + 4 * 3..offset + 4 * 4]);
                        }
                    }
                    bone_weight
                },
                merged: false,
            };
        }

        let index_count = file.read_u32() as usize * 3;
        let mut faces = vec![0_u32; index_count];
        let stride = 4;
        file.read(index_count * stride);

        for x in 0..index_count {
            let offset = stride * x;
            faces[x] = NativeEndian::read_u32(&m[offset..offset + stride]);
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

fn read_io_stream(filename: &String) -> Result<FileStream, String> {
    if let Some(x) = FileStream::new(filename, false) {
        Ok(x)
    } else {
        Err(String::from("File not found"))

    }
}

pub fn read_xps_model(filename: &String) -> Result<Data, XpsError> {
    if let Ok(mut io_stream) = read_io_stream(filename) {
        if let Ok(header) = find_header(&mut io_stream) {
            let bones = read_bones(&mut io_stream);
            let has_bones = bones.len() > 0;
            if let Ok(meshes) = read_meshes(&mut io_stream, &header, has_bones) {
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
    let pose_length =
        bin_ops::round_to_multiple(pose_length_unround as usize, constants::ROUND_MULTIPLE);
    let empty = pose_length - pose_length_unround as usize;
    file.read(empty);
    let pose_string = bin_ops::decode_bytes(&pose_bytes);
    read_ascii::pose_data(&pose_string)
}
