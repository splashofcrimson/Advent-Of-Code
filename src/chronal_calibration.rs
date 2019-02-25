use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn run() {
    let buf = BufReader::new(File::open("input/day01.txt").unwrap());
    let lines: Vec<String> = buf.lines()
        .map(|l| l.unwrap())
        .collect();
    let int_lines: Vec<i32> = lines.iter()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let sum : i32 = int_lines.iter().fold(0, |sum, val| sum + val);
    println!("Resulting frequency: {}", sum);
}
