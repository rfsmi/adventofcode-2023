use std::iter::zip;

use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut patterns = Vec::new();
    let mut pattern = Vec::new();
    for line in input.trim().lines() {
        let row: Vec<_> = line.trim().chars().collect();
        if row.is_empty() {
            if !pattern.is_empty() {
                patterns.push(pattern);
                pattern = Vec::new();
            }
        } else {
            pattern.push(row);
        }
    }
    if !pattern.is_empty() {
        patterns.push(pattern);
    }
    patterns
}

fn find_symmetry(pattern: &Vec<Vec<char>>) -> Vec<usize> {
    let mut symmetries = Vec::new();
    'next_symmetry: for y in 1..pattern.len() {
        for (a, b) in zip(pattern[..y].iter().rev(), &pattern[y..]) {
            if a != b {
                continue 'next_symmetry;
            }
        }
        symmetries.push(100 * y);
    }
    'next_symmetry: for x in 1..pattern[0].len() {
        for row in pattern {
            for (a, b) in zip(row[..x].iter().rev(), &row[x..]) {
                if a != b {
                    continue 'next_symmetry;
                }
            }
        }
        symmetries.push(x);
    }
    symmetries
}

pub fn solve(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|pattern| find_symmetry(pattern)[0])
        .sum()
}

pub fn solve_2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|mut pattern| {
            let old_symmetry = find_symmetry(&pattern)[0];
            for (y, x) in (0..pattern.len()).cartesian_product(0..pattern[0].len()) {
                let c = pattern[y][x];
                match c {
                    '.' => pattern[y][x] = '#',
                    '#' => pattern[y][x] = '.',
                    _ => panic!(),
                }
                if let Some(symmetry) = find_symmetry(&pattern)
                    .into_iter()
                    .filter(|&s| s != old_symmetry)
                    .next()
                {
                    return symmetry;
                }
                pattern[y][x] = c;
            }
            panic!()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 405);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 400);
    }
}
