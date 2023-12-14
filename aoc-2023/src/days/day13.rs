use std::usize;

fn parse(input: String) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn reflection_horizontal(puzzle: &Vec<Vec<char>>) -> Option<i32> {
    for i in 0..puzzle.len() - 1 {
        let mut flag = true;
        for (a, b) in (0..=i).rev().zip((i + 1)..puzzle.len()) {
            if puzzle[a] != puzzle[b] {
                flag = false;
                break;
            }
        }
        if flag {
            return Some(i as i32);
        }
    }
    None
}
fn reflection_vertical(puzzle: &Vec<Vec<char>>) -> Option<i32> {
    for j in 0..puzzle[0].len() - 1 {
        let mut flag = true;
        for (a, b) in (0..=j).rev().zip((j + 1)..puzzle[0].len()) {
            let is_same = puzzle.iter().all(|x| x[a] == x[b]);
            if !is_same {
                flag = false;
                break;
            }
        }
        if flag {
            return Some(j as i32);
        }
    }
    None
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
fn find_smudge_vertical(puzzle: &Vec<Vec<char>>) -> Option<i32> {
    for i in 0..puzzle.len() - 1 {
        let mut count = 0;
        let mut flag = true;
        for (a, b) in (0..=i).rev().zip((i + 1)..puzzle.len()) {
            if puzzle[a] != puzzle[b] {
                if puzzle[a]
                    .iter()
                    .zip(puzzle[b].iter())
                    .filter(|(x, y)| x != y)
                    .count()
                    == 1
                {
                    count += 1;
                    continue;
                } else {
                    flag = false;
                    break;
                }
            }
        }
        if flag {
            return Some(i as i32);
        }
    }
    None
}

fn find_smudge_horizontal(puzzle: &Vec<Vec<char>>) -> Option<i32> {
    for i in 0..puzzle.len() - 1 {
        let mut count = 0;
        let mut flag = true;
        for (a, b) in (0..=i).rev().zip((i + 1)..puzzle.len()) {
            if puzzle[a] != puzzle[b] {
                if puzzle[a]
                    .iter()
                    .zip(puzzle[b].iter())
                    .filter(|(x, y)| x != y)
                    .count()
                    == 1
                {
                    count += 1;
                } else {
                    flag = false;
                }
            }
        }
        if count == 1 && flag {
            return Some(i as i32);
        } else {
            // println!("adios");
            count = 0;
        }
    }
    None
}

pub fn part_1(input: String) {
    let puzzle = parse(input);

    let sum = puzzle
        .iter()
        .map(|p| {
            let v = reflection_horizontal(&transpose(p.clone())).unwrap_or(-1);
            let h = reflection_horizontal(p).unwrap_or(-1);

            (v + 1) + ((h + 1) * 100)
        })
        .sum::<i32>();

    println!("{sum}");
}

pub fn part_2(input: String) {
    let mut puzzle = parse(input);

    let sum = puzzle
        .iter()
        .map(|p| {
            let h = find_smudge_horizontal(p).unwrap_or(-1);

            let v = find_smudge_horizontal(&transpose(p.clone())).unwrap_or(-1);

            (v + 1) + ((h + 1) * 100)
        })
        .sum::<i32>();

    println!("{sum}");
}
