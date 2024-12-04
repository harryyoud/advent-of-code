pub mod duet_asm;
mod groups;
pub mod knot_hasher;

use aoc_lib::get_input_year;
pub use groups::count_groups;

pub fn get_input(day: usize) -> String {
    get_input_year(2017, day)
}
