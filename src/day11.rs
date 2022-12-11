use anyhow::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref R1: Regex = Regex::new(r"Monkey (\d+):").unwrap();
    static ref R2: Regex = Regex::new(r"  Starting items: ([0-9, ]+)").unwrap();
    static ref R3: Regex = Regex::new(r"  Operation: new = old (.) ([a-z0-9]+)").unwrap();
    static ref R4: Regex = Regex::new(r"  Test: divisible by (\d+)").unwrap();
    static ref R5: Regex = Regex::new(r"    If true: throw to monkey (\d+)").unwrap();
    static ref R6: Regex = Regex::new(r"    If false: throw to monkey (\d+)").unwrap();
}

#[derive(Debug, Clone)]
pub enum Op {
    Add(usize),
    Mul(usize),
    MulSelf,
}
use Op::*;

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<usize>,
    op: Op,
    div_by: usize,
    throw_to: (usize, usize),
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Monkey, Self::Err> {
        let mut lines = s.lines().skip(1);

        let l2 = R2.captures(lines.next().unwrap()).unwrap();
        let l3 = R3.captures(lines.next().unwrap()).unwrap();
        let l4 = R4.captures(lines.next().unwrap()).unwrap();
        let l5 = R5.captures(lines.next().unwrap()).unwrap();
        let l6 = R6.captures(lines.next().unwrap()).unwrap();

        let items = l2[1]
            .split(", ")
            .map(|i| i.parse::<usize>().unwrap())
            .collect();
        let op = match &l3[1] {
            "+" => Add(l3[2].parse().unwrap()),
            "*" if &l3[2] == "old" => MulSelf,
            "*" if &l3[2] != "old" => Mul(l3[2].parse().unwrap()),
            _ => unreachable!(),
        };
        let div_by = l4[1].parse::<usize>().unwrap();
        let if_true = l5[1].parse::<usize>().unwrap();
        let if_false = l6[1].parse::<usize>().unwrap();
        Ok(Monkey {
            items,
            op,
            div_by,
            throw_to: (if_true, if_false),
        })
    }
}
#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect()
}

pub fn one_round(monkeys: &mut Vec<Monkey>, div: bool, pgcm: usize) -> Vec<usize> {
    let mut res = vec![0; monkeys.len()];
    for i in 0..monkeys.len() {
        for j in 0..monkeys[i].items.len() {
            let item = monkeys[i].items[j];
            let throw_to = monkeys[i].throw_to;

            let mut res = match monkeys[i].op {
                Add(x) => item + x,
                Mul(x) => item * x,
                MulSelf => item * item,
            };

            if div {
                res = res / 3;
            }

            let res = res % pgcm;

            if res % monkeys[i].div_by == 0 {
                monkeys[throw_to.0].items.push(res);
            } else {
                monkeys[throw_to.1].items.push(res);
            }
        }
        res[i] += monkeys[i].items.len();
        monkeys[i].items = vec![];
    }
    res
}

#[aoc(day11, part1)]
pub fn part1(input: &[Monkey]) -> usize {
    let mut input = input.to_vec();
    let pgcm = input.iter().fold(1, |acc, m| acc * m.div_by);
    let mut res = (0..20).fold(vec![0; input.len()], |acc: Vec<usize>, _| {
        acc.iter()
            .zip(one_round(&mut input, true, pgcm))
            .map(|(&x, y)| x + y)
            .collect()
    });
    res.sort_by(|a, b| b.partial_cmp(a).unwrap());
    res[0] * res[1]
}

#[aoc(day11, part2)]
pub fn part2(input: &[Monkey]) -> usize {
    let mut input = input.to_vec();
    let pgcm = input.iter().fold(1, |acc, m| acc * m.div_by);
    let mut res = (0..10000).fold(vec![0; input.len()], |acc: Vec<usize>, _| {
        acc.iter()
            .zip(one_round(&mut input, false, pgcm))
            .map(|(&x, y)| x + y)
            .collect()
    });
    res.sort_by(|a, b| b.partial_cmp(a).unwrap());
    res[0] * res[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 10605)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 2713310158)
    }
}
