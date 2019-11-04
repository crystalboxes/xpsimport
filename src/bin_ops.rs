use super::file_stream::FileStream;
use byteorder::{ByteOrder, NativeEndian};
use std::fmt::Write;
use std::io::Read;

pub fn round_to_multiple(num_to_round: usize, multiple: usize) -> usize {
  let remainder = num_to_round % multiple;
  if remainder == 0 {
    return num_to_round;
  }
  return num_to_round + multiple - remainder;
}

pub fn read_byte(file: &mut FileStream) -> u8 {
  let mut bin = [0_u8; 1];
  file.file.read(&mut bin).unwrap();
  bin[0]
}

pub fn read_u16(file: &mut FileStream) -> u16 {
  let mut bin = [0_u8; 2];
  file.file.read(&mut bin).unwrap();
  NativeEndian::read_u16(&mut bin)
}

pub fn read_i16(file: &mut FileStream) -> i16 {
  let mut bin = [0_u8; 2];
  file.file.read(&mut bin).unwrap();
  NativeEndian::read_i16(&mut bin)
}

pub fn read_u32(file: &mut FileStream) -> u32 {
  let mut bin = [0_u8; 4];
  file.file.read(&mut bin).unwrap();
  NativeEndian::read_u32(&mut bin)
}

pub fn read_f32(file: &mut FileStream) -> f32 {
  let mut bin = [0_u8; 4];
  file.file.read(&mut bin).unwrap();
  NativeEndian::read_f32(&mut bin)
}

pub fn read_string(file: &mut FileStream, length: usize) -> String {
  let mut bin = vec![0_u8; length];
  file.file.read(&mut bin).unwrap();
  decode_bytes(&bin)
}

pub fn decode_bytes(bytes: &Vec<u8>) -> String {
  let mut put_string = String::new();
  for byte in bytes {
    put_string.write_char(*byte as char).unwrap();
  }
  put_string
}
