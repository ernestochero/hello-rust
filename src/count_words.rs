use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn open_file(p: &str) -> BufReader<File> {
    BufReader::new(
        File::open(Path::new(p)).expect("Unable to open file")
    )
}

pub fn count_lines() -> i32 {
    let file = open_file("./resources/sample.txt");
    file.lines().fold(0, |acc, _| acc + 1)
}