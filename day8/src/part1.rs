use std::collections::HashMap;

fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();

    let mut network = HashMap::new();
    for line in lines.skip(1) {
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        network.insert(key, (left, right));
    }
    (instructions, network)
}

fn main() {
    let input: &str = include_str!("my_input.txt");
    let (instructions, network) = parse_input(input);

    let mut current_node = "AAA";
    dbg!(&current_node);
    let mut steps = 0;

    while current_node != "ZZZ" {
        for c in instructions.chars() {
            match c {
                'L' => current_node = network[current_node].0,
                'R' => current_node = network[current_node].1,
                _ => panic!("Invalid instruction"),
            }
            steps += 1;

            if current_node == "ZZZ" {
                break;
            }
        }
    }

    dbg!(steps);
}