use std::collections::{BTreeSet, HashMap, HashSet};

use glam::{IVec2, IVec3, Vec3Swizzles};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{pair, preceded},
    IResult,
};

struct Brick {
    p1: IVec3,
    p2: IVec3,
}

impl Brick {
    fn parse(input: &str) -> IResult<&str, Self> {
        fn point(input: &str) -> IResult<&str, IVec3> {
            map(
                separated_list1(tag(","), map_res(digit1, str::parse)),
                |coords| IVec3::from_slice(&coords),
            )(input)
        }
        map(pair(point, preceded(tag("~"), point)), |(p1, p2)| Brick {
            p1,
            p2,
        })(input)
    }

    fn footprint(&self) -> impl Iterator<Item = IVec2> {
        let min = self.p1.xy().min(self.p2.xy());
        let max = self.p1.xy().max(self.p2.xy());
        (min.x..=max.x)
            .cartesian_product(min.y..=max.y)
            .map(IVec2::from)
    }

    fn z(&self) -> i32 {
        self.p1.min(self.p2).z
    }

    fn height(&self) -> i32 {
        self.p1.max(self.p2).z - self.z() + 1
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Brick>> {
    many1(preceded(multispace0, Brick::parse))(input)
}

fn num_dependents(bricks: &[Brick]) -> Vec<usize> {
    // For each brick, find its new z value, the bricks it's resting on below,
    // and the bricks that are resting on it above.
    let mut below = vec![HashSet::<usize>::new(); bricks.len()];
    let mut above = vec![HashSet::<usize>::new(); bricks.len()];
    let mut z = vec![0; bricks.len()];
    let mut columns: HashMap<IVec2, usize> = HashMap::new();
    for i in (0..bricks.len()).sorted_by_key(|&i| bricks[i].z()) {
        let mut layers: HashMap<i32, HashSet<usize>> = HashMap::new();
        for p in bricks[i].footprint() {
            if let Some(j) = columns.insert(p, i) {
                layers
                    .entry(z[j] + bricks[j].height())
                    .or_default()
                    .insert(j);
            }
        }
        z[i] = *layers.keys().max().unwrap_or(&1);
        for j in layers.remove(&z[i]).unwrap_or_else(HashSet::new) {
            below[i].insert(j);
            above[j].insert(i);
        }
    }
    // Now for each brick, figure out which ones will fall when it's
    // disintegrated.
    (0..bricks.len())
        .map(|start_i| {
            let mut seen = HashSet::new();
            let mut falling: HashSet<usize> = [start_i].into();
            let mut queue: BTreeSet<_> = [(z[start_i], start_i)].into();
            while let Some((_, i)) = queue.pop_first() {
                if !seen.insert(i) {
                    continue;
                }
                if below[i].iter().all(|j| falling.contains(j)) {
                    falling.insert(i);
                }
                queue.extend(above[i].iter().map(|&j| (z[j], j)));
            }
            falling.len() - 1
        })
        .collect()
}

pub fn solve(input: &str) -> usize {
    let bricks = parse(input).unwrap().1;
    num_dependents(&bricks)
        .into_iter()
        .filter(|&n| n == 0)
        .count()
}

pub fn solve_2(input: &str) -> usize {
    let bricks = parse(input).unwrap().1;
    num_dependents(&bricks).into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 5);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 7);
    }
}
