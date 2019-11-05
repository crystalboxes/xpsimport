use std::fmt::Write;

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
