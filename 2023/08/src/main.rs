use regex::Regex;
use std::{collections::HashMap, fs, ops::Add};
use num_integer::lcm;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file!");
    let mut input_iter = input.lines().into_iter();
    let movements: Vec<char> = input_iter.next().unwrap().chars().collect();

    // Skip empty line
    input_iter.next();

    let regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in input_iter {
        let captures: Vec<&str> = regex
            .captures(line)
            .unwrap()
            .iter()
            .map(|c| c.unwrap().as_str())
            .collect();
        map.insert(captures[1], (captures[2], captures[3]));
    }

    part1(&map, &movements);
    part2(&map, &movements);
}

fn part1(map: &HashMap<&str, (&str, &str)>, movements: &Vec<char>) {
    let mut current_node = "AAA";
    let mut steps = 0;
    let mut movement_index = 0;
    while current_node != "ZZZ" {
        steps += 1;
        if movements.get(movement_index).unwrap() == &'L' {
            current_node = map.get(current_node).unwrap().0;
        } else {
            current_node = map.get(current_node).unwrap().1
        }
        if movement_index == movements.len() - 1 {
            movement_index = 0;
        } else {
            movement_index += 1;
        }
    }
    println!("Part 1: {steps}");
}

fn part2(map: &HashMap<&str, (&str, &str)>, movements: &Vec<char>) {
    let current_nodes: Vec<&&str> = map.keys().filter(|k| k.ends_with("A")).collect();
    let mut steps_vec: Vec<u64> = Vec::new();

    for node in current_nodes {
        let mut movement_index = 0;
        let mut steps = 0;

        let mut current_node = node;
        while !current_node.ends_with("Z") {
            steps += 1;
            if movements.get(movement_index).unwrap() == &'L' {
                current_node = &map.get(current_node).unwrap().0;
            } else {
                current_node = &map.get(current_node).unwrap().1;
            }
            if movement_index == movements.len() - 1 {
                movement_index = 0;
            } else {
                movement_index += 1;
            }
        }
        steps_vec.push(steps);
    }


    let value = steps_vec.into_iter().fold(1, |acc, x| lcm(acc, x));
    println!("Part 2: {value}")
}
