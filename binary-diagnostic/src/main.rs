use std::{collections::HashMap, io::BufRead};

fn main() {
    println!("Hello, world!");
    let stdin = std::io::stdin();
    let mut counts: HashMap<usize, (u32, u32)> = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for (i, c) in line.chars().enumerate() {
            let e = counts.entry(i).or_insert((0, 0));
            match c {
                '0' => e.0 += 1,
                '1' => e.1 += 1,
                _ => unreachable!(),
            }
        }
    }

    let mut gamma = 0_u64;
    let mut epsilon = 0_u64;

    for (position, (zeros, ones)) in &counts {
        let gamma_bit: u64 = if zeros < ones { 1 } else { 0 };
        let epsilon_bit: u64 = if zeros > ones { 1 } else { 0 };
        let position = counts.len() - 1 - position;
        let gamma_bit = gamma_bit << position;
        let epsilon_bit = epsilon_bit << position;
        gamma |= gamma_bit;
        epsilon |= epsilon_bit;
    }

    dbg!(gamma * epsilon);
}
