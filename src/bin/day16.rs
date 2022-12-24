use adventofcode::read_file;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const example: &str = "
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow: u64,
    neighbors: Vec<String>,
}

type Distances = HashMap<String, u64>;

impl Valve {
    fn get_neighbors(&self) -> Vec<String> {
        self.neighbors.clone()
    }
}

fn main() {
    let file = read_file("16").unwrap();
    let valves = parse_input(&file);

    let result1 = part1(&valves);
    println!("Part1: {result1}");
}

fn parse_input(input: &str) -> HashMap<String, Valve> {
    input
        .trim()
        .split('\n')
        .map(|s| {
            let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); (tunnels lead to valves|tunnel leads to valve) (.*)").unwrap();
            let captures = re.captures(s).unwrap();

            let neighbors = captures
                .get(4)
                .unwrap()
                .as_str()
                .split(", ").map(|s| s.to_string())
                .collect::<Vec<String>>();

            let name = captures.get(1).unwrap().as_str().to_string();
            (name.clone(), Valve {
                name,
                flow: captures.get(2).unwrap().as_str().parse().unwrap(),
                neighbors
            })
        })
        .collect()
}

fn part1(valves: &HashMap<String, Valve>) -> u64 {
    // let mut distances: HashMap<String, Distances> = HashMap::new();
    // for valve in valves.keys() {
    //     let mut distance: Distances = HashMap::new();
    //     get_distances(valves, valve, 0, &mut distance, &HashSet::new());
    //     distances.insert(valve.to_owned(), distance);
    // }

    let distances = get_distances2(valves);

    println!("start travel");
    travel(
        &valves,
        valves.get(&"AA".to_string()).unwrap(),
        &distances,
        HashSet::new(),
        30,
        0,
    )
}

fn travel(
    valves: &HashMap<String, Valve>,
    current: &Valve,
    _distances: &HashMap<String, Distances>,
    opened: HashSet<String>,
    time_left: u64,
    flow: u64,
) -> u64 {
    if time_left <= 0 || opened.len() == valves.len() {
        return flow;
    }

    let distances = _distances.get(&current.name).unwrap();
    let current_flow = flow + current.flow * time_left;

    let mut max_flow = current_flow;
    for valve in valves.values() {
        if valve.name != current.name && !opened.contains(&valve.name) && valve.flow > 0 {
            let mut _opened = opened.clone();
            _opened.insert(current.name.clone());

            let distance = distances.get(&valve.name).unwrap();

            if time_left > *distance {
                let new_flow = travel(
                    valves,
                    valve,
                    _distances,
                    _opened,
                    time_left - distance - 1,
                    current_flow,
                );
                if new_flow > max_flow {
                    max_flow = new_flow;
                }
            }
        }
    }
    max_flow
}

fn get_distances(
    valves: &HashMap<String, Valve>,
    start: &String,
    depth: u64,
    distances: &mut Distances,
    visited: &HashSet<String>,
) {
    if visited.contains(&start.to_string()) {
        return;
    }

    distances
        .entry(start.clone())
        .and_modify(|e| {
            if *e > depth {
                *e = depth;
            }
        })
        .or_insert(depth);

    let mut visited = visited.clone();
    visited.insert(start.clone());

    for neighbor in valves.get(start).unwrap().get_neighbors() {
        get_distances(valves, &neighbor, depth + 1, distances, &visited);
    }
}

// this is "inspired" (ok... copied) from: https://github.com/mebeim/aoc/tree/master/2022#day-16---proboscidea-volcanium
// get_distances() works but is much slower
fn get_distances2(valves: &HashMap<String, Valve>) -> HashMap<String, Distances> {
    let mut distances: HashMap<String, Distances> = HashMap::new();

    for (key, value) in valves {
        for b_name in &value.get_neighbors() {
            let a = distances.entry(key.clone()).or_insert(HashMap::new());
            a.insert(key.clone(), 0);
            a.insert(b_name.clone(), 1);

            distances
                .entry(b_name.clone())
                .or_insert(HashMap::new())
                .insert(b_name.clone(), 0);
        }
    }

    for (a, b, c) in product(&valves.keys().cloned().collect()) {
        let bc = distances
            .get(&b)
            .unwrap()
            .get(&c)
            .unwrap_or(&100_000)
            .clone();
        let ba = distances
            .get(&b)
            .unwrap()
            .get(&a)
            .unwrap_or(&100_000)
            .clone();
        let ac = distances
            .get(&a)
            .unwrap()
            .get(&c)
            .unwrap_or(&100_000)
            .clone();

        if ba + ac < bc {
            distances.entry(b).and_modify(|x| {
                x.insert(c, ba + ac);
            });
        }
    }

    distances
}

fn product(keys: &Vec<String>) -> Vec<(String, String, String)> {
    let mut res: Vec<(String, String, String)> = vec![];
    for k1 in keys.clone() {
        for k2 in keys.clone() {
            for k3 in keys.clone() {
                res.push((k1.clone(), k2.clone(), k3.clone()));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_part1() {
        let valves = parse_input(example);
        assert_eq!(part1(&valves), 1651);
    }
}
