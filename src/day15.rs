use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{pair, preceded, terminated},
    IResult,
};

enum Instr<'a> {
    Add(&'a str, usize),
    Remove(&'a str),
}

impl Display for Instr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Add(label, fl) => write!(f, "{label}={fl}"),
            Instr::Remove(label) => write!(f, "{label}-"),
        }
    }
}

fn parse(input: &str) -> Vec<Instr> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, str::parse)(input)
    }
    separated_list1(
        tag(","),
        preceded(
            multispace0,
            alt((
                map(terminated(alpha1, tag("-")), Instr::Remove),
                map(pair(alpha1, preceded(tag("="), number)), |(l, n)| {
                    Instr::Add(l, n)
                }),
            )),
        ),
    )(input)
    .unwrap()
    .1
}

fn hash(s: impl AsRef<str>) -> usize {
    s.as_ref()
        .chars()
        .map(|c| c as usize)
        .fold(0, |acc, c| ((acc + c) * 17) % 256)
}

pub fn solve(input: &str) -> usize {
    parse(input).iter().map(Instr::to_string).map(hash).sum()
}

pub fn solve_2(input: &str) -> usize {
    let mut boxes = vec![HashMap::<&str, (usize, usize)>::new(); 256];
    for (i, instr) in parse(input).into_iter().enumerate() {
        match instr {
            Instr::Add(label, fl) => {
                boxes[hash(label)]
                    .entry(label)
                    .and_modify(|(_, old_fl)| *old_fl = fl)
                    .or_insert((i, fl));
            }
            Instr::Remove(label) => {
                boxes[hash(label)].remove(label);
            }
        }
    }
    let mut result = 0;
    for (box_n, hm) in boxes.into_iter().enumerate() {
        for (i, (_, fl)) in hm.into_values().sorted().enumerate() {
            result += (box_n + 1) * (i + 1) * fl;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 1320);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 145);
    }
}
