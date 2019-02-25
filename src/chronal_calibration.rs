use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

fn read_from_file() -> Vec<i32> {
    let buf = BufReader::new(File::open("input/day01.txt").unwrap());
    let lines: Vec<String> = buf.lines()
        .map(|l| l.unwrap())
        .collect();
    lines.iter()
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}

pub fn one() {
    let sum : i32 = read_from_file().iter().fold(0, |sum, val| sum + val);
    println!("Resulting frequency: {}", sum);
}

pub fn two() {
    let mut frequencies = HashSet::new();
    let int_lines = read_from_file();
    let mut running_sum = 0;
    let mut flag = 0;

    loop {
        for frequency in &int_lines {
            running_sum += frequency;
            if !frequencies.contains(&running_sum) {
                frequencies.insert(running_sum);
            }
            else {
                println!("Repeated frequency: {}", running_sum);
                flag = 1;
                break;
            }
        }
        if flag == 1 {
            break;
        }
    }
}
