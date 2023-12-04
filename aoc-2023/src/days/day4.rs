#[derive(Debug)]
struct Card {
    id: u32,
    left: Vec<i32>,
    right: Vec<i32>,
}
pub fn part_1(input: String) -> i32 {
    let games = input
        .lines()
        .map(|x| {
            let (id, lr) = x.split_once(':').unwrap();
            let id = id.chars().last().unwrap().to_digit(10).unwrap();

            let (left, right) = lr.trim().split_once('|').unwrap();

            let left = left
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let right = right
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            Card {
                id: id,
                left: left,
                right: right,
            }
        })
        .collect::<Vec<_>>();

    //println!("{:#?}", games);

    games
        .iter()
        .map(|card| {
            let amm = card
                .left
                .iter()
                .map(|x| {
                    if let Some(_) = card.right.iter().find(|y| *y == x) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u32>();

            if amm > 0 {
                (2 as i32).pow(amm - 1)
            } else {
                0
            }
        })
        .sum::<i32>()
}
