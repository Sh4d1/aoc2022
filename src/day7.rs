use std::collections::{HashMap, HashSet};

type Output = HashMap<String, usize>;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Output {
    let mut hm = HashMap::new();
    let mut current_dir = Vec::new();
    for l in input.split("$").skip(1).map(|l| l.trim()) {
        match l.lines().next().unwrap() {
            "cd /" => {
                current_dir.push("");
            }
            "cd .." => {
                current_dir.pop();
            }
            c if c.starts_with("cd ") => {
                current_dir.push(c.split_once(' ').unwrap().1);
            }
            "ls" => {
                hm.entry(current_dir.join("/").clone())
                    .or_insert((0, HashSet::new()))
                    .1
                    .extend(l.lines().skip(1).map(|output| {
                        let (s, n) = output.split_once(' ').unwrap();
                        (n, s.parse::<usize>().unwrap_or(0))
                    }));
            }
            _ => unreachable!(),
        }
    }

    let h = hm.clone();
    let mut keys = h.keys().collect::<Vec<_>>();
    keys.sort_by_key(|dir| dir.matches("/").count());
    keys.reverse();
    keys.iter()
        .map(|k| {
            let size: usize = hm[*k]
                .1
                .iter()
                .map(|(n, s)| match s {
                    0 => hm[&format!("{}/{}", k, n)].0,
                    s => *s,
                })
                .sum();
            hm.get_mut(*k).unwrap().0 = size;
            ((*k).clone(), size)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &Output) -> usize {
    input.values().filter(|&&s| s <= 100000).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &Output) -> usize {
    input
        .values()
        .filter(|&&s| s + 40000000 >= input[""])
        .min()
        .copied()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 95437)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 24933642)
    }
}
