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

fn rec(
    hash_map: &HashMap<String, Vec<String>>,
    start: String,
    rules: &[String],
    combinations: (
        RangeInclusive<usize>,
        RangeInclusive<usize>,
        RangeInclusive<usize>,
        RangeInclusive<usize>,
    ),
) {
    let curr = start.clone();

    if curr == "A" {
        return;
    }
    if curr == "R" {
        return;
    }

    if rules.len() == 1 {
        rec(
            hash_map,
            rules[0].to_string(),
            &hash_map.get(&rules[0]).unwrap_or(&vec![])[..],
            combinations.clone(),
        );
    }
    if rules.len() == 0 {
        println!("what");
        return;
    }

    let check = rules[0].clone();
    println!("{curr} {:?} {}", check, rules.len());

    let (ev, goto) = check.split_once(':').unwrap();
    let ev: [&str; 2] = ev
        .split_inclusive('<')
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|x| {
            ev.split_inclusive('>')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        });
    let letter = ev[0].get(0..1).unwrap();
    let comp = ev[0].get(1..2).unwrap();
    let amm = ev[1].parse::<usize>().unwrap();

    println!("{:?}", ev);
    match comp {
        "<" => match letter {
            "x" => {
                println!("{goto}");
                rec(
                    hash_map,
                    goto.to_string(),
                    &hash_map.get(goto).unwrap_or(&vec![])[..],
                    (
                        *combinations.0.start()..=(amm - 1),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
                rec(
                    hash_map,
                    start.to_string(),
                    &rules[1..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
            }
            "m" => {
                rec(
                    hash_map,
                    goto.to_string(),
                    &hash_map.get(goto).unwrap_or(&vec![])[..],
                    (
                        combinations.0.clone(),
                        *combinations.1.start()..=(amm - 1),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
                rec(
                    hash_map,
                    start.to_string(),
                    &rules[1..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
            }
            "a" => {
                rec(
                    hash_map,
                    goto.to_string(),
                    &hash_map.get(goto).unwrap_or(&vec![])[..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        *combinations.2.start()..=(amm - 1),
                        combinations.3.clone(),
                    ),
                );
                rec(
                    hash_map,
                    start.to_string(),
                    &rules[1..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
            }
            "s" => {
                rec(
                    hash_map,
                    goto.to_string(),
                    &hash_map.get(goto).unwrap_or(&vec![])[..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        *combinations.3.start()..=(amm - 1),
                    ),
                );
                rec(
                    hash_map,
                    start.to_string(),
                    &rules[1..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
            }
            _ => {}
        },
        ">" => match letter {
            "x" => {
                rec(
                    hash_map,
                    goto.to_string(),
                    &hash_map.get(goto).unwrap_or(&vec![])[..],
                    (
                        (amm + 1)..=*combinations.0.start(),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
                rec(
                    hash_map,
                    start.to_string(),
                    &rules[1..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
            }
            "m" => {
                rec(
                    hash_map,
                    goto.to_string(),
                    &hash_map.get(goto).unwrap_or(&vec![])[..],
                    (
                        combinations.0.clone(),
                        (amm + 1)..=*combinations.1.start(),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
                rec(
                    hash_map,
                    start.to_string(),
                    &rules[1..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
            }
            "a" => {
                rec(
                    hash_map,
                    goto.to_string(),
                    &hash_map.get(goto).unwrap_or(&vec![])[..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        (amm + 1)..=*combinations.1.start(),
                        combinations.3.clone(),
                    ),
                );
                rec(
                    hash_map,
                    start.to_string(),
                    &rules[1..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
            }
            "s" => {
                rec(
                    hash_map,
                    goto.to_string(),
                    &hash_map.get(goto).unwrap_or(&vec![])[..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        (amm + 1)..=*combinations.3.start(),
                    ),
                );
                rec(
                    hash_map,
                    start.to_string(),
                    &rules[1..],
                    (
                        combinations.0.clone(),
                        combinations.1.clone(),
                        combinations.2.clone(),
                        combinations.3.clone(),
                    ),
                );
            }
            _ => {}
        },

        _ => panic!("No"),
    }

    // println!("{letter} {comp} {amm} {}", goto);
}
pub fn part_1(input: &str) {
    let mut hash_map = HashMap::new();
    let parts = parse_1(input, &mut hash_map);
    proc(&hash_map, parts.clone(), "in".to_string());
}

pub fn part_2(input: &str) {
    let mut hash_map = HashMap::new();
    let _parts = parse_1(input, &mut hash_map);

    rec(
        &hash_map,
        "in".to_string(),
        hash_map.get("in").unwrap(),
        (0..=4000, 0..=4000, 0..=4000, 0..=4000),
    );
}
