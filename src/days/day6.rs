use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;
use std::collections::HashMap;

pub fn lanternfish_count() {
    let mut file = File::open("data/day6.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let fish: Vec<i32> = contents.split(",").map(|c| c.parse::<i32>().unwrap()).collect();
    old_way(fish.clone());
    new_way(fish);
}

fn old_way(mut fish: Vec<i32>) {
    for _ in 1..=80 {
      let mut new_count = 0;

      for i in 0..fish.len() {
        if fish[i] == 0 {
          new_count += 1;

          fish[i] = 6;
        } else {
          fish[i] -= 1;
        }
      }

      for _ in 0..new_count {
        fish.push(8);
      }
    }

    println!("Lanternfish Count: {}", fish.len());
}

fn new_way(fish: Vec<i32>) {
  let mut hash = fish.iter().cloned().counts();

  for _ in 0..256 {
    let mut new_hash: HashMap<i32, usize> = HashMap::new();

    for (key, value) in hash {
        if key == 0 {
          *new_hash.entry(6).or_default() += value;
          *new_hash.entry(8).or_default() += value;
        } else {
          let next_key = key - 1;
          *new_hash.entry(next_key).or_default() += value;
        };
    }

    hash = new_hash;
  }

  let f = hash.values().sum::<usize>() as u64;
  println!("FINAL: {}", f);

}