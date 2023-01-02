use adventofcode::read_file;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Rock {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Blueprint {
    index: u32,
    costs: HashMap<Rock, HashMap<Rock, u32>>,
}

impl Blueprint {
    fn max_cost_per_rock(&self, rock: Rock) -> Option<u32> {
        self.costs
            .iter()
            .map(|(_, costs)| {
                costs
                    .iter()
                    .filter_map(|(r, cost)| if *r == rock { Some(*cost) } else { None })
            })
            .flatten()
            .max()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    time: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

fn main() {
    let file = read_file("19").unwrap();
    let blueprints = parse_input(&file);

    let result1 = part1(&blueprints);
    println!("Part1: {result1}");
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input.trim().split('\n').map(|s| {
        let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        let captures = re.captures(s).unwrap();

        Blueprint {
            index: captures.get(1).unwrap().as_str().parse().unwrap(),
            costs: HashMap::from([
                (Rock::Ore, HashMap::from([(Rock::Ore, captures.get(2).unwrap().as_str().parse().unwrap())])),
                (Rock::Clay, HashMap::from([(Rock::Ore, captures.get(3).unwrap().as_str().parse().unwrap())])),
                (Rock::Obsidian, HashMap::from([(Rock::Ore, captures.get(4).unwrap().as_str().parse().unwrap()), (Rock::Clay, captures.get(5).unwrap().as_str().parse().unwrap())])),
                (Rock::Geode, HashMap::from([(Rock::Ore, captures.get(6).unwrap().as_str().parse().unwrap()),(Rock::Obsidian, captures.get(7).unwrap().as_str().parse().unwrap())]))
            ]),
        }
    }).collect()
}

fn part1(blueprints: &Vec<Blueprint>) -> u32 {
    let time = 24;
    blueprints
        .iter()
        .map(|bp| bp.index * run_blueprint(bp, time))
        .sum()
}

fn run_blueprint(bp: &Blueprint, time: u32) -> u32 {
    let robots: HashMap<Rock, u32> = HashMap::from([
        (Rock::Ore, 1),
        (Rock::Clay, 0),
        (Rock::Obsidian, 0),
        (Rock::Geode, 0),
    ]);
    let rocks: HashMap<Rock, u32> = HashMap::from([
        (Rock::Ore, 0),
        (Rock::Clay, 0),
        (Rock::Obsidian, 0),
        (Rock::Geode, 0),
    ]);

    let mut already_tested: HashSet<State> = HashSet::new();
    let result = run_blueprint_inner(bp, &rocks, &robots, time, &mut already_tested);

    println!("bp {} {result}", bp.index);
    result
}

fn run_blueprint_inner(
    bp: &Blueprint,
    rocks: &HashMap<Rock, u32>,
    robots: &HashMap<Rock, u32>,
    time_left: u32,
    already_tested: &mut HashSet<State>,
) -> u32 {
    if time_left == 0 {
        return *rocks.get(&Rock::Geode).unwrap();
    }

    let mut rocks = rocks.clone();

    for rock in &mut rocks {
        if let Some(max_cost) = bp.max_cost_per_rock(*rock.0) {
            // https://aoc.just2good.co.uk/2022/19
            let max_needed = time_left * max_cost - (robots.get(rock.0).unwrap() * (time_left - 1));
            if *rock.1 > max_needed {
                *rock.1 = max_needed;
            }
        }
    }

    let state = State {
        time: time_left,
        ore: *rocks.get(&Rock::Ore).unwrap(),
        clay: *rocks.get(&Rock::Clay).unwrap(),
        obsidian: *rocks.get(&Rock::Obsidian).unwrap(),
        geode: *rocks.get(&Rock::Geode).unwrap(),
        ore_robots: *robots.get(&Rock::Ore).unwrap(),
        clay_robots: *robots.get(&Rock::Clay).unwrap(),
        obsidian_robots: *robots.get(&Rock::Obsidian).unwrap(),
        geode_robots: *robots.get(&Rock::Geode).unwrap(),
    };
    if already_tested.contains(&state) {
        return *rocks.get(&Rock::Geode).unwrap();
    }
    already_tested.insert(state);

    let mut bests = vec![*rocks.get(&Rock::Geode).unwrap()];

    let mut _rocks = rocks.clone();
    for robot in robots {
        _rocks.entry(*robot.0).and_modify(|r| *r += robot.1);
    }

    bests.push(run_blueprint_inner(
        bp,
        &_rocks,
        &robots,
        time_left - 1,
        already_tested,
    ));

    'main_loop: for (robot_type, costs) in &bp.costs {
        if let Some(max_cost) = bp.max_cost_per_rock(*robot_type) {
            if robots.get(robot_type).unwrap() >= &max_cost {
                continue;
            }
        }

        let mut _robots = robots.clone();
        let mut _rocks = rocks.clone();

        for (_rock, _cost) in costs {
            if _rocks.get(_rock).unwrap() < _cost {
                continue 'main_loop;
            }
            _rocks.entry(*_rock).and_modify(|r| *r -= _cost);
        }

        for robot in &_robots {
            _rocks.entry(*robot.0).and_modify(|r| *r += robot.1);
        }
        _robots.entry(*robot_type).and_modify(|r| *r += 1);
        bests.push(run_blueprint_inner(
            bp,
            &_rocks,
            &_robots,
            time_left - 1,
            already_tested,
        ));
    }

    *bests.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_part1() {
        let blueprints = parse_input(example);
        assert_eq!(part1(&blueprints), 33);
    }
}
