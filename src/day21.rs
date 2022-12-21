use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Op {
    Shout(isize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}
use Op::*;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> HashMap<String, Op> {
    input
        .lines()
        .map(|l| {
            let (name, op) = l.split_once(": ").unwrap();
            let name = name.to_owned();
            let shout = op.parse::<isize>();
            if shout.is_ok() {
                return (name, Shout(shout.unwrap()));
            }
            let op: Vec<&str> = op.split_whitespace().collect();
            match op[1] {
                "+" => (name, Add(op[0].to_string(), op[2].to_string())),
                "-" => (name, Sub(op[0].to_string(), op[2].to_string())),
                "*" => (name, Mul(op[0].to_string(), op[2].to_string())),
                "/" => (name, Div(op[0].to_string(), op[2].to_string())),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn walk(hm: &HashMap<String, Op>, start: &str) -> isize {
    match hm.get(start).unwrap() {
        Shout(n) => *n,
        Add(a, b) => walk(hm, a) + walk(hm, b),
        Sub(a, b) => walk(hm, a) - walk(hm, b),
        Mul(a, b) => walk(hm, a) * walk(hm, b),
        Div(a, b) => walk(hm, a) / walk(hm, b),
    }
}

pub fn contains(hm: &HashMap<String, Op>, start: &str, until: &str) -> bool {
    start == until
        || match hm.get(start).unwrap() {
            Shout(_) => false,
            Add(a, b) | Div(a, b) | Mul(a, b) | Sub(a, b) => {
                contains(hm, a, until) || contains(hm, b, until)
            }
        }
}

pub fn find(hm: &HashMap<String, Op>, start: &str, val: isize, x: &str) -> isize {
    if start == x {
        return val;
    }
    match hm.get(start).unwrap() {
        Add(a, b) if contains(hm, a, x) => find(hm, a, val - walk(hm, b), x),
        Add(a, b) if contains(hm, b, x) => find(hm, b, val - walk(hm, a), x),
        Sub(a, b) if contains(hm, a, x) => find(hm, a, val + walk(hm, b), x),
        Sub(a, b) if contains(hm, b, x) => find(hm, b, walk(hm, a) - val, x),
        Mul(a, b) if contains(hm, a, x) => find(hm, a, val / walk(hm, b), x),
        Mul(a, b) if contains(hm, b, x) => find(hm, b, val / walk(hm, a), x),
        Div(a, b) if contains(hm, a, x) => find(hm, a, val * walk(hm, b), x),
        Div(a, b) if contains(hm, b, x) => find(hm, b, walk(hm, a) / val, x),
        _ => 0,
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &HashMap<String, Op>) -> isize {
    walk(input, "root")
}

#[aoc(day21, part2)]
pub fn part2(input: &HashMap<String, Op>) -> isize {
    let input = input.clone();
    if let Add(s0, s1) = input.get("root").unwrap() {
        if contains(&input, s0, "humn") {
            find(&input, s0, walk(&input, s1), "humn")
        } else {
            find(&input, s1, walk(&input, s0), "humn")
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 152)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 301)
    }
}
