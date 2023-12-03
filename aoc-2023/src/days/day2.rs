struct Game {
    id: u8,
    red: u8,
    green: u8,
    blue: u8,
}
pub fn part_1(input: String) -> i32 {
    let games = input
        .lines()
        .map(|line| {
            let (_, rem) = line.split_once(':').unwrap();
            let games = rem
                .split_terminator([',', ';'])
                .map(|x| x.trim())
                .map(|x| {
                    let (num, color) = x.split_once(' ').unwrap();

                    (num.parse::<u8>().unwrap(), color)
                })
                .collect::<Vec<_>>();

            games
        })
        .collect::<Vec<_>>();

    // print!("{:?}", games);
    let mut ans = 0;
    for (i, game) in games.into_iter().enumerate() {
        if game
            .into_iter()
            .map(|(num, colors)| match colors {
                "blue" => num <= 14,
                "green" => num <= 13,
                "red" => num <= 12,
                _ => true,
            })
            .fold(true, |acc, x| acc && x)
        {
            ans += (i + 1) as i32;
        }
    }
    ans
}
