mod days;
mod util;

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

fn main() {
  get_calories();
  shoot();
  sort_pack();
  find_overlaps();
  crane_it_up();
  start_packet();
  traverse();
  visible();
  move_knot();
  cycle();
  monkey();
}