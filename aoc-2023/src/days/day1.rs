use std::{cell::RefCell, rc::Rc};

type TrieNodeRef = Rc<RefCell<TrieNode>>;

impl TrieNode {
    fn find(&self, line: &Vec<char>, start: usize) -> (usize, usize) {
        let mut curr = Rc::new(RefCell::new((*self).clone()));
        let mut number = String::new();
        for i in start..line.len() {
            let new = curr.borrow().childs.clone();
            let mut flag = false;
            for child in new {
                if child.borrow().val == line[i] {
                    curr = child;
                    number.push(line[i]);
                    flag = true;
                    if curr.borrow().childs.len() == 0 {
                        return (
                            match &number[..] {
                                "one" => 1,
                                "two" => 2,
                                "three" => 3,
                                "four" => 4,
                                "five" => 5,
                                "six" => 6,
                                "seven" => 7,
                                "eight" => 8,
                                "nine" => 9,
                                _ => 0,
                            },
                            i,
                        );
                    }
                    break;
                }
            }

            if !flag {
                return (0, i);
            }
        }
        (0, 0)
    }
    fn fill(dic: String, root: Rc<RefCell<TrieNode>>) {
        let mut curr = root;

        for letter in dic.chars() {
            let mut flag = false;
            let list = curr.borrow().childs.clone();
            for child in list {
                if child.borrow().val == letter {
                    flag = true;
                    curr = child;
                    break;
                }
            }
            if !flag {
                let new = Rc::new(RefCell::new(TrieNode {
                    val: letter,
                    childs: Vec::new(),
                }));
                curr.borrow_mut().childs.push(new.clone());
                curr = new.clone();
            }
        }
    }
    fn new(dic: Vec<String>) -> Rc<RefCell<TrieNode>> {
        let root = Rc::new(RefCell::new(TrieNode {
            val: '*',
            childs: Vec::new(),
        }));

        for word in dic {
            TrieNode::fill(word, root.clone());
        }
        root
    }
}
#[derive(Debug, Clone)]
struct TrieNode {
    val: char,
    childs: Vec<TrieNodeRef>,
}

pub fn part_1(input: String) -> u32 {
    let lines = input
        .lines()
        .map(|x| x.to_string().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;
    for line in lines {
        let mut left = 0 as usize;
        let mut right = line.len() - 1;

        while left < right && (!line[left].is_numeric() || !line[right].is_numeric()) {
            if !line[left].is_numeric() {
                left += 1;
            }

            if !line[right].is_numeric() {
                right -= 1;
            }
        }

        ans += line[left].to_digit(10).unwrap() * 10 + line[right].to_digit(10).unwrap();
    }
    ans
}
pub fn part_2(input: String) -> i32 {
    let lines = input
        .lines()
        .map(|x| x.to_string().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    lines
        .iter()
        .map(|line| {
            let mut left = 0 as usize;
            let mut first = -1;
            let mut second = -1;

            while left < line.len() {
                if line[left].is_numeric() {
                    if first == -1 {
                        first = line[left].to_digit(10).unwrap() as i32;
                    }
                    second = line[left].to_digit(10).unwrap() as i32;
                    left += 1;
                    continue;
                }
                let mut right = left;

                while right < line.len() {
                    let case = String::from(&line[left..right + 1].iter().collect::<String>());
                    let value = match case.as_str() {
                        "one" => 1,
                        "two" => 2,
                        "three" => 3,
                        "four" => 4,
                        "five" => 5,
                        "six" => 6,
                        "seven" => 7,
                        "eight" => 8,
                        "nine" => 9,
                        _ => -1,
                    };

                    if value > 0 {
                        if first == -1 {
                            first = value;
                        }
                        second = value;
                    }
                    right += 1
                }
                left += 1;
            }
            first * 10 + second
        })
        .sum::<i32>()
}

pub fn part_2_trie(input: String) -> i32 {
    let dic = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    let trie = TrieNode::new(dic);

    let lines = input
        .lines()
        .map(|x| x.to_string().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut ans = 0;
    for line in lines {
        let mut first = -1 as i32;
        let mut last = -1;
        let mut i = 0;
        println!("{:?}", line);
        while i < line.len() {
            if line[i].is_numeric() {
                if first == -1 {
                    first = line[i].to_digit(10).unwrap() as i32;
                }
                last = line[i].to_digit(10).unwrap() as i32;
                i += 1;
                continue;
            }
            println!("GOES {i}");
            let (number, index) = trie.borrow().find(&line, i.clone());

            if number != 0 {
                println!("num: {number}");
                if first == -1 {
                    first = number as i32;
                }
                last = number as i32;
                i = index;
                println!("{}", i);
                continue;
            } else {
                println!("other {}", i);
                if index == 0 {
                    i += 1;
                    println!("    {}", i);
                    continue;
                }
            }
            i += 1;
        }
        ans += first * 10 + last;
    }
    ans
}
