use lazy_static::lazy_static;

const W: usize = 7;
const HEIGHT_BUF: usize = 2000;

lazy_static! {
    static ref ROCKS: Vec<Rock> = {
        let mut r = Vec::new();
        r.push(Rock(vec![(0, 0), (1, 0), (2, 0), (3, 0)]));
        r.push(Rock(vec![(1, 0), (0, 1), (1, 1), (1, 2), (2, 1)]));
        r.push(Rock(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]));
        r.push(Rock(vec![(0, 0), (0, 1), (0, 2), (0, 3)]));
        r.push(Rock(vec![(0, 0), (0, 1), (1, 0), (1, 1)]));
        r
    };
}

#[derive(Debug, Clone)]
pub struct Rock(Vec<(usize, usize)>);

impl Rock {
    pub fn height(&self) -> usize {
        self.0.iter().map(|&(_, j)| j).max().unwrap() + 1
    }
    pub fn width(&self) -> usize {
        self.0.iter().map(|&(i, _)| i).max().unwrap() + 1
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
}
use Direction::*;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Game {
    let jets = input
        .as_bytes()
        .iter()
        .map(|c| match c {
            b'<' => Left,
            b'>' => Right,
            _ => unreachable!(),
        })
        .collect();
    let map = vec![vec![0u8; HEIGHT_BUF]; W];
    Game { jets, map }
}

#[derive(Debug, Clone)]
pub struct Game {
    jets: Vec<Direction>,
    map: Vec<Vec<u8>>,
}

impl Game {
    pub fn get_cell(&mut self, i: usize, j: usize) -> u8 {
        if let Some(&v) = self.map[i].get(j) {
            v
        } else {
            let new_len = self.map[i].len() * 2;
            self.map.get_mut(i).unwrap().resize(new_len, 0);
            0
        }
    }

    pub fn can_move_rock(&mut self, rock: usize, x: usize, y: usize) -> bool {
        ROCKS[rock]
            .0
            .iter()
            .all(|&(i, j)| self.get_cell(x + i, y + j) == 0)
    }

    pub fn play_tetris(&mut self, n: usize, all: bool) -> usize {
        let jets = self.jets.clone();
        let mut jets = jets.iter().enumerate().cycle();

        let mut states = vec![vec![(0, 0); self.jets.len()]; ROCKS.len()];
        let mut h = 0;

        for (i, rock) in (0..n).zip((0..ROCKS.len()).cycle()) {
            let (mut x, mut y) = (2, h + 3);

            let jet = loop {
                let (jet_i, jet) = jets.next().unwrap();
                match jet {
                    Left if x > 0 && self.can_move_rock(rock, x - 1, y) => x -= 1,
                    Right if x + ROCKS[rock].width() < W && self.can_move_rock(rock, x + 1, y) => {
                        x += 1
                    }
                    _ => (),
                }
                if y == 0 || !self.can_move_rock(rock, x, y - 1) {
                    break jet_i;
                }
                y -= 1;
            };

            // yeah just to allocate the vec if needed
            if self.can_move_rock(rock, x, y) {
                ROCKS[rock]
                    .0
                    .iter()
                    .for_each(|&(i, j)| self.map[i + x][j + y] = 1);
            }

            h = std::cmp::max(h, y + ROCKS[rock].height());

            if i == n - 1 {
                return h;
            }

            if !all {
                let state = states[rock][jet];
                if state == (0, 0) {
                    states[rock][jet] = (i, h)
                } else {
                    let p = ((n - i) / (i - state.0), (n - i) % (i - state.0));
                    if p.1 == 0 && !all {
                        return h + p.0 * (h - state.1) - 1;
                    }
                }
            }
        }
        unreachable!();
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &Game) -> usize {
    input.clone().play_tetris(2022, true)
}

#[aoc(day17, part2)]
pub fn part2(input: &Game) -> usize {
    input.clone().play_tetris(1000000000000, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 3068)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 1514285714288)
    }
}
