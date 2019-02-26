use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_from_file() -> Vec<String> {
    let buf = BufReader::new(File::open("input/day02.txt").unwrap());
    buf.lines()
        .map(|l| l.unwrap())
        .collect()
}

// pub fn one() {
//     let mut guards = HashMap::new();

//     for guard in &read_from_file() {

//     }
// }