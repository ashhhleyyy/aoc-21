use std::io::BufRead;
use rayon::prelude::*;

const NUMBER_OF_GENERATIONS: i32 = 256;

fn main() {
    let input = std::io::stdin().lock().lines().next().unwrap().unwrap().split(",").map(|s| s.parse::<u8>().unwrap()).collect::<Vec<_>>();

    let mut fishes = input.clone();

    for i in 0..NUMBER_OF_GENERATIONS {
        let start_count = fishes.len();
        let number_of_fish_to_spawn = fishes.par_iter_mut().map(|fish| {
            if fish == &0 {
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
        println!("[gen {}] {} fish => {} fish ()", i, start_count, fishes.len());
    }
    println!("After all generations, there are {} fish", fishes.len());
}
