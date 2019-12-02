use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::process::exit;

pub fn file_to_lines(file_name: &String) -> Vec<String> {
    let f = match File::open(file_name) {
        Ok(_r) => _r,
        Err(_) => {
            println!("failed to open {}", file_name);
            exit(1);
        }
    };

    let reader = BufReader::new(f);

    let mut vec: Vec<String> = Vec::new();

    for line in reader.lines() {
        vec.push(line.unwrap());
    }

    vec
}

pub fn parse_int(string: String) -> i32 {
    match string.trim().parse() {
        Ok(_r) => _r,
        Err(_e) => 0
    }
}

pub fn vec_to_int(lines: Vec<String>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();

    for line in lines {
        let n: i32 = parse_int(line);
        vec.push(n);
    }

    vec
}
