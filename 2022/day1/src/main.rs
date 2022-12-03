//bin/true; rustc -o "/tmp/$0.bin" 1>&2 "$0" && "/tmp/$0.bin" "$@"; exit $?

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // let mut v: Vec<i32> = Vec::new();

    let mut a = contents
        .split("\n\n")
        .map(|item| item.lines().map(|num| num.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();

    println!("{}", a.iter().max().unwrap());

    let mut heap = BinaryHeap::with_capacity(3);

    for n in a.clone() {
        heap.push(Reverse(n));
        if heap.len() > 3 {
            heap.pop();
        }
    }
    let n = heap.iter().map(|m| m.0).sum::<i32>();

    println!("{:?}", n);

    a.sort_by(|a, b| b.cmp(a));
    let sum = &a[0..3].iter().sum::<i32>();

    println!("{sum}");

    // let mut elf_cal = 0;
    // for line in contents.lines() {
    //     if line.is_empty() {
    //         v.push(elf_cal);
    //         elf_cal = 0;
    //     } else {
    //         let num: i32 = line.parse().unwrap();
    //         elf_cal += num;
    //     }
    // }

    // v.sort_by(|a, b| b.cmp(a));
    // let top3 = &v[0..3];
    // let m: i32 = top3.iter().sum();
    // println!("{:?}", m);
}
