use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("src/my_input.txt")?;

    // Part 1
    let result:u32 = io::BufReader::new(file).lines()
        .map(|line_result| {
            let line = line_result.unwrap();
            let first_digit = &line
                .chars()
                .find(|c| c.is_digit(10)).unwrap()
                .to_digit(10).unwrap();
            let last_digit = &line
                .chars()
                .rev()
                .find(|c| c.is_digit(10)).unwrap()
                .to_digit(10).unwrap();
            first_digit * 10 + last_digit
        })
        .sum();

    print!("{:?}", result);

    Ok(())
}
