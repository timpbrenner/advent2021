use crate::util::open_file;

use std::ops::Range;

pub fn visible() {
  let contents = open_file("data/day8.txt");

  let mut grid: Vec<Vec<u32>> = Vec::new();
  for line in contents.lines() {
    let mut row:Vec<u32> = Vec::new();

    for c in line.chars() {
      row.push(c.to_digit(10).unwrap());
    }

    grid.push(row);
  }

  let width:u32 = grid[0].len().try_into().unwrap();
  let height:u32 = grid.len().try_into().unwrap();

  let border = (width * 2) + (height - 2) * 2;
  let mut interior_visible = 0;

  let mut best_view = 0;
  for i in 1..=(grid[0].len() - 2) {
    for j in 1..=(grid.len() - 2) {
      if is_visible(
        i.try_into().unwrap(), 
        j.try_into().unwrap(), 
        &grid,
        width,
        height
      ) {
        interior_visible += 1;
      }
    }
  }

  for i in 0..=(grid[0].len() - 1) {
    for j in 0..=(grid.len() - 1) {
      let view_score = get_view_score(
        i.try_into().unwrap(), 
        j.try_into().unwrap(), 
        &grid,
        width,
        height
      );

      if view_score > best_view {
        best_view = view_score;
      }
    }
  }

  println!("{} + {} = {}", border, interior_visible, border + interior_visible);
  println!("Best View: {}", best_view);
}

fn is_visible(x:u32, y:u32, grid: &Vec<Vec<u32>>, width: u32, height: u32) -> bool {
  let tree_height = grid[y as usize][x as usize] as u32;

  return check_x_dir(0..x, &grid[y as usize], tree_height) ||
    check_x_dir(x+1..width, &grid[y as usize], tree_height) ||
    check_y_dir(0..y, x, &grid, tree_height) ||
    check_y_dir(y+1..height, x, &grid, tree_height);

}

fn check_x_dir(mut x_range:Range<u32>, row: &Vec<u32>, height:u32) -> bool {
  return x_range.all(|x| row[x as usize] < height);
}

fn check_y_dir(mut y_range:Range<u32>, x: u32, grid: &Vec<Vec<u32>>, height:u32) -> bool {
  return y_range.all(|y| grid[y as usize][x as usize] < height);
}

fn get_view_score(x:u32, y:u32, grid: &Vec<Vec<u32>>, width: u32, height: u32) -> u32 {
  let tree_height = grid[y as usize][x as usize] as u32;

  let mut total_score = 1;

  let local_x: i32 = x as i32;
  let local_y: i32 = y as i32;

  let mut offset:i32 = 1;
  let mut direction_score:i32 = 0;
  while local_x - offset >= 0 {
    let this_x = local_x - offset;
    let val = grid[y as usize][this_x as usize];
    direction_score +=1;

    if val >= tree_height {
      break;
    }

    offset += 1;
  }
  total_score *= direction_score;

  offset = 1;
  direction_score = 0;
  while local_x + offset < width as i32 {
    let this_x = local_x + offset;
    let val = grid[y as usize][this_x as usize];
    direction_score +=1;

    if val >= tree_height {
      break;
    }

    offset += 1;
  }
  total_score *= direction_score;

  offset = 1;
  direction_score = 0;
  while local_y - offset >= 0 {
    let this_y = local_y - offset;
    let val = grid[this_y as usize][x as usize];
    direction_score +=1;

    if val >= tree_height {
      break;
    }

    offset += 1;
  }
  total_score *= direction_score;

  offset = 1;
  direction_score = 0;
  while local_y + offset < height as i32 {
    let this_y = local_y + offset;
    let val = grid[this_y as usize][x as usize];
    direction_score +=1;

    if val >= tree_height {
      break;
    }

    offset += 1;
  }
  total_score *= direction_score;

  return total_score as u32;
}