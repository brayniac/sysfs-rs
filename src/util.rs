use std::io::prelude::*;
use std::io;
use std::fs::File;
use bit_vec::BitVec;

pub fn usize_from_file(path: &str) -> Result<usize, &'static str> {
    if let Ok(s) = string_from_file(path) {
        if let Ok(i) = s.trim().parse() {
            Ok(i)
        } else {
            Err("Unable to parse")
        }
    } else {
        Err("Unable to open file")
    }
}

fn string_from_file(path: &str) -> Result<String, io::Error> {
    match File::open(&path) {
        Ok(mut f) => {
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

pub fn bitmask_from_hex_file(path: &str) -> Result<BitVec, &'static str> {
    if let Ok(s) = string_from_file(path) {
        bitmask_from_hex(&s)
    } else {
        Err("Error reading file")
    }
}


/// parse a hex string to bytes_from_hex
///
/// # Examples
///
/// ```
/// use sysfs_rs::util;
///
/// assert_eq!(util::bytes_from_hex("00"), Ok(vec![0]));
/// assert_eq!(util::bytes_from_hex("0"), Ok(vec![0]));
/// assert_eq!(util::bytes_from_hex("f"), Ok(vec![240]));
/// assert_eq!(util::bytes_from_hex("F"), Ok(vec![240]));
/// assert_eq!(util::bytes_from_hex("FF"), Ok(vec![255]));
/// ```
pub fn bytes_from_hex(hex: &str) -> Result<Vec<u8>, &'static str> {
    let mut bytes = Vec::<u8>::new();
    let mut chars = Vec::<char>::new();
    for c in hex.trim().chars() {
        // skip common delimiters
        match c {
            ',' | ' ' | '_' => continue,
            _ => {}
        }
        chars.push(c);
    }
    for chunk in chars.chunks(2) {
        let string = if chunk.len() == 1 {
            format!("{}0", chunk[0])
        } else {
            format!("{}{}", chunk[0], chunk[1])
        };

        if let Ok(byte) = u8::from_str_radix(&string.to_string(), 16) {
            bytes.push(byte);
        } else {
            return Err("Failure parsing hex string");
        }
    }
    Ok(bytes)
}

pub fn bitmask_from_hex(hex: &str) -> Result<BitVec, &'static str> {
    match bytes_from_hex(hex) {
        Ok(bytes) => Ok(BitVec::from_bytes(&bytes)),
        Err(e) => Err(e),
    }
}
