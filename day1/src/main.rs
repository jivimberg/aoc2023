use std::fs::File;
use std::io;
use std::io::BufRead;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

fn main() -> Result<(), io::Error> {
    let file = File::open("src/my_input.txt")?;

    // Part 1
    let result: u32 = io::BufReader::new(file).lines()
        .map(|line_result| {
            let line = line_result.unwrap();
            let first_numeric_digit: Option<(usize, u32)> = line
                .find(|c: char| c.is_digit(10))
                .map(|position| {
                    let digit = line.chars()
                        .nth(position).unwrap()
                        .to_digit(10).unwrap();
                    (position, digit)
                });
            let first_letter_digit: Option<(usize, u32)> = Number::iter()
                .map(|number| line.find(&number.to_string()))
                .enumerate()
                .filter(|(_, position)| position.is_some())
                .min_by_key(|(_, position)| position.unwrap())
                .map(|(digit, position)| (position.unwrap(), digit as u32));

            let first_digit: u32 = match (first_numeric_digit, first_letter_digit) {
                (Some((numeric_position, numeric_digit)), Some((letter_position, letter_digit))) => {
                    if numeric_position < letter_position {
                        numeric_digit
                    } else {
                        letter_digit
                    }
                },
                (Some((_, numeric_digit)), None) => numeric_digit,
                (None, Some((_, letter_digit))) => letter_digit,
                (None, None) => panic!("No digit found")
            };

            let last_numeric_digit: Option<(usize, u32)> = line
                .rfind(|c: char| c.is_digit(10))
                .map(|position| {
                    let digit = line.chars()
                        .nth(position).unwrap()
                        .to_digit(10).unwrap();
                    (position, digit)
                });
            let last_letter_digit: Option<(usize, u32)> = Number::iter()
                .map(|number| line.rfind(&number.to_string()))
                .enumerate()
                .filter(|(_, position)| position.is_some())
                .max_by_key(|(_, position)| position.unwrap())
                .map(|(digit, position)| (position.unwrap(), digit as u32));

            let last_digit: u32 = match (last_numeric_digit, last_letter_digit) {
                (Some((numeric_position, numeric_digit)), Some((letter_position, letter_digit))) => {
                    if numeric_position > letter_position {
                        numeric_digit
                    } else {
                        letter_digit
                    }
                },
                (Some((_, numeric_digit)), None) => numeric_digit,
                (None, Some((_, letter_digit))) => letter_digit,
                (None, None) => panic!("No digit found")
            };

            first_digit * 10 + last_digit
        })
        .sum();

    print!("{:?}", result);

    Ok(())
}

#[derive(Debug, EnumIter, Display)]
#[strum(serialize_all = "lowercase")]
enum Number {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}
