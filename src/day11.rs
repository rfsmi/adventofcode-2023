use std::collections::HashSet;

use itertools::Itertools;

fn parse(input: &str) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            match c {
                '#' => {
                    result.insert((x, y));
                }
                '.' => (),
                _ => panic!(),
            }
        }
    }
    result
}

fn expand_dim<'a>(
    factor: usize,
    galaxies: impl IntoIterator<Item = &'a (usize, usize)>,
    get_dim: impl Fn((usize, usize)) -> usize,
) -> Vec<usize> {
    let occupied: HashSet<_> = galaxies.into_iter().copied().map(get_dim).collect();
    let &max = occupied.iter().max().unwrap();
    let mut result = Vec::new();
    for p in 0..=max {
        let &last = result.last().unwrap_or(&0);
        result.push(last + if occupied.contains(&p) { 1 } else { factor });
    }
    result
}

fn run(input: &str, factor: usize) -> usize {
    let galaxies = parse(input);
    let real_cols = expand_dim(factor, &galaxies, |(x, _)| x);
    let real_rows = expand_dim(factor, &galaxies, |(_, y)| y);
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| {
            let a = (real_cols[a.0], real_rows[a.1]);
            let b = (real_cols[b.0], real_rows[b.1]);
            a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
        })
        .sum()
}

pub fn solve(input: &str) -> usize {
    run(input, 2)
}

pub fn solve_2(input: &str) -> usize {
    run(input, 1_000_000)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 374);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(run(SAMPLE, 10), 1030);
        assert_eq!(run(SAMPLE, 100), 8410);
    }
}
