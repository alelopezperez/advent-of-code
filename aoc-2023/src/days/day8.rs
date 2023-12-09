use num::integer::lcm;
use std::collections::HashMap;

fn parse_1(input: &str) -> (Vec<char>, Vec<(&str, usize, usize)>, HashMap<&str, usize>) {
    let (instructions, parse) = input.split_once("\n\n").unwrap();
    let insctructions = instructions.chars().collect::<Vec<_>>();

    let mut k_v = HashMap::new();
    let parse = parse
        .lines()
        .enumerate()
        .map(|(i, x)| {
            let (src, dest) = x.split_once('=').unwrap();
            let src = src.trim();
            let (left, right) = dest.split_once(',').unwrap();

            k_v.insert(src, i);
            (
                (src, i),
                (
                    left.trim().trim_matches('('),
                    right.trim().trim_matches(')'),
                ),
            )
        })
        .collect::<Vec<_>>();

    let adj_list = parse
        .into_iter()
        .clone()
        .map(|((src, _), (l, r))| (src, *k_v.get(l).unwrap(), *k_v.get(r).unwrap()))
        .collect::<Vec<_>>();

    (insctructions, adj_list, k_v)
}
pub fn part_1(input: String) {
    let (instructions, graph, init) = parse_1(&input);

    let mut curr = graph[*init.get("AAA").unwrap()];

    let mut count = 0;
    let mut index = 0;

    while curr.0 != "ZZZ" {
        if index >= instructions.len() {
            index = 0;
        }

        if instructions[index] == 'L' {
            // println!(
            //     " At {:?}  going {}  so {:?}",
            //     curr, instructions[index], graph[curr.1]
            // );
            curr = graph[curr.1];
        } else {
            // println!(
            //     " At {:?}  going {}  so {:?}",
            //     curr, instructions[index], graph[curr.2]
            // );
            curr = graph[curr.2];
        }
        index += 1;
        count += 1;
    }
    println!("{count}");
}

pub fn part_2(input: String) {
    let (instructions, graph, _init) = parse_1(&input);

    let mut inits = graph
        .iter()
        .enumerate()
        .filter_map(|(i, x)| if x.0.ends_with('A') { Some(i) } else { None })
        .collect::<Vec<_>>();

    let multiples = inits
        .iter()
        .map(|x| count_steps(*x, &graph, &instructions) as u64)
        .collect::<Vec<_>>();

    let ans = multiples.into_iter().reduce(lcm);

    println!("{:?}", ans.unwrap())
}

fn count_steps(index: usize, graph: &Vec<(&str, usize, usize)>, instruction: &Vec<char>) -> u32 {
    let mut count = 0;
    let mut ins = 0;
    let mut init = index;
    while !graph[init].0.ends_with('Z') {
        if ins >= instruction.len() {
            ins = 0;
        }
        if instruction[ins] == 'L' {
            init = graph[init].1;
        } else {
            init = graph[init].2;
        }
        count += 1;
        ins += 1;
    }
    count
}
