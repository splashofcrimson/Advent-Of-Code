use std::fs;

pub fn run() {
    let input = fs::read_to_string("../input/day01.txt").expect("Unable to read file");

    println!("{:?}", input);

}
