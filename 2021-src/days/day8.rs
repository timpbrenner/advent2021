use std::fs::File;
use std::io::prelude::*;

pub fn weird_number_thing() {
    let mut file = File::open("data/day8.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut total = 0;

    for line in contents.lines() {
      let mut parts = line.split(" | ");
      let input = parts.next().unwrap();
      let output = parts.next().unwrap();

      let mut one: Vec<char> = Vec::new();
      let mut four: Vec<char> = Vec::new();

      for data in input.split(' ') {
        if data.len() == 2 {
          one = data.chars().collect();
        }

        if data.len() == 4 {
          four = data.chars().collect();
        }
      }

      let mut output_string = String::from("");
      for data in output.split(' ') {
        let num = get_number(one.clone(), four.clone(), data.to_string());

        output_string.push(char::from_digit(num.try_into().unwrap(), 10).unwrap());
      }

      total += output_string.parse::<i32>().unwrap();
    }

    println!("Weird Number Total: {}", total);
}

fn get_number(one:Vec<char>, four: Vec<char>, input: String) -> i32 {
  match input.len() {
    2 => return 1,
    4 => return 4,
    3 => return 7,
    7 => return 8,

    5 => return two_or_3_or_5(input, one, four),
    6 => return six_or_nine_or_zero(input, one, four),
    _ => {
      println!("BAD BAD BAD");
      0
    },
  }
}

fn two_or_3_or_5(input: String, one: Vec<char>, four: Vec<char>) -> i32 {
  if one.iter().all(|c| input.contains(*c) ){
    return 3;
  }

  let data: Vec<_> = input.chars().collect();
  let without_one_or_four: Vec<_> = data.clone().into_iter().filter(|item| !four.contains(item) && !one.contains(item)).collect();

  if without_one_or_four.len() == 3 {
    return 2;
  }

  return 5;
}

fn six_or_nine_or_zero(input: String, one: Vec<char>, four: Vec<char>) -> i32 {
  if four.iter().all(|c| input.contains(*c) ){
    return 9;
  }

  if one.iter().all(|c| input.contains(*c) ){
    return 0;
  }

  return 6;
}