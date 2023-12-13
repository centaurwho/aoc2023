use std::fs::File;
use std::io::Read;

pub fn read_input(filename: &str) -> String {
    let mut file = File::open(filename)
        .expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
}