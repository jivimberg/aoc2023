use std::collections::HashMap;
use num_integer::lcm;

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
    dbg!(&network);

    let result = network.iter()
        .filter(|(node, _)| node.ends_with('A'))
        .map(|(node, _)| {
            dbg!(node);
            let mut current_node = *node;
            let mut steps: i64 = 0;
            while !current_node.ends_with("Z") {
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
            steps
        })
        .fold(1, |acc, x| lcm(acc, x));

    dbg!(result);
}