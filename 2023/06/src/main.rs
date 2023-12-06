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

    part_1(&races, &distances);
    part_2(&races, &distances);
}

fn part_1(races: &Vec<u32>, distances: &Vec<u32>) {
    let mut results: Vec<u32> = Vec::new();

    for i in 0..races.len() {
        let race = races.get(i).unwrap();
        let record = distances.get(i).unwrap();

        for j in 0..*race {
            let remaining = race - j;
            let distance = j * remaining;
            if &distance > record {
                let val = results.get(i as usize).unwrap_or(&0) + 1;
                if results.len() < i + 1 {
                    results.insert(i, val);
                } else {
                    results[i] = val;
                }
            }
        }
    }

    let mut res = 1;
    results.iter().for_each(|r| res *= r);
    println!("part1 is {res}");
}

fn part_2(races: &Vec<u32>, distances: &Vec<u32>) {
    let race = get_combined_num(races);
    let record = get_combined_num(distances);

    let mut result = 0;
    for j in 0..race {
        let remaining = race - j;
        let distance = j * remaining;
        if distance > record {
            result += 1;
        }
    }
    println!("part2 is {result}");
}

fn get_combined_num(nums: &Vec<u32>)-> u64{
    let mut combined = String::new();
    for num in nums {
        combined.push_str(&num.to_string());
    }
    return combined.parse::<u64>().unwrap();

}
