use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;

const DIRS: &[IVec2] = &[IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

fn parse(input: &str) -> HashMap<IVec2, &'static [IVec2]> {
    let mut result = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let kind = match c {
                '.' => DIRS,
                '>' => &DIRS[0..1],
                '<' => &DIRS[1..2],
                'v' => &DIRS[2..3],
                '^' => &DIRS[3..4],
                '#' => continue,
                _ => panic!(),
            };
            result.insert(IVec2::new(x as i32, y as i32), kind);
        }
    }
    result
}

fn adjacent<'a>(
    trails: &'a HashMap<IVec2, &'static [IVec2]>,
    p: IVec2,
) -> impl Iterator<Item = IVec2> + 'a {
    trails
        .get(&p)
        .unwrap_or(&&DIRS[0..0])
        .into_iter()
        .map(move |&d| p + d)
        .filter(|p| trails.contains_key(p))
}

fn run(trails: HashMap<IVec2, &'static [IVec2]>) -> usize {
    let &start_pos = trails.keys().min_by_key(|p| p.y).unwrap();
    let &end_pos = trails.keys().max_by_key(|p| p.y).unwrap();
    // List of start and end, and all intersections (points with 3 options).
    let mut intersections: HashSet<_> = [start_pos, end_pos].into();
    intersections.extend(
        trails
            .keys()
            .copied()
            .filter(|&p| adjacent(&trails, p).count() >= 3),
    );
    // Perform a BFS from each intersection to find the linear routes to the
    // neighbouring intersections
    let mut neighbours: HashMap<IVec2, Vec<(usize, IVec2)>> = HashMap::new();
    for &start in &intersections {
        let mut seen = HashSet::new();
        let mut queue: VecDeque<_> = [(0, start)].into();
        while let Some((distance, p)) = queue.pop_front() {
            if !trails.contains_key(&p) || !seen.insert(p) {
                continue;
            }
            if p != start && intersections.contains(&p) {
                neighbours.entry(start).or_default().push((distance, p));
                continue;
            }
            queue.extend(adjacent(&trails, p).map(|p| (distance + 1, p)));
        }
    }
    // Perform a DFS over the intersections to find the longest path
    enum DFS {
        Search(usize, IVec2),
        Backtrack(IVec2),
    }
    let mut stack: Vec<_> = [DFS::Search(0, start_pos)].into();
    let mut seen = HashSet::new();
    let mut max_distance = 0;
    while let Some(dfs) = stack.pop() {
        match dfs {
            DFS::Search(distance, p) => {
                if !seen.insert(p) {
                    continue;
                }
                stack.push(DFS::Backtrack(p));
                if p == end_pos {
                    max_distance = max_distance.max(distance);
                    continue;
                }
                if let Some(e) = neighbours.get(&p) {
                    for &(delta, p) in e {
                        stack.push(DFS::Search(distance + delta, p));
                    }
                }
            }
            DFS::Backtrack(p) => {
                seen.remove(&p);
            }
        }
    }
    max_distance
}

pub fn solve(input: &str) -> usize {
    run(parse(input))
}

pub fn solve_2(input: &str) -> usize {
    run(parse(input).into_keys().map(|p| (p, DIRS)).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    #.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#
    ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 94);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 154);
    }
}
