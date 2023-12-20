use std::collections::HashSet;

use tailcall::tailcall;

#[derive(Hash, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Hash, PartialEq, Eq, Clone)]
struct Beam {
    pos: (i32, i32),
    direction: Direction,
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn walk(
    puzzle: &Vec<Vec<char>>,
    beam: Beam,
    set: &mut HashSet<(Beam, char)>,
    ans: &mut Vec<Vec<char>>,
) {
    if beam.pos.0 < 0
        || beam.pos.0 >= puzzle.len() as i32
        || beam.pos.1 < 0
        || beam.pos.1 >= puzzle[0].len() as i32
    {
        return;
    }

    if set.contains(&(
        beam.clone(),
        puzzle[beam.pos.0 as usize][beam.pos.1 as usize],
    )) {
        return;
    }

    let curr = puzzle[beam.pos.0 as usize][beam.pos.1 as usize];
    ans[beam.pos.0 as usize][beam.pos.1 as usize] = '#';

    match curr {
        '/' => {
            set.insert((beam.clone(), curr));
            let beam = match beam.direction {
                Direction::Up => {
                    let new_pos = (beam.pos.0, beam.pos.1 + 1);
                    Beam {
                        pos: new_pos,
                        direction: Direction::Right,
                    }
                }
                Direction::Down => {
                    let new_pos = (beam.pos.0, beam.pos.1 - 1);
                    Beam {
                        pos: new_pos,
                        direction: Direction::Left,
                    }
                }
                Direction::Left => {
                    let new_pos = (beam.pos.0 + 1, beam.pos.1);
                    Beam {
                        pos: new_pos,
                        direction: Direction::Down,
                    }
                }
                Direction::Right => {
                    let new_pos = (beam.pos.0 - 1, beam.pos.1);
                    Beam {
                        pos: new_pos,
                        direction: Direction::Up,
                    }
                }
            };
            walk(puzzle, beam, set, ans);
        }
        '\\' => {
            set.insert((beam.clone(), curr));
            let beam = match beam.direction {
                Direction::Up => {
                    let new_pos = (beam.pos.0, beam.pos.1 - 1);
                    Beam {
                        pos: new_pos,
                        direction: Direction::Left,
                    }
                }
                Direction::Down => {
                    let new_pos = (beam.pos.0, beam.pos.1 + 1);
                    Beam {
                        pos: new_pos,
                        direction: Direction::Right,
                    }
                }
                Direction::Left => {
                    let new_pos = (beam.pos.0 - 1, beam.pos.1);
                    Beam {
                        pos: new_pos,
                        direction: Direction::Up,
                    }
                }
                Direction::Right => {
                    let new_pos = (beam.pos.0 + 1, beam.pos.1);
                    Beam {
                        pos: new_pos,
                        direction: Direction::Down,
                    }
                }
            };
            walk(puzzle, beam, set, ans);
        }
        '|' => {
            set.insert((beam.clone(), curr));
            match beam.direction {
                Direction::Up => {
                    let new_pos = (beam.pos.0 - 1, beam.pos.1);
                    let beam = Beam {
                        pos: new_pos,
                        direction: beam.direction,
                    };
                    walk(puzzle, beam, set, ans);
                }
                Direction::Down => {
                    let new_pos = (beam.pos.0 + 1, beam.pos.1);
                    let beam = Beam {
                        pos: new_pos,
                        direction: beam.direction,
                    };
                    walk(puzzle, beam, set, ans);
                }
                Direction::Left => {
                    let new_pos = (beam.pos.0 + 1, beam.pos.1);
                    let new_beam = Beam {
                        pos: new_pos,
                        direction: Direction::Down,
                    };
                    walk(puzzle, new_beam, set, ans);

                    let new_pos = (beam.pos.0 - 1, beam.pos.1);
                    let new_beam = Beam {
                        pos: new_pos,
                        direction: Direction::Up,
                    };
                    walk(puzzle, new_beam, set, ans);
                }
                Direction::Right => {
                    let new_pos = (beam.pos.0 + 1, beam.pos.1);
                    let new_beam = Beam {
                        pos: new_pos,
                        direction: Direction::Down,
                    };
                    walk(puzzle, new_beam, set, ans);

                    let new_pos = (beam.pos.0 - 1, beam.pos.1);
                    let new_beam = Beam {
                        pos: new_pos,
                        direction: Direction::Up,
                    };
                    walk(puzzle, new_beam, set, ans);
                }
            };
        }
        '-' => {
            set.insert((beam.clone(), curr));
            match beam.direction {
                Direction::Up => {
                    let new_pos = (beam.pos.0, beam.pos.1 - 1);
                    let new_beam = Beam {
                        pos: new_pos,
                        direction: Direction::Left,
                    };
                    walk(puzzle, new_beam, set, ans);

                    let new_pos = (beam.pos.0, beam.pos.1 + 1);
                    let new_beam = Beam {
                        pos: new_pos,
                        direction: Direction::Right,
                    };
                    walk(puzzle, new_beam, set, ans);
                }
                Direction::Down => {
                    let new_pos = (beam.pos.0, beam.pos.1 - 1);
                    let new_beam = Beam {
                        pos: new_pos,
                        direction: Direction::Left,
                    };
                    walk(puzzle, new_beam, set, ans);

                    let new_pos = (beam.pos.0, beam.pos.1 + 1);
                    let new_beam = Beam {
                        pos: new_pos,
                        direction: Direction::Right,
                    };
                    walk(puzzle, new_beam, set, ans);
                }
                Direction::Left => {
                    let new_pos = (beam.pos.0, beam.pos.1 - 1);
                    let beam = Beam {
                        pos: new_pos,
                        direction: beam.direction,
                    };
                    walk(puzzle, beam, set, ans);
                }
                Direction::Right => {
                    let new_pos = (beam.pos.0, beam.pos.1 + 1);
                    let beam = Beam {
                        pos: new_pos,
                        direction: beam.direction,
                    };
                    walk(puzzle, beam, set, ans);
                }
            };
        }
        _ => {
            let new_pos = match beam.direction {
                Direction::Up => (beam.pos.0 - 1, beam.pos.1),
                Direction::Down => (beam.pos.0 + 1, beam.pos.1),
                Direction::Left => (beam.pos.0, beam.pos.1 - 1),
                Direction::Right => (beam.pos.0, beam.pos.1 + 1),
            };
            let beam = Beam {
                pos: new_pos,
                direction: beam.direction.clone(),
            };
            walk(puzzle, beam, set, ans);
        }
    }
}

pub fn part1(input: &str) {
    let puzzle = parse(input);
    let beam = Beam {
        pos: (0, 0),
        direction: Direction::Right,
    };

    let mut ans = puzzle.clone();
    let mut set = HashSet::new();
    walk(&puzzle, beam, &mut set, &mut ans);

    let ans = ans
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum::<usize>();

    println!("{}", ans);
}

pub fn part2(input: &str) {
    let puzzle = parse(input);

    let mut most = usize::MIN;

    let rows = puzzle.len();
    let cols = puzzle[0].len();

    for j in 0..cols {
        let mut ans = puzzle.clone();
        let mut set = HashSet::new();
        let beam = Beam {
            pos: (0, j as i32),
            direction: Direction::Down,
        };
        walk(&puzzle, beam, &mut set, &mut ans);
        let ans = ans
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum::<usize>();
        most = std::cmp::max(ans, most);

        let mut ans = puzzle.clone();
        let mut set = HashSet::new();
        let beam = Beam {
            pos: (rows as i32 - 1, j as i32),
            direction: Direction::Up,
        };
        walk(&puzzle, beam, &mut set, &mut ans);
        let ans = ans
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum::<usize>();
        most = std::cmp::max(ans, most);
    }

    for i in 0..rows {
        let mut ans = puzzle.clone();
        let mut set = HashSet::new();
        let beam = Beam {
            pos: (i as i32, 0),
            direction: Direction::Right,
        };
        walk(&puzzle, beam, &mut set, &mut ans);
        let ans = ans
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum::<usize>();
        most = std::cmp::max(ans, most);

        let mut ans = puzzle.clone();
        let mut set = HashSet::new();
        let beam = Beam {
            pos: (i as i32, cols as i32 - 1),
            direction: Direction::Left,
        };
        walk(&puzzle, beam, &mut set, &mut ans);
        let ans = ans
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum::<usize>();
        most = std::cmp::max(ans, most);
    }

    println!("{most}");
}
