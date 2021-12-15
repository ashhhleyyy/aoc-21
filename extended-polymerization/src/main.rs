use std::{io::BufRead, collections::HashMap};

fn main() {
    let stdin = std::io::stdin();
    let mut polymer = String::new();
    let mut rules = Vec::new();
    for line in stdin.lock().lines().map(Result::unwrap) {
        if line.is_empty() { continue; }
        if let Some((from, to)) = line.split_once(" -> ") {
            rules.push((from.to_string(), to.to_string()));
        } else {
            polymer.push_str(&line);
        }
    }

    for gen in 0..10 {
        let mut new_polymer = String::new();
        for i in 0..(polymer.len()-1) {
            let a = &polymer[i..i+2];
            let mut matched = false;
            for (from, to) in &rules {
                if from == a {
                    new_polymer.push_str(&format!("{}{}", &a[0..1], to));
                    matched = true;
                    break;
                }
            }
            if !matched {
                new_polymer.push_str(&a[0..1]);
            }
        }
        new_polymer.push_str(&polymer[polymer.len() - 1..]);
        let inserted_count = new_polymer.len() - polymer.len();
        polymer = new_polymer;
        let mut element_counts = HashMap::<char, i64>::new();
        for c in polymer.chars() {
            *element_counts.entry(c).or_insert(0) += 1;
        }

        let max = element_counts.iter().max_by_key(|(_,c)| **c).unwrap();
        let min = element_counts.iter().min_by_key(|(_,c)| **c).unwrap();

        println!("After step {}: inserted={} len={} max={} ({} times) min={} ({} times) max-min={}", gen+1, inserted_count, polymer.len(), max.0, max.1, min.0, min.1, max.1 - min.1);
    }
}
