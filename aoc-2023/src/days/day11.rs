use std::{cmp::max, cmp::min, usize};

fn parse_1(input: String) -> Vec<Vec<char>> {
    let universe = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    universe
}

fn find_galaxies(universe: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();

    for (i, vec) in universe.iter().enumerate() {
        for (j, letter) in vec.iter().enumerate() {
            if *letter == '#' {
                galaxies.push((i, j));
            }
        }
    }
    galaxies
}

fn find_expansion(universe: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let rows = universe
        .iter()
        .enumerate()
        .filter(|(_, x)| x.iter().all(|x| *x == '.'))
        .map(|x| x.0)
        .collect::<Vec<_>>();

    let mut columns = Vec::new();
    for j in 0..universe[0].len() {
        let mut flag = true;
        for i in 0..universe.len() {
            if universe[i][j] != '.' {
                flag = false;
                break;
            }
        }
        if flag {
            columns.push(j);
        }
    }
    (rows, columns)
}
fn find_shortest_distance(
    p1: &(usize, usize),
    p2: &(usize, usize),
    rows: &Vec<usize>,
    column: &Vec<usize>,
    multiplier: usize,
) -> usize {
    let mut count = 0;

    count += p1.0.abs_diff(p2.0);

    count += p1.1.abs_diff(p2.1);

    count += rows
        .iter()
        .filter(|&&x| x >= min(p1.0, p2.0) && x <= max(p1.0, p2.0))
        .count()
        * multiplier;

    count += column
        .iter()
        .filter(|&&x| x >= min(p1.1, p2.1) && x <= max(p1.1, p2.1))
        .count()
        * multiplier;

    count
}
pub fn part_1(input: String) {
    let mut universe = parse_1(input);

    let (rows, columns) = find_expansion(&universe);

    let galaxies = find_galaxies(&universe);

    let count = galaxies
        .iter()
        .enumerate()
        .map(|(i, x)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|y| find_shortest_distance(x, y, &rows, &columns, 1))
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{count}");
}

pub fn part_2(input: String) {
    let mut universe = parse_1(input);

    let (rows, columns) = find_expansion(&universe);

    let galaxies = find_galaxies(&universe);

    let count = galaxies
        .iter()
        .enumerate()
        .map(|(i, x)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|y| find_shortest_distance(x, y, &rows, &columns, 999999))
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{count}");
}
