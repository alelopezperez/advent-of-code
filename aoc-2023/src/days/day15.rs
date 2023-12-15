fn parse_1(input: String) -> Vec<String> {
    input
        .trim()
        .split(',')
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
}

fn get_hash(data: &str) -> u32 {
    data.chars().fold(0, |curr, c| {
        let ascii = c as u32;
        ((curr + ascii) * 17) % 256
    })
}

pub fn part_1(input: String) {
    let data = parse_1(input);
    let ans = data.iter().map(|x| get_hash(x) as usize).sum::<usize>();
    println!("{ans}");
}

pub fn part_2(input: String) {
    let data = parse_1(input);
    let mut boxes = vec![vec!["".to_string(); 256]; 256];

    data.iter().for_each(|cmm| match cmm.split_once('-') {
        Some((cmd, _)) => {
            let hash = get_hash(cmd);
            let boc = boxes[hash as usize]
                .iter()
                .position(|x| x.contains(format!("{} ", cmd).as_str()));
            if let Some(pos) = boc {
                boxes[hash as usize][pos] = "".to_string();
                //let j = boxes[hash as usize].len() - 1;
            }
        }
        None => match cmm.split_once('=') {
            Some((cmm, num)) => {
                let hash = get_hash(cmm);
                let boc: Option<usize> = boxes[hash as usize]
                    .iter()
                    .position(|x| x.contains(format!("{} ", cmm).as_str()));
                if let Some(pos) = boc {
                    boxes[hash as usize][pos] = format!("{} {}", cmm, num);
                } else {
                    let count = boxes[hash as usize].iter().all(|s| s.is_empty());
                    if count {
                        boxes[hash as usize][0] = format!("{} {}", cmm, num);
                    } else {
                        let mut npos = 0;
                        let mut init = 0;
                        let count2 = boxes[hash as usize]
                            .iter()
                            .filter(|s| !s.is_empty())
                            .count();
                        while npos < boxes[hash as usize].len() && init < count2 {
                            if !boxes[hash as usize][npos].is_empty() {
                                init += 1;
                            }
                            npos += 1;
                        }

                        // println!("{} {}\n", cmm, num);
                        boxes[hash as usize][npos] = format!("{} {}", cmm, num);
                    }
                }
            }
            None => panic!(""),
        },
    });

    let ans = boxes.iter().enumerate().fold(0, |acc, (i, boxi)| {
        let bpos = i + 1;

        let mut posi = 0;
        let mut suma = 0;
        for lens in boxi.iter() {
            if !lens.is_empty() {
                let focal = lens.split_once(' ').unwrap().1.parse::<u32>().unwrap();
                posi += 1;
                suma += bpos as u32 * posi * focal;
            }
        }

        acc + suma
    });

    println!("{}", ans);
}
