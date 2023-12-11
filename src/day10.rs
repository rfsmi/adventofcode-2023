use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse(input: &str) -> ((i64, i64), HashMap<(i64, i64), Vec<(i64, i64)>>) {
    let mut edges = HashMap::new();
    let mut start = None;
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let pos = (x as i64, y as i64);
            let ends = match c {
                '-' => [(-1, 0), (1, 0)],
                '|' => [(0, -1), (0, 1)],
                '7' => [(-1, 0), (0, 1)],
                'F' => [(1, 0), (0, 1)],
                'J' => [(-1, 0), (0, -1)],
                'L' => [(1, 0), (0, -1)],
                'S' => {
                    start = Some(pos);
                    continue;
                }
                '.' => continue,
                _ => panic!(),
            };
            edges.insert(
                pos,
                ends.into_iter()
                    .map(|(dx, dy)| (x as i64 + dx, y as i64 + dy))
                    .collect(),
            );
        }
    }
    (start.unwrap(), edges)
}

fn main_loop(start: (i64, i64), edges: HashMap<(i64, i64), Vec<(i64, i64)>>) -> Vec<(i64, i64)> {
    let &first = edges.iter().find(|(_, v)| v.contains(&start)).unwrap().0;
    let mut path = vec![start];
    let (mut prev, mut pos) = (start, first);
    while pos != start {
        path.push(pos);
        for &next in edges.get(&pos).unwrap() {
            if next != prev {
                (prev, pos) = (pos, next);
                break;
            }
        }
    }
    path
}

pub fn solve(input: &str) -> usize {
    let (start, edges) = parse(input);
    main_loop(start, edges).len() / 2
}

fn turn_left(prev: (i64, i64), pos: (i64, i64)) -> (i64, i64) {
    let (dx, dy) = (pos.0 - prev.0, pos.1 - prev.1);
    (pos.0 + dy, pos.1 - dx)
}

fn turn_right(prev: (i64, i64), pos: (i64, i64)) -> (i64, i64) {
    let (dx, dy) = (pos.0 - prev.0, pos.1 - prev.1);
    (pos.0 - dy, pos.1 + dx)
}

pub fn solve_2(input: &str) -> usize {
    let (start, edges) = parse(input);
    let path = main_loop(start, edges);
    let (mut left_turns, mut right_turns) = (0, 0);
    let (mut left_seeds, mut right_seeds) = (Vec::new(), Vec::new());
    // Find all the spaces to the left and right of the path
    for (&prev, &pos, &next) in path.iter().cycle().tuple_windows().take(path.len()) {
        if next == turn_left(prev, pos) {
            left_turns += 1;
        } else if next == turn_right(prev, pos) {
            right_turns += 1;
        }
        // These are the three possible cases
        //    ^         r         l
        //  l | r     <-+ r     l +->
        //    |         |         |
        left_seeds.push(turn_left(prev, pos));
        left_seeds.push(turn_right(next, pos));
        right_seeds.push(turn_right(prev, pos));
        right_seeds.push(turn_left(next, pos));
    }
    // Whichever side has the most turns is the inside.
    let mut stack = if left_turns > right_turns {
        left_seeds
    } else {
        right_seeds
    };
    // Flood fill from the corresponding seeds, don't count any pipes
    let mut seen: HashSet<_> = path.into_iter().collect();
    let mut enclosed = 0;
    while let Some((x, y)) = stack.pop() {
        if !seen.insert((x, y)) {
            continue;
        }
        enclosed += 1;
        stack.extend(
            [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .map(|(dx, dy)| (x + dx, y + dy)),
        );
    }
    enclosed
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        const SAMPLE: &str = "
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";
        assert_eq!(solve(SAMPLE), 8);
    }

    #[test]
    fn test_sample_small() {
        const SAMPLE: &str = "
        .....
        .S-7.
        .|.|.
        .L-J.
        .....";
        assert_eq!(solve(SAMPLE), 4);
    }

    #[test]
    fn test_sample_2_small() {
        const SAMPLE: &str = "
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";
        assert_eq!(solve_2(SAMPLE), 4);
    }

    #[test]
    fn test_sample_2() {
        const SAMPLE: &str = "
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(solve_2(SAMPLE), 10);
    }

    #[test]
    fn test_sample_2_another() {
        const SAMPLE: &str = "
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";
        assert_eq!(solve_2(SAMPLE), 8);
    }
}
