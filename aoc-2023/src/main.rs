use std::fs;

mod days;
use days::day1;
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    println!(
        "{} {}",
        day1::part_1(input.clone()),
        day1::part_2(input.clone())
    );
}
