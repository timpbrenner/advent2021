use crate::util::open_file;
use std::collections::VecDeque;
use std::collections::HashMap;

const GRID_SIZE: usize = 25;


pub fn lava() {
  let contents = open_file("data/day18.txt");

  let mut grid:Vec<Vec<Vec<bool>>> = vec![vec![vec![false;GRID_SIZE];GRID_SIZE];GRID_SIZE];

  for line in contents.lines() {
    let mut coords = line.split(',');
    let x = coords.next().unwrap().parse::<usize>().unwrap();
    let y = coords.next().unwrap().parse::<usize>().unwrap();
    let z = coords.next().unwrap().parse::<usize>().unwrap();
    
    grid[x][y][z] = true;
  }

  let mut surface_area:u32 = 0;
  for x in 0..GRID_SIZE {
    for y in 0..GRID_SIZE {
      for z in 0..GRID_SIZE {
        if !grid[x][y][z] {
          continue;
        }

        surface_area += grid_surface_area(x,y,z, &grid);
      }
    }
  }

  println!("Surface Area: {}", surface_area);


  // Part 2
  let a = fill_air(&grid);
  println!("Exterior Surface Area: {}", surface_area - a as u32);
}

fn fill_air(grid: &Vec<Vec<Vec<bool>>> ) -> i32 {
  let mut air: HashMap<(i32, i32, i32), bool> = HashMap::new();
  for x in 0..GRID_SIZE {
    for y in 0..GRID_SIZE {
      for z in 0..GRID_SIZE {
        if !grid[x][y][z] {
          air.insert((x as i32, y as i32, z as i32), false);
        }
      }
    }
  }
  let start: (i32, i32, i32) = (0,0,0);
  let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
  q.push_back(start);
  while !q.is_empty() {
    let coord = q.pop_front().unwrap();
    air.insert(coord, true);
    let (cx, cy, cz) = coord;

    for neighbor in neighbors(cx as i32,cy as i32,cz as i32) {
      if air.contains_key(&neighbor) && (! air[&neighbor]) && (! q.contains(&neighbor)) {
        q.push_back(neighbor);
      }
    }
  }
  let inside_cubes: Vec<(i32, i32, i32)> = air.keys().cloned().filter(|c| air[c] == false).collect();
  return calc_area(&inside_cubes);
}

fn neighbors(x:i32,y:i32,z:i32) -> [(i32, i32, i32); 6] {
  [
    (x + 1, y    , z    ), (x - 1, y,     z    ),
    (x,     y + 1, z    ), (x,     y - 1, z    ),
    (x,     y    , z + 1), (x,     y,     z - 1)
  ]
}

fn calc_area(points: &Vec<(i32, i32, i32)>) -> i32 {
  let mut area: i32 = 0;
  for (x, y, z) in points {
    for n in neighbors(*x,*y,*z) {
      if ! points.contains(&n) {
        area += 1;
      }
    }
  }

  return area;
}

fn grid_surface_area(x:usize, y:usize, z:usize, grid:& Vec<Vec<Vec<bool>>>) -> u32 {
  let mut surface_area:u32 = 0;
  
  if !grid[x + 1][y][z] {
    surface_area += 1;
  }

  if x == 0 || !grid[x - 1][y][z] {
    surface_area += 1;
  }

  if !grid[x][y + 1][z] {
    surface_area += 1;
  }

  if y == 0 || !grid[x][y - 1][z] {
    surface_area += 1;
  }

  if !grid[x][y][z + 1] {
    surface_area += 1;
  }

  if z == 0 || !grid[x][y][z - 1] {
    surface_area += 1;
  }

  return surface_area;
}