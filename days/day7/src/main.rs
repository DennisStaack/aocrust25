use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

const PATH: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day7input.txt";
const outPATH: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day7inputOut.txt";
// const PATH: &str = "D:/code/repos/aoc/aocrust25/inputFiles/day5inputTest.txt";
// const PATH: &str = "../inputFiles/inputTest.txt";
// const outPATH: &str = "../inputFiles/inputTestOut.txt";

fn main() -> io::Result<()> {
    let input = File::open(PATH)?;
    let output = File::create(outPATH)?;

    let buffer = BufReader::new(input);
    let writer = BufWriter::new(output);

    task1(buffer, writer)?;
    // task2();

    Ok(())
}

fn task1(buffer: impl BufRead, mut writer: impl Write) -> io::Result<()> {
    let mut prev_positions: Vec<usize> = Vec::new();
    let mut count = 0;

    for line_result in buffer.lines() {
        let line_str = line_result?;

        let mut chars: Vec<char> = line_str.chars().collect();
        let len = chars.len();

        for &pos in &prev_positions {
            if pos < len {
                if chars[pos] != '^' {
                    chars[pos] = '|';
                }
            }
        }

        let mut new_indices = Vec::new();

        for (i, &c) in chars.iter().enumerate() {
            if c == '^' {
                if prev_positions.contains(&i) {
                    new_indices.push(i);
                    count += 1;
                }
            }
        }

        for i in new_indices {
            if i > 0 {
                chars[i - 1] = '|';
            }
            if i + 1 < len {
                chars[i + 1] = '|';
            }
        }
        let mut next_pos: Vec<usize> = Vec::new();

        for (i, &c) in chars.iter().enumerate() {
            if c == '|' || c == 'S' {
                next_pos.push(i);
            }
        }

        prev_positions = next_pos;

        let output_line: String = chars.into_iter().collect();
        writeln!(writer, "{}", output_line)?;
    }

    println!("{}", count);
    Ok(())
}
