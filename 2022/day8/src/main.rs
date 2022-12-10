use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let forrest: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree| tree.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    // println!("{:?}", forrest);

    let mut count = 0;
    for i in 0..forrest.len() {
        for j in 0..forrest[i].len() {
            if i == 0 || i == forrest.len() - 1 || j == 0 || j == forrest[i].len() - 1 {
                count += 1;
            } else {
                if up(i, j, &forrest, forrest[i][j])
                    || down(i, j, &forrest, forrest[i][j])
                    || left(i, j, &forrest, forrest[i][j])
                    || right(i, j, &forrest, forrest[i][j])
                {
                    count += 1;
                } else {
                }
            }
        }
    }

    println!("{}", count);

    let mut max = 0;
    for i in 0..forrest.len() {
        for j in 0..forrest[i].len() {
            if i == 0 || i == forrest.len() - 1 || j == 0 || j == forrest[i].len() - 1 {
                count += 1;
            } else {
                let total = up_count(i, j, &forrest, forrest[i][j])
                    * down_count(i, j, &forrest, forrest[i][j])
                    * left_count(i, j, &forrest, forrest[i][j])
                    * right_count(i, j, &forrest, forrest[i][j]);

                if total > max {
                    max = total;
                }
            }
        }
    }
    println!("{}", max);
}
fn up(i: usize, j: usize, forrest: &Vec<Vec<u32>>, val: u32) -> bool {
    if i <= 0 || i >= forrest.len() - 1 || j == 0 || j == forrest[i].len() - 1 {
        true
    } else {
        val > forrest[i + 1][j] && up(i + 1, j, &forrest, val)
    }
}
fn down(i: usize, j: usize, forrest: &Vec<Vec<u32>>, val: u32) -> bool {
    if i == 0 || i == forrest.len() - 1 || j == 0 || j == forrest[i].len() - 1 {
        true
    } else {
        val > forrest[i - 1][j] && down(i - 1, j, &forrest, val)
    }
}

fn left(i: usize, j: usize, forrest: &Vec<Vec<u32>>, val: u32) -> bool {
    if i == 0 || i == forrest.len() - 1 || j == 0 || j == forrest[i].len() - 1 {
        true
    } else {
        val > forrest[i][j - 1] && left(i, j - 1, &forrest, val)
    }
}

fn right(i: usize, j: usize, forrest: &Vec<Vec<u32>>, val: u32) -> bool {
    if i == 0 || i == forrest.len() - 1 || j == 0 || j == forrest[i].len() - 1 {
        true
    } else {
        val > forrest[i][j + 1] && right(i, j + 1, &forrest, val)
    }
}

fn up_count(i: usize, j: usize, forrest: &Vec<Vec<u32>>, val: u32) -> i32 {
    if i <= 0 || i >= forrest.len() - 1 || j == 0 || j == forrest[i].len() - 1 {
        0
    } else {
        if val > forrest[i + 1][j] {
            1 + up_count(i + 1, j, &forrest, val)
        } else {
            1
        }
    }
}
fn down_count(i: usize, j: usize, forrest: &Vec<Vec<u32>>, val: u32) -> i32 {
    if i == 0 || i == forrest.len() - 1 || j == 0 || j == forrest[i].len() - 1 {
        0
    } else {
        if val > forrest[i - 1][j] {
            1 + down_count(i - 1, j, &forrest, val)
        } else {
            1
        }
    }
}

fn left_count(i: usize, j: usize, forrest: &Vec<Vec<u32>>, val: u32) -> i32 {
    if i == 0 || i == forrest.len() - 1 || j == 0 || j == forrest[i].len() - 1 {
        0
    } else {
        if val > forrest[i][j - 1] {
            1 + left_count(i, j - 1, &forrest, val)
        } else {
            1
        }
    }
}

fn right_count(i: usize, j: usize, forrest: &Vec<Vec<u32>>, val: u32) -> i32 {
    if i == 0 || i == forrest.len() - 1 || j == 0 || j == forrest[i].len() - 1 {
        0
    } else {
        if val > forrest[i][j + 1] {
            1 + right_count(i, j + 1, &forrest, val)
        } else {
            1
        }
    }
}
