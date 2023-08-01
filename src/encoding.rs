use flate2::{
    write::{
        GzEncoder,
        GzDecoder
    },
    Compression,
};
use std::{
    io,
    io::prelude::*,
};

pub fn decode(bytes: Vec<u8>) -> io::Result<String> {
    let mut writer = Vec::new();
    let mut decoder = GzDecoder::new(writer);
    decoder.write_all(&bytes[..])?;
    writer = decoder.finish()?;
    let return_string = String::from_utf8(writer).expect("String parsing error");
    Ok(return_string)
}

pub fn encode(string: String) -> io::Result<Vec<u8>> {
    let mut writer = Vec::new();
    let mut encoder = GzEncoder::new(writer, Compression::default());
    encoder.write_all(string.as_bytes())?;
    writer = encoder.finish()?;
    Ok(writer)
}
