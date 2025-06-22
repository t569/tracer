use std::fs::File;
use std::io::Write;

pub mod core;

pub fn write_to_file(filename: &str, data: &str) {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}