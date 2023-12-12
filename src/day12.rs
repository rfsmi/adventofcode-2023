use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, one_of, space1},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{pair, preceded},
    IResult,
};

struct Problem {
    memo: HashMap<(usize, usize, usize), usize>,
    springs: Vec<char>,
    groups: Vec<usize>,
}

impl Problem {
    fn new(input: &str) -> IResult<&str, Self> {
        map(
            pair(
                many1(one_of("#.?")),
                preceded(
                    space1,
                    separated_list1(tag(","), map_res(digit1, str::parse)),
                ),
            ),
            |(springs, groups)| Self {
                memo: HashMap::new(),
                springs,
                groups,
            },
        )(input)
    }

    fn unfold(mut self) -> Self {
        let (springs, groups) = (self.springs.clone(), self.groups.clone());
        for _ in 1..5 {
            self.springs.push('?');
            self.springs.extend(springs.iter().copied());
            self.groups.extend(groups.iter().copied());
        }
        self
    }

    fn solve(&mut self, group_i: usize, current_len: usize, spring_i: usize) -> usize {
        // Some yucky dynamic programming. Couldn't think of a better way to
        // handle the base case.
        if spring_i == self.springs.len()
            && (group_i == self.groups.len() && current_len == 0
                || group_i + 1 == self.groups.len() && current_len == self.groups[group_i])
        {
            return 1;
        }
        if spring_i == self.springs.len()
            || group_i >= self.groups.len() && current_len != 0
            || group_i < self.groups.len() && current_len > self.groups[group_i]
        {
            return 0;
        }
        if let Some(&answer) = self.memo.get(&(group_i, current_len, spring_i)) {
            return answer;
        }
        let broken = self.solve(group_i, current_len + 1, spring_i + 1);
        let ok = if current_len == 0 {
            self.solve(group_i, 0, spring_i + 1)
        } else if group_i < self.groups.len() && current_len == self.groups[group_i] {
            self.solve(group_i + 1, 0, spring_i + 1)
        } else {
            0
        };
        let answer = match self.springs[spring_i] {
            '#' => broken,
            '.' => ok,
            '?' => broken + ok,
            _ => panic!(),
        };
        self.memo.insert((group_i, current_len, spring_i), answer);
        answer
    }
}

fn parse(input: &str) -> Vec<Problem> {
    many1(preceded(multispace0, Problem::new))(input).unwrap().1
}

pub fn solve(input: &str) -> usize {
    parse(input).into_iter().map(|mut p| p.solve(0, 0, 0)).sum()
}

pub fn solve_2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(Problem::unfold)
        .map(|mut p| p.solve(0, 0, 0))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 21);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 525152);
    }
}
