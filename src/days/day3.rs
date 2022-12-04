use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

pub fn sort_pack() {
    let mut file = File::open("data/day3.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines();
    let mut total_priority = 0;
    while let Some(sack_3) = lines.next() {
      let sack_1 = lines.next().unwrap();
      let sack_2 = lines.next().unwrap();
      let common_char = get_common_char(sack_1, sack_2, sack_3);

      let priority;
      if common_char as i32 > 95 {
        priority = common_char as i32 - 96;
      } else {
        priority = common_char as i32 - 38;
      }

      total_priority += priority;
    }

    println!("Total Priority: {}", total_priority);
}

fn get_common_char(part1:&str, part2:&str, part3:&str) -> char {
  let set: HashSet<char> = part2.chars().collect();
  let set2: HashSet<char> = part3.chars().collect();

  for c in part1.chars() {
    if set.contains(&c) && set2.contains(&c) {
      return c;
    }
  }

  return 'A';
}