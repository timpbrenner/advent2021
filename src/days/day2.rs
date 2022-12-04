use std::fs::File;
use std::io::prelude::*;

// Rock
// A  

// Paper
// B

// Scissor
// C

// lose = X
// draw = y
// win = z

pub fn shoot() {
    let mut file = File::open("data/day2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut total_score = 0;
    for line in contents.lines() {
      let mut score = 0;

      let mut chars = line.chars();
      let player1 = chars.nth(0).unwrap();
      let outcome = chars.nth(1).unwrap();
      let player2 = get_player2(player1, outcome);

      if player2 == 'X' {
        score += 1;
      } else if player2 == 'Y' {
        score += 2;
      } else if player2 == 'Z' {
        score += 3;
      }

      if outcome == 'Y' {
        score += 3;
      }

      if outcome == 'Z' {
        score += 6;
      }

      total_score += score;
    }

    println!("Total Score: {}", total_score);
}

fn get_player2(player1:char, outcome:char) -> char {
  if outcome == 'X' {
    if player1 == 'A' {
      return 'Z';
    } else if player1 == 'B' {
      return 'X';
    } else if player1 == 'C' {
      return 'Y';
    }
  }

  if outcome == 'Y' {
    if player1 == 'A' {
      return 'X';
    } else if player1 == 'B' {
      return 'Y';
    } else if player1 == 'C' {
      return 'Z';
    }
  }

  if player1 == 'A' {
    return 'Y';
  } else if player1 == 'B' {
    return 'Z';
  }

  return 'X';
}
