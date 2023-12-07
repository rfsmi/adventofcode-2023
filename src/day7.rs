use nom::{
    character::complete::{alphanumeric1, digit1, multispace0, space0},
    combinator::{map, map_res},
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<usize>,
}

impl Hand {
    fn new(chars: &str) -> Hand {
        const CARDS: &[char] = &[
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];
        let cards: Vec<_> = chars
            .chars()
            .map(|c| CARDS.iter().rev().position(|&card| card == c).unwrap())
            .collect();
        let mut counts = vec![0; CARDS.len()];
        for &c in &cards {
            counts[c] += 1;
        }
        counts.sort();
        let hand_type = match &counts[..] {
            [.., _, 5] => HandType::FiveOfAKind,
            [.., 1, 4] => HandType::FourOfAKind,
            [.., 2, 3] => HandType::FullHouse,
            [.., _, 3] => HandType::ThreeOfAKind,
            [.., 2, 2] => HandType::TwoPair,
            [.., _, 2] => HandType::Pair,
            _ => HandType::HighCard,
        };
        Self { hand_type, cards }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(Hand, usize)>> {
    let hand = map(alphanumeric1, Hand::new);
    let bid = preceded(space0, map_res(digit1, str::parse));
    many1(preceded(multispace0, pair(hand, bid)))(input)
}

pub fn solve(input: &str) -> usize {
    let (_, mut hands) = parse(input).unwrap();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i + 1) * bid)
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
}
