use std::fs;

const PATH: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day5input.txt";
// const PATH: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day5inputTest.txt";
// const PATH: &str = "../inputFiles/inputTest.txt";


fn main() {
    let input = fs::read_to_string(PATH).expect("Couldnt read file");
    let input: Vec<&str> = input.split("\n").collect();


    // task1();
    // task2();
}