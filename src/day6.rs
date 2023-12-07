use std::iter::zip;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space0},
    combinator::map_res,
    multi::many1,
    sequence::{pair, preceded, tuple},
    IResult,
};

fn parse(input: &str) -> (impl Iterator<Item = usize>, impl Iterator<Item = usize>) {
    fn number(input: &str) -> IResult<&str, usize> {
        preceded(space0, map_res(digit1, str::parse))(input)
    }
    let (_, (times, distances)) = pair(
        preceded(tuple((multispace0, tag("Time:"), space0)), many1(number)),
        preceded(
            tuple((multispace0, tag("Distance:"), space0)),
            many1(number),
        ),
    )(input)
    .unwrap();
    (times.into_iter(), distances.into_iter())
}

fn n_wins(time: usize, distance: usize) -> usize {
    (1..time).filter(|t| (time - t) * t > distance).count()
}

pub fn solve(input: &str) -> usize {
    let (times, distances) = parse(input);
    zip(times, distances).map(|(t, d)| n_wins(t, d)).product()
}

pub fn solve_2(input: &str) -> usize {
    let (mut times, mut distances) = parse(input);
    n_wins(
        times.join("").parse().unwrap(),
        distances.join("").parse().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    Time:      7  15   30
    Distance:  9  40  200";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 288);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 71503);
    }
}
