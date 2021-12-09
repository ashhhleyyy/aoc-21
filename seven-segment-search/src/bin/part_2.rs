use std::{io::BufRead, collections::{HashMap, HashSet}};

fn main() {
    let stdin = std::io::stdin();

    let segment_values = build_value_mappings();

    for line in stdin.lock().lines().map(Result::unwrap) {
        let (unique_values, output_digits) = line.split_once(" | ").unwrap();
        let (unique_values, output_digits) = (unique_values.split(" "), output_digits.splitn(4, " "));
        let mut mappings = HashMap::<char, char>::new();
        let mut potential_mappings = HashMap::new();
        for c in 'a'..='g' {
            potential_mappings.insert(c, ('a'..'g').collect::<HashSet<_>>());
        }
        for unique_value in unique_values {
            let l = unique_value.len();
            if l == 2 || l == 3 || l == 4 || l == 7 {
                let value: u8 = match l {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    _ => unreachable!(),
                };
                let digit_chars = *segment_values.get(&value).unwrap();
                let target_chars = unique_value.chars().collect::<HashSet<_>>();
                for c in digit_chars {
                    let possible = potential_mappings.get_mut(&c).unwrap();
                    let mut to_remove = Vec::new();
                    for possible_char in possible.iter() {
                        if !target_chars.contains(possible_char) {
                            to_remove.push(*possible_char);
                        }
                    }
                    to_remove.iter().for_each(|c| { possible.remove(c); });
                }
            }
        }

        // lets see if we can find pairs to cleanup
        let mut did_something = true;
        while did_something {
            did_something = false;
            for a in 'a'..='g' {
                for b in 'a'..='g' {
                    if a == b { continue; }
                    let potential_mappings_2 = potential_mappings.clone();
                    let pairs = potential_mappings_2.iter()
                        .filter(|(_, v)| v.len() == 2)
                        .filter(|(_, v)| v.contains(&a) && v.contains(&b))
                        .collect::<Vec<_>>();
                    if pairs.len() == 2 {
                        for (k, v) in &mut potential_mappings {
                            if pairs.iter().any(|(k2, _)| &k == k2) {
                                continue;
                            }
                            v.remove(&a);
                            v.remove(&b);
                            did_something = true;
                        }
                    }
                }
            }
        }

        println!("{:?}", potential_mappings);
    }
}

fn build_value_mappings() -> HashMap<u8, &'static [char]> {
    let mut mappings = HashMap::new();

    mappings.insert(0, &['a', 'b', 'c', 'e', 'f', 'g'][..]);
    mappings.insert(1, &['c', 'f'][..]);
    mappings.insert(2, &['a', 'c', 'd', 'e', 'g'][..]);
    mappings.insert(3, &['a', 'c', 'd', 'f', 'g'][..]);
    mappings.insert(4, &['b', 'c', 'd', 'f'][..]);
    mappings.insert(5, &['a', 'b', 'd', 'f', 'g', ][..]);
    mappings.insert(6, &['a', 'b', 'd', 'e', 'f', 'g'][..]);
    mappings.insert(7, &['a', 'c', 'f'][..]);
    mappings.insert(8, &['a', 'b', 'c', 'd', 'e', 'f', 'g'][..]);
    mappings.insert(9, &['a', 'b', 'c', 'd', 'f', 'g'][..]);

    mappings
}
