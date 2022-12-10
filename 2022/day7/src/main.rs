use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut stack: Vec<String> = Vec::new();
    let mut hmap: HashMap<String, i32> = HashMap::new();

    contents.lines().for_each(|line| {
        if line.contains("$ cd") {
            if line.contains("/") {
                stack.clear();
                stack.push("<root>".to_string());
            } else if line.contains("..") {
                stack.pop();
            } else {
                let (_, dir_name) = line.split_once(" ").unwrap();
                let (_, dir_name) = dir_name.split_once(" ").unwrap();
                stack.push(String::from(dir_name.to_string()));
            }
        } else if line.contains("dir") {
            let (_, dir_name) = line.split_once(' ').unwrap();
            let mut full_path = stack.join("/");
            full_path.push_str("/");
            full_path.push_str(dir_name);

            if !hmap.contains_key(&full_path) {
                hmap.insert(full_path, 0);
            }
        } else if !line.contains("$ ls") {
            let (size, _dir_name) = line.split_once(" ").unwrap();
            let size = size.parse::<i32>().unwrap();

            let mut curr_path = String::from("");
            for path in &stack {
                curr_path.push_str(path);
                if hmap.contains_key(&curr_path) {
                    let val = hmap.get_mut(&curr_path).unwrap();
                    *val += size;
                } else {
                    hmap.insert(curr_path.clone(), size);
                }
                println!("mir: {curr_path} {size}");
                curr_path.push_str("/")
            }
        }
    });

    println!("{:#?}", hmap.keys());

    let mut sum = 0;

    for (_, v) in &hmap {
        if *v <= 100000 {
            sum += v;
        }
    }

    println!("{sum}");

    let space = 70000000 - hmap.get("<root>").unwrap();

    let mut ans2 = i32::MAX;

    let target = 30000000 - space;
    println!("target: {target}");
    for (_, v) in &hmap {
        if *v == target {
            ans2 = *v;
            break;
        } else {
            if *v > target && *v < ans2 {
                ans2 = *v;
            }
        }
    }

    println!("{ans2}");
}
