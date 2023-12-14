fn main() {
    let input: &str = include_str!("my_input.txt");
    let result: u64 = input.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let line_parts = vec![parts[0];5];
            let line = &line_parts.join("?");
            let group_parts: Vec<u64> = parts[1]
                .split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            let groups: Vec<u64> = vec![group_parts; 5].into_iter()
                .flatten()
                .collect();
            let mut cache = vec![vec![None; line.len()]; groups.len()];
            count_arrangements(line, &groups, &mut cache)
        })
        .sum();
    dbg!(result);

    // Formula = n! /
}

fn count_arrangements(line: &str, groups: &Vec<u64>, cache: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if groups.is_empty() {
        return if line.contains('#') {
            0
        } else {
            1
        }
    }

    if line.is_empty() {
        return 0;
    }

    if let Some(cached) = cache[groups.len() - 1][line.len() - 1] {
        return cached;
    }

    let arrangements =  match line.chars().next() {
        Some('.') => {
            count_arrangements(&line[1..], groups, cache)
        },
        Some('#') => {
            let current_group = groups[0];
            if line.len() < (groups.iter().sum::<u64>() + groups.len() as u64 - 1) as usize {
                // can't match
                0
            } else if current_group > line.len() as u64 || line[..current_group as usize].contains('.') {
                // can't match
                0
            } else {
                return if current_group as usize > line.len() {
                    0
                } else if current_group as usize == line.len() {
                    // Last group
                    count_arrangements(&line[current_group as usize..], &groups[1..].to_vec(), cache)
                } else if line.chars().nth(current_group  as usize).unwrap() != '#' { // check the group doesn't continue
                    // continue with next group
                    count_arrangements(&line[current_group as usize + 1..], &groups[1..].to_vec(), cache)
                } else {
                    0
                };
            }
        },
        Some('?') => {
            let mut with_hashtag = "#".to_owned();
            with_hashtag.push_str(&line[1..]);
            count_arrangements(&line[1..], groups, cache)
                + count_arrangements(&with_hashtag, groups, cache)
        }
        _ => panic!("Invalid character in line"),
    };

    cache[groups.len() - 1][line.len() - 1] = Some(arrangements);

    arrangements
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
    fn it_works(#[case] line: &str, #[case] groups: Vec<u64>, #[case] expected: u64) {
        let mut cache = vec![vec![None; line.len()]; groups.len()];
        assert_eq!(count_arrangements(line, &groups, &mut cache), expected);
    }
}