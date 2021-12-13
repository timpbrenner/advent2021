use std::fs::File;
use std::io::prelude::*;

pub fn flashes() {
    let mut file = File::open("data/day11.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in contents.lines() {
      grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut flash_count = 0;
    for day in 1..1000 {
      for x in 0..10 {
        for y in 0..10 {
          grid[x][y] += 1;

          if grid[x][y] == 10 {
            flash_count += 1;

            flash(x.try_into().unwrap(), y.try_into().unwrap(), &mut grid, &mut flash_count);
          }
        }
      }

      let mut all_flash = true;
      for x in 0..10 {
        for y in 0..10 {
          if grid[x][y] >= 10 {
            grid[x][y] = 0;
          } else {
            all_flash = false;
          }
        }
      }

      if all_flash {
        println!("All Flash on {}", day);
        break;
      }
    }
}

fn flash(x: usize, y: usize, grid: &mut Vec<Vec<u32>>, flash_count: &mut u32) {
  for i in 0..=2 {
    if x + i == 0 {
      continue;
    }

    let new_x = x + i - 1;
    if new_x > 9 {
      continue;
    }

    for j in 0..=2 {
      if y + j == 0 {
        continue;
      }

      let new_y = y + j - 1;
      if new_y > 9 {
        continue;
      }

      grid[new_x][new_y] += 1;

      if grid[new_x][new_y] == 10 {
        *flash_count += 1;

        flash(new_x, new_y, grid, flash_count);
      }
    }
  }
}
