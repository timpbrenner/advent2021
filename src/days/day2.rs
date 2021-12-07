use std::fs::File;
use std::io::prelude::*;

pub fn move_sub() {
    let mut file = File::open("data/day2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in contents.lines() {
        let mut chunks = line.split_whitespace();
        let direction = chunks.next().unwrap();
        let val = chunks.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => {
                x += val;
                depth += aim * val;
            }
            "up" => {
                aim -= val;
            }
            "down" => {
                aim += val;
            }
            _ => println!("Error: {}", direction),
        }
    }

    println!("Depth: {} Horizontal Position: {}", depth, x);
}