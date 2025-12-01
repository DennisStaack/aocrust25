use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    task1();
    task2();
}

fn task1() {
    //let path = "D:/code/repos/aoc/aocrust25/inputFiles/day1input.txt";
    let path = "/home/djs/docs/projects/adventOfCode/25/aocrust25/inputFiles/day1input.txt";
    let input = File::open(path).expect("File not found");
    let input = BufReader::new(input);

    let mut count: i32 = 0;
    let mut pos: i32 = 50;

    for line in input.lines() {
        let line = line.expect("Couldnt read line");

        let rot_abs = line[1..].parse::<i32>().unwrap();

        if line.starts_with('R') {
            pos += rot_abs;

            while pos >= 100 {
                pos -= 100;
            }
        } else if line.starts_with('L') {
            pos -= rot_abs;

            while pos < 0 {
                pos += 100;
            }
        }

        if pos == 0 || pos == 100 {
            count += 1;
        }
    }

    println!("Task1 Pw: {}", count);
}
//7972

fn task2() {
    //let path = "D:/code/repos/aoc/aocrust25/inputFiles/day1input.txt";
    let path = "/home/djs/docs/projects/adventOfCode/25/aocrust25/inputFiles/day1input.txt";
    let input = File::open(path).expect("File not found");
    let input = BufReader::new(input);

    let mut count: i32 = 0;
    let mut pos: i32 = 50;

    println!("Task2 Pw: {}", count);
}
