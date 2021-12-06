use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    do_stuff();
    compare_ranges(3);
    move_sub();
    get_power();
    get_o2_and_co2();
}

fn do_stuff() {
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

fn compare_ranges(_array_count: i32) {
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


fn move_sub() {
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

fn get_power() {
    let mut file = File::open("data/day3.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut common_bits: Vec<Vec<i32>> = Vec::new();

    for line in contents.lines() {
        let mut char_pos = 0;
        for c in line.chars() { 
            if common_bits.len() <= char_pos {
                common_bits.push(Vec::from([0,0]));
            }

            let bit = c.to_digit(10).unwrap();
            common_bits[char_pos as usize][bit as usize] +=  1;
            char_pos += 1;
        }
    }

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for result in common_bits.iter() {
        if result[0] > result[1] {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("Gamma: {} Epsilon: {}", gamma_int, epsilon_int);
    println!("Power Consumption: {}", gamma_int * epsilon_int);
}

fn get_o2_and_co2() {
    let file = File::open("data/day3.txt").expect("no such file");
    let buf = BufReader::new(file);
    let start = buf.lines()
                    .map(|l| l.expect("Could not parse line"))
                    .collect::<Vec<String>>();

    let mut lines = start.clone();
    let mut char_pos = 0;
    while lines.len() > 1 {
        lines = get_o2(lines, char_pos);

        char_pos += 1;
    }
    let o2 = isize::from_str_radix(&lines[0], 2).unwrap();

    lines = start.clone();
    char_pos = 0;
    while lines.len() > 1 {
        lines = get_co2(lines, char_pos);

        char_pos += 1;
    }

    let co2 = isize::from_str_radix(&lines[0], 2).unwrap();

    println!("End - o2: {}, co2: {} Total: {}", o2, co2, o2 * co2);
} 

fn get_o2(lines: Vec<String>, char_pos: i32) -> Vec<String> {
    let mut char_count = [0; 2];

    for line in lines.iter() {
        let c = line.chars().nth(char_pos.try_into().unwrap()).unwrap();
        let bit = c.to_digit(10).unwrap();

        char_count[bit as usize] +=  1;
    }

    let mut most_popular = '0';
    if char_count[1] >= char_count[0] {
        most_popular = '1';
    }

    return lines.into_iter().filter(|line| line.chars().nth(char_pos.try_into().unwrap()).unwrap() ==  most_popular ).collect();
}

fn get_co2(lines: Vec<String>, char_pos: i32) -> Vec<String> {
    let mut char_count = [0; 2];

    for line in lines.iter() {
        let c = line.chars().nth(char_pos.try_into().unwrap()).unwrap();
        let bit = c.to_digit(10).unwrap();

        char_count[bit as usize] +=  1;
    }

    let mut least_popular = '1';
    if char_count[1] >= char_count[0] {
        least_popular = '0';
    }

    return lines.into_iter().filter(|line| line.chars().nth(char_pos.try_into().unwrap()).unwrap() ==  least_popular ).collect();
}