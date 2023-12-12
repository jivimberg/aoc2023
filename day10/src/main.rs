fn main() {
    let input: &str = include_str!("my_input.txt");
    let mut map = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // find initial position
    let start: (i32, i32) = find_start(&map);
    let possible_first_steps = vec![
        (start.0 - 1, start.1),
        (start.0 + 1, start.1),
        (start.0, start.1 - 1),
        (start.0, start.1 + 1),
    ];
    let first_moves: Vec<(i32, i32)> = possible_first_steps.into_iter()
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

    //let (fm1, fm2) = tuple2(&first_moves);

    // TODO taking a shortcut here
    map[start.0 as usize][start.1 as usize] = '|';

    let mut current_position1 = first_moves[0];
    let mut previous_position1= start;
    let mut just_the_cycle: Vec<Vec<char>> = vec![vec!['_'; map[0].len()]; map.len()];
    just_the_cycle[current_position1.0 as usize][current_position1.1 as usize] = map[current_position1.0 as usize][current_position1.1 as usize];
    while current_position1 != start {
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
        just_the_cycle[current_position1.0 as usize][current_position1.1 as usize] = map[current_position1.0 as usize][current_position1.1 as usize];
    }

    let mut inside = false;
    for row in 0..map.len() {
        inside = false;
        for col in 0..map[row].len() {
            let current_char = map[row][col];
            let is_vertical = current_char == '|';
            if just_the_cycle[row][col] != '_' && is_vertical {
                inside = !inside;
            } else if just_the_cycle[row][col] == '_' && inside {
                just_the_cycle[row][col] = '*';
            }
        }
    }

    // print just the cycle
    // for row in 0..map.len() {
    //     for col in 0..map[row].len() {
    //         print!("{}", just_the_cycle[row][col]);
    //     }
    //     println!();
    // }
    // println!("-------------------");

    // clean up from top
    for col in 0..map[0].len() {
        for row in 0..map.len() {
            if just_the_cycle[row][col] == '*' {
                just_the_cycle[row][col] = '_';
            } else if just_the_cycle[row][col] != '_' {
                break;
            }
        }
    }

    // clean up from bottom
    for col in (0..map[0].len()).rev() {
        for row in (0..map.len()).rev() {
            if just_the_cycle[row][col] == '*' {
                just_the_cycle[row][col] = '_';
            } else if just_the_cycle[row][col] != '_' {
                break;
            }
        }
    }

    // clean up from right
    for row in 0..map.len() {
        for col in (0..map[row].len()).rev() {
            if just_the_cycle[row][col] == '*' {
                just_the_cycle[row][col] = '_';
            } else if just_the_cycle[row][col] != '_' {
                break;
            }
        }
    }

    // print just the cycle
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            print!("{}", just_the_cycle[row][col]);
        }
        println!();
    }

    let mut  inside_count = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if just_the_cycle[row][col] == '*' {
                inside_count += 1;
            }
        }
    }
    dbg!(inside_count);
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

fn find_start_char(start_pos: (i32, i32), first_moves: &Vec<(i32, i32)>) -> char {
    let fm1 = first_moves[0];
    let fm2 = first_moves[1];
    let start_row = start_pos.0;
    let start_col = start_pos.1;
    let left = (start_row, start_col - 1);
    let right = (start_row, start_col + 1);
    if let (left, right) = (fm1, fm2) {
        '-'
    } else {
        panic!("No start char found")
    }
}

fn tuple2<T>(a: &[T]) -> (&T, &T) { (&a[0], &a[1]) }