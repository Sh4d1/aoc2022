#[derive(Copy, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
pub enum Letter {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone)]
pub enum Res {
    Loose,
    Draw,
    Win,
}

use Hand::*;
use Letter::*;
use Res::*;

impl Hand {
    fn get_points(&self, other: &Hand) -> u64 {
        match (*self, *other) {
            (Rock, Rock) => 4,
            (Rock, Paper) => 1,
            (Rock, Scissors) => 7,

            (Paper, Rock) => 8,
            (Paper, Paper) => 5,
            (Paper, Scissors) => 2,

            (Scissors, Rock) => 3,
            (Scissors, Paper) => 9,
            (Scissors, Scissors) => 6,
        }
    }
}

impl Res {
    fn get_points(&self, other: &Hand) -> u64 {
        match (*self, *other) {
            (Loose, Rock) => Scissors,
            (Loose, Paper) => Rock,
            (Loose, Scissors) => Paper,

            (Draw, a) => a,

            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
        }
        .get_points(other)
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Hand, Letter)> {
    input
        .lines()
        .map(|l| {
            let mut c = l.chars();
            let one = match c.next() {
                Some('A') => Rock,
                Some('B') => Paper,
                Some('C') => Scissors,
                _ => unreachable!(),
            };
            c.next();
            let two = match c.next() {
                Some('X') => X,
                Some('Y') => Y,
                Some('Z') => Z,
                _ => unreachable!(),
            };
            (one, two)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(Hand, Letter)]) -> u64 {
    input
        .iter()
        .map(|(h, l)| {
            match l {
                X => Rock,
                Y => Paper,
                Z => Scissors,
            }
            .get_points(h)
        })
        .sum()
}
#[aoc(day2, part2)]
pub fn part2(input: &[(Hand, Letter)]) -> u64 {
    input
        .iter()
        .map(|(h, l)| {
            match l {
                X => Loose,
                Y => Draw,
                Z => Win,
            }
            .get_points(h)
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "A Y
B X
C Z"
            )),
            15
        )
    }
    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "A Y
B X
C Z"
            )),
            12
        )
    }
}
