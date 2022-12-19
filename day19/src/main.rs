use std::{io::Error, collections::{HashSet, VecDeque}, str::FromStr};

#[derive(PartialEq, Debug, Copy, Clone)]
struct Blueprint{
    id: i32,
    ore_robot_cost: i32,
    clay_robot_cost: i32,
    obsidian_robot_cost: (i32,i32),
    geode_robot_cost: (i32,i32),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseBluePrintError;

impl FromStr for Blueprint {
    type Err = ParseBluePrintError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s.split(' ').collect::<Vec<_>>();
        Ok(Self{ id: fields[1].strip_suffix(':').unwrap().parse().unwrap(),
            ore_robot_cost: fields[6].parse().unwrap(),
            clay_robot_cost: fields[12].parse().unwrap(),
            obsidian_robot_cost: (fields[18].parse().unwrap(), fields[21].parse().unwrap()),
            geode_robot_cost: (fields[27].parse().unwrap(), fields[30].parse().unwrap()),
            })
    }

}

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
struct GameState {
    ore: i32,
    ore_robots: i32,
    clay: i32,
    clay_robots: i32,
    obsidian: i32,
    obsidian_robots: i32,
    geodes: i32,
    geode_robots: i32,
    time: i32
}

impl GameState {
    fn new() -> Self{
        Self {
            ore:0,
            clay:0,
            obsidian:0,
            geodes: 0,
            ore_robots:1,
            clay_robots:0,
            geode_robots:0,
            obsidian_robots:0,
            time:0
        }
    }

    fn robots_dig(&mut self)  {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
        self.time +=1;
    }

}


fn set_robots_to_work(blueprint: &Blueprint, time_limit: i32) -> i32 {
    let mut unseen = VecDeque::new();
    let starting_state = GameState::new();
    let mut best_geodes = 0;

    unseen.push_back(starting_state);

    let mut seen_states = HashSet::new();

    // Don't make more ore robots than can the max ore cost of any robot
    let max_ore_cost = *[blueprint.ore_robot_cost, 
                              blueprint.clay_robot_cost, 
                              blueprint.obsidian_robot_cost.0, 
                              blueprint.geode_robot_cost.0].iter().max().unwrap();

    while let Some(mut state) = unseen.pop_front() {
        best_geodes = best_geodes.max(state.geodes);
        let mut cacheable_state = state;
        cacheable_state.time = 0;
        if state.geodes < best_geodes -1  || seen_states.contains(&cacheable_state) || state.time == time_limit {
            continue;
        };

        seen_states.insert(cacheable_state);

        if state.ore >= blueprint.geode_robot_cost.0 && state.obsidian >= blueprint.geode_robot_cost.1 {
            let mut next_state = state;
            next_state.ore -= blueprint.geode_robot_cost.0;
            next_state.obsidian -= blueprint.geode_robot_cost.1;
            next_state.robots_dig();
            next_state.geode_robots +=1;
            unseen.push_back(next_state);
        } else {
            if state.ore >= blueprint.ore_robot_cost && state.ore_robots < max_ore_cost {
                let mut next_state = state;
                next_state.ore -= blueprint.ore_robot_cost;
                next_state.robots_dig();
                next_state.ore_robots +=1;
                unseen.push_back(next_state);
            }
            if state.ore >= blueprint.clay_robot_cost  && state.clay_robots < blueprint.obsidian_robot_cost.1 {
                let mut next_state = state;
                next_state.ore -= blueprint.clay_robot_cost;
                next_state.robots_dig();
                next_state.clay_robots +=1;
                unseen.push_back(next_state);
            }
            if state.ore >= blueprint.obsidian_robot_cost.0 &&  state.clay >= blueprint.obsidian_robot_cost.1 {
                let mut next_state = state;
                next_state.ore -= blueprint.obsidian_robot_cost.0;
                next_state.clay -= blueprint.obsidian_robot_cost.1;
                next_state.robots_dig();
                next_state.obsidian_robots +=1;
                unseen.push_back(next_state);
            }
            state.robots_dig();
            unseen.push_back(state);
        }
    }

    best_geodes
}

fn main() -> Result<(), Error> {
    let data = std::fs::read_to_string("./day19/input.txt")?;
    let mut total = 0;
    let blueprints = data.lines().map(|d| d.parse::<Blueprint>().unwrap()).collect::<Vec<_>>();
    for blueprint in blueprints.iter() {
        total += set_robots_to_work(blueprint, 24) * blueprint.id;    
    }
    println!("Part 1: {}", total);
    
    let mut total = 1;
    for blueprint in &blueprints[0..3] {
        total *= set_robots_to_work(blueprint, 32);
    }
    println!("Part 2: {}", total);

    Ok(())
}
