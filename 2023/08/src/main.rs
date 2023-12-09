use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file!");

    let mut input_iter = input.lines().into_iter();
    let movements: Vec<char> = input_iter.next().unwrap().chars().collect();

    let mut steps = 0;

    // Skip empty line
    input_iter.next();

    let regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut current_node = "AAA";

    for line in input_iter {
        let captures: Vec<&str> = regex
            .captures(line)
            .unwrap()
            .iter()
            .map(|c| c.unwrap().as_str())
            .collect();
       map.insert(captures[1], (captures[2], captures[3]));
    }

    let mut movement_index = 0;
    while current_node != "ZZZ" {
        steps +=1;
        if movements.get(movement_index).unwrap() == &'L' {
            current_node = map.get(current_node).unwrap().0; 
        } else {
            current_node = map.get(current_node).unwrap().1
        }
        if movement_index == movements.len()-1 {
            movement_index = 0;
        } else {
            movement_index += 1;
        }
    }

    println!("used steps: {steps}");
}
