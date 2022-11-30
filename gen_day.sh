#!/bin/bash

day=$1
type=$2

export day type

if [[ ! -f "./src/day${day}.rs" ]]; then
    envsubst <<EOF > ./src/day${day}.rs
#[aoc_generator(day$day)]
pub fn input_generator(input: &str) -> $type {
    // input.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}
#[aoc(day$day, part1)]
pub fn part1(input: &$type) -> usize {
    0
}
#[aoc(day$day, part2)]
pub fn part2(input: &$type) -> usize {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                ""
            )),
            0
        )
    }
    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                ""
            )),
            0
        )
    }
}
EOF
fi

if ! grep -q day${day} src/lib.rs; then
    sed -i "/aoc_lib.*/i pub mod day${day};" ./src/lib.rs
fi
