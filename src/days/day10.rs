use crate::util::open_file;

pub fn cycle() {
  let contents = open_file("data/day10.txt");

  let relevant_cycles:Vec<i32> = Vec::from([20,60,100,140,180,220]);
  let mut sum_total = 0;

  let mut x: i32 = 1;
  let mut cycle: i32 = 1;

  let mut grid:Vec<Vec<char>> = Vec::from([
    Vec::new(),
    Vec::new(),
    Vec::new(),
    Vec::new(),
    Vec::new(),
    Vec::new()
  ]);

  for line in contents.lines() {
    let row = ((cycle as f64 - 1.0) / 40.0).floor();
    grid[row as usize].push(display_char(cycle - 1, x));

    if line.starts_with("noop") {
      if relevant_cycles.contains(&cycle) {
        println!("NOOP {}: {}", cycle, x);
        sum_total += cycle * x;
      }
      cycle += 1;

      continue;
    }

    let mut command_parts = line.split(' ').skip(1);
    let integer:i32 = command_parts.next().unwrap().parse::<i32>().unwrap();
    if relevant_cycles.contains(&cycle) {
      println!("ADD {}: {}", cycle, x);
      sum_total += cycle * x;
    }
    cycle += 1;

    if cycle == 9 {
      println!("MIDDLE: {}", x);
    }

    let row = ((cycle as f64 - 1.0) / 40.0).floor();
    grid[row as usize].push(display_char(cycle - 1, x));

    if relevant_cycles.contains(&cycle) {
      println!("ADD {}: {}", cycle, x);
      sum_total += cycle * x;
    }
    cycle += 1;

    x += integer;
  }
  println!("Signal Strength: {}", sum_total);

  println!("{}", String::from_iter(grid[0].clone()));
  println!("{}", String::from_iter(grid[1].clone()));
  println!("{}", String::from_iter(grid[2].clone()));
  println!("{}", String::from_iter(grid[3].clone()));
  println!("{}", String::from_iter(grid[4].clone()));
  println!("{}", String::from_iter(grid[5].clone()));
}

fn display_char(cycle:i32, x:i32) -> char {
  if x - 1 <= cycle % 40 && cycle % 40 <= x + 1 { 
    return '#'; 
  }

  '.' 
}