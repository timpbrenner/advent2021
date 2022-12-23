use crate::util::open_file;
use std::fmt::*;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Display for Pos {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

pub fn drip() {
  let contents = open_file("data/day14.txt");

  let mut highest_y = 0;
  let mut grid:Vec<Vec<bool>> = build_grid(contents, &mut highest_y);

  for i in 0..1000 {
    grid[i][(highest_y + 2) as usize] = true;
  }

  let mut sands = 0;
  loop {
    if !drop_sand(&mut grid) {
      break;
    }

    sands +=1 ;
  }

  display_grid(&grid);
  println!("Sands: {}", sands);
}

fn drop_sand(grid:&mut Vec<Vec<bool>>) -> bool {
  let mut sand_pos = Pos(500,0);

  loop {
    if grid[500][0] {
      return false;
    }

    let mut down = sand_pos.clone();
    let mut left = sand_pos.clone();
    let mut right = sand_pos.clone();
    down.1 +=1;

    left.1 +=1;
    left.0 -=1;

    right.1 +=1;
    right.0 +=1;

    if grid[down.0 as usize][down.1 as usize] {
      if grid[left.0 as usize][left.1 as usize] {
        if grid[right.0 as usize][right.1 as usize] {
          break;
        } else {
          sand_pos = right;
        }
      } else {
        sand_pos = left;
      }
    } else {
      sand_pos = down;
    }
  }

  grid[sand_pos.0 as usize][sand_pos.1 as usize] = true;
  return true;
}

fn build_grid(input: String, highest_y: &mut i32) -> Vec<Vec<bool>> {
  let mut grid:Vec<Vec<bool>> = vec![vec![false; 1000]; 1000]; 

  for line in input.lines() {
    let mut points = line.split(" -> ");
    let mut start:Pos = point_to_pos(points.next().unwrap().to_string());

    for point in points {
      let end = point_to_pos(point.to_string());

      if start.0 == end.0 {
        let mut s = start.1;
        let mut e = end.1;

        if start.1 > end.1 {
          s = end.1;
          e = start.1;
        }

        for y in s..=e {
          if y > *highest_y {
            *highest_y = y;
          }
          grid[start.0 as usize][y as usize] = true;
        }
      } else if start.1 == end.1 {
        let mut s = start.0;
        let mut e = end.0;

        if start.0 > end.0 {
          s = end.0;
          e = start.0;
        }

        for x in s..=e {
          if start.1 > *highest_y {
            *highest_y = start.1;
          }

          grid[x as usize][start.1 as usize] = true;
        }
      }

      start = end;
    }
  }

  grid
}

fn point_to_pos(point:String) -> Pos {
  let mut start_point = point.split(',');
  Pos(
    start_point.next().unwrap().parse::<i32>().unwrap(),
    start_point.next().unwrap().parse::<i32>().unwrap()
  )
}

fn display_grid(grid:&Vec<Vec<bool>>) {
  for y in 0..11 {
    let mut line:Vec<char> = Vec::new();
    for x in 493..=505 {
      let display = if grid[x][y] { 'x' } else { '.' };
      line.push(display);
    }
    println!("{}", line.iter().collect::<String>());
  }
}