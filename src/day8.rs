use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::map,
    multi::many1,
    sequence::{pair, preceded, terminated},
    IResult,
};

fn parse<'a>(input: &str) -> IResult<&str, (&str, HashMap<&str, (&str, &str)>)> {
    pair(
        preceded(multispace0, alpha1),
        map(
            many1(preceded(
                multispace0,
                pair(
                    terminated(alphanumeric1, tag(" = (")),
                    pair(
                        terminated(alphanumeric1, tag(", ")),
                        terminated(alphanumeric1, tag(")")),
                    ),
                ),
            )),
            |v| v.into_iter().collect(),
        ),
    )(input)
}

fn run<'a, 'b>(
    instructions: &'a str,
    edges: &'b HashMap<&'a str, (&'a str, &'a str)>,
    terminate: impl Fn(&'a str) -> bool,
    mut pos: &'a str,
) -> usize {
    for (step, instruction) in instructions.chars().cycle().enumerate() {
        if terminate(pos) {
            return step;
        }
        let &(l, r) = edges.get(pos).unwrap();
        match instruction {
            'L' => pos = l,
            'R' => pos = r,
            _ => panic!(),
        }
    }
    unreachable!()
}

pub fn solve(input: &str) -> usize {
    let (_, (instructions, edges)) = parse(input).unwrap();
    run(instructions, &edges, |p| p == "ZZZ", "AAA")
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn solve_2(input: &str) -> usize {
    let (_, (instructions, edges)) = parse(input).unwrap();
    edges
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| run(instructions, &edges, |p| p.ends_with('Z'), k))
        .reduce(|a, b| a * b / gcd(a, b))
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(
            solve(
                "
                RL
    
                AAA = (BBB, CCC)
                BBB = (DDD, EEE)
                CCC = (ZZZ, GGG)
                DDD = (DDD, DDD)
                EEE = (EEE, EEE)
                GGG = (GGG, GGG)
                ZZZ = (ZZZ, ZZZ)"
            ),
            2
        );
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            solve(
                "
                LLR

                AAA = (BBB, BBB)
                BBB = (AAA, ZZZ)
                ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[test]
    fn test_sample_part_2() {
        assert_eq!(
            solve_2(
                "
                LR

                11A = (11B, XXX)
                11B = (XXX, 11Z)
                11Z = (11B, XXX)
                22A = (22B, XXX)
                22B = (22C, 22C)
                22C = (22Z, 22Z)
                22Z = (22B, 22B)
                XXX = (XXX, XXX)"
            ),
            6
        );
    }
}
