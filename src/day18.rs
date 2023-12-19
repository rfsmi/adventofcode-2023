use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, multispace0, one_of, space0},
    combinator::map_res,
    multi::many1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn parse(input: &str) -> Vec<(char, i64, &str)> {
    fn number(input: &str) -> IResult<&str, i64> {
        map_res(digit1, str::parse)(input)
    }
    many1(tuple((
        preceded(multispace0, one_of("RDLU")),
        preceded(space0, number),
        preceded(space0, delimited(tag("(#"), alphanumeric1, tag(")"))),
    )))(input)
    .unwrap()
    .1
}

#[derive(Default)]
struct RangeSet {
    segments: Vec<(i64, i64)>,
}

impl RangeSet {
    fn insert(&mut self, mut start: i64, mut end: i64) {
        let i = self.segments.partition_point(|&(_, e)| e < start);
        while i != self.segments.len() && self.segments[i].0 <= end {
            start = self.segments[i].0.min(start);
            end = self.segments[i].1.max(end);
            self.segments.remove(i);
        }
        self.segments.insert(i, (start, end));
    }

    fn count(&self, start: i64, end: i64) -> i64 {
        let mut total = 0;
        let first_i = self.segments.partition_point(|&(_, e)| e < start);
        for i in first_i..self.segments.len() {
            if end <= self.segments[i].0 {
                break;
            }
            total += self.segments[i].1.min(end) - self.segments[i].0.max(start);
        }
        total
    }

    fn contains(&self, p: i64) -> bool {
        let i = self.segments.partition_point(|&(_, e)| e <= p);
        if i == self.segments.len() {
            return false;
        }
        self.segments[i].0 <= p
    }
}

pub fn run(instrs: impl Iterator<Item = (char, i64)>) -> i64 {
    let instrs: Vec<_> = instrs.collect();
    let (mut x, mut y) = (0, 0);
    let (mut xs, mut ys) = (BTreeSet::new(), BTreeSet::new());
    let mut verticals = BTreeMap::<i64, RangeSet>::new();
    let mut horizontals = BTreeMap::<i64, RangeSet>::new();
    let mut total_area = 0;
    for &(dir, len) in &instrs {
        let (dx, dy) = match dir {
            'R' => (1, 0),
            'D' => (0, 1),
            'L' => (-1, 0),
            'U' => (0, -1),
            _ => panic!(),
        };
        xs.insert(x);
        ys.insert(y);
        let (x2, y2) = (x + dx * len, y + dy * len);
        if dx == 0 {
            let (min, max) = (y.min(y2), y.max(y2));
            verticals.entry(x).or_default().insert(min, max);
        } else {
            let (min, max) = (x.min(x2), x.max(x2));
            horizontals.entry(y).or_default().insert(min, max + 1);
        }
        (x, y) = (x2, y2);
        total_area += len;
    }
    for (y1, y2) in ys.into_iter().tuple_windows() {
        for (&x1, &x2) in xs
            .iter()
            .filter(|x| verticals.get(x).is_some_and(|v| v.contains(y1)))
            .tuples()
        {
            total_area += (x2 - x1 - 1) * (y2 - y1);
            total_area -= horizontals.get(&y1).map_or(0, |h| h.count(x1 + 1, x2));
        }
    }
    total_area
}

pub fn solve(input: &str) -> i64 {
    run(parse(input).into_iter().map(|(dir, len, _)| (dir, len)))
}

pub fn solve_2(input: &str) -> i64 {
    run(parse(input).into_iter().map(|(.., s)| {
        let code = i64::from_str_radix(s, 16).unwrap();
        (['R', 'D', 'L', 'U'][(code % 16) as usize], code / 16)
    }))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 62);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 952408144115);
    }
}
