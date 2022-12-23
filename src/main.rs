mod days;
mod util;

use std::env;

use crate::days::day1::get_calories;
use crate::days::day2::shoot;
use crate::days::day3::sort_pack;
use crate::days::day4::find_overlaps;
use crate::days::day5::crane_it_up;
use crate::days::day6::start_packet;
use crate::days::day7::traverse;
use crate::days::day8::visible;
use crate::days::day9::move_knot;
use crate::days::day10::cycle;
use crate::days::day11::monkey;
use crate::days::day12::hike;
use crate::days::day13::{pairs, pairs_two};
use crate::days::day14::drip;
use crate::days::day15::beacon;
use crate::days::day16::pipes;
use crate::days::day17::tetris;
use crate::days::day18::lava;

fn main() {
  let args: Vec<String> = env::args().collect();

  match args[1].parse::<u32>().unwrap() {
    1 =>  get_calories(),
    2 => shoot(),
    3 => sort_pack(),
    4 => find_overlaps(),
    5 => crane_it_up(),
    6 => start_packet(),
    7 => traverse(),
    8 => visible(),
    9 => move_knot(),
    10 => cycle(),
    11 => monkey(),
    12 => hike(),
    13 => {
      pairs();
      pairs_two();
    },
    14 => drip(),
    15  => beacon(),
    16 => pipes(),
    17 => tetris(),
    18 => lava(),
    _ => println!("Need a day"),
  }
}