use std::fs;

fn main() {
    let inputs = fs::read_to_string("input.txt").expect("Can't read file!");
    let lines: Vec<String> = inputs.lines().map(String::from).collect(); 

    let mut res = 0;
    for line in lines{
        let first_digit = line.chars().find(|c| c.is_digit(10));
        let last_digit = line.chars().rev().find(|c| c.is_digit(10));

        let value = first_digit.unwrap().to_string() + &last_digit.unwrap().to_string();
        res += value.parse::<i32>().unwrap(); 
    }
    println!("Result = {res}");    
}
