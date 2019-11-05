use std::string::String;
use std::vec::Vec;


pub fn split_values(line: &String) -> Vec<String> {
  line
    .replace("#", " ")
    .split_whitespace()
    .map(|x| x.to_string())
    .collect()
}

pub fn ignore_comment(line: &String) -> String {
  if let Some(x) = line.replace("#", " ").split_whitespace().next() {
    x.to_string()
  } else {
    "".to_string()
  }
}

pub fn ignore_string_comment(line: &String) -> String {
  if let Some(x) = line.split("#").next() {
    x.to_string()
  } else {
    "".to_string()
  }
}
pub fn get_float(value: &String) -> f32 {
  if let Ok(x) = value.parse() {
    x
  } else {
    std::f32::NAN
  }
}

pub fn get_int(value: &String) -> i32 {
  if let Ok(x) = value.parse() {
    x
  } else {
    0
  }
}