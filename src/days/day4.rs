
use std::str::Lines;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct BingoBoard {
    board: Vec::<Vec::<i32>>,
    selection: Vec::<Vec::<i32>>,
}

struct Winner {
    board: BingoBoard,
    last_number: i32,
}

impl BingoBoard {
    fn new() -> BingoBoard {
        return BingoBoard {
            board: vec![vec![0; 5]; 5],
            selection: vec![vec![0; 5]; 5],
        };
    }

    fn make_selection(&mut self, selection: i32) {
        for x in 0..5 {
            for y in 0..5 {
                if self.board[y][x] == selection {
                    self.selection[y][x] = 1;
                }
            }
        }
    }

    fn winner(&self) -> bool {
        for x in 0..5 {
            if self.selection[x] == vec![1,1,1,1,1] {
                return true;
            }
        }

        for y in 0..5 {
            let column = vec![
                self.selection[0][y],
                self.selection[1][y],
                self.selection[2][y],
                self.selection[3][y],
                self.selection[4][y],
            ];

            if column == vec![1,1,1,1,1] {
                   return true;
            }
        }

        return false;
    }

    fn score(&self) -> i32 {
        let mut sum = 0;

        for x in 0..5 {
            for y in 0..5 {
                if self.selection[y][x] == 0 {
                    sum += self.board[y][x];
                }
            }
        }

        return sum;
    }
}

pub fn bingo() {
    let mut file = File::open("data/day4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines();
    let selections = lines.next().unwrap().to_string();

    lines.next();

    let boards = populate_boards(lines);

    let winner = make_selections(boards, selections).unwrap();
    let board = winner.board;
    println!("Board Score: {} Last: {} Score: {}", board.score(), winner.last_number, winner.last_number * board.score());
}

pub fn last_bingo() {
    let mut file = File::open("data/day4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines();
    let selections = lines.next().unwrap().to_string();

    lines.next();

    let boards = populate_boards(lines);

    let winner = make_last_selections(boards, selections).unwrap();
    let board = winner.board;
    println!("Last Board Score: {} Last: {} Score: {}", board.score(), winner.last_number, winner.last_number * board.score());
}

fn populate_boards(lines: Lines) -> Vec<BingoBoard> {
    let mut boards = Vec::<BingoBoard>::new();

    let mut line_count = 0;
    let mut board = BingoBoard::new();

    for line in lines {
        if line_count == 5 {
            line_count = 0;
            continue;
        }

        board.board[line_count] = line.split_whitespace().map(|s| s.parse::<i32>().unwrap() ).collect();

        if line_count == 4 {
            boards.push(board);
            board = BingoBoard::new();
        }

        line_count += 1;
    }

    return boards;
}

fn make_selections(mut boards: Vec<BingoBoard>, selections: String) -> Option<Winner> {
    for selection in selections.split(',') {
        for board in boards.iter_mut() {
            let pull = selection.parse::<i32>().unwrap();
            board.make_selection(pull);

            if board.winner() {
                let winner = Winner {
                    board: board.clone(),
                    last_number: pull,
                };

                return Some(winner);
            }
        }
    }

    None
}

fn make_last_selections(mut boards: Vec<BingoBoard>, selections: String) -> Option<Winner> {
    for selection in selections.split(',') {
        let pull = selection.parse::<i32>().unwrap();

        for board in boards.iter_mut() {
            board.make_selection(pull);
        }

        if boards.len() > 1 {
            boards.retain(|b| !b.winner());
        } else if boards[0].winner() {
            let winner = Winner {
                board: boards[0].clone(),
                last_number: pull,
            };

            return Some(winner);
        }
    }

    None
}