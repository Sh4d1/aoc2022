use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;

fn solve(input: &[char], n: usize) -> usize {
    input
        .windows(n)
        .fold_while(0, |acc, cs| {
            let hs: HashSet<&char> = cs.iter().collect();
            if hs.len() == n {
                return Done(acc);
            }
            Continue(acc + 1)
        })
        .into_inner()
        + n
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<char> {
    input.chars().map(|l| l).collect()
}
#[aoc(day6, part1)]
pub fn part1(input: &[char]) -> usize {
    solve(input, 4)
}
#[aoc(day6, part2)]
pub fn part2(input: &[char]) -> usize {
    solve(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 11)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 26)
    }
}
