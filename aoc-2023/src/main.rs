use std::fs;

mod days;
use days::day1;
use days::day2;
fn main() {
    // let input = fs::read_to_string("./day1.txt").unwrap();

    // println!(
    //     "{} {}",
    //     day1::part_2(input.clone()),
    //     day1::part_2_trie(input.clone())
    // );

    let input = fs::read_to_string("./day2.txt").unwrap();

    println!("{}", day2::part_1(input));
}
