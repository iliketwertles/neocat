use flate2::read::GzDecoder;
use std::io::prelude::*;
use std::fs;

pub fn ungz(file: String) {
    let contents = fs::read(file).unwrap();
    let mut d = GzDecoder::new(contents.as_slice());
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    println!("{}", s); 
}