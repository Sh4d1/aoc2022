#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Blizzard {
    Left,
    Right,
    Up,
    Down,
}
use std::collections::{HashMap, HashSet, VecDeque};

use Blizzard::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Blizzards(Vec<Blizzard>),
}
use Tile::*;

impl Tile {
    pub fn add(&mut self, b: Blizzard) {
        match self {
            Wall => unreachable!(),
            Blizzards(bs) => bs.push(b),
            Empty => *self = Blizzards(vec![b]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    h: usize,
    w: usize,
    pos: (usize, usize),
    end: (usize, usize),
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Map {
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| match *c as char {
                    '#' => Wall,
                    '.' => Empty,
                    '>' => Blizzards(vec![Right]),
                    '<' => Blizzards(vec![Left]),
                    '^' => Blizzards(vec![Up]),
                    'v' => Blizzards(vec![Down]),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let h = tiles.len();
    let w = tiles[0].len();
    let pos = (tiles[0].iter().position(|t| t == &Empty).unwrap(), 0);
    let end = (
        tiles[h - 1].iter().position(|t| t == &Empty).unwrap(),
        h - 1,
    );
    Map {
        tiles,
        h,
        w,
        pos,
        end,
    }
}

impl Map {
    pub fn tiles_cleaned(&self) -> Vec<Vec<Tile>> {
        self.tiles
            .iter()
            .map(|l| {
                l.iter()
                    .map(|t| match t {
                        Blizzards(_) | Empty => Empty,
                        Wall => Wall,
                    })
                    .collect()
            })
            .collect()
    }

    pub fn get_blizzard_next_pos(&self, mut pos: (usize, usize), b: Blizzard) -> (usize, usize) {
        match b {
            Left => pos.0 -= 1,
            Right => pos.0 += 1,
            Up => pos.1 -= 1,
            Down => pos.1 += 1,
        }
        match self.tiles[pos.1][pos.0] {
            Wall => match b {
                Left => pos.0 = self.w - 2,
                Right => pos.0 = 1,
                Up => pos.1 = self.h - 2,
                Down => pos.1 = 1,
            },
            _ => (),
        };
        pos
    }

    pub fn move_blizzards(&mut self) {
        let mut new_tiles = self.tiles_cleaned();
        for j in 1..self.h - 1 {
            for i in 1..self.w - 1 {
                match &self.tiles[j][i] {
                    Empty => (),
                    Wall => unreachable!(),
                    Blizzards(bs) => {
                        for k in 0..bs.len() {
                            let new_pos = self.get_blizzard_next_pos((i, j), bs[k]);
                            new_tiles[new_pos.1][new_pos.0].add(bs[k]);
                        }
                    }
                }
            }
        }
        self.tiles = new_tiles;
    }
}

pub fn bfs(map: Map) -> (Map, usize) {
    let mut maps = HashMap::new();
    maps.insert(0, map.clone());
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((map.pos, 0));
    while let Some((pos, t)) = queue.pop_front() {
        if pos == map.end {
            return (maps.get(&t).unwrap().clone(), t);
        }
        if !visited.insert((pos, t)) {
            continue;
        }
        if !maps.contains_key(&(t + 1)) {
            let mut new_map = maps.get(&t).unwrap().clone();
            new_map.move_blizzards();
            maps.insert(t + 1, new_map);
        }
        let current_map = maps.get(&(t + 1)).clone().unwrap();

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_pos = (
                (pos.0 as isize + dx).max(0).min(map.w as isize - 1) as usize,
                (pos.1 as isize + dy).max(0).min(map.h as isize - 1) as usize,
            );
            if let Empty = current_map.tiles[new_pos.1][new_pos.0] {
                queue.push_back((new_pos, t + 1));
            }
        }
        if let Empty = current_map.tiles[pos.1][pos.0] {
            queue.push_back((pos, t + 1));
        }
    }
    unreachable!()
}

#[aoc(day24, part1)]
pub fn part1(input: &Map) -> usize {
    bfs(input.clone()).1
}
#[aoc(day24, part2)]
pub fn part2(input: &Map) -> usize {
    let (mut map, d1) = bfs(input.clone());
    std::mem::swap(&mut map.end, &mut map.pos);
    let (mut map, d2) = bfs(map);
    std::mem::swap(&mut map.end, &mut map.pos);
    let (_, d3) = bfs(map);
    d1 + d2 + d3
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 18)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 54)
    }
}
