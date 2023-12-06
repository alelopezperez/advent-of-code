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
}
fn day6_rec(init: u32, remaining: u32, goal: u32) -> u32 {
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
