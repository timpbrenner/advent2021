use crate::util::open_file;
use regex::Regex;
use std::fmt::*;
use std::collections::HashSet;

use std::ops::RangeInclusive;

use itertools::Itertools;

struct Signal {
  location: Pos,
  beacon: Pos,
  distance: i32,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Display for Pos {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

pub fn beacon() {
  let contents = open_file("data/day15.txt");
  let signals = get_signals(contents);

  let mut x_offset:i32 = 0;
  let mut y_offset:i32 = 0;
  for signal in &signals {
    if signal.location.0 - signal.distance < x_offset {
      x_offset = signal.location.0 - signal.distance;
    }

    if signal.beacon.0 - signal.distance < x_offset {
      x_offset = signal.beacon.0 - signal.distance;
    }

    if signal.location.1 - signal.distance < y_offset {
      y_offset = signal.location.1 - signal.distance;
    }

    if signal.beacon.1 - signal.distance < y_offset {
      y_offset = signal.beacon.1 - signal.distance;
    }
  }

  // let mut grid:Vec<Vec<char>> = populate_grid(&signals);
  // display_grid(&grid, y_offset as usize);

  // print_bad_count(&signals);

  let input = parse().unwrap();
  let size = 4_000_000;

  let (row, col_ranges) = (0..=size)
      // not needed but faster
      .rev()
      .map(|row| (row, row_ranges(row, &input)))
      // if there is more than one range covering the row, there is a gap!
      .find(|(_, ranges)| ranges.len() > 1)
      .unwrap();

  let col = col_ranges.first().unwrap().end() + 1;

  println!("PART 2: {}", i64::from(col) * 4_000_000 + i64::from(row));
}



impl Pos {
    fn manhattan(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

fn parse() -> Option<Vec<[Pos; 2]>> {
    let input = open_file("data/day15.txt");

    let mut pairs = Vec::new();
    for line in input.lines() {
      let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
      let cap = re.captures(line).unwrap();

      let signal_x:i32 = cap[1].parse::<i32>().unwrap();
      let signal_y:i32 = cap[2].parse::<i32>().unwrap();

      let beacon_x:i32 = cap[3].parse::<i32>().unwrap();
      let beacon_y:i32 = cap[4].parse::<i32>().unwrap();

      let pair = [
        Pos(signal_x, signal_y),
        Pos(beacon_x, beacon_y),
      ];

      pairs.push(pair);
    }

    Some(pairs)
}

fn beacon_row_range(sensor: &Pos, beacon: &Pos, row: i32) -> Option<RangeInclusive<i32>> {
    let radius = sensor.manhattan(beacon);
    let offset = radius - (sensor.0 - row).abs();
    if offset < 0 {
        None
    } else {
        Some(sensor.1 - offset..=sensor.1 + offset)
    }
}

fn row_ranges(row: i32, pairs: &[[Pos; 2]]) -> Vec<RangeInclusive<i32>> {
    let mut ranges: Vec<_> = pairs
        .iter()
        .flat_map(|pair| beacon_row_range(&pair[0], &pair[1], row))
        .collect();
    ranges.sort_unstable_by_key(|range| *range.start());

    let mut merged_ranges = vec![ranges[0].clone()];
    for next in &ranges[1..] {
        let last_idx = merged_ranges.len() - 1;
        let last = &merged_ranges[last_idx];
        // check if the two sorted ranges overlap
        // create a single bigger range if possible
        if next.start() <= last.end() || last.end() + 1 == *next.start() {
            if next.end() > last.end() {
                let old = &merged_ranges[last_idx];
                let new = *old.start()..=*next.end();
                merged_ranges[last_idx] = new;
            }
        } else {
            merged_ranges.push(next.clone());
        }
    }

    merged_ranges
}

fn print_bad_count(signals: &Vec<Signal>) {
  let mut bad_spots:HashSet<Pos> = HashSet::new();

  let mut bad_count = 0;
  let mut conflict_count = 0;
  let y = 2_000_000;

  for s in signals {
    if y < (s.location.1 - s.distance) || y > (s.location.1 + s.distance) {
      continue;
    }

    for x in (s.location.0 - s.distance)..(s.location.0 + s.distance) {
      let this_distance = (s.location.0 - x).abs() + (s.location.1 - y).abs();

      if (this_distance > s.distance) ||
        (s.location.0 == x && s.location.1 == y) ||
        (s.beacon.0 == x && s.beacon.1 == y) {
        continue;
      }

      bad_spots.insert(Pos(x, y));
    }
  }

  // println!("Bad Spots: {:?}", bad_spots);
  println!("Bad Count: {}", bad_spots.len());

}

fn get_signals(contents:String) -> Vec<Signal> {
  let mut signals:Vec<Signal> = Vec::new();

  for line in contents.lines() {
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let cap = re.captures(line).unwrap();

    let signal_x:i32 = cap[1].parse::<i32>().unwrap();
    let signal_y:i32 = cap[2].parse::<i32>().unwrap();

    let beacon_x:i32 = cap[3].parse::<i32>().unwrap();
    let beacon_y:i32 = cap[4].parse::<i32>().unwrap();

    let signal = Signal {
      location: Pos(signal_x, signal_y),
      beacon: Pos(beacon_x, beacon_y),
      distance: (signal_x - beacon_x).abs() + (signal_y - beacon_y).abs(),
    };

    signals.push(signal);
  }

  return signals;
}

fn display_grid(grid:&Vec<Vec<char>>, y_offset:usize) {
  for y in 0..=grid.len() {
    let mut line:Vec<char> = Vec::new();

    for x in 0..=grid[0].len() {
      if x >= grid.len() {
        continue;
      }
          
      line.push(grid[x][y]);
    }
    println!("{} {}", y as i32 - y_offset as i32, line.iter().collect::<String>());
  }
}

fn populate_grid(signals: &Vec<Signal>) -> Vec<Vec<char>> {
  let mut grid:Vec<Vec<char>> = vec![vec!['.'; 4_000_000]; 4_000_000]; 

  for signal in signals {
    // grid[(signal.location.0) as usize][(signal.location.1) as usize] = 'S';
    // grid[(signal.beacon.0) as usize][(signal.beacon.1) as usize] = 'B';

    for x in 0..4_000_000 {
      println!("X: {}", x);
      for y in 0..4_000_000 {
        let distance = (x - signal.location.0).abs() + (y - signal.location.1).abs();

        if grid[x as usize][y as usize] != '.' || distance > signal.distance {
          continue;
        }

        grid[x as usize][y as usize] = '#';
      }
    }
  }

  return grid;
}