use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Rock {
    Empty,
    Ball,
    Cube,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Platform {
    origin: (isize, isize),
    rocks: Vec<Vec<Rock>>,
    x_basis: (isize, isize),
    y_basis: (isize, isize),
}

impl Platform {
    fn new(input: &str) -> Self {
        let rocks: Vec<Vec<Rock>> = input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| match c {
                        '.' => Rock::Empty,
                        'O' => Rock::Ball,
                        '#' => Rock::Cube,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        Self {
            rocks,
            origin: (0, 0),
            x_basis: (1, 0),
            y_basis: (0, 1),
        }
    }

    fn tilt_next(&mut self) {
        let max_x = self.rocks[0].len() as isize - 1;
        let max_y = self.rocks.len() as isize - 1;
        let (mut x0, mut y0) = (self.origin.0 * max_x, self.origin.1 * max_y);
        while (0..=max_x).contains(&x0) && (0..=max_y).contains(&y0) {
            let (mut x, mut y) = (x0, y0);
            let (mut xi, mut yi) = (x0, y0);
            while (0..=max_x).contains(&x) && (0..=max_y).contains(&y) {
                match self.rocks[y as usize][x as usize] {
                    Rock::Empty => (),
                    Rock::Cube => (xi, yi) = (x + self.y_basis.0, y + self.y_basis.1),
                    Rock::Ball => {
                        self.rocks[y as usize][x as usize] = Rock::Empty;
                        self.rocks[yi as usize][xi as usize] = Rock::Ball;
                        (xi, yi) = (xi + self.y_basis.0, yi + self.y_basis.1);
                    }
                }
                (x, y) = (x + self.y_basis.0, y + self.y_basis.1);
            }
            (x0, y0) = (x0 + self.x_basis.0, y0 + self.x_basis.1);
        }
        self.x_basis = (self.x_basis.1, -self.x_basis.0);
        self.y_basis = (self.y_basis.1, -self.y_basis.0);
        self.origin = (self.origin.1, 1 - self.origin.0);
    }

    fn total_load(&self) -> usize {
        let width = self.rocks[0].len();
        let height = self.rocks.len();
        (0..height)
            .cartesian_product(0..width)
            .filter(|&(y, x)| self.rocks[y][x] == Rock::Ball)
            .map(|(y, _)| height - y)
            .sum()
    }
}

fn run(mut platform: Platform, iters: usize) -> Platform {
    let mut seen: HashMap<Platform, usize> = HashMap::new();
    for i in 0..iters {
        if let Some(prev_i) = seen.insert(platform.clone(), i) {
            return run(platform, (iters - prev_i) % (i - prev_i));
        }
        platform.tilt_next();
    }
    platform
}

pub fn solve(input: &str) -> usize {
    run(Platform::new(input), 1).total_load()
}

pub fn solve_2(input: &str) -> usize {
    run(Platform::new(input), 4 * 1_000_000_000).total_load()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 136)
    }

    #[test]
    fn test_rotation() {
        assert_eq!(solve_2(SAMPLE), 64)
    }
}
