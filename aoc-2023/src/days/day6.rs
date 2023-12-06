use tailcall::tailcall;

pub fn part_1() {
    let m = day6_rec(1, 49 - 1, 298);
    let m2 = day6_rec(1, 78 - 1, 1185);
    let m3 = day6_rec(1, 79 - 1, 1066);
    let m4 = day6_rec(1, 80 - 1, 1181);

    println!("FINAL: {m}");
    println!("FINAL: {m2}");
    println!("FINAL: {m3}");
    println!("FINAL: {m4}");

    println!("{}", m * m2 * m3 * m4);

    let x = day6_rec_tail(1, 49787980 - 1, 298118510661181, 0);
    println!("FINAL: {x}");
}
fn day6_rec(init: u64, remaining: u64, goal: u64) -> u64 {
    if remaining > 0 {
        if (init * remaining) > goal {
            1 + day6_rec(init + 1, remaining - 1, goal)
        } else {
            day6_rec(init + 1, remaining - 1, goal)
        }
    } else {
        0
    }
}

#[tailcall] // <- This is great!!!! it loops indefently instead of overflowing
fn day6_rec_tail(init: u64, remaining: u64, goal: u64, accum: u64) -> u64 {
    if remaining > 0 {
        if (init * remaining) > goal {
            day6_rec_tail(init + 1, remaining - 1, goal, accum + 1)
        } else {
            day6_rec_tail(init + 1, remaining - 1, goal, accum)
        }
    } else {
        accum
    }
}

fn day6_math(time: u64, distance: u64) {
    let time = time as f64;
    // let floor = time - f64::sqrt(time.p);
}
