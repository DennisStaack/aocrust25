use std::fs;

const PATHWIN: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day4input.txt";
const PATHWINTEST: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day4inputTest.txt";


fn main() {
    let input = fs::read_to_string(PATHWINTEST).expect("Couldnt read file");
    
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    task1(&grid);
}

fn task1(grid: &Vec<Vec<char>>) {


}