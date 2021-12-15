use std::{collections::{HashMap}, io::BufRead};

use ansi_term::{Style, Colour};

const PART_2: bool = false;

fn main() {
    let mut grid = HashMap::<(i64, i64), u8>::new();
    let stdin = std::io::stdin();
    for (y, line) in stdin.lock().lines().map(Result::unwrap).enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i64, y as i64), c as u8 - '0' as u8);
        }
    }

    if PART_2 {
        display_grid(&grid, None);
        let (og_max_x, max_y) = dbg!(max(&grid));
        // repeat on x-axis
        for i in 1..5u8 {
            for x in 0..=og_max_x {
                for y in 0..=max_y {
                    let v = *grid.get(&(x, y)).unwrap();
                    let new_x = x + ((i as i64) * (og_max_x + 1));
                    let mut new_level = v + i;
                    while new_level > 9 {
                        new_level -= 9;
                    }
                    grid.insert((new_x, y), new_level);
                }
            }
        }
        let (max_x, max_y) = dbg!(max(&grid));
        // repeat on y-axis
        for i in 1..5u8 {
            for x in 0..=max_x {
                for y in 0..=max_y {
                    let v = *grid.get(&(x, y)).unwrap();
                    let new_y = y + ((i as i64) * (max_y + 1));
                    let x_off = (x / (og_max_x+1)) as u8;
                    let mut new_level = v + i;
                    while new_level > 9 {
                        new_level -= 9;
                    }
                    grid.insert((x, new_y), new_level);
                }
            }
        }
    }
    let (max_x, max_y) = dbg!(max(&grid));

    // if !PART_2 {
        display_grid(&grid, None);
    // }

    let start_time = std::time::Instant::now();
    let path = pathfind(&grid, (0, 0), (max_x, max_y));
    let end_time = std::time::Instant::now();

    // if !PART_2 {
        println!();
        display_grid(&grid, Some(&path));
    // }

    println!("Pathfinding took {:?}!", end_time - start_time);
    println!("Risk: {}", path_risk(&grid, &path));
}

fn pathfind(grid: &HashMap<(i64, i64), u8>, start: (i64, i64), end: (i64, i64)) -> Vec<(i64, i64)> {
    let mut costs = HashMap::<(i64, i64), i64>::new();
    let mut parents = HashMap::<(i64, i64), (i64, i64)>::new();
    let mut open_set = vec![start];
    let mut closed_set = Vec::new();

    let (max_x, max_y) = dbg!(max(&grid));

    while !open_set.is_empty() {
        open_set.sort_by_cached_key(|p| costs.get(p).unwrap_or(&0));
        let current = open_set.remove(0);
        let current_cost = *costs.get(&current).unwrap_or(&0);
        closed_set.push(current);
        if current == end {
            return trace_path(parents, current, start);
        }
        for neighbour in neighbours(current, max_x, max_y) {
            if closed_set.contains(&neighbour) { continue; }
            let new_cost = current_cost + *grid.get(&neighbour).unwrap() as i64;// + manhatten_distance(neighbour, end);
            if let Some(cost) = costs.get_mut(&neighbour) {
                if *cost > new_cost {
                    *cost = new_cost;
                    parents.insert(neighbour, current);
                }
            } else {
                costs.insert(neighbour, new_cost);
                parents.insert(neighbour, current);
            }
            if !open_set.contains(&neighbour) {
                open_set.push(neighbour);
            }
        }
        dbg!(closed_set.len());
    }

    unreachable!()
}

fn trace_path(parents: HashMap<(i64, i64), (i64, i64)>, end: (i64, i64), start: (i64, i64)) -> Vec<(i64, i64)> {
    let mut path = Vec::new();
    let mut current = end;
    while current != start {
        path.push(current);
        current = *parents.get(&current).unwrap();
    }
    path.reverse();
    path
}

fn path_risk(grid: &HashMap<(i64, i64), u8>, path: &[(i64, i64)]) -> i64 {
    let mut score = 0;

    for &(x, y) in path {
        score += *grid.get(&(x, y)).unwrap() as i64;
    }

    score
}

fn manhatten_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs();
}

fn neighbours((x, y): (i64, i64), max_x: i64, max_y: i64) -> Vec<(i64, i64)> {
    let mut neighbours = Vec::with_capacity(4);
    if x > 0 {
        neighbours.push((x-1, y));
    }
    if y > 0 {
        neighbours.push((x, y-1));
    }
    if x < max_x {
        neighbours.push((x+1, y));
    }
    if y < max_y {
        neighbours.push((x, y+1));
    }
    neighbours
}

fn max(grid: &HashMap<(i64, i64), u8>) -> (i64, i64) {
    let max_x = *grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *grid.keys().map(|(_, y)| y).max().unwrap();
    (max_x, max_y)
}

fn display_grid(grid: &HashMap<(i64, i64), u8>, path: Option<&[(i64, i64)]>) {
    let (max_x, max_y) = max(grid);
    let mut bold = Style::new().bold();
    let dimmed = Style::new().dimmed();
    bold.foreground = Some(Colour::Cyan);
    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(path) = path {
                if path.contains(&(x, y)) {
                    print!("{}", bold.paint(format!("{}", grid.get(&(x, y)).unwrap())));
                } else {
                    print!("{}", dimmed.paint(format!("{}", grid.get(&(x, y)).unwrap())));
                }
            } else {
                print!("{}", grid.get(&(x, y)).unwrap());
            }
        }
        println!();
    }
}
