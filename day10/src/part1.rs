fn main() {
    let input: &str = include_str!("my_input.txt");
    let map = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // find initial position
    let start = find_start(&map);
    dbg!(start);
    let possible_first_steps = vec![
        (start.0 - 1, start.1),
        (start.0 + 1, start.1),
        (start.0, start.1 - 1),
        (start.0, start.1 + 1),
    ];
    let first_moves = possible_first_steps.into_iter()
        .filter(|possible_first_pos| {
            if possible_first_pos.0 < 0 || possible_first_pos.1 < 0 {
                return false;
            }
            let next_pos_char = map[possible_first_pos.0 as usize][possible_first_pos.1 as usize];
            let next_pos_res = next_position(next_pos_char, *possible_first_pos);
            if next_pos_res.is_none() {
                return false;
            }
            next_pos_res.unwrap().0 == start || next_pos_res.unwrap().1 == start
        })
        .collect::<Vec<(i32, i32)>>();
    let mut current_position1 = first_moves[0];
    let mut current_position2 = first_moves[1];
    let mut previous_position1= start;
    let mut previous_position2= start;
    let mut step_count = 1;
    while current_position1 != current_position2 {
        // move 1 step
        let current_char1 = map[current_position1.0 as usize][current_position1.1 as usize];
        let next_pos1 = next_position(current_char1, current_position1).unwrap();
        let aux = if next_pos1.0 == previous_position1 {
            next_pos1.1
        } else {
            next_pos1.0
        };
        previous_position1 = current_position1;
        current_position1 = aux;

        let current_char2 = map[current_position2.0 as usize][current_position2.1 as usize];
        let next_pos2 = next_position(current_char2, current_position2).unwrap();
        let aux2 = if next_pos2.0 == previous_position2 {
            next_pos2.1
        } else {
            next_pos2.0
        };
        previous_position2 = current_position2;
        current_position2 = aux2;

        step_count += 1;
    }

    dbg!(step_count);
}

fn next_position(current_char: char, current_position: (i32, i32)) -> Option<((i32, i32), (i32, i32))> {
    let (row , col) = current_position;
    match current_char {
        '|' => Some(((row - 1, col), (row + 1, col))),
        '-' => Some(((row, col - 1), (row, col + 1))),
        'L' => Some(((row, col + 1), (row - 1, col))),
        'J' => Some(((row, col - 1), (row - 1, col))),
        '7' => Some(((row, col - 1), (row + 1, col))),
        'F' => Some(((row, col + 1), (row + 1, col))),
        _ => None,
    }
}

fn find_start(map: &Vec<Vec<char>>) -> (i32, i32) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 'S' {
                return (row as i32, col as i32);
            }
        }
    }
    panic!("No start found");
}
