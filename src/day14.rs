use std::{cmp::min, collections::HashSet};

use itertools::Itertools;

#[derive(Clone)]
pub struct Map {
    m: HashSet<(usize, usize)>,
    lowest_line: usize,
    start: (usize, usize),
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Map {
    let mut m = HashSet::new();
    let mut lowest = 0 as usize;
    input.lines().for_each(|l| {
        l.split(" -> ")
            .map(|c| {
                let (x, y) = c.split_once(",").unwrap();
                let x = x.parse::<isize>().unwrap();
                let y = y.parse::<isize>().unwrap();
                (x, y)
            })
            .tuple_windows::<((isize, isize), (isize, isize))>()
            .for_each(|(p1, p2)| {
                let min = min(p1.1, p2.1);
                if min as usize > lowest {
                    lowest = min as usize;
                }
                if p1.0 == p2.0 {
                    let dy = (p2.1 - p1.1).signum();
                    let mut ty = p2.1;
                    while ty != p1.1 {
                        m.insert((p1.0 as usize, (ty) as usize));
                        ty -= dy;
                    }
                    m.insert((p1.0 as usize, p1.1 as usize));
                } else if p1.1 == p2.1 {
                    let dx = (p2.0 - p1.0).signum();
                    let mut tx = p2.0;
                    while tx != p1.0 {
                        m.insert(((tx) as usize, p1.1 as usize));
                        tx -= dx;
                    }
                    m.insert((p1.0 as usize, p1.1 as usize));
                } else {
                    unimplemented!()
                }
            });
    });
    Map {
        m,
        lowest_line: lowest,
        start: (500, 0),
    }
}

impl Map {
    pub fn drop_sand(&mut self, floor: bool) -> usize {
        let mut count = 0;
        loop {
            let mut pos = self.start;
            loop {
                pos.1 += 1;
                if let Some(_) = self.get(pos, floor) {
                    pos.0 -= 1;
                    if let Some(_) = self.get(pos, floor) {
                        pos.0 += 2;
                        if let Some(_) = self.get(pos, floor) {
                            let pos = (pos.0 - 1, pos.1 - 1);
                            self.m.insert(pos);
                            count += 1;
                            if pos == self.start {
                                return count;
                            }
                            break;
                        }
                    }
                }
                if !floor && pos.1 == self.lowest_line {
                    return count;
                }
            }
        }
    }

    pub fn get(&self, p: (usize, usize), floor: bool) -> Option<()> {
        if floor && p.1 == self.lowest_line + 2 {
            return Some(());
        }
        return self.m.get(&p).and(Some(()));
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &Map) -> usize {
    input.clone().drop_sand(false)
}

#[aoc(day14, part2)]
pub fn part2(input: &Map) -> usize {
    input.clone().drop_sand(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 24)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 93)
    }
}
