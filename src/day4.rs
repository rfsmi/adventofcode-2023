use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space0, space1},
    combinator::{map, map_res},
    multi::{fold_many0, many1},
    sequence::{pair, preceded, tuple},
    IResult,
};

fn n_winners(input: &str) -> impl Iterator<Item = usize> {
    fn numbers(input: &str) -> IResult<&str, HashSet<usize>> {
        fold_many0(
            preceded(space0, map_res(digit1, str::parse)),
            HashSet::new,
            |mut acc, n| {
                assert!(acc.insert(n));
                acc
            },
        )(input)
    }
    many1(preceded(
        multispace0,
        preceded(
            tuple((tag("Card"), space1, digit1, tag(":"))),
            map(
                pair(numbers, preceded(pair(space0, tag("|")), numbers)),
                |(a, b)| a.intersection(&b).count(),
            ),
        ),
    ))(input)
    .unwrap()
    .1
    .into_iter()
}

pub fn solve(input: &str) -> usize {
    n_winners(input)
        .map(|n| match n {
            0 => 0,
            i => 2usize.pow(i as u32 - 1),
        })
        .sum()
}

pub fn solve_2(input: &str) -> usize {
    let mut queue = BinaryHeap::new();
    let mut running_copies = 0;
    let mut total_cards = 0;
    for (i, n) in n_winners(input).enumerate() {
        let count = 1 + running_copies;
        total_cards += count;
        running_copies += count;
        queue.push(Reverse((i + n, count)));
        while let Some(&Reverse((i2, count))) = queue.peek() {
            if i < i2 {
                break;
            }
            running_copies -= count;
            queue.pop();
        }
    }
    total_cards
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 13);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 30);
    }
}
