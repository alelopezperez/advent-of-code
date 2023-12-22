use std::collections::HashSet;

pub fn get_points(steps: &Vec<[&str; 3]>) -> (Vec<(i32, i32)>, i32) {
    let mut curr = (0, 0);
    let mut points = Vec::new();
    let mut boundries = 0;
    points.push(curr);
    for step in steps {
        let ammount = step[1].parse::<i32>().unwrap();
        boundries += ammount;

        match step[0] {
            "R" => {
                curr = (curr.0, curr.1 + ammount);
                points.push(curr);
            }

            "L" => {
                curr = (curr.0, curr.1 - ammount);
                points.push(curr);
            }

            "U" => {
                curr = (curr.0 - ammount, curr.1);
                points.push(curr);
            }
            "D" => {
                curr = (curr.0 + ammount, curr.1);
                points.push(curr);
            }

            _ => {}
        }
    }
    (points, boundries)
}

pub fn get_points_2(steps: &Vec<[&str; 3]>) -> (Vec<(i64, i64)>, i64) {
    let mut curr = (0, 0);
    let mut points = Vec::new();
    let mut boundries = 0;
    points.push(curr);

    for step in steps {
        let hex = step[2];
        let instruction = &hex[7..8];
        let ammount = i64::from_str_radix(&hex[2..7], 16).unwrap();

        boundries += ammount;

        match instruction {
            "0" => {
                curr = (curr.0, curr.1 + ammount);
                points.push(curr);
            }

            "2" => {
                curr = (curr.0, curr.1 - ammount);
                points.push(curr);
            }

            "3" => {
                curr = (curr.0 - ammount, curr.1);
                points.push(curr);
            }
            "1" => {
                curr = (curr.0 + ammount, curr.1);
                points.push(curr);
            }

            _ => {}
        }
    }
    (points, boundries)
}

fn parse_1(input: &str) -> Vec<[&str; 3]> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn shoelace(points: &Vec<(i32, i32)>) -> i32 {
    let mut shoelace = 0;

    for i in 0..points.len() {
        let prev = (i as i32 - 1).rem_euclid(points.len() as i32) as usize;
        let next = (i as i32 + 1).rem_euclid(points.len() as i32) as usize;

        shoelace += points[i].0 * (points[next].1 - points[prev].1);
    }
    shoelace.abs() / 2
}

fn shoelace2(points: &Vec<(i64, i64)>) -> i64 {
    let mut shoelace = 0;

    for i in 0..points.len() {
        let prev = (i as i32 - 1).rem_euclid(points.len() as i32) as usize;
        let next = (i as i32 + 1).rem_euclid(points.len() as i32) as usize;

        shoelace += points[i].0 * (points[next].1 - points[prev].1);
    }
    shoelace.abs() / 2
}

pub fn part_1(input: &str) {
    let steps = parse_1(input);
    let (points, b) = get_points(&steps);

    let shoelace = shoelace(&points);
    let picks = shoelace - b / 2 + 1;
    // println!("{shoelace} {picks}");
    println!("{}", picks + b);
}

pub fn part_2(input: &str) {
    let steps = parse_1(input);
    let (points, b) = get_points_2(&steps);

    let shoelace = shoelace2(&points);
    let picks = shoelace - b / 2 + 1;
    println!("{shoelace} {picks}");
    println!("{}", picks + b);
}
