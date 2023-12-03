use std::fs::File;
use std::io;
use std::thread::current;
use nom_locate::position;

#[derive(Debug)]
struct Position {
    line_number: usize,
    col_number: usize,
}

fn main() {
    let input = include_str!("my_input.txt").lines();

    let mut numbers: Vec<(u32, Position)> = vec![];
    let mut symbols: Vec<(char, Position)> = vec![];

    // Part 1
    for (line_number, line) in input.enumerate() {
        let mut current_number: Option<(u32, Position)> = None;

        for (col_number, char) in line.chars().enumerate() {
            match char {
                c if c.is_digit(10) => {
                    if current_number.is_none() {
                        current_number = Some((c.to_digit(10).unwrap(), Position { line_number, col_number }));
                    } else {
                        let (number, position) = current_number.unwrap();
                        current_number = Some((number * 10 + c.to_digit(10).unwrap(), position));
                    }
                },
                '.' => {
                    // Do nothing
                    if current_number.is_some() {
                        numbers.push(current_number.unwrap());
                        current_number = None;
                    }
                }
                c => {
                    symbols.push((c, Position { line_number, col_number }));
                    if current_number.is_some() {
                        numbers.push(current_number.unwrap());
                        current_number = None;
                    }
                }
            }
        }

        if current_number.is_some() {
            numbers.push(current_number.unwrap());
            current_number = None;
        }
    }

    let result: u32 = numbers.iter()
        .filter(|(number, n_pos)| {
            symbols.iter().any(|(symbol, s_pos)| {
                (s_pos.line_number - 1 <= n_pos.line_number && n_pos.line_number <= s_pos.line_number + 1)
                    && (s_pos.col_number - 1 <= n_pos.col_number + number.to_string().len() - 1 && n_pos.col_number <= s_pos.col_number + 1)
            })
        })
        .map(|(number, _)| number)
        .sum();

    dbg!(result);
}
