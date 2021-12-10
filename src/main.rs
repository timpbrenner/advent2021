mod days;

use crate::days::day1::{compare_ranges, get_depth};
use crate::days::day2::move_sub;
use crate::days::day3::{get_power, get_o2_and_co2};
use crate::days::day4::{bingo, last_bingo};
use crate::days::day5::grid;
use crate::days::day6::lanternfish_count;
use crate::days::day7::crab_sub;
use crate::days::day8::weird_number_thing;
use crate::days::day9::low_danger;

fn main() {
    get_depth();
    compare_ranges(3);
    move_sub();
    get_power();
    get_o2_and_co2();
    bingo();
    last_bingo();
    grid();
    lanternfish_count();
    crab_sub();
    weird_number_thing();

    low_danger();
}