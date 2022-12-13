use crate::util::open_file;
use std::fmt::*;

extern crate pathfinding;

use pathfinding::prelude::astar;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
  fn distance(&self, other: &Pos) -> u32 {
    (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
  }
}

impl Display for Pos {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

pub fn hike() {
  let contents = open_file("data/day12.txt");

  let mut grid: Vec<Vec<char>> = Vec::new();
  // let mut start:Pos = Pos(0,0);
  let mut end:Pos = Pos(0,0);

  let mut starts:Vec<Pos> = Vec::new();

  for line in contents.lines() {
    let mut row:Vec<char> = Vec::new();

    for c in line.chars() {
      if c == 'a' {
        starts.push(Pos(grid.len().try_into().unwrap(), row.len().try_into().unwrap()));
      }

      // if c == 'S' {
      //   start = Pos(grid.len().try_into().unwrap(), row.len().try_into().unwrap());
      // }

      if c == 'E' {
        end = Pos(grid.len().try_into().unwrap(), row.len().try_into().unwrap());
      }

      row.push(c);
    }

    grid.push(row);
  }

  let mut results:Vec<u32> = Vec::new();
  for s in starts {
    results.push(run_algo(s, end, grid.clone()));
  }

  println!("Smallest Path: {}", results.iter().min().unwrap());
}

fn run_algo(start:Pos, end:Pos, grid:Vec<Vec<char>>) -> u32 {
  let successors = |p: &Pos| -> Vec<(Pos, u32)> { 
    let &Pos(x, y) = p;
     
    let get_height = |grid:&Vec<Vec<char>>, x:usize, y:usize| {
      if end.0 == x.try_into().unwrap() && end.1 == y.try_into().unwrap() {
        return 123;
      }

      grid[x][y] as u32
    };

    let x_exists:bool = grid.get(x as usize).is_some();
    let y_exists:bool = grid[x as usize].get(y as usize).is_some();

    let mut base_height:u32 = 0;
    // This is for section 1, could clean up but won't
    // if &start == p {
    //   base_height = 100000000;
    // } else 
    if &end == p {
      base_height = 130;
    } else if x_exists && y_exists {
      base_height = grid[x as usize][y as usize] as u32;
    }

    let mut s:Vec<Pos> = Vec::new();

    if grid.get((x + 1) as usize).is_some() && 
      grid[(x + 1) as usize].get(y as usize).is_some() &&
      get_height(&grid,(x + 1) as usize, y as usize) <= base_height + 1
      {
      s.push(Pos(x+1,y));
    }

    if grid.get((x - 1) as usize).is_some() && 
      grid[(x - 1) as usize].get(y as usize).is_some() &&
      get_height(&grid,(x - 1) as usize, y as usize) <= base_height + 1

      {

      s.push(Pos(x-1,y));
    }

    if grid.get(x as usize).is_some() && 
      grid[x as usize].get((y + 1) as usize).is_some() &&
      get_height(&grid,x as usize, (y + 1) as usize) <= base_height + 1
      {

      s.push(Pos(x,y + 1));
    }

    if grid.get(x as usize).is_some() && 
      grid[x as usize].get((y - 1) as usize).is_some() &&
      get_height(&grid,x as usize, (y - 1) as usize) <= base_height + 1
      {

      s.push(Pos(x,y - 1));
    }

    s.into_iter().map(|p| (p, 1)).collect()
  };

  let result = astar(
    &start, 
    successors, 
    |p| p.distance(&end),
    |p| *p == end
  );

  if result.is_none() {
    return 10000;
  }

  let b = result.unwrap();
  return b.1;
}
