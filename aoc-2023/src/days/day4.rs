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

pub fn part_2(input: String) -> u32 {
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

    (0..games.len()).map(|i| part_2_rec(&games, i, 0)).sum()
}
fn part_2_rec(games: &Vec<Card>, i: usize, accum: u32) -> u32 {
    if let Some(card) = games.get(i) {
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

        (1..=amm as usize)
            .map(|j| part_2_rec(games, i + j, accum + amm))
            .fold(1, |total, x| total + x)
    } else {
        accum
    }
}

pub fn part_2_normal(input: String) -> u32 {
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

    let total = 0;

    let mut copy = vec![1; games.len()];

    println!("{}", copy.len());

    for (i, card) in games.iter().enumerate() {
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

        for extra in 1..=amm {
            copy[i + extra as usize] += copy[i];
        }
    }

    copy.iter().sum()
}
