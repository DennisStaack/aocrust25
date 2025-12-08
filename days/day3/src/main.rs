use std::fs;

const PATHWIN: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day3input.txt";
const PATHWINTEST: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day3inputTest.txt";

fn main() {
    task1();
    task2();
}

fn task1() {
    let input = fs::read_to_string(pathwin).expect("couldnt read file");

    let mut result: i64 = 0;
    for line in input.lines() {
        let length: i64 = line.len().try_into().unwrap();
        let mut max: i64 = 0;
        let mut max_pos: usize = 0;
        for (pos, value_char) in line.chars().enumerate() { //get number and pos of number in string
            let pos = pos as i64;
            let value= value_char.to_digit(10).unwrap() as i64; //char to int

            let mut max_sec: i64 = 0;

            if value > max && pos != length-1 { //first digit is highest number in string unless highest is last
                max = value;
                max_pos = pos as usize; //keep track of pos 
            }
            for  value_char_sec in line.chars().skip(max_pos+1) { //start to search for second highest at pos of first digit+1
                let value_sec= value_char_sec.to_digit(10).unwrap() as i64; //once again get int
                //we dont need the pos for the second digit
                if value_sec > max_sec {
                    max_sec = value_sec;
                }
            }
            if pos == length-1 {
                result = result + (max * 10) + max_sec; //first digit*10 to get the decimal  
            }
        }
    }
    println!("result: {}", result);
}


fn task2() {
    let input = fs::read_to_string(PATHWIN).expect("Couldnt read file");

    let mut result: u64 = 0;
    let jolt_length = 12; //lenght of the jolt batteries

    for line in input.lines() {
        let length = line.len();
        let mut del_spots = length - jolt_length;
        let mut num_stack: Vec<char> = Vec::with_capacity(jolt_length); //stack of char vecs to store the 12 digits we find

        for num in line.chars() { 
            while del_spots > 0 && !num_stack.is_empty() && *num_stack.last().unwrap() < num { //as long as we have digit to remove from
                num_stack.pop();                                                               //and are not at the end of the stack 
                del_spots -= 1;
            }
            num_stack.push(num);                                                               //then add the current digit to the stack
        }

        num_stack.truncate(jolt_length); //cut the stack at the length of the batts

        let result_string: String = num_stack.into_iter().collect(); //convert the stack back into string
        result += result_string.parse::<u64>().expect(""); //convert string to int and add the result up
    }
    println!("result2: {}", result);
}