use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space0},
    combinator::{map_res, opt, recognize},
    multi::many1,
    sequence::{pair, preceded},
    IResult,
};

fn parse(input: &str) -> Vec<Vec<i64>> {
    fn number(input: &str) -> IResult<&str, i64> {
        map_res(recognize(pair(opt(tag("-")), digit1)), str::parse)(input)
    }
    many1(preceded(multispace0, many1(preceded(space0, number))))(input)
        .unwrap()
        .1
}

fn run(nums: Vec<i64>) -> i64 {
    if nums.iter().all(|&n| n == 0) {
        return 0;
    }
    nums[nums.len() - 1]
        + run(nums
            .into_iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect())
}

pub fn solve(input: &str) -> i64 {
    parse(input).into_iter().map(run).sum()
}

pub fn solve_2(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|mut seq| {
            seq.reverse();
            seq
        })
        .map(run)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    #[test]
    fn test_sample_negative() {
        // 0   1  -1   C
        //   1  -2   B
        //    -3   A
        //       0
        assert_eq!(solve("0 1 -1"), -6)
    }

    #[test]
    fn test_sample_small() {
        assert_eq!(solve("0 3 6 9 12 15"), 18)
    }

    #[test]
    fn test_sample_small_2() {
        assert_eq!(solve_2("10 13 16 21 30 45"), 5)
    }

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 114)
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 2)
    }
}
