use std::fs;

mod days;
use days::day1;
use days::day10;
use days::day11;
use days::day12;
use days::day13;
use days::day14;
use days::day15;
use days::day16;
use days::day17;
use days::day18;
use days::day19;
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

    // let input = fs::read_to_string("./day12.txt").unwrap();
    // day12::part_1(input.clone());

    // let input = fs::read_to_string("./day13.txt").unwrap();

    // day13::part_1(input.clone());
    // day13::part_2(input.clone());

    // let input = fs::read_to_string("./day14.txt").unwrap();
    // day14::part_1(input.clone());
    // day14::part_2(input.clone());

    // let input = fs::read_to_string("./day15.txt").unwrap();
    // day15::part_1(input.clone());
    // day15::part_2(input.clone());

    // let input = include_str!("../day16.txt");
    // day16::part1(input);
    // day16::part2(input);

    // let input = include_str!("../day17.txt");
    // day17::part_1(input);
    // day17::part_2(input);

    // let input = include_str!("../day18.txt");
    // day18::part_1(input);
    // day18::part_2(input);

    let input = include_str!("../day19.txt");
    day19::part_1(input);
    day19::part_2(input);
}

#[tailcall] // <- This is great!!!! it loops indefently instead of overflowing
fn infinite() {
    infinite();
}
