use evalexpr::*;
use std::ops::RangeInclusive;
use std::{collections::HashMap, usize};

fn parse_1(input: &str, hash_map: &mut HashMap<String, Vec<String>>) -> Vec<Vec<usize>> {
    let (input, values) = input.split_once("\n\n").unwrap();
    input.lines().for_each(|l| {
        let (flow, rest) = l.split_once('{').unwrap();
        let ins = rest
            .trim_end_matches('}')
            .split(',')
            .map(|i| i.to_string())
            .collect::<Vec<_>>();

        hash_map.insert(flow.to_string(), ins.clone());
    });

    let values = values
        .lines()
        .map(|l| {
            l.trim_matches(|x| x == '{' || x == '}')
                .split(',')
                .map(|v| {
                    let (_, num) = v.split_once('=').unwrap();

                    num.parse::<usize>().unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    values
}

fn proc(hash_map: &HashMap<String, Vec<String>>, parts: Vec<Vec<usize>>, start: String) {
    let mut total = 0;

    for part in parts {
        let mut curr = start.clone();

        loop {
            if curr == "A" {
                total += part.iter().sum::<usize>();
                break;
            }
            if curr == "R" {
                break;
            }
            let mut rules: Vec<String> = hash_map.get(&curr).unwrap().clone();

            for (i, check) in rules.iter().enumerate() {
                let mut check = check.clone();
                if i != rules.len() - 1 {
                    match &check[0..1] {
                        "x" => {
                            check.replace_range(0..1, &part[0].to_string());
                        }
                        "m" => {
                            check.replace_range(0..1, &part[1].to_string());
                        }
                        "a" => {
                            check.replace_range(0..1, &part[2].to_string());
                        }
                        "s" => {
                            check.replace_range(0..1, &part[3].to_string());
                        }
                        _ => {}
                    }
                    let (ev, goto) = check.split_once(':').unwrap();
                    if let Ok(x) = eval(ev) {
                        if x == Value::from(true) {
                            curr = goto.to_string();
                            break;
                        }
                    }
                } else {
                    curr = check.to_string();
                }
            }
        }
    }
    println!("{total}");
}

fn all_combo(
    hash_map: &HashMap<String, Vec<String>>,
    curr: &str,
    mut combo: (
        RangeInclusive<usize>,
        RangeInclusive<usize>,
        RangeInclusive<usize>,
        RangeInclusive<usize>,
    ),
    approved_combos: &mut Vec<(
        RangeInclusive<usize>,
        RangeInclusive<usize>,
        RangeInclusive<usize>,
        RangeInclusive<usize>,
    )>,
) {
    if curr == "A" {
        // println!("ans {:?}", combo);
        approved_combos.push(combo);
        return;
    }
    if curr == "R" {
        return;
    }
    let rules = hash_map.get(curr).unwrap();
    for (i, rule) in rules.iter().enumerate() {
        if i == rules.len() - 1 {
            // println!("LAST {} len {}", rule, rules.len());
            all_combo(hash_map, rule, combo.clone(), approved_combos);
        } else {
            let (evaluation, goto) = rule.split_once(':').unwrap();
            let xmas = &evaluation[0..1];
            let comparison = &evaluation[1..2];
            let number = &evaluation[2..].parse::<usize>().unwrap();

            // println!("'{xmas}'-'{comparison}'-'{number}'");
            match comparison {
                "<" => match xmas {
                    "x" => {
                        let mut new_combo = combo.clone();
                        new_combo.0 = *new_combo.0.start()..=(number - 1);
                        all_combo(hash_map, goto, new_combo, approved_combos);
                        combo.0 = *number..=*combo.0.end();
                    }
                    "m" => {
                        let mut new_combo = combo.clone();
                        new_combo.1 = *new_combo.1.start()..=(number - 1);
                        all_combo(hash_map, goto, new_combo, approved_combos);
                        combo.1 = *number..=*combo.1.end();
                    }
                    "a" => {
                        let mut new_combo = combo.clone();
                        new_combo.2 = *new_combo.2.start()..=(number - 1);
                        all_combo(hash_map, goto, new_combo, approved_combos);
                        combo.2 = *number..=*combo.2.end();
                    }
                    "s" => {
                        let mut new_combo = combo.clone();
                        new_combo.3 = *new_combo.3.start()..=(number - 1);
                        all_combo(hash_map, goto, new_combo, approved_combos);
                        combo.3 = *number..=*combo.3.end();
                    }
                    _ => panic!("{xmas}"),
                },
                ">" => match xmas {
                    "x" => {
                        let mut new_combo = combo.clone();
                        new_combo.0 = (number + 1)..=*new_combo.0.end();
                        all_combo(hash_map, goto, new_combo, approved_combos);
                        combo.0 = *combo.0.start()..=*number;
                    }
                    "m" => {
                        let mut new_combo = combo.clone();
                        new_combo.1 = (number + 1)..=*new_combo.1.end();
                        all_combo(hash_map, goto, new_combo, approved_combos);
                        combo.1 = *combo.1.start()..=*number;
                    }
                    "a" => {
                        let mut new_combo = combo.clone();
                        new_combo.2 = (number + 1)..=*new_combo.2.end();
                        all_combo(hash_map, goto, new_combo, approved_combos);
                        combo.2 = *combo.2.start()..=*number;
                    }
                    "s" => {
                        let mut new_combo = combo.clone();
                        new_combo.3 = (number + 1)..=*new_combo.3.end();
                        all_combo(hash_map, goto, new_combo, approved_combos);
                        combo.3 = *combo.3.start()..=*number;
                    }
                    _ => panic!("{xmas}"),
                },
                _ => panic!("{comparison}"),
            }
            // println!("{:?}", combo);
        }
    }
}

pub fn part_1(input: &str) {
    let mut hash_map = HashMap::new();
    let parts = parse_1(input, &mut hash_map);
    proc(&hash_map, parts.clone(), "in".to_string());
}

pub fn part_2(input: &str) {
    let mut hash_map = HashMap::new();
    let _parts = parse_1(input, &mut hash_map);
    let mut combos = Vec::new();
    all_combo(
        &hash_map,
        "in",
        (1..=4000, 1..=4000, 1..=4000, 1..=4000),
        &mut combos,
    );

    println!("{}", combos.len());

    let ans = combos
        .iter()
        .map(|combo| {
            combo.0.clone().count()
                * combo.1.clone().count()
                * combo.2.clone().count()
                * combo.3.clone().count()
        })
        .sum::<usize>();
    println!("{ans}");
}
