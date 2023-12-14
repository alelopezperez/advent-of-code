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

fn parse2(input: String) -> Vec<(String, Vec<u32>)> {
    input
        .lines()
        .map(|x| {
            let (conf, dup) = x.split_once(' ').unwrap();
            let conf = String::from(conf);
            let conf = conf.clone()
                + "?"
                + conf.as_str()
                + "?"
                + conf.as_str()
                + "?"
                + conf.as_str()
                + "?"
                + conf.as_str();

            let dup = dup
                .trim()
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let dup = dup.repeat(5);
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
    cache: &mut HashMap<(String, usize, usize), usize>,
) -> usize {
    if index >= onesen.len() {
        if list.len() == 1 {
            if *list.first().unwrap() as usize == dmg_count {
                cache.insert((word.clone(), list.len(), dmg_count), 1);
                return 1;
            } else {
                cache.insert((word.clone(), list.len(), dmg_count), 0);
                return 0;
            }
        } else if list.is_empty() && dmg_count == 0 {
            cache.insert((word.clone(), list.len(), dmg_count), 1);
            return 1;
        }

        cache.insert((word.clone(), list.len(), dmg_count), 0);
        return 0;
    }

    if list.is_empty() {
        let c = onesen.chars().skip(index).filter(|x| *x == '#').count();
        if c > 0 {
            cache.insert((word.clone(), list.len(), dmg_count), 0);
            return 0;
        }
        if dmg_count > 0 {
            cache.insert((word.clone(), list.len(), dmg_count), 0);
            return 0;
        }
        cache.insert((word.clone(), list.len(), dmg_count), 1);
        return 1;
    }

    if dmg_count > *list.first().unwrap() as usize {
        cache.insert((word.clone(), list.len(), dmg_count), 0);
        return 0;
    }

    match cache.get(&(word.clone(), list.len(), dmg_count)) {
        Some(ammount) => {
            println!("g");
            *ammount
        }

        None => {
            let letter = onesen.get(index..index + 1).unwrap();
            match letter {
                "." => {
                    let new_word = word.clone() + letter;
                    let mut t = 0;
                    if dmg_count == 0 {
                        let new_list = list.clone();

                        if let Some(ca) = cache.get(&(new_word.clone(), new_list.len(), dmg_count))
                        {
                            println!("f");

                            return *ca;
                        }

                        let ans = find_rec(
                            onesen,
                            new_word.clone(),
                            new_list.clone(),
                            0,
                            dmg_list,
                            index + 1,
                            cache,
                        );
                        cache.insert((new_word.clone(), new_list.len(), dmg_count), ans.clone());
                        t += ans;
                        return ans;
                    }
                    if dmg_count == *list.first().unwrap() as usize {
                        let new_list = list[1..].to_vec();
                        if let Some(ca) = cache.get(&(new_word.clone(), new_list.len(), 0)) {
                            println!("d");

                            return *ca;
                        }

                        let ans = find_rec(
                            onesen,
                            new_word.clone(),
                            new_list.clone(),
                            0,
                            dmg_list,
                            index + 1,
                            cache,
                        );
                        t += ans;
                        cache.insert((new_word.clone(), new_list.len(), 0), t);
                        t
                    } else {
                        if let Some(ca) = cache.get(&(new_word.clone(), list.len(), dmg_count)) {
                            println!("a");

                            return *ca;
                        }
                        cache.insert((new_word.clone(), list.len(), dmg_count), 0);
                        0
                    }
                }
                "#" => {
                    let new_word = word.clone() + letter;
                    let new_list = list.clone();

                    let ans = find_rec(
                        onesen,
                        new_word.clone(),
                        new_list.clone(),
                        dmg_count + 1,
                        dmg_list,
                        index + 1,
                        cache,
                    );
                    if let Some(ca) = cache.get(&(new_word.clone(), new_list.len(), dmg_count + 1))
                    {
                        // println!("p");

                        return *ca;
                    }
                    cache.insert(
                        (new_word.clone(), new_list.len(), dmg_count + 1),
                        ans.clone(),
                    );
                    ans
                }
                "?" => {
                    let new_word = word.clone() + "#";
                    let new_list = list.clone();

                    let one = {
                        if let Some(ca) =
                            cache.get(&(new_word.clone(), new_list.len(), dmg_count + 1))
                        {
                            println!("asd");

                            return *ca;
                        }
                        let ans = find_rec(
                            onesen,
                            new_word.clone(),
                            new_list.clone(),
                            dmg_count + 1,
                            dmg_list,
                            index + 1,
                            cache,
                        );
                        cache.insert((new_word.clone(), new_list.len(), dmg_count + 1), ans);
                        ans
                    };

                    let new_word = word.clone() + ".";
                    let two = {
                        if dmg_count == 0 {
                            let new_list = list.clone();

                            if let Some(ca) =
                                cache.get(&(new_word.clone(), new_list.len(), dmg_count + 1))
                            {
                                println!("x");

                                return *ca;
                            }

                            let ans = find_rec(
                                onesen,
                                new_word.clone(),
                                new_list.clone(),
                                dmg_count,
                                dmg_list,
                                index + 1,
                                cache,
                            );
                            cache.insert((new_word.clone(), new_list.len(), dmg_count), ans);
                            ans
                        } else if dmg_count == *list.first().unwrap() as usize {
                            let new_list = list[1..].to_vec();

                            if let Some(ca) =
                                cache.get(&(new_word.clone(), new_list.len(), dmg_count + 1))
                            {
                                println!("n");

                                return *ca;
                            }
                            let ans = find_rec(
                                onesen,
                                new_word.clone(),
                                new_list.clone(),
                                0,
                                dmg_list,
                                index + 1,
                                cache,
                            );
                            cache.insert((new_word.clone(), new_list.len(), 0), ans);
                            ans
                        } else {
                            0
                        }
                    };

                    cache.insert((word.clone(), list.len(), dmg_count), one + two);

                    one + two
                }
                _ => panic!("F"),
            }
        }
    }
}

pub fn part_1(input: String) {
    let data = parse2(input);

    let ans = data
        .iter()
        .map(|(onsen, list)| {
            let mut cache = HashMap::new();

            let an = find_rec(onsen, "".to_string(), list.clone(), 0, list, 0, &mut cache);
            println!("{}", cache.len());
            an
        })
        .sum::<usize>();
    println!("{ans}");
    //find_combinations(&mut "?###????????".to_string(), &vec![3, 2, 1], 0, &mut ans);
}

pub fn part_2(input: String) {
    let data = parse(input);

    let ans = data
        .iter()
        .map(|(onsen, list)| {
            let mut cache = HashMap::new();

            find_rec(onsen, "".to_string(), list.clone(), 0, list, 0, &mut cache)
        })
        .sum::<usize>();
    println!("{ans}");
    //find_combinations(&mut "?###????????".to_string(), &vec![3, 2, 1], 0, &mut ans);
}
