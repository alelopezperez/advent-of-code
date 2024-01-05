use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use tailcall::tailcall;

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn steps(
    garden: &mut Vec<Vec<char>>,
    curr: (i32, i32),
    step: usize,
    memo: &mut HashSet<(i32, i32, usize)>,
) {
    let (i, j) = curr;

    if i < 0 || i > garden.len() as i32 - 1 || j < 0 || j > garden[0].len() as i32 - 1 {
        return;
    }

    if garden[i as usize][j as usize] == '#' {
        return;
    }

    if step == 64 {
        memo.insert((curr.0, curr.1, step));
        garden[i as usize][j as usize] = 'O';
        return;
    }

    if memo.contains(&(curr.0, curr.1, step)) {
        return;
    }

    memo.insert((curr.0, curr.1, step));

    steps(garden, (i + 1, j), step + 1, memo);
    steps(garden, (i - 1, j), step + 1, memo);
    steps(garden, (i, j + 1), step + 1, memo);
    steps(garden, (i, j - 1), step + 1, memo);
}

fn steps_2(garden: &mut Vec<Vec<char>>, start: (i32, i32), memo: &mut HashMap<(i32, i32), usize>) {
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));

    while let Some((curr, step)) = queue.pop_front() {
        if memo.contains_key(&curr) {
            continue;
        }
        if garden[curr.0 as usize][curr.1 as usize] == '#' {
            continue;
        }

        memo.insert(curr, step);

        if curr.0 > 0 {
            queue.push_back(((curr.0 - 1, curr.1), step + 1));
        }
        if curr.0 < (garden.len() - 1) as i32 {
            queue.push_back(((curr.0 + 1, curr.1), step + 1));
        }
        if curr.1 > 0 {
            queue.push_back(((curr.0, curr.1 - 1), step + 1));
        }
        if curr.1 < (garden[0].len() - 1) as i32 {
            queue.push_back(((curr.0, curr.1 + 1), step + 1));
        }
    }
}
pub fn part_1(input: &str) {
    let mut garden = parse(input);
    let mut start = (-1, -1);

    for (i, row) in garden.iter().enumerate() {
        for (j, plot) in row.iter().enumerate() {
            if *plot == 'S' {
                start = (i as i32, j as i32);
            }
        }
    }

    garden[start.0 as usize][start.1 as usize] = '.';

    let mut memo = HashSet::new();

    steps(&mut garden, start, 0, &mut memo);

    let count = garden
        .iter()
        .map(|r| r.iter().filter(|&&x| x == 'O').count() as i64)
        .sum::<i64>();

    println!("{count}");
}

pub fn part_2(input: &str) {
    let mut garden = parse(input);
    let mut start = (-1, -1);

    for (i, row) in garden.iter().enumerate() {
        for (j, plot) in row.iter().enumerate() {
            if *plot == 'S' {
                start = (i as i32, j as i32);
            }
        }
    }

    garden[start.0 as usize][start.1 as usize] = '.';

    let mut visited = HashMap::new();

    steps_2(&mut garden, start, &mut visited);

    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let n = 202300;

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    let p2 = odd * visited.values().filter(|v| **v % 2 == 1).count()
        + even * visited.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners)
        + (n * even_corners);

    println!("{p2}");
}
