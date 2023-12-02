pub fn part_1(input: String) -> u32 {
    let lines = input
        .lines()
        .map(|x| x.to_string().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;
    for line in lines {
        let mut left = 0 as usize;
        let mut right = line.len() - 1;

        while left < right && (!line[left].is_numeric() || !line[right].is_numeric()) {
            if !line[left].is_numeric() {
                left += 1;
            }

            if !line[right].is_numeric() {
                right -= 1;
            }
        }

        ans += line[left].to_digit(10).unwrap() * 10 + line[right].to_digit(10).unwrap();
    }
    ans
}
pub fn part_2(input: String) -> i32 {
    let lines = input
        .lines()
        .map(|x| x.to_string().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    lines
        .iter()
        .map(|line| {
            let mut left = 0 as usize;
            let mut first = -1;
            let mut second = -1;

            while left < line.len() {
                if line[left].is_numeric() {
                    if first == -1 {
                        first = line[left].to_digit(10).unwrap() as i32;
                    }
                    second = line[left].to_digit(10).unwrap() as i32;
                    left += 1;
                    continue;
                }
                let mut right = left;

                while right < line.len() {
                    let case = String::from(&line[left..right + 1].iter().collect::<String>());
                    let value = match case.as_str() {
                        "one" => 1,
                        "two" => 2,
                        "three" => 3,
                        "four" => 4,
                        "five" => 5,
                        "six" => 6,
                        "seven" => 7,
                        "eight" => 8,
                        "nine" => 9,
                        _ => -1,
                    };

                    if value > 0 {
                        if first == -1 {
                            first = value;
                        }
                        second = value;
                    }
                    right += 1
                }
                left += 1;
            }
            first * 10 + second
        })
        .sum::<i32>()
}
