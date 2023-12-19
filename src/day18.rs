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

pub fn run(instrs: impl Iterator<Item = (char, i64)>) -> i64 {
    let instrs: Vec<_> = instrs.collect();
    let mut total_area = 0;
    let (mut x, mut y) = (0, 0);
    for &(dir, len) in &instrs {
        let (dx, dy) = match dir {
            'R' => (1, 0),
            'D' => (0, 1),
            'L' => (-1, 0),
            'U' => (0, -1),
            _ => panic!(),
        };
        let (x2, y2) = (x + dx * len, y + dy * len);
        total_area += len + (y + y2) * (x - x2);
        (x, y) = (x2, y2);
    }
    total_area / 2 + 1
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
