use std::fs;

mod days;
use days::day1;
use days::day2;
use days::day3;
use days::day4;
use days::day5;
use days::day6;
use days::day7;

use tailcall::tailcall;

fn main() {
    // let input = fs::read_to_string("./day1.txt").unwrap();

    // println!(
    //     "{} {}",
    //     day1::part_2(input.clone()),
    //     day1::part_2_trie(input.clone())
    // );

    // let input = fs::read_to_string("./day2.txt").unwrap();
    // println!("{}", day2::part_1(input.clone()));
    // println!("{}", day2::part_2(input));

    // let input = fs::read_to_string("./day3.txt").unwrap();
    // println!("{}", day3::part_1(input.clone()));
    // println!("{}", day3::part_2(input));

    // let input = fs::read_to_string("./day4.txt").unwrap();
    // println!("{}", day4::part_1(input.clone()));
    // println!("anser {}", day4::part_2(input.clone()));
    // println!("anser {}", day4::part_2_normal(input.clone()));

    //let input = fs::read_to_string("./day5.txt").unwrap();
    // day5::part_1(input.clone());
    //day5::part_2_fast(input);

    // day6::part_1();

    let input = fs::read_to_string("./day7.txt").unwrap();
    day7::part_1(input.clone());
    day7::part_2(input);
}

#[tailcall] // <- This is great!!!! it loops indefently instead of overflowing
fn infinite() {
    infinite();
}
