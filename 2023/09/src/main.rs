use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file!");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let sum: i32 = input
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            calc(&nums, true)
        })
        .sum();

    println!("Part1: {sum}")
}

fn part2(input: &str) {
    let sum: i32 = input
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            calc(&nums, false)
        })
        .sum();

    println!("Part2: {sum}")
}

fn calc(nums: &[i32], part1_logic: bool) -> i32 {
    let delta = nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(l, r)| r - l)
        .collect::<Vec<_>>();
    if part1_logic {
        if delta.iter().all(|n| *n == 0) {
            return *nums.last().unwrap();
        } else {
            return nums.last().unwrap() + calc(&delta, part1_logic);
        }
    } else {
        if delta.iter().all(|n| *n == 0) {
            return *nums.first().unwrap();
        } else {
            return nums.first().unwrap() - calc(&delta, part1_logic);
        }
    }
}
