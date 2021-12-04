use std::{collections::HashMap, io::BufRead};

fn main() {
    println!("Hello, world!");
    let stdin = std::io::stdin();
    let mut values = Vec::new();
    let mut counts: HashMap<usize, (u32, u32)> = HashMap::new();
    let mut bit_len = 0_u64;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        values.push(u64::from_str_radix(&line, 2).unwrap());
        for (i, c) in line.chars().enumerate() {
            let e = counts.entry(i).or_insert((0, 0));
            match c {
                '0' => e.0 += 1,
                '1' => e.1 += 1,
                _ => unreachable!(),
            }
        }
    }

    let oxygen_generator_rating = bit_search(&values, counts.len(), true);
    let co2_scrubber_rating = bit_search(&values, counts.len(), false);
    dbg!(oxygen_generator_rating, co2_scrubber_rating, oxygen_generator_rating * co2_scrubber_rating);
}

fn bit_search(candidates: &[u64], bit_len: usize, most_common: bool) -> u64 {
    let mut candidates = candidates.to_vec();
    let mut bit = bit_len;
    while candidates.len() > 1 {
        bit -= 1;
        println!("Considering bit {}", bit);
        let target_bit_mask = 1_u64 << bit;
        let (zeros, ones) = counts(&candidates, bit);
        let target_bit_value = if (zeros > ones) ^ !most_common {
            0_u64
        } else if zeros == ones {
            if most_common { 1_u64 } else { 0_u64 }
        } else {
            1_u64
        } << bit;
        candidates = candidates.into_iter().filter(|v| {
            v & target_bit_mask == target_bit_value
        }).collect::<Vec<_>>();
        println!("mask={} value={} candidates={:?}", target_bit_mask, target_bit_value, &candidates.iter().map(|v| format!("{:#b}", v)).collect::<Vec<_>>());
    }
    let value = *candidates.first().unwrap();
    println!("{:#b}", value);
    println!("-------------------------");
    return value;
}

fn counts(values: &[u64], bit: usize) -> (u64, u64) {
    let (mut zeros, mut ones) = (0, 0);

    for v in values {
        if v & (1_u64 << bit) == 0 {
            zeros += 1;
        } else {
            ones += 1;
        }
    }

    (zeros, ones)
}
