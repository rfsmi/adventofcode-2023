use std::collections::{HashMap, HashSet};

struct Schematic {
    numbers: Vec<i64>,
    number_indices: HashMap<(i64, i64), usize>,
    symbols: HashMap<(i64, i64), char>,
}

impl Schematic {
    fn new(input: &str) -> Schematic {
        let mut numbers = Vec::new();
        let mut number_indices = HashMap::new();
        let mut symbols = HashMap::new();
        for (y, line) in input.trim().lines().enumerate() {
            let mut partial_number = None;
            for (x, c) in line.trim().chars().chain(['.']).enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    let number = partial_number.unwrap_or(0);
                    partial_number = Some(number * 10 + digit as i64);
                    number_indices.insert((x as i64, y as i64), numbers.len());
                    continue;
                }
                if let Some(number) = partial_number.take() {
                    numbers.push(number);
                }
                if c != '.' {
                    symbols.insert((x as i64, y as i64), c);
                }
            }
        }
        Schematic {
            numbers,
            number_indices,
            symbols,
        }
    }

    fn part_numbers(&self) -> impl Iterator<Item = i64> + '_ {
        let mut seen = HashSet::new();
        self.number_indices.iter().filter_map(move |(&(x, y), &i)| {
            if seen.contains(&i) {
                return None;
            }
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if self.symbols.contains_key(&(x + dx, y + dy)) {
                        seen.insert(i);
                        return Some(self.numbers[i]);
                    }
                }
            }
            None
        })
    }

    fn gear_ratios(&self) -> impl Iterator<Item = i64> + '_ {
        self.symbols
            .iter()
            .filter(|(_, &c)| c == '*')
            .filter_map(|(&(x, y), _)| {
                let mut adjacent = HashSet::new();
                for dy in [-1, 0, 1] {
                    for dx in [-1, 0, 1] {
                        adjacent.extend(self.number_indices.get(&(x + dx, y + dy)));
                    }
                }
                if adjacent.len() == 2 {
                    Some(adjacent.into_iter().map(|&i| self.numbers[i]).product())
                } else {
                    None
                }
            })
    }
}

pub fn solve(input: &str) -> i64 {
    Schematic::new(input).part_numbers().sum()
}

pub fn solve_2(input: &str) -> i64 {
    Schematic::new(input).gear_ratios().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 4361);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 467835);
    }
}
