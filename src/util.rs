use std::io::prelude::*;
use std::fs::File;

pub fn usize_from_file(path: String) -> Result<usize, &'static str> {
    if let Ok(mut f) = File::open(&path) {
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => {
                match s.trim().parse() {
                    Ok(i) => Ok(i),
                    Err(_) => Err("unable to parse"),
                }
            }
            Err(_) => Err("unable to read file contents"),
        }
    } else {
        Err("unable to open file")
    }
}

pub fn bitmask_from_hex_file(file: String) -> Result<Vec<bool>, &'static str> {
    if let Ok(mut f) = File::open(&file) {
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => {
                s.trim();
                bitmask_from_hex(s)
            }
            Err(_) => Err("unable to read file contents"),
        }
    } else {
        Err("unable to open file")
    }
}

pub fn bitmask_from_hex(mask: String) -> Result<Vec<bool>, &'static str> {
    let mut bitmask = Vec::new();
    let chars = mask.trim().chars();
    for c in chars {
        match c {
            '0' => bitmask.extend_from_slice(&vec![false, false, false, false]),
            '1' => bitmask.extend_from_slice(&vec![false, false, false, true]),
            '2' => bitmask.extend_from_slice(&vec![false, false, true, false]),
            '3' => bitmask.extend_from_slice(&vec![false, false, true, true]),
            '4' => bitmask.extend_from_slice(&vec![false, true, false, false]),
            '5' => bitmask.extend_from_slice(&vec![false, true, false, true]),
            '6' => bitmask.extend_from_slice(&vec![false, true, true, false]),
            '7' => bitmask.extend_from_slice(&vec![false, true, true, true]),
            '8' => bitmask.extend_from_slice(&vec![true, false, false, false]),
            '9' => bitmask.extend_from_slice(&vec![true, false, false, true]),
            'a' | 'A' => bitmask.extend_from_slice(&vec![true, false, true, false]),
            'b' | 'B' => bitmask.extend_from_slice(&vec![true, false, true, true]),
            'c' | 'C' => bitmask.extend_from_slice(&vec![true, true, false, false]),
            'd' | 'D' => bitmask.extend_from_slice(&vec![true, true, false, true]),
            'e' | 'E' => bitmask.extend_from_slice(&vec![true, true, true, false]),
            'f' | 'F' => bitmask.extend_from_slice(&vec![true, true, true, true]),
            ',' => {}
            _ => {
                return Err("unexpected character in mask");
            }
        }
    }
    bitmask.reverse();
    Ok(bitmask)
}
