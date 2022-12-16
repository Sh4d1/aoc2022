use itertools::{iproduct, Itertools};
use ndarray::Array3;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref R: Regex = Regex::new(
        r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.*)"
    )
    .unwrap();
}

#[derive(Debug)]
pub struct Valve {
    name: String,
    rate: usize,
    tunnels: Vec<String>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> (Vec<Valve>, usize, usize) {
    let i: Vec<Valve> = input
        .lines()
        .map(|l| {
            let l = R.captures(l).unwrap();

            Valve {
                name: l[1].to_string(),
                rate: l[2].parse().unwrap(),
                tunnels: l[3].trim().split(", ").map(|s| s.to_string()).collect(),
            }
        })
        .sorted_by(|a, b| b.rate.cmp(&a.rate))
        .collect();
    let real_pipes = i.iter().filter(|v| v.rate > 0).count();
    let start = i
        .iter()
        .enumerate()
        .find(|(_, v)| v.name == "AA")
        .unwrap()
        .0;
    (i, real_pipes, start)
}

pub fn get_totals(valves: &[Valve], n_real: usize) -> Array3<u16> {
    let next_valves: Vec<Vec<_>> = valves
        .iter()
        .map(|v| {
            v.tunnels
                .iter()
                .map(|to| {
                    valves
                        .iter()
                        .enumerate()
                        .find(|(_, v)| v.name == *to)
                        .unwrap()
                        .0
                })
                .collect()
        })
        .collect();

    let mut totals = Array3::<u16>::zeros([30, valves.len(), 1 << n_real]);

    for (m, v, opened) in iproduct!(1..30, 0..valves.len(), 0..1 << n_real) {
        let bit_v = 1 << v;
        let mut total = totals[(m, v, opened)];
        if bit_v & opened != 0 {
            total = std::cmp::max(
                total,
                totals[(m - 1, v, opened - bit_v)] + (valves[v].rate * m) as u16,
            );
        }
        if let Some(t) = next_valves[v]
            .iter()
            .map(|&vi| totals[(m - 1, vi, opened)])
            .max()
        {
            total = std::cmp::max(total, t);
        }
        totals[(m, v, opened)] = total;
    }
    totals
}

#[aoc(day16, part1)]
pub fn part1(input: &(Vec<Valve>, usize, usize)) -> usize {
    get_totals(&input.0, input.1)[(29, input.2, (1 << input.1) - 1)] as usize
}

#[aoc(day16, part2)]
pub fn part2(input: &(Vec<Valve>, usize, usize)) -> usize {
    let totals = get_totals(&input.0, input.1);

    (0..(1 << input.1))
        .map(|i| {
            (0..i).fold(0u16, |mut acc, j| {
                if i & j == 0 {
                    acc = std::cmp::max(acc, totals[(25, input.2, i)] + totals[(25, input.2, j)]);
                }
                acc
            }) as usize
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 1651)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 1707)
    }
}
