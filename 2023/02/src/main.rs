extern crate regex;

use regex::Regex;
use std::fs;

const MAX_RED: i8 = 12;
const MAX_GREEN: i8 = 13;
const MAX_BLUE: i8= 14;

fn main(){
    let digit_regex = Regex::new(r"(\d+)").unwrap();
    let mut valid_games = 0; 
    let inputs = fs::read_to_string("input.txt").expect("Can't read file!");
    let lines: Vec<String> = inputs.lines().map(String::from).collect(); 
    let mut i = 1;
    for line in lines  {
        if !is_invalid(&line, &digit_regex){
            valid_games+=i;
        }
        i+=1;
    }
    println!("Valid: {valid_games}");
}

fn is_invalid(game: &str, digit_regex: &Regex) -> bool{
    let content: Vec<&str> = game.split(":").collect();
    let rounds = content[1].split(";");
    for round in rounds {
        let cubes = round.split(",");
        for cube in cubes {
            let num = digit_regex.captures(cube).unwrap().get(0).unwrap().as_str().parse::<i8>().unwrap();
            if cube.contains("red") && num > MAX_RED ||  cube.contains("green") && num > MAX_GREEN || cube.contains("blue") && num > MAX_BLUE{
                return true
            }
        }
    }
    return false;
}
