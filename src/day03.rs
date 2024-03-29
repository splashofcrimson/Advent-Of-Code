use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashSet, HashMap};

struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn read_from_file() -> Vec<String> {
    let buf = BufReader::new(File::open("input/day03.txt").unwrap());
    buf.lines()
        .map(|l| l.unwrap())
        .collect()
}

pub fn five() {
    let mut split_claim: Vec<String>;
    let mut coordinates: Vec<String>;
    let mut bounds: Vec<String>;

    let mut claims = HashMap::new();

    let mut square_inches : i32 = 0;

    let buf = read_from_file();
    for claim in &buf {
        split_claim = claim.split(" ")
            .map(|s| s.to_string())
            .collect();
        coordinates = split_claim[2 as usize].split(",")
            .map(|s| s.to_string())
            .collect();
        bounds = split_claim[3 as usize].split("x")
            .map(|s| s.to_string())
            .collect();

        let c = Claim {
            id: split_claim[0 as usize].clone().trim_matches('#').parse().unwrap(),
            x: coordinates[0 as usize].clone().parse().unwrap(),
            y: coordinates[1 as usize].clone().trim_matches(':').parse().unwrap(),
            width: bounds[0 as usize].clone().parse().unwrap(),
            height: bounds[1 as usize].clone().parse().unwrap(),
        };

        for x in (c.x+1)..=(c.x+c.width) {
            for y in (c.y+1)..=(c.y+c.height) {
                let coordinate = (x, y);
                if !claims.contains_key(&coordinate) {
                    claims.insert(coordinate, 1);
                }
                else {
                    match claims.get(&coordinate) {
                        Some(&count) => {
                            if count == 1 {
                                claims.insert(coordinate, 2);
                                square_inches += 1;
                            }
                        },
                        _ => println!("ERROR"),
                    }
                }
            }
        }

    }
    // println!("{}", square_inches);
}

pub fn six() {
    let mut split_claim: Vec<String>;
    let mut coordinates: Vec<String>;
    let mut bounds: Vec<String>;

    let mut claims = HashMap::new();
    let mut claim_ids = HashSet::new();

    let mut flag_id = 0;

    let buf = read_from_file();
    for claim in &buf {
        split_claim = claim.split(" ")
            .map(|s| s.to_string())
            .collect();
        coordinates = split_claim[2 as usize].split(",")
            .map(|s| s.to_string())
            .collect();
        bounds = split_claim[3 as usize].split("x")
            .map(|s| s.to_string())
            .collect();

        let c = Claim {
            id: split_claim[0 as usize].clone().trim_matches('#').parse().unwrap(),
            x: coordinates[0 as usize].clone().parse().unwrap(),
            y: coordinates[1 as usize].clone().trim_matches(':').parse().unwrap(),
            width: bounds[0 as usize].clone().parse().unwrap(),
            height: bounds[1 as usize].clone().parse().unwrap(),
        };

        for x in (c.x+1)..=(c.x+c.width) {
            for y in (c.y+1)..=(c.y+c.height) {
                let coordinate = (x, y);
                if !claims.contains_key(&coordinate) {
                    claims.insert(coordinate, c.id);
                    claim_ids.insert(c.id);
                }
                else {
                    match claims.get(&coordinate) {
                        Some(&id) => {
                            flag_id = id;
                            if claim_ids.contains(&flag_id) {
                                claim_ids.remove(&flag_id);
                            }
                        },
                        _ => println!("ERROR"),
                    }
                }
            }
        }

        if flag_id != 0 {
            if claim_ids.contains(&c.id) {
                claim_ids.remove(&c.id);
            }
        }

        flag_id = 0;
    }
    println!("{:?}", claim_ids);
}