use std::collections::{HashMap, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref R: Regex = Regex::new(
        r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.*)"
    )
    .unwrap();
}

#[derive(Debug, Clone)]
pub struct Valve {
    name: String,
    rate: usize,
    tunnels: HashMap<String, usize>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> HashMap<String, Valve> {
    input
        .lines()
        .map(|l| {
            let l = R.captures(l).unwrap();

            (
                l[1].to_string(),
                Valve {
                    name: l[1].to_string(),
                    rate: l[2].parse().unwrap(),
                    tunnels: l[3].trim().split(", ").fold(HashMap::new(), |mut acc, s| {
                        acc.insert(s.to_string(), 1);
                        acc
                    }),
                },
            )
        })
        .collect()
}

pub fn dijkstra(graph: &HashMap<String, Valve>, start: String, end: String) -> Option<usize> {
    // let mut dist = vec![std::usize::MAX; graph.len()];
    let mut dist = HashMap::new();
    let mut pq = std::collections::BinaryHeap::new();
    dist.insert(start.clone(), 0);
    pq.push(std::cmp::Reverse((0, start.clone())));
    while let Some(std::cmp::Reverse((d_u, u))) = pq.pop() {
        if d_u > *dist.get(&u).unwrap_or(&std::usize::MAX) {
            continue;
        }
        for (v, w) in &graph.get(&u).unwrap().tunnels {
            let d_v = d_u + *w;
            if d_v < *dist.get(v).unwrap_or(&std::usize::MAX) {
                if dist.contains_key(v) {
                    *dist.get_mut(v).unwrap() = d_v;
                } else {
                    dist.insert(v.to_string(), d_v);
                }
                pq.push(std::cmp::Reverse((d_v, v.clone())));
            }
        }
    }
    dist.get(&end).copied()
}

pub fn walk_empty(
    hm: &HashMap<String, Valve>,
    start: &String,
    visited: &mut Vec<String>,
) -> HashMap<String, usize> {
    if visited.contains(start) {
        return HashMap::new();
    }
    visited.push(start.clone());
    let mut res = HashMap::<String, usize>::new();
    for t in hm.get(start).unwrap().tunnels.clone() {
        let ti = hm.get(&t.0).unwrap();
        if ti.rate == 0 && ti.name != "AA" {
            for n in walk_empty(hm, &ti.name, visited) {
                let mut delta = 1;
                // if n.0 == "AA" {
                //     delta = 0;
                // }
                if let Some(mut v) = res.get_mut(&n.0) {
                    if *v > n.1 + delta {
                        *v = n.1 + delta;
                    }
                } else {
                    res.insert(n.0, n.1 + delta);
                };
            }
        } else {
            res.insert(ti.name.clone(), 1);
        }
    }
    res
}

pub fn simplify(hm: &mut HashMap<String, Valve>) {
    let view = hm.clone();
    // let start_tunnels = walk_empty(hm, &"AA".to_owned(), &mut Vec::new());
    // let start = hm.get_mut("AA").unwrap();
    // start.tunnels = start_tunnels;
    for k in view.keys() {
        let v = hm.get(k).unwrap();
        if v.rate == 0 && v.name != "AA" {
            continue;
        }
        let mut new_tunnels = HashMap::new();
        for (k1, v) in walk_empty(hm, &k, &mut Vec::new()) {
            if k1 == *k {
                continue;
            }
            if new_tunnels.contains_key(&k1) {
                if *new_tunnels.get(&k1).unwrap() > v {
                    *new_tunnels.get_mut(&k1).unwrap() = v;
                }
            } else {
                new_tunnels.insert(k1, v);
            }
        }
        let mut v = hm.get_mut(k).unwrap();
        v.tunnels = new_tunnels;
    }
    let view = hm.clone();
    for k in view.keys() {
        let v = hm.get(k).unwrap();
        if v.rate == 0 && v.name != "AA" {
            hm.remove(k).unwrap();
        }
    }
    let view = hm.clone();
    for k in view.keys() {
        let v = hm.get_mut(k).unwrap();
        v.tunnels = HashMap::new();
        for k2 in view.keys() {
            if k2 != k {
                if let Some(d) = dijkstra(&view, v.name.clone(), k2.clone()) {
                    v.tunnels.insert(k2.to_string(), d);
                };
            }
        }
    }
    dbg!(hm.clone());
}

pub fn walk(hm: &HashMap<String, Valve>, n: usize) -> usize {
    let size = hm.iter().filter(|(k, v)| v.rate != 0).count();
    let mut queue = VecDeque::new();
    let mut res = 0;
    queue.push_back(("AA".to_string(), 1, vec![], 0, 0));
    while let Some((name, i, opened, pres, total)) = queue.pop_front() {
        if i >= n || opened.len() == size {
            if i <= n {
                // println!(
                //     "got {} at minute {} with opens: {:?} and pression {} total {} total2 {}",
                //     name,
                //     i,
                //     opened.clone(),
                //     pres,
                //     total,
                //     total + (n - i) * pres,
                // );

                if total + (n - i) * pres > res {
                    res = total + (n - i) * pres;
                }
            }
            continue;
        }
        let current = hm.get(&name).unwrap().clone();
        for (t_name, t_w) in current.tunnels.clone() {
            if i + t_w > n {
                continue;
            }
            let t_name = t_name.clone();
            if !opened.contains(&t_name) {
                let mut no = opened.clone();
                no.push(t_name.clone());
                queue.push_back((
                    t_name.clone(),
                    i + t_w + 1,
                    no,
                    pres + hm.get(&t_name).unwrap().rate,
                    total + hm.get(&t_name).unwrap().rate + (t_w + 1) * pres,
                ));
            }
        }
    }
    res
}

#[aoc(day16, part1)]
pub fn part1(input: &HashMap<String, Valve>) -> usize {
    let mut input = input.clone();
    simplify(&mut input);
    walk(&input, 30)
}
#[aoc(day16, part2)]
pub fn part2(input: &HashMap<String, Valve>) -> usize {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator(INPUT)), 0)
    }
    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator(INPUT)), 0)
    }
}
