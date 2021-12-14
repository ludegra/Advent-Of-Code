use std::collections::HashMap;

use itertools::Itertools;

pub fn part2(base_polymer: &Vec<char>, insertion_pairs: &HashMap<(char, char), char>) {

    let mut pair_map = HashMap::new();

    for (first, second) in base_polymer.iter().tuple_windows() {
        let entry = pair_map.entry((*first, *second)).or_insert(0u64);
        *entry += 1;
    }

    for _ in  0..40 { 
        let mut new_pair_map = HashMap::new();

        for (pair, number) in &pair_map {
            match insertion_pairs.get(&(pair.0, pair.1)) {
                Some(component) => {
                    let entry = new_pair_map.entry((pair.0, *component)).or_insert(0);
                    *entry += number;

                    let entry = new_pair_map.entry((*component, pair.1)).or_insert(0);
                    *entry += number;
                }
                None => {
                    let entry = new_pair_map.entry(*pair).or_insert(0);
                    *entry += number;
                }
            }
        }
        pair_map = new_pair_map;
    }
    print!("{:?}", pair_map);
}