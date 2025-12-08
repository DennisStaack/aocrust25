use std::fs;

const PATH: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day4input.txt";
// const PATH: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day4inputTest.txt";
// const PATH: &str = "../inputFiles/inputTest.txt";

fn main() {
    let input = fs::read_to_string(PATH).expect("Couldnt read file");

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();
    task1(&mut grid, width, height);
    task2(&mut grid, width, height);
}

fn task1(grid: &mut Vec<Vec<char>>, width: usize, height: usize) {
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

fn task2(grid: &mut Vec<Vec<char>>, width: usize, height: usize) {
    let mut result: i32 = 0;
    loop {
        let mut rem_loop = false;
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
                    grid[y][x] = '.';
                    result += 1;
                    rem_loop = true;
                }
            }
        }
        if !rem_loop {
            break;
        } 
    }
    println!("result task2: {}", result);
}