use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // let mut v: Vec<i32> = Vec::new();

    let mut a = contents
        .split("\n\n")
        .map(|item| {
            item.lines()
                .map(|num| {
                    let n: i32 = num.parse().unwrap();
                    n
                })
                .fold(0, |acc, i| acc + i)
        })
        .collect::<Vec<i32>>();

    println!("{}", a.iter().max().unwrap());

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
