use tailcall::tailcall;

enum Direction {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

enum Origin {
    Left,
    Right,
    Up,
    Down,
    Start,
}

struct Position {
    x: u32,
    y: u32,
}
fn parse_1(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
#[tailcall]
fn traverse_1(
    pipes: &Vec<Vec<char>>,
    start: (usize, usize),
    curr: (usize, usize),
    steps: i32,
    origin: Origin,
) -> i32 {
    if curr == start {
        println!("arrived to loop");
        return steps;
    }

    if pipes[curr.0][curr.1] == '|' {
        return match origin {
            Origin::Down => traverse_1(pipes, start, (curr.0 - 1, curr.1), steps + 1, Origin::Down),
            Origin::Up => traverse_1(pipes, start, (curr.0 + 1, curr.1), steps + 1, Origin::Up),
            _ => panic!("crash and burn, {}", steps),
        };
    }

    if pipes[curr.0][curr.1] == '-' {
        return match origin {
            Origin::Left => traverse_1(pipes, start, (curr.0, curr.1 + 1), steps + 1, Origin::Left),
            Origin::Right => {
                traverse_1(pipes, start, (curr.0, curr.1 - 1), steps + 1, Origin::Right)
            }
            _ => panic!("crash and burn, {}", steps),
        };
    }

    //Todo
    if pipes[curr.0][curr.1] == 'L' {
        return match origin {
            Origin::Right => {
                traverse_1(pipes, start, (curr.0 - 1, curr.1), steps + 1, Origin::Down)
            }
            Origin::Up => traverse_1(pipes, start, (curr.0, curr.1 + 1), steps + 1, Origin::Left),
            _ => panic!("crash and burn, {}", steps),
        };
    }

    if pipes[curr.0][curr.1] == 'J' {
        return match origin {
            Origin::Left => traverse_1(pipes, start, (curr.0 - 1, curr.1), steps + 1, Origin::Down),
            Origin::Up => traverse_1(pipes, start, (curr.0, curr.1 - 1), steps + 1, Origin::Right),
            _ => panic!("crash and burn, {}", steps),
        };
    }

    if pipes[curr.0][curr.1] == '7' {
        return match origin {
            Origin::Left => traverse_1(pipes, start, (curr.0 + 1, curr.1), steps + 1, Origin::Up),
            Origin::Down => {
                traverse_1(pipes, start, (curr.0, curr.1 - 1), steps + 1, Origin::Right)
            }
            _ => panic!("crash and burn, {}", steps),
        };
    }

    if pipes[curr.0][curr.1] == 'F' {
        return match origin {
            Origin::Right => traverse_1(pipes, start, (curr.0 + 1, curr.1), steps + 1, Origin::Up),
            Origin::Down => traverse_1(pipes, start, (curr.0, curr.1 + 1), steps + 1, Origin::Left),
            _ => panic!("crash and burn, {}", steps),
        };
    }

    panic!("crash and burn, {}", steps)
    //Todo
}

#[tailcall]
fn traverse_2(
    pipes: &Vec<Vec<char>>,
    start: (usize, usize),
    curr: (usize, usize),
    steps: i32,
    origin: Origin,
    marked: &mut Vec<(usize, usize)>,
) -> i32 {
    if curr == start {
        println!("arrived to loop");
        return steps;
    }

    if pipes[curr.0][curr.1] == '|' {
        return match origin {
            Origin::Down => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0 - 1, curr.1),
                    steps + 1,
                    Origin::Down,
                    marked,
                )
            }
            Origin::Up => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0 + 1, curr.1),
                    steps + 1,
                    Origin::Up,
                    marked,
                )
            }
            _ => panic!("crash and burn, {}", steps),
        };
    }

    if pipes[curr.0][curr.1] == '-' {
        return match origin {
            Origin::Left => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0, curr.1 + 1),
                    steps + 1,
                    Origin::Left,
                    marked,
                )
            }
            Origin::Right => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0, curr.1 - 1),
                    steps + 1,
                    Origin::Right,
                    marked,
                )
            }
            _ => panic!("crash and burn, {}", steps),
        };
    }

    //Todo
    if pipes[curr.0][curr.1] == 'L' {
        return match origin {
            Origin::Right => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0 - 1, curr.1),
                    steps + 1,
                    Origin::Down,
                    marked,
                )
            }
            Origin::Up => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0, curr.1 + 1),
                    steps + 1,
                    Origin::Left,
                    marked,
                )
            }
            _ => panic!("crash and burn, {}", steps),
        };
    }

    if pipes[curr.0][curr.1] == 'J' {
        return match origin {
            Origin::Left => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0 - 1, curr.1),
                    steps + 1,
                    Origin::Down,
                    marked,
                )
            }
            Origin::Up => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0, curr.1 - 1),
                    steps + 1,
                    Origin::Right,
                    marked,
                )
            }
            _ => panic!("crash and burn, {}", steps),
        };
    }

    if pipes[curr.0][curr.1] == '7' {
        return match origin {
            Origin::Left => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0 + 1, curr.1),
                    steps + 1,
                    Origin::Up,
                    marked,
                )
            }
            Origin::Down => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0, curr.1 - 1),
                    steps + 1,
                    Origin::Right,
                    marked,
                )
            }
            _ => panic!("crash and burn, {}", steps),
        };
    }

    if pipes[curr.0][curr.1] == 'F' {
        return match origin {
            Origin::Right => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0 + 1, curr.1),
                    steps + 1,
                    Origin::Up,
                    marked,
                )
            }
            Origin::Down => {
                marked.push(curr);
                traverse_2(
                    pipes,
                    start,
                    (curr.0, curr.1 + 1),
                    steps + 1,
                    Origin::Left,
                    marked,
                )
            }
            _ => panic!("crash and burn, {}", steps),
        };
    }

    panic!("crash and burn, {}", steps)
    //Todo
}

fn mark_all(pipes: &mut Vec<Vec<char>>, marked: &Vec<(usize, usize)>) {
    for (i, vec) in pipes.iter_mut().enumerate() {
        for (j, letter) in vec.iter_mut().enumerate() {
            if !(marked.iter().any(|x| x.0 == i && x.1 == j)) {
                *letter = '?'
            }
        }
    }
}

pub fn part_1(input: String) {
    let pipes = parse_1(input);

    let mut start = (-1, -1);
    for (i, vec) in pipes.iter().enumerate() {
        for (j, letters) in vec.iter().enumerate() {
            if *letters == 'S' {
                start = (i as i32, j as i32);
            }
        }
    }

    pipes[start.0 as usize][start.1 as usize];

    let ans = traverse_1(
        &pipes,
        (start.0 as usize, start.1 as usize),
        ((start.0 - 1) as usize, (start.1) as usize),
        1,
        Origin::Down,
    );

    println!("{}", ans / 2);
}

#[tailcall]
fn even_odd(pipes: &Vec<Vec<char>>, point: (i32, i32), rule: u32) -> u32 {
    let (i, j) = point;

    if i < 0 || j < 0 || i >= pipes.len() as i32 || j >= pipes[0].len() as i32 {
        return rule;
    }

    if pipes[i as usize][j as usize] == '?' {
        return even_odd(pipes, (i, j - 1), rule);
    }

    if pipes[i as usize][j as usize] == '-'
        || pipes[i as usize][j as usize] == 'F'
        || pipes[i as usize][j as usize] == '7'
    {
        return even_odd(pipes, (i, j - 1), rule);
    }

    even_odd(pipes, (i, j - 1), rule + 1)
}

pub fn part_2(input: String) {
    let mut pipes = parse_1(input);

    let mut start = (-1, -1);
    for (i, vec) in pipes.iter().enumerate() {
        for (j, letters) in vec.iter().enumerate() {
            if *letters == 'S' {
                start = (i as i32, j as i32);
            }
        }
    }

    println!("{:?}", start);

    let mut marked = Vec::<(usize, usize)>::new();

    //        ((start.0 - 1) as usize, (start.1) as usize), DOWN

    marked.push((start.0 as usize, start.1 as usize));
    traverse_2(
        &pipes,
        (start.0 as usize, start.1 as usize),
        ((start.0 - 1) as usize, (start.1) as usize),
        1,
        Origin::Down,
        &mut marked,
    );

    mark_all(&mut pipes, &marked);

    // println!("{:?}", pipes);

    let mut count = 0;
    for (i, vec) in pipes.iter().enumerate() {
        for (j, letter) in vec.iter().enumerate() {
            if *letter == '?' {
                let rule = even_odd(&pipes, (i as i32, j as i32), 0);

                if rule % 2 != 0 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
