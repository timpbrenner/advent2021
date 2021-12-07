use std::fs::File;
use std::io::prelude::*;

pub fn get_depth() {
    let mut file = File::open("data/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines();
    let mut previous = lines.next().unwrap().parse::<i32>().unwrap();
    let mut depth_count = 0;

    for line in lines {
        let current = line.parse::<i32>().unwrap();

        if current > previous {
            depth_count += 1;
        }

        previous = current;
    }

    println!("Depth Count: {}", depth_count);
}

pub fn compare_ranges(_array_count: i32) {
    let mut ranges: Vec<Vec<i32>> = Vec::from([
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ]);

    let mut file = File::open("data/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines();

    let first = lines.next().unwrap().parse::<i32>().unwrap();
    ranges[0].push(first);

    let second = lines.next().unwrap().parse::<i32>().unwrap();
    ranges[0].push(second);
    ranges[1].push(second);

    let third = lines.next().unwrap().parse::<i32>().unwrap();
    ranges[0].push(third);
    ranges[1].push(third);
    ranges[2].push(third);

    let mut current_index = 0;
    let mut depth_count = 0;
    for line in lines {
        let current = line.parse::<i32>().unwrap();

        ranges[(current_index + 1) % 4].push(current);
        ranges[(current_index + 2) % 4].push(current);
        ranges[(current_index + 3) % 4].push(current);

        let range_sum: i32 = ranges[current_index].iter().sum();
        let second_sum: i32 = ranges[(current_index + 1) % 4].iter().sum();

        if second_sum > range_sum {
            depth_count += 1;
        }

        ranges[current_index] = Vec::new();

        current_index = (current_index + 1) % 4;
    }

    println!("Range Depth Count: {}", depth_count);
}
