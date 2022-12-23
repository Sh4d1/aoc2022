use std::collections::{HashMap, HashSet, VecDeque};

use lazy_static::lazy_static;

lazy_static! {
    static ref MOVES: VecDeque<([(isize, isize); 3], (isize, isize))> = {
        let mut m = VecDeque::new();
        m.push_back(([(-1, -1), (0, -1), (1, -1)], (0, -1)));
        m.push_back(([(-1, 1), (0, 1), (1, 1)], (0, 1)));
        m.push_back(([(-1, 0), (-1, 1), (-1, -1)], (-1, 0)));
        m.push_back(([(1, 0), (1, 1), (1, -1)], (1, 0)));
        m
    };
}

const ALL_DIRS: &'static [(isize, isize)] = &[
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 1),
];

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> HashSet<Pos> {
    let mut hs = HashSet::new();
    input.lines().enumerate().for_each(|(j, l)| {
        l.as_bytes().iter().enumerate().for_each(|(i, c)| {
            if *c == b'#' {
                _ = hs.insert(Pos(i as isize, j as isize))
            }
        })
    });
    hs
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Pos(isize, isize);

pub fn has_elf<I>(input: &HashSet<Pos>, pos: Pos, mut dirs: I) -> bool
where
    I: Iterator<Item = (isize, isize)>,
{
    dirs.any(|(x, y)| input.contains(&Pos(pos.0 + x, pos.1 + y)))
}

pub fn run(mut input: HashSet<Pos>, n: isize) -> isize {
    let mut moves = MOVES.clone();
    let mut i = 1;

    loop {
        let mut proposed: HashMap<Pos, Pos> = HashMap::new();
        for elf in input.iter() {
            if !has_elf(&input, *elf, ALL_DIRS.iter().copied()) {
                _ = proposed.insert(*elf, *elf);
                continue;
            }

            let len = proposed.len();
            for m in moves.iter() {
                if !has_elf(&input, *elf, m.0.iter().copied()) {
                    let pos = Pos(elf.0 + m.1 .0, elf.1 + m.1 .1);
                    if proposed.contains_key(&pos) {
                        let old = proposed.remove(&pos).unwrap();
                        proposed.insert(old, old);
                        proposed.insert(*elf, *elf);
                    } else {
                        _ = proposed.insert(pos, *elf)
                    }
                    break;
                }
            }
            if len == proposed.len() {
                _ = proposed.insert(*elf, *elf);
            }
        }
        let new_input: HashSet<Pos> = proposed.keys().into_iter().copied().collect();
        if input == new_input {
            return i as isize;
        }
        input = new_input;
        moves.rotate_left(1);
        if n == i {
            break;
        }
        i += 1;
    }

    let max_x = input.iter().map(|p| p.0).max().unwrap();
    let max_y = input.iter().map(|p| p.1).max().unwrap();
    let min_y = input.iter().map(|p| p.1).min().unwrap();
    let min_x = input.iter().map(|p| p.0).min().unwrap();

    (max_x - min_x + 1) * (max_y - min_y + 1) - input.len() as isize
}
#[aoc(day23, part1)]
pub fn part1(input: &HashSet<Pos>) -> isize {
    run(input.clone(), 10)
}
#[aoc(day23, part2)]
pub fn part2(input: &HashSet<Pos>) -> isize {
    run(input.clone(), -1)
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 110)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 20)
    }
}
