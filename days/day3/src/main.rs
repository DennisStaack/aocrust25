use std::fs;

const PATHWIN: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day3input.txt";
// const PATHWINTEST: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day3inputTest.txt";

fn main() {

    task1();
}

fn task1() {
    let input = fs::read_to_string(PATHWIN).expect("Couldnt read file");

    let mut result: i64 = 0;
    for line in input.lines() {
        let length: i64 = line.len().try_into().unwrap();
        let mut max: i64 = 0;
        let mut max_pos: usize = 0;
        for (pos, value_char) in line.chars().enumerate() {
            let pos = pos as i64;
            let value= value_char.to_digit(10).unwrap() as i64;

            let mut max_pos_sec: usize = 0;
            let mut max_sec: i64 = 0;

            if value > max && pos != length-1 {
                max = value;
                max_pos = pos as usize;
            }
            for (pos_sec, value_char_sec) in line.chars().skip(max_pos+1).enumerate() {
                let pos_sec = pos as i64;
                let value_sec= value_char_sec.to_digit(10).unwrap() as i64;

                if value_sec > max_sec {
                    max_sec = value_sec;
                    max_pos_sec = pos_sec as usize;
                }
                // println!("pos_sec:{} value_sec:{}", pos_sec, value_sec);
            }

            if pos == length-1 {
                result = result + (max * 10) + max_sec;
            }
            // println!("max:{} pos:{} max_sec:{} max_sec_pos:{}", max, pos, max_sec, max_pos_sec);
            // println!("{}{} result: {}", max, max_sec, result);
        }
    }
    println!("result: {}", result);
}