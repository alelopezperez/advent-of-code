use std::fs;
#[derive(Debug)]

enum Rps {
    Rock = 1,
    Paper = 2,
    Scisor = 3,
}
#[derive(Debug)]
enum Status {
    Win = 6,
    Draw = 3,
    Lose = 0,
}
#[derive(Debug)]
struct Player {
    play: Rps,
}

impl Player {
    fn play(two: &Player, one: &Player) -> i32 {
        match one.play {
            Rps::Rock => match two.play {
                Rps::Rock => Status::Draw as i32 + Rps::Rock as i32,
                Rps::Paper => Status::Lose as i32 + Rps::Rock as i32,
                Rps::Scisor => Status::Win as i32 + Rps::Rock as i32,
            },
            Rps::Paper => match two.play {
                Rps::Rock => Status::Win as i32 + Rps::Paper as i32,
                Rps::Paper => Status::Draw as i32 + Rps::Paper as i32,
                Rps::Scisor => Status::Lose as i32 + Rps::Paper as i32,
            },
            Rps::Scisor => match two.play {
                Rps::Rock => Status::Lose as i32 + Rps::Scisor as i32,
                Rps::Paper => Status::Win as i32 + Rps::Scisor as i32,
                Rps::Scisor => Status::Draw as i32 + Rps::Scisor as i32,
            },
        }
    }
}
fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    let games: Vec<(Player, Status)> = contents
        .lines()
        .map(|game| {
            let g = game.split_once(' ').unwrap();
            (
                match g.0 {
                    "A" => Player { play: Rps::Rock },
                    "B" => Player { play: Rps::Paper },
                    _ => Player { play: Rps::Scisor },
                },
                match g.1 {
                    "X" => Status::Lose,
                    "Y" => Status::Draw,
                    _ => Status::Win,
                },
            )
        })
        .collect();

    let total: i32 = games
        .iter()
        .map(|g| match g.1 {
            Status::Win => match g.0.play {
                Rps::Rock => Player::play(&g.0, &Player { play: Rps::Paper }),
                Rps::Paper => Player::play(&g.0, &Player { play: Rps::Scisor }),
                Rps::Scisor => Player::play(&g.0, &Player { play: Rps::Rock }),
            },
            Status::Draw => match g.0.play {
                Rps::Rock => Player::play(&g.0, &Player { play: Rps::Rock }),
                Rps::Paper => Player::play(&g.0, &Player { play: Rps::Paper }),
                Rps::Scisor => Player::play(&g.0, &Player { play: Rps::Scisor }),
            },
            Status::Lose => match g.0.play {
                Rps::Rock => Player::play(&g.0, &Player { play: Rps::Scisor }),
                Rps::Paper => Player::play(&g.0, &Player { play: Rps::Rock }),
                Rps::Scisor => Player::play(&g.0, &Player { play: Rps::Paper }),
            },
        })
        .sum();

    println!("{total}");
}
