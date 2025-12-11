use std::fs;

const PATH: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day5input.txt";
// const PATH: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day5inputTest.txt";
// const PATH: &str = "../inputFiles/inputTest.txt";

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

// impl Range {
//     fn contains(&self, value: u64) -> bool {
//         value >= self.start && value <= self.end
//     }
// }


fn main() {
    let input = fs::read_to_string(PATH).expect("Couldnt read file");
    let input = input.replace("\r", "");
    let blocks: Vec<&str> = input.split("\n\n").collect(); //split into range and number block

    let range_block = blocks[0];
    let num_block = blocks[1];

    let mut ranges: Vec<Range> = Vec::new();
    
    for line in range_block.lines() {
        if let Some((start_str, end_str)) = line.split_once("-") { //split ranges into numbers in a vec
            let start: u64 = start_str.parse().expect("no num");
            let end: u64 = end_str.parse().expect("no num");

            ranges.push(Range {start, end});
        }
    }

    let mut nums: Vec<u64> = Vec::new();

    for line in num_block.lines() { 
        let num: u64 = line.parse().expect("no num");
        nums.push(num);
    }

    // task1(ranges, nums);
    task2(ranges);
}

fn task1 (ranges: Vec<Range>, nums: Vec<u64>) {
    let mut result: u64 = 0;

    for num in &nums {
        let in_range = ranges.iter().any(|r| num >= &r.start && num <= &r.end); //check if number is in any of the ranges

        if in_range {
            result += 1;
        }
       
    }
    println!("result task1: {}", result);
}

fn task2 (mut ranges: Vec<Range>) {
    let mut result: u64 = 0;

    ranges.sort_by_key(|r| r.start); //first sort the range vecs

    let mut curr_start = ranges[0].start; //start with the first ones
    let mut curr_end = ranges[0].end;

    for range in ranges.iter().skip(1) { //skip since we did the first one already
        if range.start > curr_end {              //no overlap case
            result += curr_end - curr_start + 1; //range inclusive -> +1

            curr_start = range.start;
            curr_end = range.end;
        } else {                                 //overlap -> extend current block
            if range.end > curr_end {
                curr_end = range.end;
            }
        }
    }

    result += curr_end - curr_start + 1;         //dont forget the last block

    println!("result task2: {}", result);
}