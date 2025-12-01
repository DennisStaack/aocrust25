use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("D:/code/repos/aoc/aocrust25/inputFiles/day1input.txt").expect("File not found");
    let input = BufReader::new(input);

    let mut count: i32 = 0;
    let mut pos: i32 = 50;
    
    for line in input.lines() {
        let line = line.expect("Couldnt read line");

        let rot_abs = line[1..].parse::<i32>().unwrap();

        if line.starts_with('R') {
            pos += rot_abs;

            while pos >= 100 {
                pos = pos - 100;
                count += 1;
            }
            
        } else if line.starts_with('L') {
            pos = pos - rot_abs;

            while pos < 0 {
                pos = 100 + pos;
                count += 1;
            }
        }
        
        if pos == 0 || pos == 100 {
            count += 1;
        }
    }

    println!("Pw: {}", count);
}
