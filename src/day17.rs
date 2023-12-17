use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

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

fn run(grid: Vec<Vec<usize>>, min_straight: Option<usize>, max_straight: usize) -> usize {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut heap: BinaryHeap<_> = [Reverse((0, (0, 0), None, 0))].into();
    let mut seen = HashSet::new();
    while let Some(Reverse((cost, (x, y), prev_dir, straight_length))) = heap.pop() {
        if (x, y) == (width - 1, height - 1) {
            if min_straight.is_none() || straight_length >= min_straight.unwrap() {
                return cost;
            }
        }
        if !seen.insert(((x, y), prev_dir, straight_length)) {
            continue;
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if prev_dir == Some((-dx, -dy)) {
                continue;
            }
            if min_straight.is_some()
                && straight_length < min_straight.unwrap()
                && prev_dir.is_some()
                && prev_dir.unwrap() != (dx, dy)
            {
                continue;
            }
            let straight_length = if prev_dir == Some((dx, dy)) {
                straight_length + 1
            } else {
                1
            };
            if straight_length > max_straight {
                continue;
            }
            let (x, y) = (x + dx, y + dy);
            if !(0..height).contains(&y) || !(0..width).contains(&x) {
                continue;
            }
            let cost = cost + grid[y as usize][x as usize];
            heap.push(Reverse((cost, (x, y), Some((dx, dy)), straight_length)));
        }
    }
    panic!()
}

pub fn solve(input: &str) -> usize {
    run(parse(input), None, 3)
}

pub fn solve_2(input: &str) -> usize {
    run(parse(input), Some(4), 10)
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
