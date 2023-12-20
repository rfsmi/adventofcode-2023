use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, one_of},
    combinator::{map, map_res},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

fn parse(
    input: &str,
) -> (
    HashMap<&str, (Vec<(char, char, usize, &str)>, &str)>,
    Vec<[usize; 4]>,
) {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, str::parse)(input)
    }
    let rule = tuple((
        one_of("xmas"),
        one_of("<>"),
        number,
        preceded(tag(":"), alpha1),
    ));
    let workflow = pair(
        alpha1,
        delimited(
            tag("{"),
            pair(many0(terminated(rule, tag(","))), alpha1),
            tag("}"),
        ),
    );
    let part = delimited(
        tag("{"),
        map(
            separated_list1(tag(","), pair(one_of("xmas"), preceded(tag("="), number))),
            |catergories| {
                let mut array = [0; 4];
                for (c, n) in catergories {
                    array["xmas".find(c).unwrap()] = n;
                }
                array
            },
        ),
        tag("}"),
    );
    let (_, (workflows, parts)) = pair(
        map(many1(preceded(multispace0, workflow)), |workflows| {
            workflows.into_iter().collect::<HashMap<_, _>>()
        }),
        many1(preceded(multispace0, part)),
    )(input)
    .unwrap();
    (workflows, parts)
}

fn is_accepted<'a>(
    workflows: HashMap<&'a str, (Vec<(char, char, usize, &'a str)>, &'a str)>,
) -> impl Fn(&[usize; 4]) -> bool + 'a {
    fn resolve_workflow<'a>(
        part: &[usize; 4],
        rules: &Vec<(char, char, usize, &'a str)>,
        default: &'a str,
    ) -> &'a str {
        for &(lhs, op, rhs, res) in rules {
            let i = "xmas".find(lhs).unwrap();
            match op {
                '<' if part[i] < rhs => return res,
                '>' if part[i] > rhs => return res,
                _ => (),
            }
        }
        default
    }
    move |part| {
        let mut workflow = "in";
        while let Some((rules, default)) = workflows.get(workflow) {
            workflow = resolve_workflow(part, rules, default);
        }
        workflow == "A"
    }
}

pub fn solve(input: &str) -> usize {
    let (workflows, parts) = parse(input);
    parts
        .into_iter()
        .filter(is_accepted(workflows))
        .flatten()
        .sum()
}

fn count_parts(
    workflows: &HashMap<&str, (Vec<(char, char, usize, &str)>, &str)>,
    start: &str,
    mut rem: [(usize, usize); 4],
) -> usize {
    match start {
        "A" => return rem.into_iter().map(|(min, max)| 1 + max - min).product(),
        "R" => return 0,
        _ => (),
    }
    let mut total = 0;
    let (rules, default) = workflows.get(start).unwrap();
    for &(lhs, op, rhs, res) in rules {
        let i = "xmas".find(lhs).unwrap();
        let mut parts = rem;
        match op {
            '<' => {
                parts[i].1 = rhs - 1;
                rem[i].0 = rhs;
            }
            '>' => {
                parts[i].0 = rhs + 1;
                rem[i].1 = rhs;
            }
            _ => (),
        }
        total += count_parts(workflows, res, parts);
    }
    total + count_parts(workflows, default, rem)
}

pub fn solve_2(input: &str) -> usize {
    let (workflows, _) = parse(input);
    count_parts(&workflows, "in", [(1, 4000); 4])
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "
    px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}

    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 19114);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 167409079868000);
    }
}
