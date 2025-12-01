use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("D:/code/repos/aoc/aocrust25/inputFiles/day1input.txt").expect("Should read file");
    let input = BufReader::new(input);

    let mut count: i32 = 0;
    let mut pos: i32 = 50;
    
    for line in input.lines() {
        let line = line.expect("Should be able to read line");

        let rot_abs = line[1..].parse::<i32>().unwrap();

        if line.starts_with('R') {
            pos = pos + rot_abs;

            if pos > 100 {
                while pos > 100 {
                    pos = pos - 100;
                }
            } else if pos < 0 {
                while pos < 0 {
                    pos = 100 + pos;
                }
            }
        } else if line.starts_with('L') {
            pos = pos - rot_abs;

            if pos > 100 {
                while pos > 100 {
                    pos = pos - 100;
                }
            } else if pos < 0 {
                while pos < 0 {
                    pos = 100 + pos;
                }
            }           
        }
        
        if pos == 0 || pos == 100 {
            count = count + 1;
        }
    }

    println!("Pw: {}", count);
}
