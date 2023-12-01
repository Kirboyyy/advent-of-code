use std::{fs, collections::HashMap, i32};

fn main() {
    let inputs = fs::read_to_string("input.txt").expect("Can't read file!");
    let lines: Vec<String> = inputs.lines().map(String::from).collect(); 
    let mut res = 0;
    for line in lines{
        let first_digit = get_digit(&line, false); //line.chars().find(|c| c.is_digit(10));
        let last_digit = get_digit(&line, true); //line.chars().rev().find(|c| c.is_digit(10));
        let value = first_digit.unwrap().to_string() + &last_digit.unwrap().to_string();
        res += value.parse::<i32>().unwrap(); 
    }
    println!("Result = {res}");    
}

fn get_digit(input_line: &str, reverse: bool) -> Option<i32> {
    let mut num_map: HashMap<&str, i32> = HashMap::new();
    num_map.insert("one", 1);
    num_map.insert("two", 2);
    num_map.insert("three", 3);
    num_map.insert("four", 4);
    num_map.insert("five", 5);
    num_map.insert("six", 6);
    num_map.insert("seven", 7);
    num_map.insert("eight", 8);
    num_map.insert("nine", 9);


    let line : String;
    if reverse {
        line = input_line.chars().rev().collect::<String>();
    }else {
        line = input_line.to_owned();
    }

    // start loop
    for i in 0..line.len() {
        // end loop
        let current_i = line.chars().nth(i).unwrap();
        if current_i.is_digit(10) {
            return current_i.to_digit(10).map(|u| u as i32);
        }
        for j in i+1..=line.len(){
            let current_j = line.chars().nth(j-1).unwrap();
            if current_j.is_digit(10){
                break;
            }
            let sub_str : &str;
            let reversed_sub_str: String;
            if reverse {
                reversed_sub_str = line[i..j].chars().rev().collect::<String>();
                sub_str = &reversed_sub_str;
            } else {
                sub_str = &line[i..j];
            }

            if let Some(&value) = num_map.get(sub_str){
                return Some(value);
            }
        }
    }
    return None::<i32>;
}
