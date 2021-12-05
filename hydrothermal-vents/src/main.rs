use std::collections::HashMap;
use std::io::BufRead;

const INCLUDE_DIAGONALS: bool = true;

fn main() {
    let stdin = std::io::stdin();
    let mut counts: HashMap<(i64, i64), i64> = HashMap::new();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let (c1, c2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = c1.split_once(",").unwrap();
        let (x2, y2) = c2.split_once(",").unwrap();
        let (x1, y1) = (x1.parse::<i64>().unwrap(), y1.parse::<i64>().unwrap());
        let (x2, y2) = (x2.parse::<i64>().unwrap(), y2.parse::<i64>().unwrap());
        // dbg!(&x1, &y1, &x2, &y2);
        if x1 != x2 && y1 != y2 {
            if !INCLUDE_DIAGONALS {
                println!("Skip");
                continue;
            }
            println!("Diagonal");
            let xstep = if x2 < x1 { -1_i64 } else { 1_i64 };
            let ystep = if y2 < y1 { -1_i64 } else { 1_i64 };
            let mut x = x1 - xstep;
            let mut y = y1 - ystep;
            while x != x2 && y != y2 {
                x += xstep;
                y += ystep;
                *counts.entry((x, y)).or_insert(0) += 1;
            }
        }
        if x1 == x2 {
            eprintln!("Vertial");
            for y in i64::min(y1, y2)..=i64::max(y1, y2) {
                // dbg!(&x1, &y);
                *counts.entry((x1, y)).or_insert(0) += 1;
            }
        }
        if y1 == y2 {
            eprintln!("Horizontal");
            for x in i64::min(x1, x2)..=i64::max(x1, x2) {
                // dbg!(&x, &y1);
                *counts.entry((x, y1)).or_insert(0) += 1;
            }
        }
        // println!("---------");
    }

    // dbg!(&counts);

    let w = counts.keys().map(|k| k.0).max().unwrap() + 1;
    let h = counts.keys().map(|k| k.1).max().unwrap() + 1;

    for y in 0..h {
        for x in 0..w {
            if let Some(v) = counts.get(&(x, y)) {
                print!("{}", v);
            } else {
                print!(".");
            }
        }
        println!();
    }

    let mut above_2 = 0_u64;
    for v in counts.values() {
        if v >= &2 {
            above_2 += 1;
        }
    }
    dbg!(above_2);
}
