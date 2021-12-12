use std::io::BufRead;

use ansi_term::Style;

fn main() {
    let mut grid = [[0_u8; 10]; 10];
    for (y, line) in std::io::stdin().lock().lines().map(Result::unwrap).enumerate() {
        for (x, c) in line.chars().enumerate() {
            let v = (c as u8) - ('0' as u8);
            grid[y][x] = v;
        }
    }

    let mut flash_count = 0;
    let bold = Style::new().bold();

    for i in 0..100 {
        let mut has_flashed_this_step = [[false; 10]; 10];
        // 1. Increment light levels by 1
        for x in 0..10 {
            for y in 0..10 {
                grid[y][x] += 1;
            }
        }
        // 2. Any with level >9 flash. Increase nearby octopi by 1 energy level
        let mut flash_queue = Vec::new();
        for x in 0..10 {
            for y in 0..10 {
                if grid[y][x] > 9 {
                    flash_queue.insert(0, (x, y));
                }
            }
        }
        while let Some((x, y)) = flash_queue.pop() {
            if has_flashed_this_step[y][x] { continue; }
            flash_count += 1;
            has_flashed_this_step[y][x] = true;
            for xOffset in -1..=1 {
                for yOffset in -1..=1 {
                    if xOffset == 0 && yOffset == 0 { continue; }
                    let (x, y) = ((x as isize) + xOffset, (y as isize) + yOffset);
                    if x < 0 || x > 9 || y < 0 || y > 9 { continue; }
                    grid[y as usize][x as usize] += 1;
                    if grid[y as usize][x as usize] > 9 {
                        flash_queue.push((x as usize, y as usize));
                    }
                }
            }
        }
        // 3. Set level of all octopi that flashed to 0
        for x in 0..10 {
            for y in 0..10 {
                if has_flashed_this_step[y][x] {
                    grid[y][x] = 0;
                }
            }
        }

        if (i+1) % 10 == 0 {
            println!("After step {}:", i + 1);
            for y in 0..10 {
                for x in 0..10 {
                    if has_flashed_this_step[y][x] {
                        print!("{}", bold.paint(grid[y][x].to_string()));
                    } else {
                        print!("{}", grid[y][x]);
                    }
                }
                println!()
            }
            println!()
        }
    }

    println!("Total flash count: {}", flash_count);
}
