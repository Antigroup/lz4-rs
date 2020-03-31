extern crate lz4;

use lz4::EncoderBuilder;
use std::fs;
use std::io;

fn test_compress(filename: &str) {
    let mut file = fs::File::open(filename).unwrap();
    let uncompressed_size = file.metadata().unwrap().len();
    let buffer: Vec<u8> = Vec::new();
    let mut encoder = EncoderBuilder::new()
        .auto_flush(true)
        .level(12)
        .build(buffer)
        .unwrap();
    io::copy(&mut file, &mut encoder).unwrap();

    let (buffer, _result) = encoder.finish();
    let compressed_size = buffer.len();

    println!(
        "{} compressed: {} -> {}, Ratio {}",
        filename,
        uncompressed_size,
        compressed_size,
        uncompressed_size as f32 / compressed_size as f32
    );
}

fn main() {
    test_compress("ch1.txt");
    test_compress("ch2.txt");
    test_compress("ch345.txt");
    test_compress("moby.txt");
}
