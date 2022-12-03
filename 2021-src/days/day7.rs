use std::fs::File;
use std::io::prelude::*;

pub fn crab_sub() {
    let mut file = File::open("data/day7.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sub_pos: Vec<i32> = contents.split(",").map(|c| c.parse::<i32>().unwrap()).collect();
    let max_pos = sub_pos.iter().max().unwrap();

    let mut min_pos = 0;
    let mut min_fuel = 702278796;

    for pos in 0..=*max_pos {
      let mut fuel_cons = 0;

      for crab_pos in &sub_pos {
        fuel_cons += get_fuel_cost((crab_pos - pos).abs(), true);
      }

      if fuel_cons < min_fuel {
        min_fuel = fuel_cons;
        min_pos = pos;
      }
    }

    println!("Final Min Pos: {} Min Fuel {}", min_pos, min_fuel);
}

fn get_fuel_cost(movement: i32, simple: bool) -> i32 {
  if simple {
    return movement;
  }

  let mut total = 0;
  let mut cost = 1;

  for _ in 1..=movement {
    total += cost;
    cost += 1;
  }

  return total;
}
