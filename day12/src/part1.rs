fn main() {
    let input: &str = include_str!("my_input.txt");
    let result: u32 = input.lines()
        .map(|line| {
            let mut parts: Vec<&str> = line.split_whitespace().collect();
            let line = parts[0];
            let groups: Vec<u32> = parts[1]
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            count_arrangements(line, groups)
        })
        .sum();
    dbg!(result);
}

fn count_arrangements(line: &str, groups: Vec<u32>) -> u32 {
    let mut stack = vec![line.to_string()];
    let mut alternatives = vec![];
    while !stack.is_empty() {
        let current_candidate = stack.pop().unwrap();
        let char_pos = current_candidate.find('?');
        if char_pos == None {
            if final_check(&current_candidate, &groups) {
                alternatives.push(current_candidate);
            }
        } else {
            let char_pos = char_pos.unwrap();
            let mut with_dot = current_candidate.to_string();
            with_dot.replace_range(char_pos..char_pos+1, ".");
            if is_valid(&with_dot, &groups) {
                stack.push(with_dot);
            }

            let mut with_hashtag = current_candidate.to_string();
            with_hashtag.replace_range(char_pos..char_pos+1, "#");
            if is_valid(&with_hashtag, &groups) {
                stack.push(with_hashtag);
            }
        }
    }

    alternatives.len() as u32
}

fn is_valid(line: &str, groups: &Vec<u32>) -> bool {
    let mut group_idx = 0;
    let mut current_group_count = 0;
    let mut inside_group = false;
    for c in line.chars() {
        match c {
            '?' => break,
            '#' => {
                current_group_count += 1;
                inside_group = true;
                if group_idx >= groups.len() || current_group_count > groups[group_idx] {
                    return false;
                }
            },
            '.' => {
                if inside_group {
                    inside_group = false;
                    group_idx += 1;
                    current_group_count = 0;
                }
            },
            _ => panic!("Invalid character in line")
        }
    }

    true
}

fn final_check(line: &str, groups: &Vec<u32>) -> bool {
    let mut group_count = vec![];
    let mut current_group_count = 0;
    let mut inside_group = false;
    for c in line.chars() {
        match c {
            '#' => {
                current_group_count += 1;
                inside_group = true;
            },
            '.' => {
                if inside_group {
                    inside_group = false;
                    group_count.push(current_group_count);
                    current_group_count = 0;
                }
            },
            _ => panic!("Invalid character in line")
        }
    }

    if current_group_count != 0 {
        group_count.push(current_group_count);
    }

    group_count == *groups
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::count_arrangements;

    #[rstest]
    #[case("???.###", vec![1,1,3], 1)]
    #[case(".??..??...?##.", vec![1,1,3], 4)]
    #[case("?#?#?#?#?#?#?#?", vec![1,3,1,6], 1)]
    #[case("????.#...#...", vec![4,1,1], 1)]
    #[case("????.######..#####.", vec![1,6,5], 4)]
    #[case("?###????????",  vec![3,2,1], 10)]
    fn it_works(#[case] line: &str, #[case] groups: Vec<u32>, #[case] expected: u32) {
        assert_eq!(count_arrangements(line, groups), expected);
    }
}