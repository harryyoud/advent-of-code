pub mod knot_hasher;
mod groups;
pub mod duet_asm;

pub use groups::count_groups;
use aoc_lib::get_input_year;

pub fn get_input(day: usize) -> String {
    get_input_year(2017, day)
}