use std::collections::HashSet;

#[derive(Debug)]
pub enum Cmd {
    Right,
    Left,
    Up,
    Down,
}
use Cmd::*;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<(Cmd, usize)> {
    input
        .lines()
        .map(|l| match l.split_once(" ").unwrap() {
            ("R", s) => (Right, s.parse().unwrap()),
            ("L", s) => (Left, s.parse().unwrap()),
            ("U", s) => (Up, s.parse().unwrap()),
            ("D", s) => (Down, s.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect()
}

pub fn move_tails(tails: &mut Vec<(isize, isize)>) {
    let mut h = tails.first().cloned().unwrap();
    tails.iter_mut().skip(1).for_each(|t| {
        let (dx, dy) = (h.0 - t.0, h.1 - t.1);
        if dx.abs() > 1 || dy.abs() > 1 {
            *t = (t.0 + dx.signum(), t.1 + dy.signum());
        }
        h = *t;
    })
}

pub fn run(input: &[(Cmd, usize)], tail_size: usize) -> usize {
    let mut tails = vec![(0, 0); tail_size + 1];
    input
        .iter()
        .fold(HashSet::new(), |mut acc, c| {
            acc.extend((0..c.1).map(|_| {
                let mut head = tails.first_mut().unwrap();
                match c.0 {
                    Up => {
                        head.1 -= 1;
                    }
                    Down => {
                        head.1 += 1;
                    }
                    Left => {
                        head.0 -= 1;
                    }
                    Right => {
                        head.0 += 1;
                    }
                }
                move_tails(&mut tails);
                tails.last().cloned().unwrap()
            }));
            acc
        })
        .len()
}

#[aoc(day9, part1)]
pub fn part1(input: &[(Cmd, usize)]) -> usize {
    run(input, 1)
}

#[aoc(day9, part2)]
pub fn part2(input: &[(Cmd, usize)]) -> usize {
    run(input, 9)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 13)
    }
    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )),
            36
        )
    }
}
