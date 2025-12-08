use std::fs;

//const PATHWIN: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day4input.txt";
//const PATHWINTEST: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day4inputTest.txt";
const PATHLINTEST: &str = "../inputFiles/inputTest.txt";

fn main() {
    let input = fs::read_to_string(PATHLINTEST).expect("Couldnt read file");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();
    task1(&grid, width, height);
}

fn task1(grid: &Vec<Vec<char>>, width: usize, height: usize) {
    let mut result: i32 = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != '@' {
                continue;
            }
            let mut amount: i32 = 0;

            for y_offset in -1..=1 {
                for x_offset in -1..=1 {
                    if x_offset == 0 && y_offset == 0 {
                        continue;
                    }

                    let ny = (y as isize) + y_offset;
                    let nx = (x as isize) + x_offset;

                    if ny >= 0 && ny < (height as isize) && nx >= 0 && nx < (width as isize) {
                        if grid[ny as usize][nx as usize] == '@' {
                            amount += 1;
                        }
                    }
                }
            }
            if amount < 4 {
                result += 1;
            }
        }
    }
    println!("result task1: {}", result);
}
