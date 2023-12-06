use std::iter::zip;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded},
    IResult,
};

fn parse(input: &str) -> Vec<(usize, Vec<[usize; 3]>)> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, str::parse)(input)
    }
    fn cube_set(mut input: &str) -> IResult<&str, [usize; 3]> {
        let mut result = [0; 3];
        loop {
            let (remainder, (count, kind)) = pair(number, preceded(space1, alpha1))(input)?;
            let i = match kind {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => panic!(),
            };
            result[i] = count;
            input = remainder;
            let Some(remainder) = input.strip_prefix(", ") else {
                break;
            };
            input = remainder;
        }
        Ok((input, result))
    }
    many1(preceded(
        multispace0,
        pair(
            delimited(tag("Game "), number, tag(": ")),
            separated_list1(tag("; "), cube_set),
        ),
    ))(input)
    .unwrap()
    .1
}

pub fn solve(input: &str) -> usize {
    let target = [12, 13, 14];
    parse(input)
        .into_iter()
        .filter_map(|(id, counts)| {
            if counts
                .into_iter()
                .all(|count| zip(count, target).all(|(a, b)| a <= b))
            {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

pub fn solve_2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|(_, counts)| {
            let min_required = counts
                .into_iter()
                .reduce(|a, b| [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])])
                .unwrap();
            min_required.into_iter().product::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_sample_1() {
        assert_eq!(solve(SAMPLE), 8);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 2286);
    }
}
