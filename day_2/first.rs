use std::fs;
use std::collections::HashMap;

fn main() {
    
    let file_str = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file_str.split('\n').map(|s| s.trim()).collect();

    let mut two_count = 0;
    let mut three_count = 0;

    for line in &lines {
        let mut letters = HashMap::new();
        let mut two_seen = false;
        let mut three_seen = false;

        for c in line.chars() {
            let count = letters.entry(c).or_insert(0);
            *count += 1;
        }

        for (_, count) in letters.iter() {
            if !two_seen && *count == 2 {
                two_count += 1;
                two_seen = true;
            } else if !three_seen && *count == 3 {
                three_count += 1;
                three_seen = true;
            }
        }
    }

    println!("Checksum: {}", two_count * three_count);
}