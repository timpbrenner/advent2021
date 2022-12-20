use crate::util::open_file;

use std::collections::VecDeque;

pub fn tetris() {
  let contents = open_file("data/day17_test.txt");
  let mut jets = contents.chars().collect::<Vec<char>>();

  let mut board:VecDeque<VecDeque<char>> = VecDeque::from([
    VecDeque::from(['.','.','.','.','.','.','.']),
    VecDeque::from(['.','.','.','.','.','.','.']),
    VecDeque::from(['.','.','.','.','.','.','.']),
  ]);

  let mut jet_index = 0;

  for tick in 1..=2022 {
    trim_board(&mut board, 3);

    let piece = get_piece(tick);

    for row in piece {
      board.push_front(row);
    }
    // display_board(&board);

    loop {
      jet_of_air(jets[(jet_index % jets.len() as u32) as usize], &mut board);
      jet_index += 1;

      if !fall(&mut board){
        break;
      }
    }
    // display_board(&board);
  }

  trim_board(&mut board, 0);
  println!("ROW COUNT: {}", board.len());
}

fn trim_board(board: &mut VecDeque<VecDeque<char>>, leave_count:u32) {
  let mut empty_row_count = 0;
  for row in board.clone() {
    if !row.contains(&'#') && !row.contains(&'@') {
      empty_row_count += 1;
    }
  }

  if empty_row_count == 3 {
    return;
  }

  for i in 0..(empty_row_count - leave_count) {
    board.pop_front();
  }
}

fn fall(board: &mut VecDeque<VecDeque<char>>) -> bool {
  let mut success = true;

  for y in (0..board.len()).rev() {
    for x in 0..board[y].len() {
      if board[y][x] != '@' {
        continue;
      }

      if y + 1 == board.len() || board[y + 1][x] == '#' {
        success = false;
        break;;
      }
    }

    for x in 0..board[y].len() {
      if board[y][x] != '@' {
        continue;
      }

      if success {
        board[y + 1][x] = '@';
        board[y][x] = '.';
      } else {
        board[y][x] = '#';
      }
    }
  }

  success
}

fn jet_of_air(jet: char, board: &mut VecDeque<VecDeque<char>>) {
  let mut new_board = board.clone();

  for mut row in new_board.iter_mut().rev() {
    for x in 0..row.len() {
      let c = row[x];
      if c != '@' {
        continue;
      }

      match jet {
        '>' => {
          if x == row.len() - 1 || row[x + 1] == '#' {
            return;
          }
        },
        '<' => {
          if x == 0 || row[x - 1] == '#' {
            return;
          }
        },
        _ => todo!(),
      }
    }
  }

  for mut row in board {
    match jet {
      '>' => {
        for x in (0..row.len() - 1).rev() {
          if row[x] != '@' {
            continue;
          }

          row[x] = '.';
          row[x + 1] = '@';
        }
      },
      '<' => {
        for x in 0..row.len() {
          if row[x] != '@' {
            continue;
          }

          row[x] = '.';
          row[x - 1] = '@';
        }
      },
      _ => todo!(),
    }
  }
}

fn display_board(board: &VecDeque<VecDeque<char>>) {
  for row in board {
    println!("|{}|", row.iter().collect::<String>());
  }

  println!("+-------+");
  println!("");
}

fn get_piece(tick: u32) -> Vec<VecDeque<char>> {
  match tick % 5 {
    1 => {
      return Vec::from([
        VecDeque::from(['.','.','@','@','@','@','.'])
      ]);
    },
    2 => {
      return Vec::from([
        VecDeque::from(['.','.','.','@','.','.','.']),
        VecDeque::from(['.','.','@','@','@','.','.']),
        VecDeque::from(['.','.','.','@','.','.','.']),
      ]);
    },
    3 => {
      return Vec::from([
        VecDeque::from(['.','.','@','@','@','.','.']),
        VecDeque::from(['.','.','.','.','@','.','.']),
        VecDeque::from(['.','.','.','.','@','.','.']),
      ]);
    },
    4 => {
      return Vec::from([
        VecDeque::from(['.','.','@','.','.','.','.']),
        VecDeque::from(['.','.','@','.','.','.','.']),
        VecDeque::from(['.','.','@','.','.','.','.']),
        VecDeque::from(['.','.','@','.','.','.','.']),
      ]);
    },
    0 => {
      return Vec::from([
        VecDeque::from(['.','.','@','@','.','.','.']),
        VecDeque::from(['.','.','@','@','.','.','.']),
      ]);
    },
    _ => todo!(),
  }
}