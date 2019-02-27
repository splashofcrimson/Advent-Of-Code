use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn read_from_file() -> Vec<String> {
    let buf = BufReader::new(File::open("input/day02.txt").unwrap());
    buf.lines()
        .map(|l| l.unwrap())
        .collect()
}
pub fn three() {
    let mut box_ids = HashMap::new();
    let mut two_count = 0;
    let mut three_count = 0;
    let mut two_flag = 0;
    let mut three_flag = 0;

    for id in &read_from_file() {
        for ch in id.chars() {
            if !box_ids.contains_key(&ch) {
                box_ids.insert(ch, 1);
            }
            else {
                match box_ids.get(&ch) {
                    Some(&count) => {
                        if count == 1 {
                            box_ids.insert(ch, 2);
                            two_flag += 1;
                        }
                        else if count == 2 {
                            box_ids.insert(ch, 3);
                            two_flag -= 1;
                            three_flag += 1;
                        }
                        else if count == 3 {
                            box_ids.insert(ch, 4);
                            three_flag -= 1;
                        }
                    },
                    _ => println!("ERROR"),
                }
            }
        }
        two_count += if two_flag > 0 { 1 } else { 0 };
        three_count += if three_flag > 0 { 1 } else { 0 };
        two_flag = 0;
        three_flag = 0;
        box_ids.clear();
    }

    let checksum : i32 = two_count * three_count;
    println!("2.1: {}", checksum);
}

fn diff(id1 : String, id2 : String) -> Option<String> {
    let mut diff_count : i32 = 0;
    let mut similar = String::from("");

    for (i, c) in id1.chars().enumerate() {
        if id1.as_bytes()[i] != id2.as_bytes()[i] {
            diff_count += 1;
        }
        else {
            similar.push(c);
        }
    }
    if diff_count == 1 { Some(similar) } else { None }
}

pub fn four() {
    let box_ids = read_from_file();
    let mut flag = 0;

    for id1 in &box_ids {
        for id2 in &box_ids {
            match diff(id1.to_string(), id2.to_string()) {
                Some(similar) => {
                    println!("2.2: {}", similar);
                    flag = 1;
                    break;
                },
                _ => {
                    continue;
                }
            }
        }
        if flag == 1 {
            break;
        }
    }

}