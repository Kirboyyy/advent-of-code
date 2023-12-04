extern crate regex;

use std::fs;

use regex::Regex;

fn main() {
    
    let card = fs::read_to_string("input.txt").expect("Can't read file!");

    let pattern = Regex::new(r":([\d\s]+)\|([\d\s]+)").unwrap();
    let mut score = 0;
    for line in pattern.captures_iter(&card) {
        let mut card_score = 0;

        println!("left: {}", line.get(1).unwrap().as_str());
        println!("right: {}", line.get(2).unwrap().as_str());

        let left: Vec<&str> = line.get(1).unwrap().as_str().split_whitespace().collect();
        let right: Vec<&str> = line.get(2).unwrap().as_str().split_whitespace().collect();
        for num in left  {
            if right.contains(&num){
                if card_score == 0 {
                  card_score = 1;
                } else {
                    card_score *= 2;
                }
            }
        }
        println!("Card Score = {card_score}");
        score += card_score;
    }
    println!("score is {score}");
}
