use std::mem::swap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, space0},
    combinator::{map, map_res},
    multi::many1,
    sequence::{pair, preceded, tuple},
    IResult,
};

fn evaluator<'a>(
    layers: impl Iterator<Item = &'a Vec<(usize, usize, usize)>> + Clone,
    forwards: bool,
) -> impl Fn(usize) -> usize {
    move |mut value| {
        for layer in layers.clone() {
            for &(mut dest, mut src, len) in layer {
                if !forwards {
                    swap(&mut dest, &mut src);
                }
                if (src..src + len).contains(&value) {
                    value = dest + value - src;
                    break;
                }
            }
        }
        value
    }
}

fn mapping(layers: Vec<Vec<(usize, usize, usize)>>) -> impl Fn(usize, usize) -> usize {
    let candidates: Vec<_> = layers
        .iter()
        .enumerate()
        .flat_map(|(i, layer)| layer.iter().map(move |l| (i, l)))
        .flat_map(|(i, &(_, src, len))| {
            let invert = evaluator(layers[..i].iter().rev(), false);
            [invert(src), invert(src + len)]
        })
        .collect();
    move |start, len| {
        candidates
            .iter()
            .filter(|i| (start..start + len).contains(i))
            .copied()
            .chain([start])
            .map(evaluator(layers.iter(), true))
            .min()
            .unwrap()
    }
}

fn parse(input: &str) -> (Vec<usize>, impl Fn(usize, usize) -> usize) {
    fn number(input: &str) -> IResult<&str, usize> {
        preceded(space0, map_res(digit1, str::parse))(input)
    }
    pair(
        preceded(tuple((multispace0, tag("seeds:"))), many1(number)),
        map(
            many1(preceded(
                tuple((multispace0, alpha1, tag("-to-"), alpha1, tag(" map:"))),
                many1(preceded(multispace0, tuple((number, number, number)))),
            )),
            mapping,
        ),
    )(input)
    .unwrap()
    .1
}

pub fn solve(input: &str) -> usize {
    let (numbers, f) = parse(input);
    numbers.into_iter().map(|n| f(n, 1)).min().unwrap()
}

pub fn solve_2(input: &str) -> usize {
    let (numbers, f) = parse(input);
    numbers
        .into_iter()
        .tuples()
        .map(|(start, len)| f(start, len))
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 35);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 46);
    }
}
