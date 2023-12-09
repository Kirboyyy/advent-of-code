use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file!");
    solve(&input);
}

fn solve(input: &str) {
    let sum: i32 = input
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            calc(&nums)
        })
        .sum();

    println!("Result is {sum}")
}

fn calc(nums: &[i32]) -> i32 {
    let delta = nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(l, r)| r - l)
        .collect::<Vec<_>>();

    if delta.iter().all(|n| *n == 0) {
        *nums.last().unwrap()
    } else {
        nums.last().unwrap() + calc(&delta)
    }
}
