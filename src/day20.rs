use std::collections::{HashMap, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, one_of},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{pair, preceded},
    IResult,
};

fn parse_modules(input: &str) -> HashMap<&str, Module> {
    fn parse(input: &str) -> IResult<&str, Vec<((Option<char>, &str), Vec<&str>)>> {
        many1(preceded(
            multispace0,
            pair(
                pair(opt(one_of("%&")), alpha1),
                preceded(tag(" -> "), separated_list1(tag(", "), alpha1)),
            ),
        ))(input)
    }
    let (_, modules) = parse(input).unwrap();
    let mut inputs = HashMap::<&str, Vec<&str>>::new();
    for ((_, name), dests) in &modules {
        for dest in dests {
            inputs.entry(dest).or_default().push(name);
        }
    }
    let mut result = HashMap::new();
    for ((prefix, name), dests) in modules {
        let kind = match prefix {
            Some('%') => Kind::FlipFlop(false),
            Some('&') => Kind::Conjunction(
                inputs
                    .get(name)
                    .unwrap()
                    .iter()
                    .map(|&src| (src, false))
                    .collect(),
            ),
            None => Kind::Broadcast,
            _ => panic!(),
        };
        result.insert(name, Module { kind, dests });
    }
    result
}

#[derive(Clone)]
enum Kind<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
}

#[derive(Clone)]
struct Module<'a> {
    kind: Kind<'a>,
    dests: Vec<&'a str>,
}

fn run<'a>(
    modules: &mut HashMap<&'a str, Module<'a>>,
    start: &'a str,
    end: Option<&'a str>,
) -> ([usize; 2], bool) {
    let mut counts = [0; 2];
    let mut queue: VecDeque<_> = [(None, false, start)].into();
    while let Some((src, mut signal, dest)) = queue.pop_front() {
        counts[signal as usize] += 1;
        if (signal, Some(dest)) == (false, end) {
            return (counts, true);
        }
        let Some(module) = modules.get_mut(dest) else {
            continue;
        };
        match &mut module.kind {
            Kind::FlipFlop(value) if !signal => {
                (signal, *value) = (!*value, !*value);
            }
            Kind::Conjunction(inputs) => {
                *inputs.get_mut(&src.unwrap()).unwrap() = signal;
                signal = !inputs.values().all(|&v| v);
            }
            Kind::Broadcast => (),
            _ => continue,
        }
        for d in &module.dests {
            queue.push_back((Some(dest), signal, d));
        }
    }
    (counts, false)
}

pub fn solve(input: &str) -> usize {
    let mut modules = parse_modules(input);
    let (mut a1, mut b1) = (0, 0);
    for _ in 0..1000 {
        let ([a2, b2], _) = run(&mut modules, "broadcaster", None);
        (a1, b1) = (a1 + a2, b1 + b2);
    }
    a1 * b1
}

pub fn solve_2(input: &str) -> usize {
    let modules: HashMap<&str, Module<'_>> = parse_modules(input);
    let gcd = |mut a: usize, mut b: usize| {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    };
    let calc_length = |(start, end)| {
        let mut modules = modules.clone();
        for i in 1.. {
            if run(&mut modules, start, Some(end)).1 {
                return i;
            }
        }
        panic!()
    };
    // The input's structure looks like four independent components. When the
    // last module of each component signals false the "rx" module will also
    // signal false. So the solution is to find the period of each component,
    // then find the least common multiple of those values. The names of these
    // modules were found by inspecting the graph.
    [("nm", "pk"), ("ps", "pm"), ("sh", "hf"), ("fs", "mk")]
        .into_iter()
        .map(calc_length)
        .reduce(|a, b| a * b / gcd(a, b))
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 32000000);
    }
}
