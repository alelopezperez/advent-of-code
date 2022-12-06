use std::fs;
fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    let pairs: Vec<Vec<(i32, i32)>> = contents
        .lines()
        .map(|l| {
            l.split(',')
                .map(|p| {
                    let ton = p.split_once('-').unwrap();
                    (ton.0.parse::<i32>().unwrap(), ton.1.parse::<i32>().unwrap())
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect();

    let mut count = 0;
    pairs.iter().for_each(|p| {
        if (p[0].0 <= p[1].0 && p[0].1 >= p[1].1) || (p[1].0 <= p[0].0 && p[1].1 >= p[0].1) {
            count += 1;
            // println!("{:?}", p)
        }
    });
    println!("{count}");

    let mut count = 0;
    pairs.iter().for_each(|p| {
        if ((p[0].1 >= p[1].0) && (p[0].0 <= p[1].0)) || ((p[1].1 >= p[0].0) && (p[1].0 <= p[0].0))
        {
            count += 1;
            // println!("{:?}", p)
        }
    });
    println!("{count}");
}
