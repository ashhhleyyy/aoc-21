use std::{io::BufRead};

const PART_2: bool = true;

fn main() {
    let stdin = std::io::stdin();
    let mut paper = Vec::<(i64, i64)>::new();
    let mut folds = Vec::new();

    for line in stdin.lock().lines().map(Result::unwrap) {
        if line.is_empty() { continue; }
        if line.starts_with("fold along ") {
            let (a, v) = line.split_once("=").unwrap();
            let v = v.parse::<i64>().unwrap();
            folds.push(if a.ends_with("x") {
                Fold::X(v)
            } else {
                Fold::Y(v)
            });
        } else {
            let (x, y) = line.split_once(",").unwrap();
            let (x, y) = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
            paper.push((x, y));
        }
    }

    for fold in folds {
        match fold {
            Fold::X(fold_x) => {
                for (x, _) in paper.iter_mut() {
                    if *x > fold_x {
                        *x = fold_x - (*x - fold_x);
                    }
                }
            }
            Fold::Y(fold_y) => {
                for (_, y) in paper.iter_mut() {
                    if *y > fold_y {
                        *y = fold_y - (*y - fold_y);
                    }
                }
            }
        }
        if !PART_2 { break; }
    }

    let max_x = *paper.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *paper.iter().map(|(_, y)| y).max().unwrap();

    let mut count = 0;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.contains(&(x, y)) {
                if PART_2 { print!("#"); }
                count += 1;
            } else {
                if PART_2 { print!("."); }
            }
        }
        if PART_2 { println!(); }
    }

    if !PART_2 { println!("{}", count); }
}

enum Fold {
    X(i64),
    Y(i64),
}
