use aoc_2019::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input(4);

    let (lower, upper) = input.split_once("-").expect("Input in format xxxx-yyyy");
    let (lower, upper) = (
        lower.parse().expect("Lower limit is valid number"),
        upper.parse().expect("Upper limit is valid number"),
    );

    let increasing = (lower..=upper)
        .into_iter()
        .map(to_digits)
        .filter(|x| is_increasing(x));

    let part_1 = increasing.clone().filter(|x| has_double(x)).count();

    let part_2 = increasing.filter(|x| has_only_double(x)).count();

    dbg!(part_1);
    dbg!(part_2);
}

// quicker than converting to string and splitting into characters
fn to_digits(mut number: u32) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::with_capacity(6);

    while number > 0 {
        let n = (number % 10) as u8;
        number /= 10;
        digits.push(n);
    }

    digits.reverse();
    digits
}

fn has_double(input: &[u8]) -> bool {
    for (left, right) in input.iter().tuple_windows() {
        if *left == *right {
            return true;
        }
    }
    false
}

// We assume the slice is increasing, so we can just count occurrences of each digit
fn has_only_double(input: &[u8]) -> bool {
    input
        .into_iter()
        .counts()
        .into_iter()
        .any(|(_digit, count)| count == 2)
}

fn is_increasing(input: &[u8]) -> bool {
    let mut current_digit = 0;
    for digit in input.into_iter() {
        if *digit < current_digit {
            return false;
        }
        current_digit = *digit;
    }
    true
}
