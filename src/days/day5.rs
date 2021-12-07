use std::fs::File;
use std::io::prelude::*;

pub fn grid() {
    let mut file = File::open("data/day5.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut floor_grid = [[0; 1000]; 1000];

    for line in contents.lines() {
      let points = get_points(line.to_owned());
      let start = points[0];
      let end = points[1];

      if start[0] == end[0] {
        horizontal_add(start, end, &mut floor_grid);
      } else if start[1] == end[1] {
        vertical_add(start, end, &mut floor_grid);
      } else {
        diagonal_add(start, end, &mut floor_grid);
      }
    }

    score_grid(floor_grid);
}

fn get_points(line: String) -> [[i32; 2]; 2] {
  let mut points = line.split(" -> ");

  return [
    get_point(points.next().unwrap().to_owned()),
    get_point(points.next().unwrap().to_owned()),
  ];
}

fn get_point(point_string :String) -> [i32; 2] {
  let mut points = point_string.split(",");

  return [
    points.next().unwrap().parse::<i32>().unwrap(),
    points.next().unwrap().parse::<i32>().unwrap(),
  ];
}

fn horizontal_add(start: [i32; 2], end: [i32; 2], grid: &mut [[i32; 1000]; 1000]) {
  let x = start[0];
  let mut range = start[1]..=end[1];

  if start[1] > end[1] {
    range = end[1]..=start[1];
  }

  for y in range {
    grid[x as usize][y as usize] += 1;
  }
}

fn vertical_add(start: [i32; 2], end: [i32; 2], grid: &mut [[i32; 1000]; 1000]) {
  let y = start[1];
  let mut range = start[0]..=end[0];

  if start[0] > end[0] {
    range = end[0]..=start[0];
  }

  for x in range {
    grid[x as usize][y as usize] += 1;
  }
}

fn diagonal_add(start: [i32; 2], end: [i32; 2], grid: &mut [[i32; 1000]; 1000]) {
  let mut start_y = start[1];
  let mut slope = if start[1] < end[1] { 1 } else { -1 };
  let mut range = start[0]..=end[0];

  if start[0] > end[0] {
    range = end[0]..=start[0];
    slope = if end[1] < start[1] { 1 } else { -1 };
    start_y = end[1];
  }

  let mut count = 0;
  for x in range {
    let y = start_y + (slope * count);
    grid[x as usize][y as usize] += 1;

    count += 1;
  }
}

fn score_grid(floor_grid: [[i32; 1000]; 1000]) {
  let mut more_than_2 = 0;

  for x in 0..1000 {
    for y in 0..1000 {
      if floor_grid[x][y] >= 2 {
        more_than_2 += 1;
      }
    }
  }

  println!("Floor Line Intersections: {}", more_than_2);
}