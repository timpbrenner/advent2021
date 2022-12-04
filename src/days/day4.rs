use std::fs::File;
use std::io::prelude::*;

pub fn find_overlaps() {
    let mut file = File::open("data/day4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut overlaps = 0;
    for line in contents.lines() {
      let sections: Vec<&str> = line.split(',').collect();

      let assignment1: Vec<i32> = sections[0].split('-').into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
      let assignment2: Vec<i32> = sections[1].split('-').into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

      if assignment1[0] <= assignment2[1] && assignment2[0] <= assignment1[1] {
        overlaps +=1;
      }
    }
    println!("Overlaps: {}", overlaps);
}