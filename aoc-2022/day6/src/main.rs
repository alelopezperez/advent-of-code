use std::{
    collections::{HashSet, VecDeque},
    fs,
};
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut vq = VecDeque::new();
    let mut counter = 0;

    for ch in contents.chars() {
        vq.push_back(ch);
        counter += 1;
        if vq.len() == 4 {
            if has_unique_char(&vq) {
                println!("{counter}");
                break;
            } else {
                vq.pop_front();
            }
        }
    }

    let mut vq = VecDeque::new();
    let mut counter = 0;
    for ch in contents.chars() {
        vq.push_back(ch);
        counter += 1;
        if vq.len() == 14 {
            if has_unique_char(&vq) {
                println!("{counter}");
                break;
            } else {
                vq.pop_front();
            }
        }
    }
}

fn has_unique_char(vq: &VecDeque<char>) -> bool {
    let mut hset = HashSet::with_capacity(4);

    for elem in vq {
        if hset.contains(elem) {
            return false;
        } else {
            hset.insert(elem.clone());
        }
    }

    true
}
