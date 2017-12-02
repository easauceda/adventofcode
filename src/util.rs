use std::fs::File;
use std::io::prelude::*;

pub fn get_input(day: u32) -> String {
    let path = format!("{}/inputs/{}.txt", env!("CARGO_MANIFEST_DIR"), day.to_string());
    let mut input = String::new();

    let mut file = File::open(path).expect("Failed to open file");

    file.read_to_string(&mut input).expect("Failed to read file");
    return input;
}