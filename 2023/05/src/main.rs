use regex::Regex;
use std::{fs, time::Instant};

fn main() {
    let start_time = Instant::now();
    let input = fs::read_to_string("input.txt").expect("Can't read file!");
    let lines: Vec<String> = input.lines().map(String::from).collect();

    let number_re = Regex::new(r"(\d+)").unwrap();

    let mut seeds = number_re
        .captures_iter(&lines[0])
        .map(|cap| cap[1].parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    for seed in seeds.iter_mut() {
        let mut found = false;
        let mut i = 1; // Skip seeds line
        while i < lines.len() {
            if !lines[i].is_empty() && !found {
                let range = number_re
                    .captures_iter(&lines[i])
                    .map(|c| c[1].parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                if *seed >= range[1] && *seed < range[1] + range[2] {
                    *seed = range[0] + (*seed - range[1]);
                    found = true;
                }
            } else if lines[i].is_empty(){
                // Found next map
                i += 1;
                found = false;
            }

            i += 1;
        }
    }
    let end_time = Instant::now();
    println!("Result is: {:?} in {:?}", seeds.iter().min().unwrap(), end_time - start_time);
}
