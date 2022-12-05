use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

pub fn crane_it_up() {
  let mut file = File::open("data/day5.txt").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut crates: Vec<Vec<char>> = Vec::from([
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
  ]);

  for line in contents.lines() {
    if line.starts_with("move") {
      move_crates(line, &mut crates);
    } else {
      load_shit(line, &mut crates);
    }
  }

  for i in 0..=8 {
    println!("{} {:?}", i + 1, crates[i]);
  }
}

fn move_crates(line: &str, crates: &mut Vec<Vec<char>>) {
  let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

  let cap = re.captures(line).unwrap();
  let move_count = &cap[1].parse::<i32>().unwrap();
  let source = &cap[2].parse::<i32>().unwrap() - 1;
  let destination = &cap[3].parse::<i32>().unwrap() - 1;

  let mut moving = Vec::new();
  for _ in 1..=*move_count {
    moving.push(crates[source as usize].pop().unwrap());
  }

  moving.reverse();
  crates[destination as usize].extend(moving.iter().copied());
}

fn load_shit(line: &str, crates: &mut Vec<Vec<char>>) {
  if line.starts_with(" 1") {
    for c in crates.iter_mut() {
      c.reverse();
    }
  } else if line != ""{
    let mut chars = line.chars();

    add_val(crates, 0, chars.nth(1).unwrap());

    for i in 1..=8 {
      add_val(crates, i, chars.nth(3).unwrap());
    }
  }
}

fn add_val(crates: &mut Vec<Vec<char>>, crate_index: usize, crate_contents: char) {
  if crate_contents == ' ' {
    return;
  }

  crates[crate_index].push(crate_contents);
}