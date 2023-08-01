mod encoding;

use encoding::{
    decode,
    encode,
};
use std::{
    fs::File,
    io::{
        Read,
        Write,
    },
};

fn main() {
    let encoded = encode("Hello, world!".to_string()).expect("Encoding error");

    println!("{:?}", encoded);

    let mut file = File::create("test.gz").expect("File creation error");
    file.write_all(&encoded[..]).expect("Error writing to file");
    file.sync_all().expect("Error syncing file");

    let mut file = File::open("test.gz").expect("File open error");
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded).expect("Error reading file");
    let decoded = decode(encoded).expect("Decoding error");

    println!("{}", decoded);
}