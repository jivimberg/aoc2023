fn main() {
    let input: &str = include_str!("my_input.txt");
    let result = input.lines()
        .map(|line| {
            let mut numbers = line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let mut last_col = vec![];
            while numbers.iter().any(|x| *x != 0) {
                let mut new_numbers = Vec::new();
                for (i, number) in numbers.iter().enumerate() {
                    if i == 0 {
                        continue;
                    }
                    new_numbers.push(number - numbers.get(i - 1).unwrap())
                }
                last_col.push(*numbers.last().unwrap());
                numbers = new_numbers;
            }
            last_col.iter().fold(0i64, |acc, x| acc + x)
        })
        .sum::<i64>();
    dbg!(result);
}
