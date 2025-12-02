use std::fs::File;
use std::io::Read;

const PATHWIN: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day2input.txt";
//const PATHWINTEST: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day2inputTest.txt";


fn main() {
    task1();
}


fn task1() {
    let mut input = String::new();
    let mut f = File::open(PATHWIN).expect("Couldnt read file");
    f.read_to_string(&mut input).expect("Couldnt read file content");

    let id_range = input.trim().split(","); //split into ranges

    let mut sum_invalid_id: i64 = 0;

    for range in id_range {
        let id: Vec<&str> = range.split("-").collect(); //split ranges into beginning and end
        
        for id_step in id[0].parse::<i64>().unwrap()..id[1].parse::<i64>().unwrap()+1 { 

            if is_kinda_mirrored(id_step) {
                sum_invalid_id += id_step;
                println!("range:{} step:{}", range, id_step);
            }
        }
    }

    println!("{}", sum_invalid_id);
}

// fn find_recurrence(id_in:i64) -> bool {
    // let id = id_in.to_string();
    // let len = id.len();
    // if len < 2 { return false; } //cant have recurrences with only 1 char
// 
    // for pos in 1..=len / 2 { //try all possible pattern lengths
        // if len % pos == 0 { //cant be a pattern if length not divisible through pattern
            // let pattern = &id[0..pos];
// 
            // if pattern.repeat(len / pos) == id { //check if repeating pattern is id
                // return true;
            // }
        // }
    // }
    // false
// }

fn is_kinda_mirrored(id_in: i64) -> bool {
    let id = id_in.to_string();
    let len = id.len();

    if len == 0 || len % 2 != 0 { //if 0 or scheps no can do homeboy
        return false;
    }

    let mid = len / 2;
    let pattern = &id[0..mid];
    let pattern_candidate = &id[mid..];

    pattern == pattern_candidate
}