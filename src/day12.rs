use std::collections::VecDeque;

use itertools::Itertools;

pub struct Map {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
}

impl Map {
    fn get_height(&self, pos: (usize, usize)) -> Option<u8> {
        self.map.get(pos.0).and_then(|r| {
            r.get(pos.1).and_then(|c| {
                Some(match c {
                    b'S' => b'a',
                    b'E' => b'z',
                    c => *c,
                })
            })
        })
    }

    pub fn reverse_bfs(&self, end: u8) -> usize {
        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
        let mut queue = VecDeque::new();
        queue.push_back((self.start, 0));
        while let Some((pos, d)) = queue.pop_front() {
            if self.map[pos.0][pos.1] == end {
                return d;
            }
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let new_pos = (
                    (pos.0 as isize + dx) as usize,
                    (pos.1 as isize + dy) as usize,
                );
                if let Some(h) = self.get_height(new_pos) {
                    let v = &mut visited[new_pos.0][new_pos.1];
                    if self.get_height(pos).unwrap() - 1 <= h && !*v {
                        *v = true;
                        queue.push_back((new_pos, d + 1));
                    }
                }
            }
        }
        unreachable!()
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Map {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().iter().copied().collect())
        .collect();

    let start = (0..map.len())
        .cartesian_product(0..map[0].len())
        .find(|&(x, y)| map[x][y] == b'E')
        .unwrap();

    Map { map, start }
}

#[aoc(day12, part1)]
pub fn part1(map: &Map) -> usize {
    map.reverse_bfs(b'S')
}

#[aoc(day12, part2)]
pub fn part2(map: &Map) -> usize {
    map.reverse_bfs(b'a')
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 31)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 29)
    }
}
