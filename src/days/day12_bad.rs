use crate::util::open_file;

// Sabqponm
// abcryxxl
// accszExk
// acctuvwj
// abdefghi

#[derive(Debug, Eq, Hash, Copy, Clone)]
struct Point {
  x: i32,
  y: i32
}

impl Point {
  fn new(x:i32, y:i32) -> Point {
    Point { x: x, y: y }
  }

  fn add(&mut self, x:i32, y:i32) {
    self.x += x;
    self.y += y;
  }
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
      self.x == other.x && 
        self.y == other.y
  }
}

pub fn hike() {
  let contents = open_file("data/day12.txt");

  let mut grid: Vec<Vec<char>> = Vec::new();
  let mut start:Point = Point::new(0,0);
  let mut end:Point = Point::new(0,0);

  for line in contents.lines() {
    let mut row:Vec<char> = Vec::new();

    for c in line.chars() {
      if c == 'S' {
        start = Point::new(grid.len().try_into().unwrap(), row.len().try_into().unwrap());
      }

      if c == 'E' {
        end = Point::new(grid.len().try_into().unwrap(), row.len().try_into().unwrap());
      }

      row.push(c);
    }

    grid.push(row);
  }

  println!("Start: {},{}", start.x, start.y);
  println!("End: {},{}", end.x, end.y);

  for row in &grid {
    println!("{:?}", row);
  }

  let mut passed:Vec<Point> = Vec::from([start]);
  let mut move_count = 0;
  let smallest_path = move_dir(start, end, grid, &mut passed, move_count);

  println!("Smallest Path: {}", smallest_path);
}

fn move_dir(start:Point, end:Point, grid:Vec<Vec<char>>, passed:&mut Vec<Point>, mut move_count:u32) -> u32 {
  println!("{},{}", start.x, start.y);

  let width:i32 = grid[0].len().try_into().unwrap();
  let height:i32 = grid.len().try_into().unwrap();

  let right = Point::new(start.x + 1, start.y);
  let left = Point::new(start.x - 1, start.y);
  let up = Point::new(start.x, start.y - 1);
  let down = Point::new(start.x, start.y + 1);

  if start == end {
    println!("REACHED END");
    return move_count;
  }

  let mut move_counts:Vec<u32> = Vec::new();
  if right.x <= width && right.x >= 0 && !passed.contains(&right) {
    passed.push(start);
    move_count += 1;

    move_counts.push(move_dir(right, end, grid.clone(), &mut passed.clone(), move_count));
  }

  if left.x <= width && left.x >= 0 && !passed.contains(&left) {
    passed.push(start);
    move_count += 1;

    move_counts.push(move_dir(left, end, grid.clone(), &mut passed.clone(), move_count));
  }

  if up.y <= height && up.y >= 0 && !passed.contains(&up) {
    passed.push(start);
    move_count += 1;

    move_counts.push(move_dir(up, end, grid.clone(), &mut passed.clone(), move_count));
  }

  if down.y <= height && down.y >= 0 && !passed.contains(&down) {
    passed.push(start);
    move_count += 1;

    move_counts.push(move_dir(down, end, grid.clone(), &mut passed.clone(), move_count));
  }

  let counts:Vec<u32> = move_counts.into_iter().filter(|c| c > &0).collect::<Vec<u32>>();
  if counts.len() > 0 {
    println!("{:?}", counts);
    return counts.into_iter().min().unwrap();
  }

  return 0;
}