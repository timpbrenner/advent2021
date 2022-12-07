mod days;
mod util;

use crate::days::day1::get_calories;
use crate::days::day2::shoot;
use crate::days::day3::sort_pack;
use crate::days::day4::find_overlaps;
use crate::days::day5::crane_it_up;
use crate::days::day6::start_packet;

fn main() {
  get_calories();
  shoot();
  sort_pack();
  find_overlaps();
  crane_it_up();
  start_packet();
}