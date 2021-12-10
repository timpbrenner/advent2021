use std::fs::File;
use std::io::prelude::*;

pub fn low_danger() {
    let mut file = File::open("data/day9.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in contents.lines() {
      grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let dim = grid.len();
    let mut total_risk = 0;
    let mut basin_sizes = Vec::<u32>::new();

    for x in 0..dim {
      for y in 0..dim {
        let mut surrounding: Vec<u32> = Vec::new();

        if x != 0 { surrounding.push(grid[x - 1][y]); }
        if y != 0 { surrounding.push(grid[x][y - 1]); }
        if x != (dim - 1) { surrounding.push(grid[x + 1][y]); }
        if y != (dim - 1) { surrounding.push(grid[x][y + 1]); }

        if surrounding.iter().all(|s| s > &grid[x][y]) {
          total_risk += 1 + grid[x][y];

          basin_sizes.push(find_basin(vec!(x as u32, y as u32), grid.clone()));
        }
      }
    }

    println!("Total Risk: {}", total_risk);
    basin_sizes.sort();
    println!("Basin Sizes: {:?}", basin_sizes);
}

fn find_basin(point: Vec<u32>, grid: Vec<Vec<u32>>) -> u32 {
  let mut basin = Vec::<Vec<u32>>::new();

  add_to_basin(&mut basin, point.clone(), grid);
  return basin.len().try_into().unwrap();
}

fn add_to_basin(basin: &mut Vec<Vec<u32>>, point: Vec<u32>, grid: Vec<Vec<u32>>) {
  let x = point[0];
  let y = point[1];

  basin.push(point.clone());

  if x > 0 && grid[(x - 1) as usize][y as usize] < 9 && !basin.contains(&Vec::from([x - 1, y])) {
    add_to_basin(basin, Vec::from([x - 1, y]), grid.clone());
  }

  if x < 99 && grid[(x + 1) as usize][y as usize] < 9 && !basin.contains(&Vec::from([x + 1, y])) {
    add_to_basin(basin, Vec::from([x + 1, y]), grid.clone());
  }

  if y > 0 && grid[x as usize][(y - 1) as usize] < 9 && !basin.contains(&Vec::from([x, y - 1])) {
    add_to_basin(basin, Vec::from([x, y - 1]), grid.clone());
  }

  if y < 99 && grid[x as usize][(y + 1) as usize] < 9 && !basin.contains(&Vec::from([x, y + 1])) {
    add_to_basin(basin, Vec::from([x, y + 1]), grid.clone());
  }
}