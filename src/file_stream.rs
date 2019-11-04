use std::fs::File;
use std::io::Read;

pub struct FileStream {
  pub file: File,
}

impl FileStream {
  pub fn new(filename: &String) -> Option<FileStream> {
    if let Ok(x) = File::open(filename) {
      Some(FileStream { file: x })
    } else {
      return None;
    }
  }

  pub fn read(&mut self, size: usize) -> Vec<u8> {
    let mut bytes = vec![0_u8; size];
    self.file.read(&mut bytes).unwrap();
    bytes
  }

  pub fn readline(&mut self) -> String {
    let mut out_string = String::new();
    let mut single_byte = [0_u8; 1];
    while single_byte[0] != '\0' as u8 {
      self.file.read(&mut single_byte).unwrap();
      out_string.push(single_byte[0] as char);
    }
    out_string
  }

  pub fn read_string(&mut self) -> String {
    let mut out_string = String::new();
    self.file.read_to_string(&mut out_string).unwrap();
    out_string
  }
}
