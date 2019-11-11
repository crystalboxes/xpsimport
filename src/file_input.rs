use std::fs::File;
use std::io::{Read, BufReader, SeekFrom, Seek, Error};
use super::ascii;
use super::binary;
use byteorder::{ByteOrder, NativeEndian};

trait SeekRead: Seek + Read {}

impl SeekRead for File {}

impl SeekRead for BufReader<File> {}

pub struct FileStream {
    inner: Box<dyn SeekRead>,
    position: u64,
}

impl FileStream {
    pub fn new(filename: &String, is_ascii: bool) -> Option<FileStream> {
        if let Ok(x) = File::open(filename) {
            Some(FileStream {
                inner: {
                    if is_ascii {
                        Box::new(BufReader::new(x))
                    } else {
                        Box::new(x)
                    }
                },
                position: 0,
            })
        } else {
            return None;
        }
    }

    pub fn seek(&mut self, pos: i32) -> Result<u64, Error> {
        self.position = pos as u64;
        self.inner.seek(SeekFrom::Start(pos as u64))
    }

    pub fn read(&mut self, size: usize) -> Vec<u8> {
        let mut bytes = vec![0_u8; size];
        if let Err(_) = self.inner.read(&mut bytes) {
            return vec![];
        }
        self.position += size as u64;
        bytes
    }

    pub fn read_line(&mut self) -> String {
        let mut out_string = String::new();
        let mut single_byte = [0_u8; 1];
        while single_byte[0] != '\n' as u8 {
            if let Err(_) = self.inner.read(&mut single_byte) {
                return String::new();
            }
            out_string.push(single_byte[0] as char);
            self.position += 1;
        }
        out_string
    }


    pub fn read_line_trim(self: &mut FileStream) -> String {
        self.read_line().trim().to_string()
    }

    pub fn read_int(self: &mut FileStream) -> i32 {
        let line = self.read_line_trim();
        let value = ascii::ignore_comment(&line);
        ascii::get_int(&value)
    }

    pub fn read_string(self: &mut FileStream) -> String {
        let line = self.read_line_trim();
        ascii::ignore_string_comment(&line)
    }

    pub fn read_byte(self: &mut FileStream) -> u8 {
        let mut bin = [0_u8; 1];
        if let Err(_) = self.inner.read(&mut bin) {
            return 0_u8;
        }
        self.position += 1;

        bin[0]
    }

    pub fn read_u16(self: &mut FileStream) -> u16 {
        let mut bin = [0_u8; 2];
        if let Err(_) = self.inner.read(&mut bin) {
            return 0_u16;
        }
        self.position += 2;

        NativeEndian::read_u16(&bin)
    }

    pub fn read_i16(self: &mut FileStream) -> i16 {
        let mut bin = [0_u8; 2];
        if let Err(_) = self.inner.read(&mut bin) {
            return 0_i16;
        }
        self.position += 2;

        NativeEndian::read_i16(&bin)
    }

    pub fn read_u32(self: &mut FileStream) -> u32 {
        let mut bin = [0_u8; 4];
        if let Err(_) = self.inner.read(&mut bin) {
            return 0_u32;
        }
        self.position += 4;

        NativeEndian::read_u32(&bin)
    }

    pub fn read_f32(self: &mut FileStream) -> f32 {
        let mut bin = [0_u8; 4];
        if let Err(_) = self.inner.read(&mut bin) {
            return 0_f32;
        }
        self.position += 4;
        NativeEndian::read_f32(&bin)
    }

    pub fn read_string_bin(self: &mut FileStream, length: usize) -> String {
        let mut bin = vec![0_u8; length];
        if let Err(_) = self.inner.read(&mut bin) {
            return String::new();
        }
        self.position += length as u64;
        binary::decode_bytes(&bin)
    }
}
