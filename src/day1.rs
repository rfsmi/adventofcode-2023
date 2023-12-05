use std::iter::from_fn;

pub fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(str::trim)
        .map(|l| {
            let mut digits = l.chars().filter(char::is_ascii_digit);
            let first = digits.clone().next().unwrap();
            let last = digits.next_back().unwrap();
            format!("{first}{last}")
        })
        .map(|n| n.parse::<u32>().unwrap())
        .sum()
}

pub fn solve_2(input: &str) -> u32 {
    fn digits(mut s: &str) -> impl Iterator<Item = u32> + '_ {
        from_fn(move || {
            while !s.is_empty() {
                for (i, name) in [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .into_iter()
                .enumerate()
                {
                    if s.starts_with(name) {
                        let (_, rest) = s.split_at(1);
                        s = rest;
                        return Some(i as u32 + 1);
                    }
                }
                let (c, rest) = s.split_at(1);
                s = rest;
                if let Ok(digit) = c.parse() {
                    return Some(digit);
                }
            }
            None
        })
    }
    input
        .trim()
        .lines()
        .map(str::trim)
        .map(|s| {
            let first = digits(s).next().unwrap();
            let last = digits(s).last().unwrap();
            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(
            solve(
                "1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet"
            ),
            142
        )
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            solve_2(
                "two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen"
            ),
            281
        )
    }

    #[test]
    fn test_simple_sample_2() {
        assert_eq!(solve_2("onetwo"), 12)
    }
}
