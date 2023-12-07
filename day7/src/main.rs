use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Debug)]
struct Hand {
    cards: String,
    kind: Kind,
    bid: u64,
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_chars = self.cards.chars()
            .map(card_value)
            .collect::<Vec<u64>>();
        let other_chars = other.cards.chars()
            .map(card_value)
            .collect::<Vec<u64>>();
        (self.kind, self_chars).cmp(&(other.kind, other_chars))
    }
}

fn card_value(c: char) -> u64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u64,
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Kind {
    HIGH_CARD,
    ONE_PAIR,
    TWO_PAIR,
    THREE_OF_A_KIND,
    FULL_HOUSE,
    FOUR_OF_A_KIND,
    FIVE_OF_A_KIND
}

fn parse_hand(input: &str) -> Hand {
    let (cards, bid) = input.split_whitespace().next_tuple().unwrap();
    let char_count  = cards.chars()
        .into_grouping_map_by(|&x| x)
        .fold(0, |acc, _key, _value| acc + 1);
    let mut char_counts_desc = char_count.values().sorted().rev();
    let max1 = char_counts_desc.next().unwrap();
    let max2 = char_counts_desc.next().unwrap_or(&0);
    let kind = match (max1, max2) {
        (5, _) => Kind::FIVE_OF_A_KIND,
        (4, _) => Kind::FOUR_OF_A_KIND,
        (3, 2) => Kind::FULL_HOUSE,
        (3, _) => Kind::THREE_OF_A_KIND,
        (2, 2) => Kind::TWO_PAIR,
        (2, _) => Kind::ONE_PAIR,
        _ => Kind::HIGH_CARD,
    };

    Hand {
        cards: cards.to_string(),
        kind: kind,
        bid: bid.parse::<u64>().unwrap(),
    }
}

fn main() {
    let input: &str = include_str!("my_input.txt");
    let result = input.lines()
        .map(parse_hand)
        .sorted()
        .enumerate()
        .fold(0, |acc, (i, hand)| {
            acc + hand.bid * (i as u64 + 1)
        });

    dbg!(result);
}
