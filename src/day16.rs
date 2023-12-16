use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn run(grid: &[Vec<char>], start: ((i32, i32), (i32, i32))) -> usize {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut stack = vec![start];
    let mut seen = HashMap::<_, HashSet<_>>::new();
    while let Some(((x, y), (dx, dy))) = stack.pop() {
        if !(0..height).contains(&y) || !(0..width).contains(&x) {
            continue;
        }
        if !seen.entry((x, y)).or_default().insert((dx, dy)) {
            continue;
        }
        let dirs = match grid[y as usize][x as usize] {
            '\\' => vec![(dy, dx)],
            '/' => vec![(-dy, -dx)],
            '-' if dx == 0 => vec![(-1, 0), (1, 0)],
            '|' if dy == 0 => vec![(0, -1), (0, 1)],
            '.' | '-' | '|' => vec![(dx, dy)],
            _ => panic!(),
        };
        for (dx, dy) in dirs {
            stack.push(((x + dx, y + dy), (dx, dy)));
        }
    }
    seen.len()
}

pub fn solve(input: &str) -> usize {
    run(&parse(input), ((0, 0), (1, 0)))
}

pub fn solve_2(input: &str) -> usize {
    let grid = parse(input);
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    (0..height)
        .flat_map(|y| [((0, y), (1, 0)), ((width - 1, y), (-1, 0))])
        .chain((0..width).flat_map(|x| [((x, 0), (0, 1)), ((x, height - 1), (0, -1))]))
        .map(|start| run(&grid, start))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"
    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 46);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 51);
    }
}
