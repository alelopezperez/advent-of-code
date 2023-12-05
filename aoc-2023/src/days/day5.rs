use std::cmp::min;

pub fn parse_data(lines: String) -> (Vec<u32>, Vec<Vec<(u32, u32, u32)>>) {
    let (seeds, data) = lines.split_once("\n\n").unwrap();

    let seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let almanac = data
        .split("\n\n")
        .map(|x| x.split_once(':').unwrap().1.trim())
        .map(|x| {
            x.split('\n')
                .map(|x| {
                    let mut it = x.split_whitespace();
                    (
                        it.next().unwrap().parse::<u32>().unwrap(),
                        it.next().unwrap().parse::<u32>().unwrap(),
                        it.next().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (seeds, almanac)
}

pub fn part_1(input: String) {
    let (seeds, almanac) = parse_data(input);

    println!("{}", find_lowest(seeds, almanac));
}

pub fn part_2(input: String) {
    let (seeds, almanac) = parse_data(input);

    let seeds = seeds
        .chunks(2)
        .flat_map(|x| (x[0]..=x[0] + x[1] - 1))
        .collect::<Vec<_>>();

    println!("{:?}", find_lowest(seeds, almanac));
}

fn find_lowest(seeds: Vec<u32>, almanac: Vec<Vec<(u32, u32, u32)>>) -> u32 {
    let mut positions = u32::MAX;

    for seed in seeds {
        let mut seed = seed;
        for part in almanac.iter() {
            // print!("{seed} -> ");
            let list = part.iter().find(|x| seed >= x.1 && seed <= x.1 + x.2 - 1);

            if let Some(list) = list {
                seed = list.0 + (seed - list.1);
            }

            // println!("{seed}    {:?}", list);
        }
        positions = min(positions, seed)
    }

    positions
}
