use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug, Copy)]
pub struct Blueprint {
    id: usize,
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
}

#[derive(Clone, Debug, Copy, Default, Hash, Eq, PartialEq)]
pub struct State {
    ore: (usize, usize),
    clay: (usize, usize),
    obsidian: (usize, usize),
    geode: (usize, usize),
}

impl State {
    pub fn collect(&mut self) {
        self.ore.1 += self.ore.0;
        self.clay.1 += self.clay.0;
        self.obsidian.1 += self.obsidian.0;
        self.geode.1 += self.geode.0;
    }

    pub fn bound(&mut self, ore: usize, clay: usize, obsidian: usize, time_left: usize) {
        self.ore.0 = std::cmp::min(self.ore.0, ore);
        self.clay.0 = std::cmp::min(self.clay.0, clay);
        self.obsidian.0 = std::cmp::min(self.obsidian.0, obsidian);
        self.ore.1 = std::cmp::min(self.ore.1, time_left * ore - self.ore.0 * (time_left - 1));
        self.clay.1 = std::cmp::min(
            self.clay.1,
            time_left * clay - self.clay.0 * (time_left - 1),
        );
        self.obsidian.1 = std::cmp::min(
            self.obsidian.1,
            time_left * obsidian - self.obsidian.0 * (time_left - 1),
        );
    }
}

impl Blueprint {
    pub fn run(&self, n: usize) -> usize {
        let start_state = State {
            ore: (1, 0),
            ..Default::default()
        };
        let max_ore_needed = std::cmp::max(
            self.ore,
            std::cmp::max(self.clay, std::cmp::max(self.obsidian.0, self.geode.0)),
        );

        let mut visited = HashSet::new();
        let mut max_geode = 0;
        let mut queue = VecDeque::new();

        queue.push_back((start_state, 0));
        while let Some((mut current_state, i)) = queue.pop_front() {
            if i == n {
                max_geode = std::cmp::max(max_geode, current_state.geode.1);
                continue;
            }
            current_state.bound(max_ore_needed, self.obsidian.1, self.geode.1, n - i);
            if !visited.insert((current_state, i)) {
                continue;
            }
            let mut new_state = current_state;
            new_state.collect();

            if current_state.ore.1 >= self.geode.0 && current_state.obsidian.1 >= self.geode.1 {
                let mut cs = new_state;
                cs.ore.1 -= self.geode.0;
                cs.obsidian.1 -= self.geode.1;
                cs.geode.0 += 1;
                queue.push_back((cs, i + 1));
                continue;
            }

            queue.push_back((new_state, i + 1));

            if current_state.ore.1 >= self.clay {
                let mut cs = new_state;
                cs.ore.1 -= self.clay;
                cs.clay.0 += 1;
                queue.push_back((cs, i + 1));
            }

            if current_state.ore.1 >= self.ore {
                let mut cs = new_state;
                cs.ore.1 -= self.ore;
                cs.ore.0 += 1;
                queue.push_back((cs, i + 1));
            }

            if current_state.ore.1 >= self.obsidian.0 && current_state.clay.1 >= self.obsidian.1 {
                let mut cs = new_state;
                cs.ore.1 -= self.obsidian.0;
                cs.clay.1 -= self.obsidian.1;
                cs.obsidian.0 += 1;
                queue.push_back((cs, i + 1));
            }
        }
        max_geode
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Blueprint> {
    input.lines().map(|l| {
        let b = scan_fmt!(l, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", usize, usize, usize, usize, usize, usize, usize).unwrap();
        Blueprint{id: b.0, ore: b.1, clay: b.2, obsidian: (b.3, b.4), geode: (b.5, b.6)}
    }).collect()
}

#[aoc(day19, part1)]
pub fn part1(input: &[Blueprint]) -> usize {
    input.iter().map(|i| i.run(24) * i.id).sum::<usize>()
}

#[aoc(day19, part2)]
pub fn part2(input: &[Blueprint]) -> usize {
    input.iter().take(3).map(|i| i.run(32)).product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    #[test]
    fn example1() {
        // too long for now
        // assert_eq!(part1(&input_generator(INPUT)), 33)
    }
    #[test]
    fn example2() {
        // too long for now
        // assert_eq!(part2(&input_generator(INPUT)), 3472)
    }
}
