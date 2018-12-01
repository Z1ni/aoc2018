use std::fs;

fn main() {
    
    let nums: Vec<i32> = fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let freq: i32 = nums.iter().sum();

    println!("{}", freq);
}