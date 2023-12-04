#[derive(Debug)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}
pub fn part_1(input: String) -> i32 {
    let games = input
        .lines()
        .map(|line| {
            let (_, rem) = line.split_once(':').unwrap();
            let games = rem
                .split_terminator([',', ';'])
                .map(|x| x.trim())
                .map(|x| {
                    let (num, color) = x.split_once(' ').unwrap();

                    (num.parse::<u8>().unwrap(), color)
                })
                .collect::<Vec<_>>();

            games
        })
        .collect::<Vec<_>>();

    // print!("{:?}", games);
    let mut ans = 0;
    for (i, game) in games.into_iter().enumerate() {
        if game
            .into_iter()
            .map(|(num, colors)| match colors {
                "blue" => num <= 14,
                "green" => num <= 13,
                "red" => num <= 12,
                _ => true,
            })
            .fold(true, |acc, x| acc && x)
        {
            ans += (i + 1) as i32;
        }
    }
    ans
}

pub fn part_2(input: String) -> i32 {
    let games: u32 = input
        .lines()
        .map(|line| {
            let (_, rem) = line.split_once(':').unwrap();
            let games = rem
                .split_terminator([',', ';'])
                .map(|x| {
                    let (num, color) = x.trim().split_once(' ').unwrap();

                    (num.parse::<u32>().unwrap(), color)
                })
                .fold(
                    Game {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    |accum, item| match item.1 {
                        "red" => Game {
                            red: std::cmp::max(accum.red, item.0),
                            green: accum.green,
                            blue: accum.blue,
                        },
                        "green" => Game {
                            red: accum.red,
                            green: std::cmp::max(accum.green, item.0),
                            blue: accum.blue,
                        },
                        "blue" => Game {
                            red: accum.red,
                            green: accum.green,
                            blue: std::cmp::max(accum.blue, item.0),
                        },
                        _ => accum,
                    },
                );

            games
        })
        .fold(0, |accum, game| accum + (game.red * game.blue * game.green));

    println!("{:#?}", games);

    0
}
