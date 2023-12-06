use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file!");
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let races: Vec<u32> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut results: Vec<u32> = Vec::new();

    for i in 0..races.len() {
        let race = races.get(i).unwrap();
        let record = distances.get(i).unwrap();

        for j in 0..*race {
            let remaining = race - j;
            let distance = j * remaining;
            if &distance > record {
                let val = results.get(i as usize).unwrap_or(&0) + 1;
                if results.len() < i+1 {
                    results.insert(i, val);
                } else {
                    results[i] = val;
                }
            }
        }
    }

    let mut res = 1;
    results.iter().for_each(|r| res *= r);
    println!("value is {res}");
}
