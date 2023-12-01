use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");
    let mut count = 0;
    contents.lines().for_each(|l| {
        let (st1, st2) = l.split_at(l.len() / 2);
        // println!("{} {}", st1, st2);
        let mut hmap: HashMap<char, i32> = HashMap::new();
        for ch in st1.chars() {
            hmap.entry(ch).or_insert(1);
        }

        for ch in st2.chars() {
            if hmap.contains_key(&ch) {
                hmap.insert(ch, 2);
            }
        }

        for (k, v) in hmap.clone() {
            if v == 2 {
                if k.is_uppercase() {
                    count += k as u32 - 'A' as u32 + 27;
                } else {
                    count += k as u32 - 'a' as u32 + 1;
                }
            }
        }
    });
    println!("{count}");

    let line: Vec<&str> = contents.lines().into_iter().collect();

    let mut i = 0;
    let mut count = 0;
    while i < line.len() {
        let mut hmap: HashMap<char, i32> = HashMap::new();

        let one = line[i];
        let two = line[i + 1];
        let three = line[i + 2];

        for ch in one.chars() {
            hmap.entry(ch).or_insert(1);
        }

        for ch in two.chars() {
            if hmap.contains_key(&ch) {
                match hmap.get(&ch) {
                    Some(v) => {
                        if v == &1 {
                            hmap.insert(ch, 2);
                        }
                    }
                    None => {}
                }
            }
        }

        for ch in three.chars() {
            if hmap.contains_key(&ch) {
                match hmap.get(&ch) {
                    Some(v) => {
                        if v == &2 {
                            hmap.insert(ch, 3);
                        }
                    }
                    None => {}
                }
            }
        }

        for (k, v) in hmap {
            if v == 3 {
                if k.is_uppercase() {
                    count += k as u32 - 'A' as u32 + 27;
                } else {
                    count += k as u32 - 'a' as u32 + 1;
                }
            }
        }

        i += 3;
    }

    println!("{count}");
}
