use std::fs;
use std::process;

fn main() {
    
    let file_str = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file_str.split('\n').map(|s| s.trim()).collect();

    for line in &lines {
        for line2 in &lines {
            let mut diff = 0;
            let mut common: Vec<char> = Vec::new();

            let mut line_c_iter = line.chars();
            let mut line2_c_iter = line2.chars();

            for _ in 0..line.len() {
                let c = line_c_iter.next().unwrap();
                let c2 = line2_c_iter.next().unwrap();

                if c != c2 {
                    diff += 1;
                } else {
                    common.push(c);
                }
            }

            if diff != 1 {
                continue;
            }

            let s: String = common.into_iter().collect();
            println!("{}", s);
            process::exit(0);
        }
    }
}