use tailcall::tailcall;

fn parse_1(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[tailcall]
fn predict_rec(row: Vec<i32>, accum: i32) -> i32 {
    let accum = accum + row.last().unwrap();

    let row = row.windows(2).map(|x| (x[1] - x[0])).collect::<Vec<_>>();

    if row.iter().all(|&x| x == 0) {
        return accum;
    }

    predict_rec(row, accum)
}

#[tailcall]
fn predict_rec_rev(row: Vec<i32>, accum: i32, flip: i8) -> i32 {
    let row = row.windows(2).map(|x| (x[1] - x[0])).collect::<Vec<_>>();
    let accum = accum - (row.first().unwrap() * (flip as i32));

    if row.iter().all(|&x| x == 0) {
        return accum;
    }

    predict_rec_rev(row, accum, flip * -1)
}

fn predict_1(data: Vec<Vec<i32>>) -> i32 {
    data.into_iter().map(|row| predict_rec(row, 0)).sum()
}

fn predict_2(data: Vec<Vec<i32>>) -> i32 {
    data.into_iter()
        .map(|row| {
            let accum = *row.first().unwrap();
            predict_rec_rev(row, accum, 1)
        })
        .sum()
}
pub fn part_1(input: String) {
    let data = parse_1(input);
    let ans = predict_1(data);
    dbg!(ans);
}

pub fn part_2(input: String) {
    let data = parse_1(input);
    let ans = predict_2(data);
    dbg!(ans);
}
