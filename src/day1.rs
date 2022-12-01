#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .split("\n\n")
        .map(|l| l.lines().map(|ll| ll.parse::<u64>().unwrap()).sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u64]) -> u64 {
    *input.iter().max().unwrap()
}
#[aoc(day1, part2)]
pub fn part2(input: &[u64]) -> u64 {
    let mut i = input.to_owned();
    i.sort();
    i.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            )),
            24000
        )
    }
    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            )),
            45000
        )
    }
}
