use itertools::Itertools;
use std::collections::HashSet;

pub type Rucksack = (HashSet<char>, HashSet<char>);

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(s1, s2)| {
            (
                s1.chars().into_iter().collect(),
                s2.chars().into_iter().collect(),
            )
        })
        .collect()
}

pub fn get_score(c: &char) -> u32 {
    if c.is_lowercase() {
        *c as u32 - 96
    } else {
        *c as u32 - 38
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &[Rucksack]) -> u32 {
    input.into_iter().fold(0, |acc, i| {
        acc + get_score(i.0.intersection(&i.1).into_iter().next().unwrap())
    })
}

#[aoc(day3, part2)]
pub fn part2(input: &[Rucksack]) -> u32 {
    input
        .into_iter()
        .map(|(a, b)| a.union(&b).collect::<HashSet<_>>())
        .chunks(3)
        .into_iter()
        .map(|c| {
            c.reduce(|acc, r| acc.intersection(&r).cloned().collect::<HashSet<_>>())
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
        })
        .fold(0, |acc, c| acc + get_score(c))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            )),
            157
        )
    }
    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            )),
            70
        )
    }
}
