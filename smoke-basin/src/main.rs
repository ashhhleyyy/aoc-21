use std::{collections::HashMap, io::BufRead};

fn main() {
    let stdin = std::io::stdin();
    let mut map = HashMap::<(i64, i64), i64>::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in stdin.lock().lines().map(Result::unwrap).enumerate() {
        width = line.len() as i64;
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i64, y as i64), (c as i64)  - ('0' as i64));
        }
        height += 1;
    }

    let get = |x: i64, y: i64| -> i64 {
        return *map.get(&(x, y)).unwrap();
    };

    let mut total_risk_level = 0;

    for y in 0..height {
        for x in 0..width {
            let v = get(x, y);
            let mut is_min = true;
            if x > 0 {
                if get(x - 1, y) <= v {
                    is_min = false;
                }
            }
            if x < width - 1 {
                if get(x + 1, y) <= v {
                    is_min = false;
                }
            }
            if y > 0 {
                if get(x, y - 1) <= v {
                    is_min = false;
                }
            }
            if y < height - 1 {
                if get(x, y + 1) <= v {
                    is_min = false;
                }
            }
            if is_min {
                print!("{}", v);
                let risk_level = 1 + v;
                total_risk_level += risk_level;
            } else {
                print!("•");
            }
        }
        println!();
    }
    println!("Total risk level: {}", total_risk_level);
}
