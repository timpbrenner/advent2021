mod days;

use crate::days::day1::get_calories;
use crate::days::day2::shoot;
use crate::days::day3::sort_pack;
use crate::days::day4::find_overlaps;

fn main() {
    get_calories();
    shoot();
    sort_pack();
    find_overlaps();
}