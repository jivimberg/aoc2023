use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, digit0, digit1};
use nom::combinator::{all_consuming, map, map_res};
use nom::{Finish, IResult};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}
#[derive(Debug, PartialEq)]
struct Set {
    rgb: [u32; 3],
}

fn parse_game(s: &str) -> IResult<&str, Game> {
    let f = tuple((
            tag("Game "),
            map_res(digit1, str::parse),
            tag(": "),
            separated_list1(tag("; "), parse_set),
        ));
    map(f, |(_, id, _, sets)| Game { id, sets })(s)
}

fn parse_set(s: &str) -> IResult<&str, Set> {
    let f = separated_list1(tag(", "), parse_balls);
    map(f, |balls| {
        let mut rgb = [0; 3];
        for (count, color) in balls {
            match color {
                "red" => rgb[0] = count,
                "green" => rgb[1] = count,
                "blue" => rgb[2] = count,
                _ => unreachable!(),
            }
        }
        Set { rgb }
    })(s)
}

fn parse_balls(s: &str) -> IResult<&str, (u32, &str)>{
    separated_pair(
        map_res(digit1, str::parse),
        char(' '),
        alt((tag("red"), tag("green"), tag("blue")))
    )(s)
}

fn main() {
    let mut games = vec![];
    for line in include_str!("my_input.txt").lines() {
        if let Ok((_rest, crate_line)) = all_consuming(parse_game)(line).finish() {
            games.push(crate_line);
        }
    }

    let result = games.iter()
        .filter(|game| {
            let max_r = game.sets.iter().map(|set| set.rgb[0]).max().unwrap();
            let max_g = game.sets.iter().map(|set| set.rgb[1]).max().unwrap();
            let max_b = game.sets.iter().map(|set| set.rgb[2]).max().unwrap();
            max_r <= 12 && max_g <= 13 && max_b <= 14
        })
        .map(|game| game.id)
        .sum::<u32>();

    dbg!(result);
}
