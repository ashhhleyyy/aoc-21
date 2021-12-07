use std::io::BufRead;

use rayon::prelude::*;

fn main() {
    let crabs = std::io::stdin().lock().lines().next().unwrap().unwrap().split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let max_position = *crabs.iter().max().unwrap();
    let best = (0..=max_position).into_iter().map(|target_pos| {
        let fuel_cost = crabs.par_iter().map(|crab| (target_pos - crab).abs())
            .map(crab_movement_cost_2)
            .sum::<i64>();
        (target_pos, fuel_cost)
    }).min_by_key(|(_, fuel)| *fuel).unwrap();
    println!("Best result is to move to {} which has a total fuel cost of {}", best.0, best.1);
}

fn crab_movement_cost(distance: i64) -> i64 {
    distance
}

fn crab_movement_cost_2(distance: i64) -> i64 {
    let mut total = 0;
    for i in 1..=distance {
        total += i;
    }
    total
}
