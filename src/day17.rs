use std::collections::{BTreeSet, HashSet};

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn run(grid: Vec<Vec<usize>>, min_straight: usize, max_straight: usize) -> usize {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut heap: BTreeSet<_> = [(0, (0, 0), (1, 0)), (0, (0, 0), (0, 1))].into();
    let mut seen = HashSet::new();
    while let Some((cost, (x, y), (dx, dy))) = heap.pop_first() {
        if (x, y) == (width - 1, height - 1) {
            return cost;
        }
        if !seen.insert(((x, y), (dx, dy))) {
            continue;
        }
        for (dx, dy) in [(dy, -dx), (-dy, dx)] {
            let (mut x, mut y) = (x, y);
            let mut cost = cost;
            for i in 1..=max_straight {
                x += dx;
                y += dy;
                if !(0..height).contains(&y) || !(0..width).contains(&x) {
                    break;
                }
                cost += grid[y as usize][x as usize];
                if i >= min_straight {
                    heap.insert((cost, (x, y), (dx, dy)));
                }
            }
        }
    }
    panic!()
}

pub fn solve(input: &str) -> usize {
    run(parse(input), 0, 3)
}

pub fn solve_2(input: &str) -> usize {
    run(parse(input), 4, 10)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 102);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 94);
    }

    #[test]
    fn test_sample_2_small() {
        const SAMPLE: &str = "
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991";
        assert_eq!(solve_2(SAMPLE), 71);
    }
}
