use adventofcode::read_file;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct Valve {
    flow: u64,
    neighbors: Vec<String>,
}

type Distances = HashMap<String, u64>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Solution(HashSet<String>);
impl Hash for Solution {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut a: Vec<&String> = self.0.iter().collect();
        a.sort();
        for s in a.iter() {
            s.hash(state);
        }
    }
}

impl Valve {
    fn get_neighbors(&self) -> Vec<String> {
        self.neighbors.clone()
    }
}

fn main() {
    let file = read_file("16").unwrap();
    let valves = parse_input(&file);

    let result2 = part2(&valves);
    println!("Part2: {result2}");
}

fn part2(valves: &HashMap<String, Valve>) -> u64 {
    let distances = get_distances(valves);

    println!("start travel");
    let mut solutions: Vec<HashMap<String, u64>> = vec![];
    travel(
        &valves
            .iter()
            .filter(|v| v.1.flow > 0)
            .map(|v| v.0.clone())
            .collect(),
        &"AA".to_string(),
        &distances,
        26,
        &HashMap::new(),
        &mut solutions,
    );
    println!("done travel {}", solutions.len());

    let mut unique_solutions: HashMap<Solution, u64> = HashMap::new();

    for solution in solutions {
        let key: Solution = Solution(
            solution
                .keys()
                .into_iter()
                .map(|k| k.clone())
                .collect::<HashSet<String>>(),
        );
        let flow: u64 = solution
            .iter()
            .fold(0, |acc, i| acc + (i.1 * valves.get(i.0).unwrap().flow));

        unique_solutions
            .entry(key)
            .and_modify(|x| {
                if flow > *x {
                    *x = flow;
                }
            })
            .or_insert(flow);
    }

    println!("unique_solutions: {}", unique_solutions.len());

    let mut result = 0;
    for (k1, flow1) in &unique_solutions {
        for (k2, flow2) in &unique_solutions {
            if k1.0.is_disjoint(&k2.0) {
                if flow1 + flow2 > result {
                    result = flow1 + flow2;
                }
            }
        }
    }

    result
}

fn travel(
    valves: &Vec<String>,
    current: &String,
    _distances: &HashMap<String, Distances>,
    time_left: u64,
    path: &HashMap<String, u64>,
    solutions: &mut Vec<HashMap<String, u64>>,
) -> HashMap<String, u64> {
    let mut valves = valves.clone();

    if let Some(index) = valves.iter().position(|v| v == current) {
        valves.remove(index);
    }

    let distances = _distances.get(current).unwrap();

    for valve in &valves {
        let distance = distances.get(valve).unwrap();
        if valve == current || time_left <= *distance {
            continue;
        }

        let new_time = time_left - distance - 1;
        let mut path = path.clone();
        path.insert(valve.clone(), new_time);

        let _path = travel(&valves, &valve, _distances, new_time, &path, solutions);
        solutions.push(_path);
    }

    return path.clone();
}

fn get_distances(valves: &HashMap<String, Valve>) -> HashMap<String, Distances> {
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
                flow: captures.get(2).unwrap().as_str().parse().unwrap(),
                neighbors
            })
        })
        .collect()
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
    fn test_part2() {
        let valves = parse_input(example);
        assert_eq!(part2(&valves), 1707);
    }
}
