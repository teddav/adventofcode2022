use adventofcode::read_file;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Instant;
use std::{mem, thread};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Rock {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct StateWithPreviousUnbuiltRobots {
    state: State,
    not_built: Vec<Rock>,
}

fn main() {
    let file = read_file("19").unwrap();
    let blueprints = parse_input(&file);

    let now = Instant::now();
    let result1 = part1(&blueprints);
    println!("Part1: {result1}");
    println!("Time: {:.2?}", now.elapsed());

    let now = Instant::now();
    let result2 = part2(&blueprints);
    println!("Part2: {result2}");
    println!("Time: {:.2?}", now.elapsed());
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
    // blueprints
    //     .iter()
    //     .map(|bp| bp.index * run_blueprint(bp, time))
    //     .sum()
    let (sender, receiver) = mpsc::channel();
    for bp in blueprints.clone() {
        let sender = sender.clone();
        thread::spawn(move || sender.send(bp.index * run_blueprint(&bp, time)).unwrap());
    }
    mem::drop(sender);
    receiver.iter().sum()
}

/*
For Part 2, used optimisations from:
https://aoc.just2good.co.uk/2022/19
https://github.com/mebeim/aoc/blob/master/2022/README.md#day-19---not-enough-minerals
*/
fn part2(blueprints: &Vec<Blueprint>) -> u32 {
    let time = 32;
    // blueprints
    //     .iter()
    //     .filter(|bp| bp.index <= 3)
    //     .map(|bp| {
    //         let res = run_blueprint(bp, time);
    //         println!("{} -> {}", bp.index, res);
    //         res
    //     })
    //     .fold(1, |acc, v| acc * v)

    let result = Arc::new(Mutex::new(1));
    let mut handles = vec![];
    for bp in blueprints.iter().filter(|bp| bp.index <= 3).cloned() {
        let result = Arc::clone(&result);
        handles.push(thread::spawn(move || {
            let res = run_blueprint(&bp, time);
            let mut guard = result.lock().unwrap();
            *guard *= res;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *result.lock().unwrap();
    result
}

fn run_blueprint(bp: &Blueprint, time: u32) -> u32 {
    let mut max_geodes = 0;

    let mut states = VecDeque::from([StateWithPreviousUnbuiltRobots {
        state: State {
            time,

            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        },
        not_built: vec![],
    }]);
    let mut already_tested: HashSet<State> = HashSet::new();

    let cost_ore_in_ore = bp.costs[&Rock::Ore][&Rock::Ore];
    let cost_clay_in_ore = bp.costs[&Rock::Clay][&Rock::Ore];
    let cost_obsidian_in_ore = bp.costs[&Rock::Obsidian][&Rock::Ore];
    let cost_obsidian_in_clay = bp.costs[&Rock::Obsidian][&Rock::Clay];
    let cost_geode_in_ore = bp.costs[&Rock::Geode][&Rock::Ore];
    let cost_geode_in_obsidian = bp.costs[&Rock::Geode][&Rock::Obsidian];

    let max_ore_needed = bp.max_cost_per_rock(Rock::Ore).unwrap();
    let max_clay_needed = bp.max_cost_per_rock(Rock::Clay).unwrap();
    let max_obsidian_needed = bp.max_cost_per_rock(Rock::Obsidian).unwrap();

    while let Some(state) = states.pop_front() {
        let StateWithPreviousUnbuiltRobots {
            state:
                State {
                    mut time,
                    mut ore,
                    mut clay,
                    mut obsidian,
                    geode,
                    mut ore_robots,
                    mut clay_robots,
                    mut obsidian_robots,
                    geode_robots,
                },
            not_built,
        } = state;

        ore_robots = std::cmp::min(ore_robots, max_ore_needed);
        clay_robots = std::cmp::min(clay_robots, max_clay_needed);
        obsidian_robots = std::cmp::min(obsidian_robots, max_obsidian_needed);
        ore = std::cmp::min(ore, time * max_ore_needed - (ore_robots * (time - 1)));
        clay = std::cmp::min(clay, time * max_clay_needed - (clay_robots * (time - 1)));
        obsidian = std::cmp::min(
            obsidian,
            time * max_obsidian_needed - (obsidian_robots * (time - 1)),
        );
        time -= 1;

        if time == 0 {
            max_geodes = std::cmp::max(max_geodes, geode + geode_robots);
            continue;
        }

        let new_state = State {
            time,
            ore,
            clay,
            obsidian,
            geode,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots,
        };
        if already_tested.contains(&new_state) {
            continue;
        }
        already_tested.insert(new_state);

        let mut can_build = vec![];

        if ore >= cost_ore_in_ore && ore_robots < max_ore_needed {
            if !not_built.contains(&Rock::Ore) {
                can_build.push(Rock::Ore);
                states.push_back(StateWithPreviousUnbuiltRobots {
                    state: State {
                        time,
                        ore: ore + ore_robots - cost_ore_in_ore,
                        clay: clay + clay_robots,
                        obsidian: obsidian + obsidian_robots,
                        geode: geode + geode_robots,
                        ore_robots: ore_robots + 1,
                        clay_robots,
                        obsidian_robots,
                        geode_robots,
                    },
                    not_built: vec![],
                });
            }
        }

        if ore >= cost_clay_in_ore && clay_robots < max_clay_needed {
            if !not_built.contains(&Rock::Clay) {
                can_build.push(Rock::Clay);
                states.push_back(StateWithPreviousUnbuiltRobots {
                    state: State {
                        time,
                        ore: ore + ore_robots - cost_clay_in_ore,
                        clay: clay + clay_robots,
                        obsidian: obsidian + obsidian_robots,
                        geode: geode + geode_robots,
                        ore_robots,
                        clay_robots: clay_robots + 1,
                        obsidian_robots,
                        geode_robots,
                    },
                    not_built: vec![],
                });
            }
        }

        if ore >= cost_obsidian_in_ore
            && clay >= cost_obsidian_in_clay
            && obsidian_robots < max_obsidian_needed
        {
            if !not_built.contains(&Rock::Obsidian) {
                can_build.push(Rock::Obsidian);
                states.push_back(StateWithPreviousUnbuiltRobots {
                    state: State {
                        time,
                        ore: ore + ore_robots - cost_obsidian_in_ore,
                        clay: clay + clay_robots - cost_obsidian_in_clay,
                        obsidian: obsidian + obsidian_robots,
                        geode: geode + geode_robots,
                        ore_robots,
                        clay_robots,
                        obsidian_robots: obsidian_robots + 1,
                        geode_robots,
                    },
                    not_built: vec![],
                });
            }
        }

        if ore >= cost_geode_in_ore && obsidian >= cost_geode_in_obsidian {
            if !not_built.contains(&Rock::Geode) {
                can_build.push(Rock::Geode);
                states.push_back(StateWithPreviousUnbuiltRobots {
                    state: State {
                        time,
                        ore: ore + ore_robots - cost_geode_in_ore,
                        clay: clay + clay_robots,
                        obsidian: obsidian + obsidian_robots - cost_geode_in_obsidian,
                        geode: geode + geode_robots,
                        ore_robots,
                        clay_robots,
                        obsidian_robots,
                        geode_robots: geode_robots + 1,
                    },
                    not_built: vec![],
                });
            }
        }

        if ore < max_ore_needed
            || (clay_robots > 0 && clay < max_clay_needed)
            || (obsidian_robots > 0 && obsidian < max_obsidian_needed)
        {
            states.push_back(StateWithPreviousUnbuiltRobots {
                state: State {
                    time,
                    ore: ore + ore_robots,
                    clay: clay + clay_robots,
                    obsidian: obsidian + obsidian_robots,
                    geode: geode + geode_robots,
                    ore_robots,
                    clay_robots,
                    obsidian_robots,
                    geode_robots,
                },
                not_built: can_build,
            });
        }
    }

    max_geodes
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_part2() {
        let blueprints = parse_input(example);
        assert_eq!(part2(&blueprints), 3472);
    }
}
