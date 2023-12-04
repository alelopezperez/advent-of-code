use std::collections::HashSet;

impl Point {
    fn contains(&self, i: i32, j: i32) -> bool {
        self.pos.0 as i32 == i
            && (j >= self.pos.1 as i32)
            && (j < (self.pos.1 as i32 + self.len as i32))
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    val: u32,
    pos: (usize, usize),
    len: usize,
}
pub fn part_1(input: String) -> u32 {
    let matrix = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut points = Vec::new();
    for (i, line) in matrix.iter().enumerate() {
        let mut it = line.iter().enumerate();
        while let Some((j, character)) = it.next() {
            let mut num = 0;
            let mut digit = character;
            while digit.is_ascii_digit() {
                num *= 10;
                num += digit.to_digit(10).unwrap();

                match it.next() {
                    Some((_, num)) => {
                        digit = num;
                    }
                    None => {
                        break;
                    }
                }
            }

            if num > 0 {
                points.push(Point {
                    val: num,
                    pos: (i, j),
                    len: (num as f32).log10().floor() as usize + 1,
                });
            }
        }
    }

    trait Extra {
        fn is_symbol(&self) -> bool;
    }

    impl Extra for char {
        fn is_symbol(&self) -> bool {
            !self.is_ascii_digit() && (*self != '.')
        }
    }

    let valid = &points
        .iter()
        .filter(|point| {
            (0..point.len).into_iter().fold(false, |accum, index| {
                if (point.pos.0 + 1) < matrix.len()
                    && matrix[point.pos.0 + 1][point.pos.1 + index].is_symbol()
                {
                    accum || true
                } else if (point.pos.0 as i32 - 1) > 0
                    && matrix[point.pos.0 - 1][point.pos.1 + index].is_symbol()
                {
                    accum || true
                } else if (point.pos.1 + index + 1) < matrix[0].len()
                    && matrix[point.pos.0][point.pos.1 + 1 + index].is_symbol()
                {
                    accum || true
                } else if (point.pos.1 as i32 + index as i32 - 1) > 0
                    && matrix[point.pos.0][point.pos.1 + index - 1].is_symbol()
                {
                    accum || true
                } else if (point.pos.1 as i32 + index as i32 - 1) > 0
                    && (point.pos.0 as i32 - 1) > 0
                    && matrix[point.pos.0 - 1][point.pos.1 + index - 1].is_symbol()
                {
                    accum || true
                } else if (point.pos.1 + index + 1) < matrix[0].len()
                    && point.pos.0 + 1 < matrix.len()
                    && matrix[point.pos.0 + 1][point.pos.1 + 1 + index].is_symbol()
                {
                    accum || true
                } else if (point.pos.1 + index + 1) < matrix[0].len()
                    && (point.pos.0 as i32 - 1) > 0
                    && matrix[point.pos.0 - 1][point.pos.1 + 1 + index].is_symbol()
                {
                    accum || true
                } else if (point.pos.1 as i32 + index as i32 - 1) > 0
                    && (point.pos.0 + 1) < matrix.len()
                    && matrix[point.pos.0 + 1][point.pos.1 + index - 1].is_symbol()
                {
                    accum || true
                } else {
                    accum || false
                }
            })
        })
        .collect::<Vec<_>>();

    valid.iter().map(|x| x.val).sum::<u32>()
}

pub fn part_2(input: String) -> u32 {
    let matrix = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut points = Vec::new();
    for (i, line) in matrix.iter().enumerate() {
        let mut it = line.iter().enumerate();
        while let Some((j, character)) = it.next() {
            let mut num = 0;
            let mut digit = character;
            while digit.is_ascii_digit() {
                num *= 10;
                num += digit.to_digit(10).unwrap();

                match it.next() {
                    Some((_, num)) => {
                        digit = num;
                    }
                    None => {
                        break;
                    }
                }
            }

            if num > 0 {
                points.push(Point {
                    val: num,
                    pos: (i, j),
                    len: (num as f32).log10().floor() as usize + 1,
                });
            }
        }
    }

    let mut ans = 0;
    for (i, line) in matrix.iter().enumerate() {
        for (j, character) in line.iter().enumerate() {
            let mut gears = HashSet::new();
            if *character == '*' {
                let i = i as i32;
                let j = j as i32;
                if let Some(point) = points.iter().find(|x| x.contains(i + 1, j)) {
                    gears.insert(point.clone());
                }
                if let Some(point) = points.iter().find(|x| x.contains(i - 1, j)) {
                    gears.insert(point.clone());
                }
                if let Some(point) = points.iter().find(|x| x.contains(i, j - 1)) {
                    gears.insert(point.clone());
                }
                if let Some(point) = points.iter().find(|x| x.contains(i, j + 1)) {
                    gears.insert(point.clone());
                }

                if let Some(point) = points.iter().find(|x| x.contains(i + 1, j + 1)) {
                    gears.insert(point.clone());
                }
                if let Some(point) = points.iter().find(|x| x.contains(i - 1, j - 1)) {
                    gears.insert(point.clone());
                }
                if let Some(point) = points.iter().find(|x| x.contains(i + 1, j - 1)) {
                    gears.insert(point.clone());
                }
                if let Some(point) = points.iter().find(|x| x.contains(i - 1, j + 1)) {
                    gears.insert(point.clone());
                }
            }

            if gears.len() == 2 {
                ans += gears.iter().map(|x| x.val).product::<u32>();
            }
        }
    }

    ans
}
