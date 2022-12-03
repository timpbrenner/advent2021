use std::fs::File;
use std::io::prelude::*;

pub fn get_calories() {
    let mut file = File::open("data/day1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut current_calories: i32 = 0;
    let mut calories: Vec<i32> = Vec::new();

    for line in contents.lines() {
      if line == "" {
        calories.push(current_calories);
        current_calories = 0;
      } else {
        let cal = line.parse::<i32>().unwrap();
        current_calories += cal;
      }
    }
    calories.sort_by(|a, b| b.cmp(a));
    println!("Most Calories: {}", calories[0]);
    println!("Top 3: {}", calories[0] + calories[1] + calories[2]);
}