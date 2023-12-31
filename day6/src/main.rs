use std::iter::zip;
use std::ops::Range;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::{map, map_res, opt};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{pair, tuple};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64
}

fn main() {
    let input: &str = include_str!("my_input.txt");
    let races = vec![parse_input(input).unwrap().1];
    dbg!(&races);

    let mut result = 1;
    for Race { time, distance } in races {
        let mut min_hold: Option<u64> = None;
        for start_time in 1..time {
            if start_time * (time - start_time) > distance {
                min_hold = Some(start_time);
                break;
            }
        }

        let mut max_hold: Option<u64> = None;
        for start_time in (1..time).rev() {
            if start_time * (time - start_time) > distance {
                max_hold = Some(start_time);
                break;
            }
        }

        dbg!(min_hold);
        dbg!(max_hold);

        let solutions = max_hold.unwrap() - min_hold.unwrap() + 1;
        dbg!(solutions);
        result *= solutions;
    }

    dbg!(result);
}

fn parse_input(input: &str) -> IResult<&str, Race> {
    let f = pair(
        parse_time,
        parse_distance,
    );
    map(f, |(time, distance)| {
        Race { time, distance }
    })(input)
}

fn parse_time(s: &str) -> IResult<&str, u64> {
    let f = tuple((
        tag("Time:"),
        space1,
        separated_list1(space1, digit1),
        newline
    ));
    map(f, |(_, _, digits, _)| digits.join("").parse::<u64>().unwrap())(s)
}

fn parse_distance(s: &str) -> IResult<&str, u64> {
    let f = tuple((
        tag("Distance:"),
        space1,
        separated_list1(space1, digit1),
        opt(newline)
    ));
    map(f, |(_, _, digits, _)| digits.join("").parse::<u64>().unwrap())(s)
}