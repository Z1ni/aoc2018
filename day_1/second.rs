use std::fs;
use std::process;
use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut seen = HashMap::new();
    let mut freq = 0i32;

    seen.insert(0, 1);

    loop {
        for num in &nums {
            freq += num;

            if seen.contains_key(&freq) {
                println!("{}", freq);
                process::exit(1);
            }
            seen.insert(freq, 1);
        }
    }
}