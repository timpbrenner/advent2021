
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn syntax_matching() {
    let mut file = File::open("data/day10.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let closure_map = closure_map();
    let mut total_scores = Vec::<i64>::new();

    for line in contents.lines() {

      if !valid_line(line) {
        continue;
      }

      let mut stack = Vec::<char>::new();
      for c in line.chars() {
        if closure_map.contains_key(&c) {
          stack.push(c);
        } else {
          stack.pop();
        }
      }

      let score = score_leftover(stack.clone());
      total_scores.push(score);
    }

    total_scores.sort();
    let selected_index = total_scores.len() / 2;
    println!("Middle Score: {}", total_scores[selected_index]);
}

fn valid_line(line: &str) -> bool {
    let closure_map = closure_map();
    let mut stack = Vec::<char>::new();

    for c in line.chars() {
      if closure_map.contains_key(&c) {
        stack.push(c);
      } else {
        let matcher = stack.pop().unwrap();
        let expected = closure_map.get(&matcher).unwrap();

        if *expected != c {
          return false;
        }
      }
    }

    return true;
}

fn score_leftover(mut chars: Vec<char>) -> i64 {
  let mut total: i64 = 0;
  let score_map = HashMap::from([
    ('(', 1),
    ('[', 2),
    ('{', 3),
    ('<', 4),
  ]);

  chars.reverse();
  for c in chars {
    total *= 5;
    total += score_map.get(&c).unwrap();
  }
  
  total
}

fn closure_map() -> HashMap<char, char> {
  HashMap::from([
    ('(', ')'),
    ('[', ']'),
    ('{', '}'),
    ('<', '>'),
  ])
}

// fn score_map() -> HashMap<char, u32> {
//   HashMap::from([
//     (')', 3),
//     (']', 57),
//     ('}', 1197),
//     ('>', 25137),
//   ])
// }