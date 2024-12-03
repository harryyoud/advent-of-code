// used in day_14
#![feature(hash_extract_if)]

pub mod intcode;

use aoc_lib::get_input_year;

pub fn get_input(day: usize) -> String {
    get_input_year(2019, day)
}

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    while b > 0 {
        (a, b) = (b, a % b);
    }

    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
}
