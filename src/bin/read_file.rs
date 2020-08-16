use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    println!(
        "the file content {}",
        test_read_file_content("Cargo.lock".to_string())
    );

    test_read_file_iter("Cargo.lock".to_string())
}


fn test_read_file_content(filename: String) -> String {
    fs::read_to_string(filename).unwrap()
}

// buffer reader, 文件迭代读取，避免一次性加载至内存
fn test_read_file_iter(filename: String) {
    let file = File::open(filename).expect("can't open file");
    let file = BufReader::new(file);
    for line in file.lines() {
        println!("{}", line.unwrap())
    }
}