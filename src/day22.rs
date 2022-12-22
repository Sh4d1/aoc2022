use std::collections::HashMap;

use lazy_static::lazy_static;
use num_derive::FromPrimitive;
use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"((\d+|[LR])+?)").unwrap();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Inst {
    Right,
    Left,
    Move(usize),
}
use Inst::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive, Hash)]
pub enum Dir {
    R = 0,
    D = 1,
    L = 2,
    U = 3,
}
use Dir::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Pos(usize, usize, Dir);

impl Pos {
    pub fn left(&self) -> Pos {
        Pos(
            self.0,
            self.1,
            num::FromPrimitive::from_u32((self.2 as u32).overflowing_sub(1).0 % 4).unwrap(),
        )
    }
    pub fn right(&self) -> Pos {
        Pos(
            self.0,
            self.1,
            num::FromPrimitive::from_u32((self.2 as u32 + 1) % 4).unwrap(),
        )
    }
    pub fn score(&self) -> usize {
        1000 * (self.1 + 1) + 4 * (self.0 + 1) + self.2 as usize
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Turn {
    Left,
    Right,
    NoTurn,
}

#[derive(Debug, Clone)]
pub struct Walker {
    map: Vec<Vec<char>>,
    inst: Vec<Inst>,
    cube_transitions: HashMap<Pos, Pos>,
}

impl Walker {
    pub fn get(&self, p: Pos) -> char {
        self.map[p.1][p.0]
    }

    pub fn gen_cube_transitions(&mut self, mut pos: Pos) {
        let start = pos;
        let mut fold = false;
        let mut unknwon_turns = vec![Turn::Right];
        let mut unknwon_pos = Vec::new();
        let max_x = self.map.iter().map(|l| l.len()).max().unwrap();
        let size = num::integer::gcd(self.map.len(), max_x);
        self.map.iter_mut().for_each(|l| l.resize(max_x + 1, ' '));
        self.map.push(vec![' '; max_x + 1]);

        loop {
            // let's walk until the next corner
            for _ in 0..size {
                if !fold {
                    unknwon_pos.push(pos);
                    pos = self.walk_one(pos);
                } else {
                    // let's fold it by adding the transitions
                    let p = unknwon_pos.pop().unwrap();
                    self.cube_transitions.insert(p.left(), pos.right());
                    self.cube_transitions.insert(pos.left(), p.right());
                    pos = self.walk_one(pos);
                }
            }

            let turn = match (self.get(pos), self.get(self.walk_one(pos.left()))) {
                // if there is no face on the left, and we are not on a face, it's a right turn
                (' ', ' ') => Turn::Right,
                // if we currently are walking on a face, and there is othing on the left, it's a
                // straight line
                (c, ' ') if c != ' ' => Turn::NoTurn,
                // else it's a left turn
                _ => Turn::Left,
            };

            if !fold {
                if turn == Turn::Left {
                    // always fold on the left
                    fold = true;
                } else {
                    unknwon_turns.push(turn);
                }
            } else {
                if let Some(last_turn) = unknwon_turns.pop() {
                    // two right after after fold (so a left) can be sum'd up to a line,
                    // ie no turn and we can stop folding
                    if last_turn == Turn::Right && turn == Turn::Right {
                        unknwon_turns.push(Turn::NoTurn);
                        fold = false;
                    }
                }
                // if we folded everything, we can wait for a new left turn
                if unknwon_turns.len() == 0 {
                    unknwon_turns.push(turn);
                    fold = false;
                }
            }
            // let's get back on the right position
            match turn {
                Turn::Left => pos = self.walk_one(pos.left()),
                Turn::Right => pos = self.walk_one(pos.right().right()).left(),
                Turn::NoTurn => (),
            }
            // every border pos have a transition
            if pos == start {
                break;
            }
        }
    }

    pub fn walk_cube(&self, pos: Pos) -> Pos {
        self.cube_transitions
            .get(&pos)
            .copied()
            .unwrap_or(self.walk_one(pos))
    }

    pub fn walk_one(&self, mut pos: Pos) -> Pos {
        match pos.2 {
            L => pos.0 = (pos.0.overflowing_sub(1).0).min(self.map[0].len() - 1),
            R => pos.0 = (pos.0 + 1) % self.map[0].len(),
            U => pos.1 = (pos.1.overflowing_sub(1).0).min(self.map.len() - 1),
            D => pos.1 = (pos.1 + 1) % self.map.len(),
        };
        pos
    }

    pub fn walk_flat(&self, mut pos: Pos) -> Pos {
        loop {
            pos = self.walk_one(pos);

            if self
                .map
                .get(pos.1)
                .is_some_and(|line| line.get(pos.0).is_some_and(|&c| c != ' '))
            {
                return pos;
            }
        }
    }

    pub fn walk(&mut self, cube: bool) -> usize {
        let mut pos = Pos(self.map[0].iter().position(|&c| c != ' ').unwrap(), 0, R);

        if cube {
            self.gen_cube_transitions(pos);
        }

        for i in self.inst.iter().copied() {
            pos = match i {
                Move(n) => {
                    let mut old_pos = pos;
                    for _ in 0..n {
                        let p = if !cube {
                            self.walk_flat(old_pos)
                        } else {
                            self.walk_cube(old_pos)
                        };
                        if self.get(p) == '#' {
                            break;
                        }
                        old_pos = p;
                    }
                    old_pos
                }
                Right => pos.right(),
                Left => pos.left(),
            };
        }
        pos.score()
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Walker {
    let (input_map, inst_input) = input.split_once("\n\n").unwrap();
    let map = input_map.lines().fold(Vec::new(), |mut acc, l| {
        acc.push(l.as_bytes().iter().map(|c| *c as char).collect());
        acc
    });

    let inst = REG
        .captures_iter(inst_input)
        .map(|c| match &c[0] {
            "R" => Right,
            "L" => Left,
            n => Move(n.parse::<usize>().unwrap()),
        })
        .collect();
    Walker {
        map,
        inst,
        cube_transitions: HashMap::new(),
    }
}

#[aoc(day22, part1)]
pub fn part1(input: &Walker) -> usize {
    input.clone().walk(false)
}

#[aoc(day22, part2)]
pub fn part2(input: &Walker) -> usize {
    input.clone().walk(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 6032)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 5031)
    }
}
