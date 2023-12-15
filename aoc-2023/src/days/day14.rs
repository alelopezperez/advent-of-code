use std::collections::{HashMap, HashSet};

fn parse(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn move_up(dish: &mut Vec<Vec<char>>) {
    for j in 0..dish[0].len() {
        let mut i = 0;

        while i < dish.len() {
            if dish[i][j] == '#' || dish[i][j] == '.' {
                i += 1;
                continue;
            }

            let mut temp = i as i32 - 1;
            let mut temp2 = i;
            while temp >= 0 && dish[temp as usize][j] != 'O' && dish[temp as usize][j] != '#' {
                dish[temp as usize][j] = 'O';
                dish[temp2][j] = '.';
                temp -= 1;
                temp2 -= 1;
            }

            i += 1;
        }
    }
}

fn move_down(dish: &mut Vec<Vec<char>>) {
    for j in (0..dish[0].len()).rev() {
        let mut i = dish.len() as i32 - 1;

        while i >= 0 {
            if dish[i as usize][j] == '#' || dish[i as usize][j] == '.' {
                i -= 1;
                continue;
            }

            let mut temp = i + 1;
            let mut temp2 = i;
            while temp < dish.len() as i32
                && dish[temp as usize][j] != 'O'
                && dish[temp as usize][j] != '#'
            {
                dish[temp as usize][j] = 'O';
                dish[temp2 as usize][j] = '.';
                temp += 1;
                temp2 += 1;
            }

            i -= 1;
        }
    }
}

fn move_left(dish: &mut Vec<Vec<char>>) {
    for i in 0..dish.len() {
        let mut j = 0;

        while j < dish[0].len() {
            if dish[i][j] == '#' || dish[i][j] == '.' {
                j += 1;
                continue;
            }

            let mut temp = j as i32 - 1;
            let mut temp2 = j;
            while temp >= 0 && dish[i][temp as usize] != 'O' && dish[i][temp as usize] != '#' {
                dish[i][temp as usize] = 'O';
                dish[i][temp2] = '.';
                temp -= 1;
                temp2 -= 1;
            }

            j += 1;
        }
    }
}

fn move_right(dish: &mut Vec<Vec<char>>) {
    for i in (0..dish.len()).rev() {
        let mut j = dish[0].len() as i32 - 1;

        while j >= 0 {
            if dish[i][j as usize] == '#' || dish[i][j as usize] == '.' {
                j -= 1;
                continue;
            }

            let mut temp = j + 1;
            let mut temp2 = j;
            while temp < dish[0].len() as i32
                && dish[i][temp as usize] != 'O'
                && dish[i][temp as usize] != '#'
            {
                dish[i][temp as usize] = 'O';
                dish[i][temp2 as usize] = '.';
                temp += 1;
                temp2 += 1;
            }

            j -= 1;
        }
    }
}

pub fn part_1(input: String) {
    let mut dish = parse(input);
    move_up(&mut dish);

    let ans = dish.iter().enumerate().fold(0, |acc, (i, d)| {
        let os = d.iter().filter(|&&x| x == 'O').count();

        acc + os * (dish.len() - i)
    });
    println!("{:?}", ans);
}

pub fn part_2(input: String) {
    let mut dish = parse(input);

    let mut set = HashMap::new();
    let mut map = HashMap::new();

    let mut count = 0;
    loop {
        move_up(&mut dish);
        move_left(&mut dish);
        move_down(&mut dish);
        move_right(&mut dish);

        if set.contains_key(&dish) {
            break;
        }
        set.insert(dish.clone(), count);
        map.insert(count, dish.clone());
        count += 1;
    }

    println!("mod {} {}", count, set.len());

    let rem = (1000000000 - 1 - set.get(&dish).unwrap()) % (count - set.get(&dish).unwrap())
        + set.get(&dish).unwrap();

    let dish = map.get(&rem).unwrap().clone();
    let ans: usize = dish.iter().enumerate().fold(0, |acc, (i, d)| {
        let os = d.iter().filter(|&&x| x == 'O').count();

        acc + os * (dish.len() - i)
    });

    println!("{:?}", ans);
}
