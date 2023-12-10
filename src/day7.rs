use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    character::complete::{alphanumeric1, digit1, multispace0, space0},
    combinator::map_res,
    multi::many1,
    sequence::{pair, preceded},
    IResult,
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn sort_key(joker: Option<char>) -> impl Fn(&str) -> (HandType, Vec<usize>) {
    let mut all_cards = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    if let Some(j) = joker {
        let i = all_cards.iter().position(|&c| c == j).unwrap();
        all_cards.remove(i);
        all_cards.push(j);
    }
    move |s: &str| {
        let mut hand: HashMap<char, usize> = HashMap::new();
        let mut cards: Vec<usize> = Vec::new();
        for card in s.chars() {
            cards.push(all_cards.iter().rev().position(|&c| c == card).unwrap());
            *hand.entry(card).or_default() += 1;
        }

        if hand.len() > 1 {
            if let Some(j) = joker {
                let jokers = hand.remove(&j).unwrap_or_default();
                *hand.values_mut().max().unwrap() += jokers;
            }
        }

        let mut counts: Vec<_> = hand.values().copied().collect();
        counts.sort();
        let hand_type = match &counts[..] {
            [.., 5] => HandType::FiveOfAKind,
            [.., 4] => HandType::FourOfAKind,
            [.., 2, 3] => HandType::FullHouse,
            [.., 3] => HandType::ThreeOfAKind,
            [.., 2, 2] => HandType::TwoPair,
            [.., 2] => HandType::Pair,
            _ => HandType::HighCard,
        };
        (hand_type, cards)
    }
}

fn run<F, K>(input: &str, key: F) -> usize
where
    F: Fn(&str) -> K,
    K: Ord,
{
    fn parse(input: &str) -> IResult<&str, Vec<(&str, usize)>> {
        many1(preceded(
            multispace0,
            pair(alphanumeric1, preceded(space0, map_res(digit1, str::parse))),
        ))(input)
    }
    let (_, hands) = parse(input).unwrap();
    hands
        .into_iter()
        .map(|(hand, bid)| (key(hand), bid))
        .sorted()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i + 1) * bid)
}

pub fn solve(input: &str) -> usize {
    run(input, sort_key(None))
}

pub fn solve_2(input: &str) -> usize {
    run(input, sort_key(Some('J')))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 6440);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 5905);
    }
}
