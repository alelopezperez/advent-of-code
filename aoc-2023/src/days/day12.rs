use std::{collections::HashMap, iter::zip};

fn parse(input: String) -> Vec<(String, Vec<u32>)> {
    input
        .lines()
        .map(|x| {
            let (conf, dup) = x.split_once(' ').unwrap();
            let conf = String::from(conf);
            let dup = dup
                .trim()
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (conf, dup)
        })
        .collect::<Vec<_>>()
}

fn check_conf(onsen: &String, dup: &Vec<u32>) -> bool {
    let spl = onsen
        .split('.')
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>();

    if spl.len() != dup.len() {
        return false;
    }

    zip(spl, dup).all(|(a, b)| a.len() == (*b as usize))
}

fn check_conf_2(onsen: &String, dup: &Vec<u32>) -> bool {
    let spl = onsen
        .split('.')
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>();

    if spl.len() != dup.len() {
        return false;
    }

    zip(spl, dup).all(|(a, b)| a.len() == (*b as usize))
}

fn find_combinations(onsen: &mut String, dup: &Vec<u32>, index: usize, ans: &mut Vec<String>) {
    if index >= onsen.len() {
        if check_conf(onsen, dup) {
            ans.push(onsen.clone());
        }
    } else if onsen.get(index..index + 1).unwrap() == "?" {
        onsen.replace_range(index..index + 1, ".");
        find_combinations(onsen, dup, index + 1, ans);
        onsen.replace_range(index..index + 1, "?");

        onsen.replace_range(index..index + 1, "#");
        find_combinations(onsen, dup, index + 1, ans);
        onsen.replace_range(index..index + 1, "?");
    } else {
        find_combinations(onsen, dup, index + 1, ans);
    }
}

fn find_rec(
    onesen: &String,
    word: String,
    list: Vec<u32>,
    dmg_count: usize,
    dmg_list: &Vec<u32>,
    index: usize,
    cache: &mut HashMap<(String, Vec<u32>), usize>,
) -> usize {
    if index >= onesen.len() {
        if list.len() == 1 {
            if *list.first().unwrap() as usize == dmg_count {
                cache.insert((word.clone(), list.clone()), 1);
                return 1;
            } else {
                cache.insert((word.clone(), list.clone()), 0);
                return 0;
            }
        } else if list.is_empty() {
            cache.insert((word.clone(), list.clone()), 1);
            return 1;
        }

        cache.insert((word.clone(), list.clone()), 0);
        return 0;
    }

    if list.is_empty() {
        let c = onesen.chars().skip(index + 1).filter(|x| *x == '#').count();
        if c > 0 {
            cache.insert((word.clone(), list.clone()), 0);
            return 0;
        }
        if dmg_count > 0 {
            cache.insert((word.clone(), list.clone()), 0);
            return 0;
        }
        cache.insert((word.clone(), list.clone()), 1);
        return 1;
    }

    if dmg_count > *list.first().unwrap() as usize {
        cache.insert((word.clone(), list.clone()), 0);
        return 0;
    }

    match cache.get(&(word.clone(), list.clone())) {
        Some(ammount) => *ammount,

        None => {
            let letter = onesen.get(index..index + 1).unwrap();
            match letter {
                "." => {
                    let new_word = word.clone() + letter;

                    if dmg_count == 0 {
                        let new_list = list.clone();

                        let ans = find_rec(
                            onesen,
                            new_word.clone(),
                            new_list,
                            dmg_count,
                            dmg_list,
                            index + 1,
                            cache,
                        );
                        cache.insert((new_word.clone(), list.clone()), ans.clone());
                        ans
                    } else if dmg_count == *list.first().unwrap() as usize {
                        let new_list = list[1..].to_vec();

                        let ans = find_rec(
                            onesen,
                            new_word.clone(),
                            new_list,
                            0,
                            dmg_list,
                            index + 1,
                            cache,
                        );
                        cache.insert((new_word.clone(), list.clone()), ans.clone());
                        ans
                    } else {
                        cache.insert((new_word.clone(), list.clone()), 0);
                        0
                    }
                }
                "#" => {
                    let new_word = word.clone() + letter;
                    let new_list = list.clone();

                    let ans = find_rec(
                        onesen,
                        new_word.clone(),
                        new_list,
                        dmg_count + 1,
                        dmg_list,
                        index + 1,
                        cache,
                    );
                    cache.insert((new_word.clone(), list.clone()), ans.clone());
                    ans
                }
                "?" => {
                    let new_word = word.clone() + "#";
                    let new_list = list.clone();

                    let one = find_rec(
                        onesen,
                        new_word,
                        new_list,
                        dmg_count + 1,
                        dmg_list,
                        index + 1,
                        cache,
                    );

                    let new_word = word.clone() + ".";
                    let two = {
                        if dmg_count == 0 {
                            let new_list = list.clone();

                            let ans = find_rec(
                                onesen,
                                new_word.clone(),
                                new_list,
                                dmg_count,
                                dmg_list,
                                index + 1,
                                cache,
                            );
                            ans
                        } else if dmg_count == *list.first().unwrap() as usize {
                            let new_list = list[1..].to_vec();

                            let ans = find_rec(
                                onesen,
                                new_word.clone(),
                                new_list,
                                0,
                                dmg_list,
                                index + 1,
                                cache,
                            );
                            ans
                        } else {
                            0
                        }
                    };

                    cache.insert((new_word.clone(), list.clone()), one + two);

                    one + two
                }
                _ => panic!("F"),
            }
        }
    }
}

pub fn part_1(input: String) {
    let data = parse(input);

    // let ans = data
    //     .iter()
    //     .map(|(onsen, dup)| {
    //         let mut ans = Vec::new();
    //         let mut on = onsen.clone();
    //         find_combinations(&mut on, dup, 0, &mut ans);
    //         ans.len()
    //     })
    //     .sum::<usize>();

    // println!("{ans}");

    // let mut cache = HashMap::new();
    // let ans = find_rec(
    //     &data[0].0,
    //     "".to_string(),
    //     data[0].1.clone(),
    //     0,
    //     &data[0].1,
    //     0,
    //     &mut cache,
    // );

    let ans = data
        .iter()
        .map(|(onsen, list)| {
            let mut cache: HashMap<(String, Vec<u32>), usize> = HashMap::new();

            find_rec(onsen, "".to_string(), list.clone(), 0, list, 0, &mut cache)
        })
        .sum::<usize>();
    println!("{ans}");
    //find_combinations(&mut "?###????????".to_string(), &vec![3, 2, 1], 0, &mut ans);
}
