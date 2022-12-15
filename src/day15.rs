use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref R: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
}

#[derive(Debug, Clone, Copy)]
pub struct Sensor {
    pos: (isize, isize),
    closest: (isize, isize),
    dist: isize,
}

impl Sensor {
    pub fn is_inside_range(&self, p: (isize, isize)) -> bool {
        if self.closest == p {
            return false;
        }
        self.dist as usize >= self.pos.0.abs_diff(p.0) + self.pos.1.abs_diff(p.1)
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|l| {
            let l = R.captures(l).unwrap();
            let mut s = Sensor {
                pos: (l[1].parse().unwrap(), l[2].parse().unwrap()),
                closest: (l[3].parse().unwrap(), l[4].parse().unwrap()),
                dist: 0,
            };
            s.dist = (s.pos.0.abs_diff(s.closest.0) + s.pos.1.abs_diff(s.closest.1)) as isize;
            s
        })
        .collect()
}

pub fn part1_n(input: &[Sensor], n: isize) -> usize {
    let l_bound = input.iter().map(|s| s.pos.0 - s.dist).min().unwrap();
    let r_bound = input.iter().map(|s| s.pos.0 + s.dist).max().unwrap();
    (l_bound..=r_bound)
        .filter(|&i| input.iter().any(|s| s.is_inside_range((i, n))))
        .collect::<Vec<_>>()
        .len()
}

pub fn part2_n(input: &[Sensor], n: isize) -> isize {
    input
        .iter()
        .find_map(|s| {
            ((s.pos.0 - s.dist - 1).max(0)..=s.pos.0.min(n))
                .zip(s.pos.1..=n)
                .find_map(|p| {
                    input
                        .iter()
                        .all(|s| !s.is_inside_range(p))
                        .then(|| p.0 * 4000000 + p.1)
                })
        })
        .unwrap()
}

#[aoc(day15, part1)]
pub fn part1(input: &[Sensor]) -> usize {
    part1_n(input, 2000000)
}

#[aoc(day15, part2)]
pub fn part2(input: &[Sensor]) -> isize {
    part2_n(input, 4000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    #[test]
    fn example1() {
        assert_eq!(part1_n(&input_generator(INPUT), 10), 26)
    }
    #[test]
    fn example2() {
        assert_eq!(part2_n(&input_generator(INPUT), 20), 56000011)
    }
}
