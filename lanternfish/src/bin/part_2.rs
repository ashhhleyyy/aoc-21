use std::{io::{BufRead, Write}, collections::HashMap};
use rayon::prelude::*;

const NUMBER_OF_GENERATIONS: i32 = 256;
const START_FROM: usize = 0;

fn main() {
    let input = std::io::stdin().lock().lines().next().unwrap().unwrap().split(",").map(|s| s.parse::<u8>().unwrap()).collect::<Vec<_>>();

    let mut result_cache = HashMap::<u8, u64>::new();

    let total_fish = input.iter().enumerate().skip(START_FROM).map(|(group_id, initial_fish)| {
        let count = if let Some(count) = result_cache.get(initial_fish) {
            println!("[group={}] Cache hit, result = {}", group_id, count);
            *count
        } else {
            let mut fishes = vec![*initial_fish];

            for i in 0..NUMBER_OF_GENERATIONS {
                let start_count = fishes.len();
                let number_of_fish_to_spawn = fishes.par_iter_mut().map(|fish| {
                    if *fish == 0 {
                        *fish = 6;
                        1
                    } else {
                        *fish -= 1;
                        0
                    }
                }).sum::<u64>();
                for _ in 0..number_of_fish_to_spawn {
                    fishes.push(8);
                }
                println!("[group={},gen={}] {} fish => {} fish", group_id, i, start_count, fishes.len());
            }

            fishes.len() as u64
        };

        result_cache.insert(*initial_fish, count);

        {
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("results.txt").unwrap();
        
            f.write_fmt(format_args!("{},{},{}\n", group_id, initial_fish, count)).unwrap();
        }

        count
    }).sum::<u64>();

    println!("After all generations, there are {} fish", total_fish);
}
