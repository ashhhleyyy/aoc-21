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

    let is_min_point = |x: i64, y: i64| -> bool {
        let v = get(x, y);
        if x > 0 {
            if get(x - 1, y) <= v {
                return false;
            }
        }
        if x < width - 1 {
            if get(x + 1, y) <= v {
                return false;
            }
        }
        if y > 0 {
            if get(x, y - 1) <= v {
                return false;
            }
        }
        if y < height - 1 {
            if get(x, y + 1) <= v {
                return false;
            }
        }
        true
    };

    let mut basin_sizes = HashMap::<(i64, i64), i64>::new();

    for y in 0..height {
        for x in 0..width {
            let v = get(x, y);
            let is_min = is_min_point(x, y);
            if is_min {
                print!("{}", v);
            } else {
                print!("•");
            }

            if v == 9 {
                continue;
            }

            let (basin_x, basin_y) = walk_and_find_basin(get, width, height, x, y);
            *basin_sizes.entry((basin_x, basin_y)).or_insert(0) += 1;
        }
        println!();
    }
    let mut basins = basin_sizes.iter().collect::<Vec<_>>();
    basins.sort_by_key(|basin| basin.1);
    basins.reverse();
    let first_three = basins.iter().take(3);
    let mut result = 1;
    for v in first_three {
        result *= v.1;
    }
    println!("Largest basins: {}", result);
}

fn walk_and_find_basin<F>(get: F, width: i64, height: i64, x: i64, y: i64) -> (i64, i64) where F: Fn(i64, i64) -> i64 {
    let v = get(x, y);
    if x > 0 {
        if get(x - 1, y) < v {
            return walk_and_find_basin(get, width, height, x - 1, y);
        }
    }
    if x < width - 1 {
        if get(x + 1, y) < v {
            return walk_and_find_basin(get, width, height, x + 1, y);
        }
    }
    if y > 0 {
        if get(x, y - 1) < v {
            return walk_and_find_basin(get, width, height, x, y - 1);
        }
    }
    if y < height - 1 {
        if get(x, y + 1) < v {
            return walk_and_find_basin(get, width, height, x, y + 1);
        }
    }
    (x, y)
}
