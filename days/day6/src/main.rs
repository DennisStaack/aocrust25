use std::fs;

const PATH: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day6input.txt";
// const PATH: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day6inputTest.txt";
// const PATH: &str = "../inputFiles/inputTest.txt";

fn main() {
    let input = fs::read_to_string(PATH).expect("Couldnt read file");
    let input = input.replace("\r", "");
    let blocks: Vec<&str> = input.split("\n").collect(); 

    let first_num: Vec<u64> = blocks[0].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let second_num: Vec<u64> = blocks[1].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let third_num: Vec<u64> = blocks[2].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let fourth_num: Vec<u64> = blocks[3].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let operator: Vec<&str> = blocks[4].split_whitespace().collect();

    // task1(first_num, second_num, third_num, fourth_num, operator);
    task2(first_num, second_num, third_num, fourth_num, operator);
}


fn task1(first_num: Vec<u64>, second_num: Vec<u64>, third_num: Vec<u64>, fourth_num: Vec<u64>, operator: Vec<&str>) {
    let mut result: u64 = 0;

    for (i, _nums) in first_num.iter().enumerate() {
        if operator[i] == '*'.to_string() {
            result += first_num[i]*second_num[i]*third_num[i]*fourth_num[i];
        } else {
            result += first_num[i]+second_num[i]+third_num[i]+fourth_num[i];
        }
    }
    println!("result task1: {}", result);
}


fn task2(first_num: Vec<u64>, second_num: Vec<u64>, third_num: Vec<u64>, fourth_num: Vec<u64>, operator: Vec<&str>) {
    let mut result: u128 = 0;
    let mut digits1: Vec<u64> = Vec::new();
    let mut digits2: Vec<u64> = Vec::new();
    let mut digits3: Vec<u64> = Vec::new();
    let mut digits4: Vec<u64> = Vec::new();

    for (i, _nums) in first_num.iter().enumerate() {
        let mut temp1 = first_num[i];
        let mut temp2 = second_num[i];
        let mut temp3 = third_num[i];
        let mut temp4 = fourth_num[i];

        loop {
            digits1.push(temp1 % 10);
            temp1 /= 10;

            if temp1 == 0 { break; }
        } 

        loop {
            digits2.push(temp2 % 10);
            temp2 /= 10;

            if temp2 == 0 { break; }
        }       

        loop {
            digits3.push(temp3 % 10);
            temp3 /= 10;

            if temp3 == 0 { break; }
        }

        loop {
            digits4.push(temp4 % 10);
            temp4 /= 10;

            if temp4 == 0 { break; }
        }

        let max_len = digits1.len().max(digits2.len()).max(digits3.len()).max(digits4.len());

        for j in 0..max_len {
            if operator[i] == '*'.to_string() {
                let d1 = digits1.get(j).unwrap_or(&1);
                let d2 = digits2.get(j).unwrap_or(&1);
                let d3 = digits3.get(j).unwrap_or(&1);
                let d4 = digits4.get(j).unwrap_or(&1);
                println!("{} {} {} {}", d1, d2, d3, d4);
                result *= (d1*d2*d3*d4) as u128;
            } else {
                let d1 = digits1.get(j).unwrap_or(&0);
                let d2 = digits2.get(j).unwrap_or(&0);
                let d3 = digits3.get(j).unwrap_or(&0);
                let d4 = digits4.get(j).unwrap_or(&0);
                result += (d1+d2+d3+d4) as u128;
            }
        }
    }
    println!("result task2: {}", result);
}


