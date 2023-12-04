extern crate regex;

use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let card = fs::read_to_string("input.txt").expect("Can't read file!");

    let pattern = Regex::new(r":([\d\s]+)\|([\d\s]+)").unwrap();
    let mut score = 0;
    let mut total_cards = 0;

    let mut i = 0;
    let mut multiplicator_map: HashMap<i32, i32> = HashMap::new();

    for line in pattern.captures_iter(&card) {
        let mut card_score = 0;
        let left: Vec<&str> = line.get(1).unwrap().as_str().split_whitespace().collect();
        let right: Vec<&str> = line.get(2).unwrap().as_str().split_whitespace().collect();
        let mut j = 1;
        let curr_multiplicator = multiplicator_map.get(&i).unwrap_or(&1).to_owned();
        for num in left {
            if right.contains(&num) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score *= 2;
                }
                let val = *multiplicator_map.get(&(i + j)).unwrap_or(&1) + (1*curr_multiplicator);
                multiplicator_map.insert(i + j, val);
                j += 1;
            }
        }
        score += card_score;
        total_cards += 1 * multiplicator_map.get(&i).unwrap_or(&1);
        i += 1;
    }
    println!("score is {score}");
    println!("total cards is {total_cards}");
}
