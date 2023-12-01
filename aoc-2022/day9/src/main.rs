use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut cols = 0;
    let mut rows = 0;

    let mut temp_cols = 0;
    let mut temp_rows = 0;
    contents.lines().for_each(|l| {
        if l.contains("R") {
            temp_cols += l.split_once(" ").unwrap().1.parse::<i32>().unwrap() + 1;
        }
        if l.contains("L") {
            temp_cols -= l.split_once(" ").unwrap().1.parse::<i32>().unwrap() + 1;
        }
        if l.contains("U") {
            temp_rows += l.split_once(" ").unwrap().1.parse::<i32>().unwrap() + 1;
        }
        if l.contains("D") {
            temp_rows -= l.split_once(" ").unwrap().1.parse::<i32>().unwrap() + 1;
        }

        if temp_cols < 0 {
            cols += temp_cols.abs();
            temp_cols = 0;
        } else if temp_cols > cols {
            cols += temp_cols - cols;
        }

        if temp_rows < 0 {
            rows += temp_rows.abs();
            temp_rows = 0;
        } else if temp_rows > rows {
            rows += temp_rows - rows;
        }
    });

    let (mut H_srow, mut H_scol) = (0, 0);
    let (mut T_srow, mut T_scol) = (H_srow, H_scol);

    let mut hset: HashSet<(i32, i32)> = HashSet::new();

    contents.lines().for_each(|l| {
        if l.contains("R") {
            let mut amount = l.split_once(" ").unwrap().1.parse::<i32>().unwrap();

            while amount > 0 {
                amount -= 1;
                H_scol += 1;

                if H_srow == T_srow {
                    if H_scol > (T_scol + 1) {
                        T_scol += 1;
                    }
                } else if !((T_srow + 1 == H_srow && T_scol + 1 == H_scol)
                    || (T_srow + 1 == H_srow && T_scol - 1 == H_scol)
                    || (T_srow - 1 == H_srow && T_scol + 1 == H_scol)
                    || (T_srow - 1 == H_srow && T_scol - 1 == H_scol))
                {
                    if H_srow > T_srow {
                        if H_scol > T_scol {
                            T_srow += 1;
                            T_scol += 1;
                        } else if H_scol < T_scol {
                            T_srow += 1;
                            T_scol -= 1;
                        }
                    } else if H_srow < T_srow {
                        if H_scol > T_scol {
                            T_srow -= 1;
                            T_scol += 1;
                        } else if H_scol < T_scol {
                            T_srow -= 1;
                            T_scol -= 1;
                        }
                    }
                }

                hset.insert((T_srow, T_scol));
            }
        }
        if l.contains("L") {
            let mut amount = l.split_once(" ").unwrap().1.parse::<i32>().unwrap();
            println!("{l}");

            while amount > 0 {
                amount -= 1;
                H_scol -= 1;

                if H_srow == T_srow {
                    if H_scol < (T_scol - 1) {
                        T_scol -= 1;
                    }
                } else if !((T_srow + 1 == H_srow && T_scol + 1 == H_scol)
                    || (T_srow + 1 == H_srow && T_scol - 1 == H_scol)
                    || (T_srow - 1 == H_srow && T_scol + 1 == H_scol)
                    || (T_srow - 1 == H_srow && T_scol - 1 == H_scol))
                {
                    if H_srow > T_srow {
                        if H_scol > T_scol {
                            T_srow += 1;
                            T_scol += 1;
                        } else if H_scol < T_scol {
                            T_srow += 1;
                            T_scol -= 1;
                        }
                    } else if H_srow < T_srow {
                        if H_scol > T_scol {
                            T_srow -= 1;
                            T_scol += 1;
                        } else if H_scol < T_scol {
                            T_srow -= 1;
                            T_scol -= 1;
                        }
                    }
                }

                hset.insert((T_srow, T_scol));
            }
        }
        if l.contains("U") {
            let mut amount = l.split_once(" ").unwrap().1.parse::<i32>().unwrap();

            while amount > 0 {
                amount -= 1;
                H_srow -= 1;

                if H_scol == T_scol {
                    if H_srow < T_srow - 1 {
                        T_srow -= 1;
                    }
                } else if !((T_srow + 1 == H_srow && T_scol + 1 == H_scol)
                    || (T_srow + 1 == H_srow && T_scol - 1 == H_scol)
                    || (T_srow - 1 == H_srow && T_scol + 1 == H_scol)
                    || (T_srow - 1 == H_srow && T_scol - 1 == H_scol))
                {
                    if H_srow > T_srow {
                        if H_scol > T_scol {
                            T_srow += 1;
                            T_scol += 1;
                        } else if H_scol < T_scol {
                            T_srow += 1;
                            T_scol -= 1;
                        }
                    } else if H_srow < T_srow {
                        if H_scol > T_scol {
                            T_srow -= 1;
                            T_scol += 1;
                        } else if H_scol < T_scol {
                            T_srow -= 1;
                            T_scol -= 1;
                        }
                    }
                }

                hset.insert((T_srow, T_scol));
            }
        }
        if l.contains("D") {
            let mut amount = l.split_once(" ").unwrap().1.parse::<i32>().unwrap();

            while amount > 0 {
                amount -= 1;
                H_srow += 1;

                if H_scol == T_scol {
                    if H_srow > T_srow + 1 {
                        T_srow += 1;
                    }
                } else if !((T_srow + 1 == H_srow && T_scol + 1 == H_scol)
                    || (T_srow + 1 == H_srow && T_scol - 1 == H_scol)
                    || (T_srow - 1 == H_srow && T_scol + 1 == H_scol)
                    || (T_srow - 1 == H_srow && T_scol - 1 == H_scol))
                {
                    if H_srow > T_srow {
                        if H_scol > T_scol {
                            T_srow += 1;
                            T_scol += 1;
                        } else if H_scol < T_scol {
                            T_srow += 1;
                            T_scol -= 1;
                        }
                    } else if H_srow < T_srow {
                        if H_scol > T_scol {
                            T_srow -= 1;
                            T_scol += 1;
                        } else if H_scol < T_scol {
                            T_srow -= 1;
                            T_scol -= 1;
                        }
                    }
                }

                hset.insert((T_srow, T_scol));
            }
        }
    });

    let mut count = hset.len();
    println!("{count}");
}
