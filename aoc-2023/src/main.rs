use std::fs;

mod days;
use days::day1;
use days::day10;
use days::day11;
use days::day12;
use days::day2;
use days::day3;
use days::day4;
use days::day5;
use days::day6;
use days::day7;
use days::day8;
use days::day9;

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

    // let input = fs::read_to_string("./day7.txt").unwrap();
    // day7::part_1(input.clone());
    // day7::part_2(input);

    // let input = fs::read_to_string("./day8.txt").unwrap();
    // day8::part_1(input.clone());
    // day8::part_2(input);

    // let input = fs::read_to_string("./day9.txt").unwrap();
    // day9::part_1(input.clone());
    // day9::part_2(input);

    // let input = fs::read_to_string("./day10.txt").unwrap();
    // //day10::part_1(input.clone());
    // day10::part_2(input.clone());

    // let input = fs::read_to_string("./day11.txt").unwrap();
    // day11::part_1(input.clone());
    // day11::part_2(input.clone());

    let input = fs::read_to_string("./day12.txt").unwrap();
    day12::part_1(input.clone());
}

#[tailcall] // <- This is great!!!! it loops indefently instead of overflowing
fn infinite() {
    infinite();
}
