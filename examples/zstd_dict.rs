extern crate zstd;

use std::io::Write;

fn main() {
    let mut files: Vec<String> = Vec::new();
    for i in 1..135 {
        files.push(format!("./chapters/{}.txt", i));
    }

    let dictionary = zstd::dict::from_files(files, 65535).unwrap();

    std::fs::File::create("./zstd-dict.txt")
        .unwrap()
        .write_all(&dictionary)
        .unwrap();
}
