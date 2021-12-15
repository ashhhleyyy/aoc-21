use std::io::{BufRead, self, Write};
use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    
    let mut polymer = String::new();
    let mut rules = HashMap::<(char, char), char>::new();
    let mut counts = HashMap::<char, u64>::new();
    
    for line in stdin.lock().lines().map(Result::unwrap) {
        if line.is_empty() { continue; }
        if let Some((from, to)) = line.split_once(" -> ") {
            let mut chars = from.chars();
            let a = chars.next().unwrap();
            let b = chars.next().unwrap();
            let to = to.chars().next().unwrap();
            rules.insert((a, b), to);
        } else {
            for c in line.chars() {
                *counts.entry(c).or_insert(0) += 1;
            }
            polymer.push_str(&line);
        }
    }

    let mut pairs = HashMap::<(char, char), u64>::new();

    for i in 0..(polymer.len()-1) {
        let mut chars = polymer.chars().skip(i);
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        *pairs.entry((a, b)).or_insert(0) += 1;
    }

    for step in 1..=40 {
        let mut inserted = 0;
        let mut new_pairs = HashMap::new();
        for (&pair, &count) in &pairs {
            if let Some(&to) = rules.get(&pair) {
                *new_pairs.entry((pair.0, to)).or_insert(0) += count;
                *new_pairs.entry((to, pair.1)).or_insert(0) += count;
                *counts.entry(to).or_insert(0) += count;
                inserted += count;
            }
        }

        pairs = new_pairs;
        println!("\r[{}] total_pairs={} total_count={} inserted={}", step, pairs.iter().map(|p| p.1).sum::<u64>(), counts.iter().map(|p| p.1).sum::<u64>(), inserted);
        io::stdout().flush().unwrap();
    }
    println!();

    let max = counts.iter().max_by_key(|(_,c)| **c).unwrap();
    let min = counts.iter().min_by_key(|(_,c)| **c).unwrap();
    println!("After 40 steps, max={}(element {}) min={}(element {}) max-min={}", max.1, max.0, min.1, min.0, max.1-min.1);
}
