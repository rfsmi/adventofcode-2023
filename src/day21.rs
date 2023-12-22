use std::{
    collections::{HashSet, VecDeque},
    iter::zip,
};

fn parse(input: &str) -> ((i64, i64), HashSet<(i64, i64)>) {
    let mut gardens = HashSet::new();
    let mut start = (0, 0);
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '.' {
                gardens.insert((x as i64, y as i64));
            }
            if c == 'S' {
                start = (x as i64, y as i64);
            }
        }
    }
    gardens.insert(start);
    (start, gardens)
}

fn size(points: &HashSet<(i64, i64)>) -> i64 {
    let Some((x, y)) = points
        .iter()
        .copied()
        .reduce(|(x1, y1), (x2, y2)| (x1.max(x2), y1.max(y2)))
    else {
        return 0;
    };
    assert_eq!(x, y);
    x + 1
}

fn run(gardens: &HashSet<(i64, i64)>, start: (i64, i64), max: i64) -> i64 {
    let mut seen = HashSet::new();
    let mut queue: VecDeque<_> = [(0, start)].into();
    let size = size(&gardens);
    while let Some((distance, (x, y))) = queue.pop_front() {
        if distance > max
            || !gardens.contains(&(x.rem_euclid(size), y.rem_euclid(size)))
            || !seen.insert((x, y))
        {
            continue;
        }
        queue.extend(
            [(-1, 0), (0, -1), (1, 0), (0, 1)]
                .into_iter()
                .map(|(dx, dy)| (distance + 1, (x + dx, y + dy))),
        );
    }
    seen.into_iter()
        .filter(|(x, y)| (x + y - start.0 - start.1).rem_euclid(2) == max % 2)
        .count() as i64
}

pub fn solve(input: &str) -> i64 {
    let (start, gardens) = parse(input);
    run(&gardens, start, 64)
}

fn interpolate(points: &[(i64, i64)], estimate: i64) -> i64 {
    let mut numerators: Vec<_> = points.iter().map(|&(_, y)| y as i128).collect();
    let mut denoms: Vec<_> = points.iter().map(|_| 1).collect();
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i != j {
                numerators[i] *= estimate as i128 - points[j].0 as i128;
                denoms[i] *= points[i].0 as i128 - points[j].0 as i128;
            }
        }
    }
    let lcm = denoms.iter().fold(1, |a: i128, b: &i128| {
        let (mut a, mut b) = (a.abs(), b.abs());
        let prod = a * b;
        while b != 0 {
            (a, b) = (b, a % b);
        }
        prod / a
    });
    (zip(numerators, denoms)
        .map(|(num, denom)| num * lcm / denom)
        .sum::<i128>()
        / lcm) as i64
}

pub fn solve_2(input: &str) -> i64 {
    let (start, gardens) = parse(input);
    let size = size(&gardens);
    let mut points = Vec::new();
    for i in 0..3 {
        let x = size / 2 + i * size;
        let y = run(&gardens, start, x);
        points.push((x, y));
    }
    println!("{points:?}");
    interpolate(&points, 26501365)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    ...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ...........";

    #[test]
    fn test_sample() {
        let (start, gardens) = parse(SAMPLE);
        assert_eq!(run(&gardens, start, 6), 16);
    }

    #[test]
    fn test_sample_2() {
        let (start, gardens) = parse(SAMPLE);
        assert_eq!(run(&gardens, start, 10), 50);
        assert_eq!(run(&gardens, start, 50), 1594);
        assert_eq!(run(&gardens, start, 100), 6536);
        assert_eq!(run(&gardens, start, 500), 167004);
        // assert_eq!(run(&gardens, start, 1000), 668697);
        // assert_eq!(run(&gardens, start, 5000), 16733044);
    }

    #[test]
    fn test_interpolation() {
        let values = [
            (65, 3720),
            (196, 33150),
            (327, 91890),
            (26501365, 599763113936220),
        ];
        for i in 0..values.len() {
            let mut values = Vec::from(values.clone());
            let (x, y) = values.remove(i);
            assert_eq!(interpolate(&values, x), y);
        }
    }
}
