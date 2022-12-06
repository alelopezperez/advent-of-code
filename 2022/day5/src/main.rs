use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let st1 = vec!['L', 'N', 'W', 'T', 'D'];
    let st2 = vec!['C', 'P', 'H'];
    let st3 = vec!['W', 'P', 'H', 'N', 'D', 'G', 'M', 'J'];
    let st4 = vec!['C', 'W', 'S', 'N', 'T', 'Q', 'L'];
    let st5 = vec!['P', 'H', 'C', 'N'];
    let st6 = vec!['T', 'H', 'N', 'D', 'M', 'W', 'Q', 'B'];
    let st7 = vec!['M', 'B', 'R', 'J', 'G', 'S', 'L'];
    let st8 = vec!['Z', 'N', 'W', 'G', 'V', 'B', 'R', 'T'];
    let st9 = vec!['W', 'G', 'D', 'N', 'P', 'L'];

    let mut crane = vec![st1, st2, st3, st4, st5, st6, st7, st8, st9];

    let mut cmd = contents.split("\n\n").skip(1);

    // println!("{:?}", crane);
    if let Some(mv) = cmd.next() {
        mv.lines().for_each(|com| {
            let moves: Vec<&str> = com.split(" ").collect();

            // println!("{com}");
            // println!("{} {} {}", moves[1], moves[3], moves[5]);

            let mut amount = moves[1].parse::<i32>().unwrap();

            let init = moves[3].parse::<usize>().unwrap() - 1;
            let dest = moves[5].parse::<usize>().unwrap() - 1;

            let mut temp: Vec<char> = Vec::new();
            while amount > 0 {
                let num = crane[init].pop().unwrap();
                temp.push(num);
                // crane[dest].push(num);
                amount -= 1;
            }

            while !temp.is_empty() {
                let num = temp.pop().unwrap();
                crane[dest].push(num);
            }
        })
    }

    crane.iter().for_each(|st| {
        print!("{}", st.last().unwrap());
    });
    print!("\n");
}
