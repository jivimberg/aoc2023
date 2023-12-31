use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, space1};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

struct Card {
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    let f = tuple((
            tag("Card"),
            space1,
            digit1,
            tag(":"),
            space1,
            separated_list1(space1, map_res(digit1, str::parse)),
            tag(" |"),
            space1,
            separated_list1(space1, map_res(digit1, str::parse)),
        ));
    map(f, |(_, _, _, _, _, winning_numbers, _, _, card_numbers)| {
        Card { winning_numbers, card_numbers }
    })(s)
}

fn main() {
    let cards: Vec<Card> = include_str!("my_input.txt").lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect();

    let mut card_copies = vec![1;cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let hits = card.card_numbers.iter()
            .filter(|card_number| card.winning_numbers.contains(card_number))
            .count();

        for j in (i + 1)..=(i + hits) {
            card_copies[j] += card_copies[i];
        }
    }

    let result: u32 = card_copies.iter().sum();

    dbg!(result);
}
